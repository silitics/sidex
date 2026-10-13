//! Encoding typed values as freeform values.

use base64::Engine;

use super::Tagging;
use super::repr::BASE64;
use super::repr::non_finite;
use super::repr::with_display;
use crate::Value;
use crate::encode::EncodeError;
use crate::encode::MapView;
use crate::encode::RecordView;
use crate::encode::SeqView;
use crate::encode::VariantView;
use crate::encode::View;
use crate::formats::view::ValueMapView;
use crate::formats::view::ValueSeqView;
use crate::formats::view::ValueView;
use crate::shape::RecordShape;
use crate::types::Map;
use crate::types::Str;

/// Callback receiving a view of a freeform value.
pub(super) type ViewFn<'f> = dyn FnMut(ValueView<'_>) -> Result<(), EncodeError> + 'f;

/// Callback receiving an entry of a map.
type EntryFn<'f> = dyn FnMut(&str, ValueView<'_>) -> Result<(), EncodeError> + 'f;

/// Maps a typed view to a freeform view and passes it to `f`.
pub(super) fn view(view: &View<'_>, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
    match view {
        View::Unit => f(ValueView::Null),
        View::Bool(value) => f(ValueView::Bool(*value)),
        View::I8(value) => f(ValueView::Int((*value).into())),
        View::I16(value) => f(ValueView::Int((*value).into())),
        View::I32(value) => f(ValueView::Int((*value).into())),
        View::U8(value) => f(ValueView::Int((*value).into())),
        View::U16(value) => f(ValueView::Int((*value).into())),
        View::U32(value) => f(ValueView::Int((*value).into())),
        View::Idx(value) => {
            f(ValueView::Int(
                i128::try_from(*value).expect("every `usize` fits into `i128`"),
            ))
        }
        View::I64(value) => int64((*value).into(), f),
        View::U64(value) => int64((*value).into(), f),
        View::F32(value) => float32(*value, f),
        View::F64(value) => float64(*value, f),
        View::String(value) => f(ValueView::String(value)),
        View::Bytes(value) => f(ValueView::String(&BASE64.encode(value))),
        View::Uuid(value) => with_display(value, |text| f(ValueView::String(text))),
        View::Timestamp(value) => with_display(value, |text| f(ValueView::String(text))),
        View::Date(value) => with_display(value, |text| f(ValueView::String(text))),
        View::Seq(seq) => f(ValueView::Seq(&MappedSeq(*seq))),
        View::Map(map) => f(ValueView::Map(&MappedMap(*map))),
        View::Record(record) => f(ValueView::Map(&MappedRecord::new(*record, None)?)),
        View::Variant(variant) => self::variant(*variant, f),
        View::Value(value) => self::value(value, f),
    }
}

/// Largest magnitude up to which JavaScript represents all integers exactly.
const MAX_SAFE_INTEGER: u128 = (1 << 53) - 1;

/// Maps a 64-bit integer to a number if JavaScript represents it exactly and to
/// a decimal string otherwise.
fn int64(value: i128, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
    if value.unsigned_abs() <= MAX_SAFE_INTEGER {
        f(ValueView::Int(value))
    } else {
        with_display(value, |text| f(ValueView::String(text)))
    }
}

fn float64(value: f64, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
    if value.is_finite() {
        f(ValueView::Float(value))
    } else {
        f(ValueView::String(non_finite(value)))
    }
}

fn float32(value: f32, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
    if !value.is_finite() {
        return f(ValueView::String(non_finite(value.into())));
    }
    // Converting with `as` would turn `0.1` into `0.10000000149011612`. Going
    // through the shortest decimal of the `f32` keeps `0.1`, which denotes the
    // same `f32` when read back.
    let value = with_display(value, |text| text.parse::<f64>())
        .expect("finite floats format as parsable decimals");
    f(ValueView::Float(value))
}

fn variant(variant: &dyn VariantView, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
    let shape = variant.shape();
    let variant_shape = shape
        .as_variant()
        .ok_or_else(|| EncodeError::new(format_args!("shape `{shape}` is not a variant")))?;
    let case = variant_shape.cases.get(variant.case()).ok_or_else(|| {
        EncodeError::new(format_args!(
            "case {} of `{shape}` does not exist",
            variant.case()
        ))
    })?;
    let name = case.json.name;
    let tag = match variant_shape.json {
        Tagging::Internal { tag, .. } | Tagging::Adjacent { tag, .. } => tag,
        Tagging::External => {
            return match variant.payload() {
                Some(payload) => {
                    f(ValueView::Map(&Entries {
                        tag: None,
                        content: Some((name, payload)),
                    }))
                }
                None => f(ValueView::String(name)),
            };
        }
        Tagging::Implicit => {
            return match variant.payload() {
                Some(payload) => self::view(&payload, f),
                None => f(ValueView::String(name)),
            };
        }
    };
    let (Some(payload), Some(payload_shape)) = (variant.payload(), case.payload) else {
        return f(ValueView::Map(&Entries {
            tag: Some((tag, name)),
            content: None,
        }));
    };
    match (
        case.json.content_field(variant_shape.json, payload_shape()),
        payload,
    ) {
        (Some(content), payload) => {
            f(ValueView::Map(&Entries {
                tag: Some((tag, name)),
                content: Some((content, payload)),
            }))
        }
        (None, View::Record(record)) => {
            f(ValueView::Map(&MappedRecord::new(
                record,
                Some((tag, name)),
            )?))
        }
        (None, _) => {
            Err(EncodeError::new(format_args!(
                "payload of case `{name}` of `{shape}` is not a record"
            )))
        }
    }
}

fn value(value: &Value, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
    match value {
        Value::Null => f(ValueView::Null),
        Value::Bool(value) => f(ValueView::Bool(*value)),
        Value::Int(value) => f(ValueView::Int(*value)),
        Value::Float(value) if value.is_finite() => f(ValueView::Float(*value)),
        Value::Float(value) => {
            Err(EncodeError::new(format_args!(
                "freeform float {value} has no JSON representation"
            )))
        }
        Value::String(value) => f(ValueView::String(value)),
        Value::Bytes(value) => f(ValueView::String(&BASE64.encode(value))),
        Value::Seq(items) => f(ValueView::Seq(&ValueSeq(items))),
        Value::Map(entries) => f(ValueView::Map(&ValueMap(entries))),
    }
}

/// Passes the text of a map key to `f`.
fn key(
    key: &View<'_>,
    f: &mut dyn FnMut(&str) -> Result<(), EncodeError>,
) -> Result<(), EncodeError> {
    match key {
        View::String(key) => f(key),
        View::I8(key) => with_display(key, f),
        View::I16(key) => with_display(key, f),
        View::I32(key) => with_display(key, f),
        View::I64(key) => with_display(key, f),
        View::U8(key) => with_display(key, f),
        View::U16(key) => with_display(key, f),
        View::U32(key) => with_display(key, f),
        View::U64(key) => with_display(key, f),
        View::Idx(key) => with_display(key, f),
        View::Uuid(key) => with_display(key, f),
        View::Timestamp(key) => with_display(key, f),
        View::Date(key) => with_display(key, f),
        View::Value(key) => {
            match key.as_ref() {
                Value::String(key) => f(key),
                Value::Int(key) => with_display(key, f),
                _ => Err(EncodeError::new("map keys must be strings or integers")),
            }
        }
        _ => {
            Err(EncodeError::new(
                "map keys must be strings, integers, UUIDs, timestamps, or dates",
            ))
        }
    }
}

/// Sequence whose items are mapped with [`view`].
struct MappedSeq<'a>(&'a dyn SeqView);

impl ValueSeqView for MappedSeq<'_> {
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn each(&self, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
        self.0.each(&mut |item| view(&item, f))
    }
}

/// Map whose values are mapped with [`view`].
struct MappedMap<'a>(&'a dyn MapView);

impl ValueMapView for MappedMap<'_> {
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn each(&self, f: &mut EntryFn<'_>) -> Result<(), EncodeError> {
        self.0.each(&mut |map_key, map_value| {
            key(&map_key, &mut |text| {
                view(&map_value, &mut |item| f(text, item))
            })
        })
    }
}

/// Record mapped to an object, optionally starting with a tag field.
struct MappedRecord<'a> {
    record: &'a dyn RecordView,
    shape: &'static RecordShape,
    tag: Option<(&'static str, &'static str)>,
}

impl<'a> MappedRecord<'a> {
    fn new(
        record: &'a dyn RecordView,
        tag: Option<(&'static str, &'static str)>,
    ) -> Result<Self, EncodeError> {
        let shape = record.shape();
        let record_shape = shape
            .as_record()
            .ok_or_else(|| EncodeError::new(format_args!("shape `{shape}` is not a record")))?;
        Ok(Self {
            record,
            shape: record_shape,
            tag,
        })
    }

    /// Passes the entries of the record's fields to `f`.
    fn fields(&self, f: &mut EntryFn<'_>) -> Result<(), EncodeError> {
        for (index, field) in self.shape.fields.iter().enumerate() {
            let Some(value) = self.record.field(index) else {
                continue;
            };
            if field.json.inline {
                let View::Record(inner) = value else {
                    return Err(EncodeError::new(format_args!(
                        "inline field `{}` is not a record",
                        field.name
                    )));
                };
                MappedRecord::new(inner, None)?.fields(f)?;
            } else {
                view(&value, &mut |item| f(field.json.name, item))?;
            }
        }
        Ok(())
    }
}

impl ValueMapView for MappedRecord<'_> {
    fn size_hint(&self) -> Option<usize> {
        None
    }

    fn each(&self, f: &mut EntryFn<'_>) -> Result<(), EncodeError> {
        if let Some((tag, name)) = self.tag {
            f(tag, ValueView::String(name))?;
        }
        self.fields(f)
    }
}

/// Object with an optional tag field and an optional content field.
struct Entries<'a> {
    tag: Option<(&'static str, &'static str)>,
    content: Option<(&'static str, View<'a>)>,
}

impl ValueMapView for Entries<'_> {
    fn size_hint(&self) -> Option<usize> {
        Some(usize::from(self.tag.is_some()) + usize::from(self.content.is_some()))
    }

    fn each(&self, f: &mut EntryFn<'_>) -> Result<(), EncodeError> {
        if let Some((tag, name)) = self.tag {
            f(tag, ValueView::String(name))?;
        }
        if let Some((content, payload)) = &self.content {
            view(payload, &mut |item| f(content, item))?;
        }
        Ok(())
    }
}

struct ValueSeq<'a>(&'a [Value]);

impl ValueSeqView for ValueSeq<'_> {
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn each(&self, f: &mut ViewFn<'_>) -> Result<(), EncodeError> {
        self.0.iter().try_for_each(|item| value(item, f))
    }
}

struct ValueMap<'a>(&'a Map<Str, Value>);

impl ValueMapView for ValueMap<'_> {
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn each(&self, f: &mut EntryFn<'_>) -> Result<(), EncodeError> {
        self.0
            .iter()
            .try_for_each(|(key, item)| value(item, &mut |item| f(key, item)))
    }
}
