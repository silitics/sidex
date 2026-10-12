//! Decoding typed values from freeform values.

use std::str::FromStr;

use base64::Engine;

use super::Tagging;
use super::repr::BASE64;
use super::repr::parse_non_finite;
use crate::Value;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::shape::CaseShape;
use crate::shape::FieldShape;
use crate::shape::RecordShape;
use crate::shape::Scalar;
use crate::shape::Shape;
use crate::shape::ShapeKind;
use crate::types::Date;
use crate::types::Map;
use crate::types::Str;
use crate::types::Timestamp;
use crate::types::Uuid;
use crate::value::PathSegment;

/// Callback receiving the index of a field or case and its decoder.
type IndexFn<'f> = dyn FnMut(usize, &mut dyn Decoder) -> Result<(), DecodeError> + 'f;

/// [`Decoder`] reading the JSON representation from a freeform value.
pub(super) struct JsonDecoder<'v> {
    item: Item<'v>,
    /// Indicates whether records reject unknown fields.
    strict: bool,
    /// Keys of the item, if it is a map, that belong to someone else.
    claimed: Claimed,
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
    Value(&'v Value),
    /// Key of a map, which represents integers as strings.
    Key(&'v str),
}

impl<'v> JsonDecoder<'v> {
    pub(super) fn new(value: &'v Value, strict: bool) -> Self {
        Self {
            item: Item::Value(value),
            strict,
            claimed: Claimed::None,
        }
    }

    fn key(key: &'v str) -> Self {
        Self {
            item: Item::Key(key),
            strict: false,
            claimed: Claimed::None,
        }
    }

    /// Returns a decoder for a nested value.
    fn child<'c>(&self, value: &'c Value) -> JsonDecoder<'c> {
        JsonDecoder::new(value, self.strict)
    }

    /// Returns the item if it is a value.
    fn as_value(&self) -> Option<&'v Value> {
        match self.item {
            Item::Value(value) => Some(value),
            Item::Key(_) => None,
        }
    }

    /// Returns the text of the item if it is a string.
    fn text(&self) -> Option<&'v str> {
        match self.item {
            Item::Value(Value::String(text)) => Some(text),
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
            Item::Value(Value::Int(value)) => {
                T::try_from(*value)
                    .map_err(|_| DecodeError::unexpected(&format!("`{value}`"), expected))
            }
            Item::Value(Value::String(text)) if from_string => parse(text, expected),
            Item::Key(text) => parse(text, expected),
            _ => Err(self.unexpected(expected)),
        }
    }

    fn float(&self) -> Result<f64, DecodeError> {
        const EXPECTED: &str = "a number or one of `NaN`, `Infinity`, and `-Infinity`";
        match self.item {
            Item::Value(Value::Float(value)) => Ok(*value),
            // Precision loss is inherent to floats.
            Item::Value(Value::Int(value)) => Ok(*value as f64),
            Item::Value(Value::String(text)) => {
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

impl Decoder for JsonDecoder<'_> {
    fn unit(&mut self) -> Result<(), DecodeError> {
        match self.as_value() {
            Some(Value::Null) => Ok(()),
            _ => Err(self.unexpected("null")),
        }
    }

    fn bool(&mut self) -> Result<bool, DecodeError> {
        match self.as_value() {
            Some(Value::Bool(value)) => Ok(*value),
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
        match self.item {
            Item::Value(Value::Bytes(bytes)) => Ok(bytes.to_vec()),
            _ => {
                let text = self.text().ok_or_else(|| self.unexpected(EXPECTED))?;
                BASE64
                    .decode(text)
                    .map_err(|_| DecodeError::unexpected("invalid Base64", EXPECTED))
            }
        }
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
        let Some(Value::Seq(items)) = self.as_value() else {
            return Err(self.unexpected("a sequence"));
        };
        for (index, item) in items.iter().enumerate() {
            f(&mut self.child(item)).map_err(|error| error.within(PathSegment::Index(index)))?;
        }
        Ok(())
    }

    fn map(
        &mut self,
        f: &mut dyn FnMut(&mut dyn Decoder, &mut dyn Decoder) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError> {
        let Some(Value::Map(entries)) = self.as_value() else {
            return Err(self.unexpected("a map"));
        };
        for (key, value) in entries {
            f(&mut JsonDecoder::key(key), &mut self.child(value))
                .map_err(|error| error.within(PathSegment::Key(key.clone())))?;
        }
        Ok(())
    }

    fn record(&mut self, shape: &'static Shape, f: &mut IndexFn<'_>) -> Result<(), DecodeError> {
        let record = shape
            .as_record()
            .ok_or_else(|| DecodeError::new(format_args!("shape `{shape}` is not a record")))?;
        let Some(map @ Value::Map(entries)) = self.as_value() else {
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
                Some(Value::Null) if field.optional && !admits_null(field) => {}
                None => return Err(missing_field(name)),
                Some(value) => {
                    f(index, &mut self.child(value))
                        .map_err(|error| error.within(PathSegment::Key(Str::from_static(name))))?
                }
            }
        }
        if self.strict {
            let claimed = match self.claimed {
                Claimed::None => None,
                Claimed::Tag(tag) => Some(tag),
                Claimed::All => return Ok(()),
            };
            reject_unknown_fields(entries, record, claimed)?;
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
            Tagging::External => externally_tagged(value, shape, self.strict, f),
            Tagging::Implicit => implicitly_tagged(value, shape, self.strict, f),
            Tagging::Internal { tag, .. } | Tagging::Adjacent { tag, .. } => {
                tagged(value, shape, variant.json, tag, self.strict, f)
            }
        }
    }

    fn value(&mut self) -> Result<Value, DecodeError> {
        Ok(match self.item {
            Item::Value(value) => value.clone(),
            Item::Key(key) => Value::String(key.into()),
        })
    }
}

fn externally_tagged(
    value: &Value,
    shape: &'static Shape,
    strict: bool,
    f: &mut IndexFn<'_>,
) -> Result<(), DecodeError> {
    const EXPECTED: &str = "a string or a map with a single entry";
    match value {
        Value::String(name) => {
            let (index, case) = find_case(shape, name)?;
            if case.payload.is_some() {
                return Err(DecodeError::new(format_args!(
                    "case `{name}` requires a payload"
                )));
            }
            f(index, &mut JsonDecoder::new(&Value::Null, strict))
        }
        Value::Map(entries) if entries.len() == 1 => {
            let (name, payload) = entries.iter().next().expect("the map has one entry");
            let (index, case) = find_case(shape, name)?;
            if case.payload.is_none() {
                return Err(DecodeError::new(format_args!(
                    "case `{name}` has no payload and must be a string"
                )));
            }
            f(index, &mut JsonDecoder::new(payload, strict))
                .map_err(|error| error.within(PathSegment::Key(name.clone())))
        }
        _ => Err(DecodeError::unexpected(value.kind(), EXPECTED)),
    }
}

/// Decodes the first case in declaration order that matches.
fn implicitly_tagged(
    value: &Value,
    shape: &'static Shape,
    strict: bool,
    f: &mut IndexFn<'_>,
) -> Result<(), DecodeError> {
    let variant = shape.as_variant().expect("callers check the shape");
    let mut failures = Vec::new();
    for (index, case) in variant.cases.iter().enumerate() {
        if case.payload.is_none() {
            if matches!(value, Value::String(name) if name == case.json.name) {
                return f(index, &mut JsonDecoder::new(&Value::Null, strict));
            }
            continue;
        }
        match f(index, &mut JsonDecoder::new(value, strict)) {
            Ok(()) => return Ok(()),
            Err(error) => failures.push(format!("`{}`: {error}", case.json.name)),
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
    value: &Value,
    shape: &'static Shape,
    tagging: Tagging,
    tag: &'static str,
    strict: bool,
    f: &mut IndexFn<'_>,
) -> Result<(), DecodeError> {
    let Value::Map(entries) = value else {
        return Err(DecodeError::unexpected(value.kind(), "a map"));
    };
    let name = match entries.get(tag) {
        Some(Value::String(name)) => name,
        Some(other) => {
            return Err(DecodeError::unexpected(other.kind(), "a string")
                .within(PathSegment::Key(Str::from(tag))));
        }
        None => return Err(missing_field(tag)),
    };
    let (index, case) =
        find_case(shape, name).map_err(|error| error.within(PathSegment::Key(Str::from(tag))))?;
    let Some(payload_shape) = case.payload else {
        return f(index, &mut JsonDecoder::new(&Value::Null, strict));
    };
    match case.json.content_field(tagging, payload_shape()) {
        Some(content) => {
            let payload = entries.get(content).ok_or_else(|| missing_field(content))?;
            f(index, &mut JsonDecoder::new(payload, strict))
                .map_err(|error| error.within(PathSegment::Key(Str::from_static(content))))
        }
        // The record's fields share the map with the tag.
        None => {
            let mut decoder = JsonDecoder {
                claimed: Claimed::Tag(tag),
                ..JsonDecoder::new(value, strict)
            };
            f(index, &mut decoder)
        }
    }
}

/// Rejects keys of the map that are not fields of the record, except for the
/// claimed key.
fn reject_unknown_fields(
    entries: &Map<Str, Value>,
    record: &RecordShape,
    claimed: Option<&str>,
) -> Result<(), DecodeError> {
    let mut known = Vec::new();
    if !known_fields(record, &mut known) {
        // Inline fields that are not records may consume any key.
        return Ok(());
    }
    for key in entries.keys() {
        if Some(key.as_str()) != claimed && !known.contains(&key.as_str()) {
            return Err(DecodeError::new(format_args!("unknown field `{key}`"))
                .within(PathSegment::Key(key.clone())));
        }
    }
    Ok(())
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
/// Optional fields accept `null` as absent, unless `null` is a value. Freeform
/// values include `null`, but many languages cannot tell an absent value from
/// `null`, so optional freeform fields treat `null` as absent.
fn admits_null(field: &FieldShape) -> bool {
    fn shape_admits_null(shape: &Shape) -> bool {
        match &shape.kind {
            ShapeKind::Scalar(scalar) => *scalar == Scalar::Unit,
            ShapeKind::Wrapper(inner) => shape_admits_null(inner()),
            ShapeKind::Variant(variant) => variant.json == Tagging::Implicit,
            ShapeKind::Opaque(opaque) => opaque.json.admits(super::Type::Null),
            ShapeKind::Value => false,
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
