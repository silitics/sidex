//! Decoding typed values from JSON values.

use std::cell::RefCell;
use std::str::FromStr;

use base64::Engine;

use super::Tagging;
use super::repr::BASE64;
use super::repr::parse_non_finite;
use crate::JsonValue;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::shape::CaseShape;
use crate::shape::FieldShape;
use crate::shape::RecordShape;
use crate::shape::Shape;
use crate::shape::ShapeKind;
use crate::types::Date;
use crate::types::Map;
use crate::types::Str;
use crate::types::Timestamp;
use crate::types::Uuid;
use crate::value::Path;
use crate::value::PathSegment;

/// Callback receiving the index of a field or case and its decoder.
type IndexFn<'f> = dyn FnMut(usize, &mut dyn Decoder) -> Result<(), DecodeError> + 'f;

/// [`Decoder`] reading the JSON representation from a JSON value.
pub(super) struct JsonDecoder<'v, 'u> {
    item: Item<'v>,
    /// Handling of fields unknown to records.
    unknown: Unknown<'u>,
    /// Keys of the item, if it is a map, that belong to someone else.
    claimed: Claimed,
}

/// Handling of fields unknown to records.
#[derive(Clone, Copy)]
pub(super) enum Unknown<'u> {
    /// Ignore unknown fields.
    Ignore,
    /// Reject unknown fields.
    Reject,
    /// Collect the paths of unknown fields.
    Collect(&'u RefCell<Vec<Path>>),
}

impl Unknown<'_> {
    /// Returns a mark for the paths that a nested item collects.
    fn mark(self) -> usize {
        match self {
            Unknown::Collect(paths) => paths.borrow().len(),
            Unknown::Ignore | Unknown::Reject => 0,
        }
    }

    /// Prepends `segment` to the paths collected since `mark`.
    fn within(self, mark: usize, segment: &PathSegment) {
        if let Unknown::Collect(paths) = self {
            for path in &mut paths.borrow_mut()[mark..] {
                path.prepend(segment.clone());
            }
        }
    }

    /// Discards the paths collected since `mark`.
    fn discard(self, mark: usize) {
        if let Unknown::Collect(paths) = self {
            paths.borrow_mut().truncate(mark);
        }
    }

    /// Decodes a nested item with `f` and attributes errors and unknown fields
    /// to `segment`.
    fn nested(
        self,
        segment: PathSegment,
        f: impl FnOnce() -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError> {
        let mark = self.mark();
        match f() {
            Ok(()) => {
                self.within(mark, &segment);
                Ok(())
            }
            Err(error) => Err(error.within(segment)),
        }
    }
}

/// Keys of a map that do not belong to the record decoded from it.
#[derive(Clone, Copy)]
enum Claimed {
    /// All keys belong to the record.
    None,
    /// The tag of an internally tagged variant shares the map.
    Tag(&'static str),
    /// The map belongs to an enclosing record, which checks its keys.
    All,
}

/// Item to decode.
#[derive(Clone, Copy)]
enum Item<'v> {
    /// Value.
    Value(&'v JsonValue),
    /// Key of a map, which represents integers as strings.
    Key(&'v str),
}

impl<'v, 'u> JsonDecoder<'v, 'u> {
    pub(super) fn new(value: &'v JsonValue, unknown: Unknown<'u>) -> Self {
        Self {
            item: Item::Value(value),
            unknown,
            claimed: Claimed::None,
        }
    }

    fn key(key: &'v str) -> Self {
        Self {
            item: Item::Key(key),
            unknown: Unknown::Ignore,
            claimed: Claimed::None,
        }
    }

    /// Returns a decoder for a nested value.
    fn child<'c>(&self, value: &'c JsonValue) -> JsonDecoder<'c, 'u> {
        JsonDecoder::new(value, self.unknown)
    }

    /// Returns the item if it is a value.
    fn as_value(&self) -> Option<&'v JsonValue> {
        match self.item {
            Item::Value(value) => Some(value),
            Item::Key(_) => None,
        }
    }

    /// Returns the text of the item if it is a string.
    fn text(&self) -> Option<&'v str> {
        match self.item {
            Item::Value(JsonValue::String(text)) => Some(text),
            Item::Key(text) => Some(text),
            Item::Value(_) => None,
        }
    }

    fn unexpected(&self, expected: &str) -> DecodeError {
        let found = match self.item {
            Item::Value(value) => value.kind(),
            Item::Key(_) => "a string",
        };
        DecodeError::unexpected(found, expected)
    }

    /// Decodes an integer, which is also accepted as a decimal string if
    /// `from_string` is set or the item is a key.
    fn int<T: TryFrom<i128> + FromStr>(
        &self,
        expected: &'static str,
        from_string: bool,
    ) -> Result<T, DecodeError> {
        match self.item {
            Item::Value(JsonValue::Int(value)) => {
                T::try_from(*value)
                    .map_err(|_| DecodeError::unexpected(&format!("`{value}`"), expected))
            }
            Item::Value(JsonValue::String(text)) if from_string => parse(text, expected),
            Item::Key(text) => parse(text, expected),
            _ => Err(self.unexpected(expected)),
        }
    }

    fn float(&self) -> Result<f64, DecodeError> {
        const EXPECTED: &str = "a number or one of `NaN`, `+Infinity`, and `-Infinity`";
        match self.item {
            Item::Value(JsonValue::Float(value)) => Ok(*value),
            // Precision loss is inherent to floats.
            Item::Value(JsonValue::Int(value)) => Ok(*value as f64),
            Item::Value(JsonValue::String(text)) => {
                parse_non_finite(text)
                    .ok_or_else(|| DecodeError::unexpected(&format!("`{text}`"), EXPECTED))
            }
            _ => Err(self.unexpected(EXPECTED)),
        }
    }

    /// Decodes a type from its text representation.
    fn parse<T: FromStr>(&self, expected: &'static str) -> Result<T, DecodeError> {
        match self.text() {
            Some(text) => parse(text, expected),
            None => Err(self.unexpected(expected)),
        }
    }
}

impl Decoder for JsonDecoder<'_, '_> {
    fn bool(&mut self) -> Result<bool, DecodeError> {
        match self.as_value() {
            Some(JsonValue::Bool(value)) => Ok(*value),
            _ => Err(self.unexpected("a boolean")),
        }
    }

    fn i8(&mut self) -> Result<i8, DecodeError> {
        self.int("an 8-bit signed integer", false)
    }

    fn i16(&mut self) -> Result<i16, DecodeError> {
        self.int("a 16-bit signed integer", false)
    }

    fn i32(&mut self) -> Result<i32, DecodeError> {
        self.int("a 32-bit signed integer", false)
    }

    fn i64(&mut self) -> Result<i64, DecodeError> {
        self.int("a 64-bit signed integer", true)
    }

    fn u8(&mut self) -> Result<u8, DecodeError> {
        self.int("an 8-bit unsigned integer", false)
    }

    fn u16(&mut self) -> Result<u16, DecodeError> {
        self.int("a 16-bit unsigned integer", false)
    }

    fn u32(&mut self) -> Result<u32, DecodeError> {
        self.int("a 32-bit unsigned integer", false)
    }

    fn u64(&mut self) -> Result<u64, DecodeError> {
        self.int("a 64-bit unsigned integer", true)
    }

    fn idx(&mut self) -> Result<usize, DecodeError> {
        self.int("an index", false)
    }

    fn f32(&mut self) -> Result<f32, DecodeError> {
        self.float().map(|value| value as f32)
    }

    fn f64(&mut self) -> Result<f64, DecodeError> {
        self.float()
    }

    fn string(&mut self) -> Result<String, DecodeError> {
        match self.text() {
            Some(text) => Ok(text.to_owned()),
            None => Err(self.unexpected("a string")),
        }
    }

    fn bytes(&mut self) -> Result<Vec<u8>, DecodeError> {
        const EXPECTED: &str = "a Base64 string";
        let text = self.text().ok_or_else(|| self.unexpected(EXPECTED))?;
        BASE64
            .decode(text)
            .map_err(|_| DecodeError::unexpected("invalid Base64", EXPECTED))
    }

    fn uuid(&mut self) -> Result<Uuid, DecodeError> {
        self.parse("a UUID")
    }

    fn timestamp(&mut self) -> Result<Timestamp, DecodeError> {
        self.parse("an RFC 3339 timestamp")
    }

    fn date(&mut self) -> Result<Date, DecodeError> {
        self.parse("a date")
    }

    fn seq(
        &mut self,
        f: &mut dyn FnMut(&mut dyn Decoder) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError> {
        let Some(JsonValue::Seq(items)) = self.as_value() else {
            return Err(self.unexpected("a sequence"));
        };
        for (index, item) in items.iter().enumerate() {
            self.unknown
                .nested(PathSegment::Index(index), || f(&mut self.child(item)))?;
        }
        Ok(())
    }

    fn map(
        &mut self,
        f: &mut dyn FnMut(&mut dyn Decoder, &mut dyn Decoder) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError> {
        let Some(JsonValue::Map(entries)) = self.as_value() else {
            return Err(self.unexpected("a map"));
        };
        for (key, value) in entries {
            self.unknown.nested(PathSegment::Key(key.clone()), || {
                f(&mut JsonDecoder::key(key), &mut self.child(value))
            })?;
        }
        Ok(())
    }

    fn record(&mut self, shape: &'static Shape, f: &mut IndexFn<'_>) -> Result<(), DecodeError> {
        let record = shape
            .as_record()
            .ok_or_else(|| DecodeError::new(format_args!("shape `{shape}` is not a record")))?;
        let Some(map @ JsonValue::Map(entries)) = self.as_value() else {
            return Err(self.unexpected("a map"));
        };
        for (index, field) in record.fields.iter().enumerate() {
            // Inline fields read their fields from the same map.
            if field.json.inline {
                let mut decoder = JsonDecoder {
                    claimed: Claimed::All,
                    ..self.child(map)
                };
                f(index, &mut decoder)?;
                continue;
            }
            let name = field.json.name;
            match entries.get(name) {
                None if field.optional => {}
                Some(JsonValue::Null) if field.optional && !admits_null(field) => {}
                None => return Err(missing_field(name)),
                Some(value) => {
                    self.unknown
                        .nested(PathSegment::Key(Str::from_static(name)), || {
                            f(index, &mut self.child(value))
                        })?
                }
            }
        }
        let claimed = match self.claimed {
            Claimed::None => None,
            Claimed::Tag(tag) => Some(tag),
            Claimed::All => return Ok(()),
        };
        match self.unknown {
            Unknown::Ignore => {}
            Unknown::Reject => {
                if let Some(key) = unknown_fields(entries, record, claimed).next() {
                    return Err(DecodeError::unknown_field(key));
                }
            }
            Unknown::Collect(paths) => {
                paths.borrow_mut().extend(
                    unknown_fields(entries, record, claimed)
                        .map(|key| Path::from_iter([PathSegment::Key(key.clone())])),
                );
            }
        }
        Ok(())
    }

    fn variant(&mut self, shape: &'static Shape, f: &mut IndexFn<'_>) -> Result<(), DecodeError> {
        let variant = shape
            .as_variant()
            .ok_or_else(|| DecodeError::new(format_args!("shape `{shape}` is not a variant")))?;
        let value = self
            .as_value()
            .ok_or_else(|| self.unexpected("a variant"))?;
        match variant.json {
            Tagging::External => externally_tagged(value, shape, self.unknown, f),
            Tagging::Implicit => implicitly_tagged(value, shape, self.unknown, f),
            Tagging::Internal { tag, .. } | Tagging::Adjacent { tag, .. } => {
                tagged(value, shape, variant.json, tag, self.unknown, f)
            }
        }
    }

    fn json(&mut self) -> Result<JsonValue, DecodeError> {
        Ok(match self.item {
            Item::Value(value) => value.clone(),
            Item::Key(key) => JsonValue::String(key.into()),
        })
    }
}

fn externally_tagged(
    value: &JsonValue,
    shape: &'static Shape,
    unknown: Unknown<'_>,
    f: &mut IndexFn<'_>,
) -> Result<(), DecodeError> {
    const EXPECTED: &str = "a string or a map with a single entry";
    match value {
        JsonValue::String(name) => {
            let (index, case) = find_case(shape, name)?;
            if case.payload.is_some() {
                return Err(DecodeError::new(format_args!(
                    "case `{name}` requires a payload"
                )));
            }
            f(index, &mut JsonDecoder::new(&JsonValue::Null, unknown))
        }
        JsonValue::Map(entries) if entries.len() == 1 => {
            let (name, payload) = entries.iter().next().expect("the map has one entry");
            let (index, case) = find_case(shape, name)?;
            if case.payload.is_none() {
                return Err(DecodeError::new(format_args!(
                    "case `{name}` has no payload and must be a string"
                )));
            }
            unknown.nested(PathSegment::Key(name.clone()), || {
                f(index, &mut JsonDecoder::new(payload, unknown))
            })
        }
        _ => Err(DecodeError::unexpected(value.kind(), EXPECTED)),
    }
}

/// Decodes the first case in declaration order that matches.
fn implicitly_tagged(
    value: &JsonValue,
    shape: &'static Shape,
    unknown: Unknown<'_>,
    f: &mut IndexFn<'_>,
) -> Result<(), DecodeError> {
    let variant = shape.as_variant().expect("callers check the shape");
    let mut failures = Vec::new();
    for (index, case) in variant.cases.iter().enumerate() {
        if case.payload.is_none() {
            if matches!(value, JsonValue::String(name) if name == case.json.name) {
                return f(index, &mut JsonDecoder::new(&JsonValue::Null, unknown));
            }
            continue;
        }
        let mark = unknown.mark();
        match f(index, &mut JsonDecoder::new(value, unknown)) {
            Ok(()) => return Ok(()),
            Err(error) => {
                // Unknown fields of cases that do not match are irrelevant.
                unknown.discard(mark);
                failures.push(format!("`{}`: {error}", case.json.name));
            }
        }
    }
    Err(DecodeError::new(format_args!(
        "no case of `{shape}` matches {} ({})",
        value.kind(),
        failures.join("; ")
    )))
}

/// Decodes an internally or adjacently tagged variant.
fn tagged(
    value: &JsonValue,
    shape: &'static Shape,
    tagging: Tagging,
    tag: &'static str,
    unknown: Unknown<'_>,
    f: &mut IndexFn<'_>,
) -> Result<(), DecodeError> {
    let JsonValue::Map(entries) = value else {
        return Err(DecodeError::unexpected(value.kind(), "a map"));
    };
    let name = match entries.get(tag) {
        Some(JsonValue::String(name)) => name,
        Some(other) => {
            return Err(DecodeError::unexpected(other.kind(), "a string")
                .within(PathSegment::Key(Str::from(tag))));
        }
        None => return Err(missing_field(tag)),
    };
    let (index, case) =
        find_case(shape, name).map_err(|error| error.within(PathSegment::Key(Str::from(tag))))?;
    let Some(payload_shape) = case.payload else {
        return f(index, &mut JsonDecoder::new(&JsonValue::Null, unknown));
    };
    match case.json.content_field(tagging, payload_shape()) {
        Some(content) => {
            let payload = entries.get(content).ok_or_else(|| missing_field(content))?;
            unknown.nested(PathSegment::Key(Str::from_static(content)), || {
                f(index, &mut JsonDecoder::new(payload, unknown))
            })
        }
        // The record's fields share the map with the tag.
        None => {
            let mut decoder = JsonDecoder {
                claimed: Claimed::Tag(tag),
                ..JsonDecoder::new(value, unknown)
            };
            f(index, &mut decoder)
        }
    }
}

/// Returns the keys of the map that are not fields of the record, except for
/// the claimed key.
fn unknown_fields<'e>(
    entries: &'e Map<Str, JsonValue>,
    record: &RecordShape,
    claimed: Option<&'static str>,
) -> impl Iterator<Item = &'e Str> {
    let mut known = Vec::new();
    // Inline fields that are not records may consume any key.
    let checked = known_fields(record, &mut known);
    entries.keys().filter(move |key| {
        checked && Some(key.as_str()) != claimed && !known.contains(&key.as_str())
    })
}

/// Collects the JSON names of the record's fields, including those of inline
/// records. Returns `false` if an inline field is not a record.
fn known_fields(record: &RecordShape, known: &mut Vec<&'static str>) -> bool {
    for field in record.fields {
        if !field.json.inline {
            known.push(field.json.name);
            continue;
        }
        match (field.shape)().as_record() {
            Some(inner) if known_fields(inner, known) => {}
            _ => return false,
        }
    }
    true
}

/// Indicates whether `null` may represent a value of the field's type.
///
/// Optional fields accept `null` as absent, unless `null` is a value. JSON
/// values include `null`, but many languages cannot tell an absent value from
/// `null`, so optional JSON value fields treat `null` as absent.
fn admits_null(field: &FieldShape) -> bool {
    fn shape_admits_null(shape: &Shape) -> bool {
        match &shape.kind {
            ShapeKind::Scalar(_) => false,
            ShapeKind::Wrapper(inner) => shape_admits_null(inner()),
            ShapeKind::Variant(variant) => variant.json == Tagging::Implicit,
            ShapeKind::Opaque(opaque) => opaque.json.admits(super::Type::Null),
            ShapeKind::Json => false,
            ShapeKind::Sequence(_) | ShapeKind::Map(..) | ShapeKind::Record(_) => false,
        }
    }
    shape_admits_null((field.shape)())
}

/// Finds the case with the given JSON name.
fn find_case(
    shape: &'static Shape,
    name: &str,
) -> Result<(usize, &'static CaseShape), DecodeError> {
    let variant = shape.as_variant().expect("callers check the shape");
    variant
        .cases
        .iter()
        .enumerate()
        .find(|(_, case)| case.json.name == name)
        .ok_or_else(|| DecodeError::new(format_args!("unknown case `{name}` of `{shape}`")))
}

fn parse<T: FromStr>(text: &str, expected: &'static str) -> Result<T, DecodeError> {
    text.parse()
        .map_err(|_| DecodeError::unexpected(&format!("`{text}`"), expected))
}

fn missing_field(name: &str) -> DecodeError {
    DecodeError::new(format_args!("missing field `{name}`"))
}
