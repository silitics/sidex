//! The `json` built-in type.

use std::borrow::Cow;
use std::fmt;

use super::Map;
use super::Seq;
use super::Str;
use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::encode::Encode;
use crate::encode::View;
use crate::shape::Shape;
use crate::shape::ShapeKind;
use crate::shape::Shaped;

/// JSON value, the Rust type of the built-in `json` type.
///
/// Values follow the JSON data model. Like [`Seq`] and [`Map`], they are cheap
/// to clone. Formats also use values to hold documents in memory, for instance,
/// when reading JSON or TOML (see [`formats`](crate::formats)).
#[derive(Debug, Clone, PartialEq, Default)]
pub enum JsonValue {
    /// The `null` value.
    #[default]
    Null,
    /// Boolean.
    Bool(bool),
    /// Number.
    Number(Number),
    /// String.
    String(Str),
    /// Sequence of values.
    Seq(Seq<JsonValue>),
    /// Map from strings to values in insertion order.
    Map(Map<Str, JsonValue>),
}

impl JsonValue {
    /// Returns whether the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    /// Returns the boolean, if the value is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the number, if the value is one.
    pub fn as_number(&self) -> Option<Number> {
        match self {
            JsonValue::Number(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the number as an `i64`, if it is an integer in range.
    pub fn as_i64(&self) -> Option<i64> {
        self.as_number()?.as_i64()
    }

    /// Returns the number as a `u64`, if it is an integer in range.
    pub fn as_u64(&self) -> Option<u64> {
        self.as_number()?.as_u64()
    }

    /// Returns the number as an `f64`, if the value is a number.
    pub fn as_f64(&self) -> Option<f64> {
        Some(self.as_number()?.as_f64())
    }

    /// Returns the string, if the value is one.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the values, if the value is a sequence.
    pub fn as_seq(&self) -> Option<&Seq<JsonValue>> {
        match self {
            JsonValue::Seq(items) => Some(items),
            _ => None,
        }
    }

    /// Returns the entries, if the value is a map.
    pub fn as_map(&self) -> Option<&Map<Str, JsonValue>> {
        match self {
            JsonValue::Map(entries) => Some(entries),
            _ => None,
        }
    }

    /// Returns the value of `key`, if the value is a map with that key.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.as_map()?.get(key)
    }

    /// Short description of the value's kind for error messages.
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            JsonValue::Null => "null",
            JsonValue::Bool(_) => "a boolean",
            JsonValue::Number(number) if number.is_f64() => "a float",
            JsonValue::Number(_) => "an integer",
            JsonValue::String(_) => "a string",
            JsonValue::Seq(_) => "a sequence",
            JsonValue::Map(_) => "a map",
        }
    }
}

/// Formats the value as compact JSON text.
#[cfg(feature = "json")]
impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = crate::formats::json::to_string(self).map_err(|_| fmt::Error)?;
        f.write_str(&text)
    }
}

impl Shaped for JsonValue {
    const SHAPE: &'static Shape = &Shape::new("::core::builtins::json", ShapeKind::Json);
}

impl Encode for JsonValue {
    fn encode(&self) -> View<'_> {
        View::Json(Cow::Borrowed(self))
    }
}

impl Decode for JsonValue {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decoder.json()
    }
}

/// JSON number: an integer in the range of `i64` or `u64`, or a finite `f64`.
///
/// Integers and floats are distinct, so `1` and `1.0` are different numbers.
/// Integers are exact across both 64-bit ranges. JSON has no representation of
/// NaN and the infinities, so numbers cannot hold them.
#[derive(Clone, Copy, PartialEq)]
pub struct Number(N);

/// Representation of a [`Number`].
///
/// Non-negative integers are always `PosInt`, so that equal integers are equal.
#[derive(Clone, Copy, PartialEq)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

/// Numbers never hold NaN.
impl Eq for Number {}

impl Number {
    /// Creates a number from a float, or returns `None` if it is not finite.
    pub fn from_f64(value: f64) -> Option<Self> {
        value.is_finite().then_some(Self(N::Float(value)))
    }

    /// Returns the number as an `i64`, if it is an integer in range.
    pub fn as_i64(&self) -> Option<i64> {
        match self.0 {
            N::PosInt(value) => i64::try_from(value).ok(),
            N::NegInt(value) => Some(value),
            N::Float(_) => None,
        }
    }

    /// Returns the number as a `u64`, if it is an integer in range.
    pub fn as_u64(&self) -> Option<u64> {
        match self.0 {
            N::PosInt(value) => Some(value),
            N::NegInt(_) | N::Float(_) => None,
        }
    }

    /// Returns the number as an `f64`, which may round integers.
    pub fn as_f64(&self) -> f64 {
        match self.0 {
            N::PosInt(value) => value as f64,
            N::NegInt(value) => value as f64,
            N::Float(value) => value,
        }
    }

    /// Indicates whether the number is a float rather than an integer.
    pub fn is_f64(&self) -> bool {
        matches!(self.0, N::Float(_))
    }

    /// Returns the integer, if the number is one.
    pub(crate) fn as_i128(&self) -> Option<i128> {
        match self.0 {
            N::PosInt(value) => Some(value.into()),
            N::NegInt(value) => Some(value.into()),
            N::Float(_) => None,
        }
    }

    /// Creates a number from an integer, or returns `None` if it is outside
    /// the ranges of `i64` and `u64`.
    pub(crate) fn from_i128(value: i128) -> Option<Self> {
        match u64::try_from(value) {
            Ok(value) => Some(Self(N::PosInt(value))),
            Err(_) => {
                i64::try_from(value)
                    .ok()
                    .map(|value| Self(N::NegInt(value)))
            }
        }
    }
}

/// Formats the number as JSON text, with floats always containing a fraction
/// or an exponent.
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            N::PosInt(value) => fmt::Display::fmt(&value, f),
            N::NegInt(value) => fmt::Display::fmt(&value, f),
            // Unlike `Display`, `Debug` writes `1.0` rather than `1`.
            N::Float(value) => fmt::Debug::fmt(&value, f),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

macro_rules! impl_from_int {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Number {
                fn from(value: $ty) -> Self {
                    Number::from_i128(value.into()).expect("64-bit integers are numbers")
                }
            }

            impl From<$ty> for JsonValue {
                fn from(value: $ty) -> Self {
                    JsonValue::Number(value.into())
                }
            }
        )*
    };
}

impl_from_int!(i8, i16, i32, i64, u8, u16, u32, u64);

impl From<Number> for JsonValue {
    fn from(value: Number) -> Self {
        JsonValue::Number(value)
    }
}

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        JsonValue::Bool(value)
    }
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        JsonValue::String(value.into())
    }
}

impl From<String> for JsonValue {
    fn from(value: String) -> Self {
        JsonValue::String(value.into())
    }
}

impl From<Str> for JsonValue {
    fn from(value: Str) -> Self {
        JsonValue::String(value)
    }
}

impl From<Vec<JsonValue>> for JsonValue {
    fn from(value: Vec<JsonValue>) -> Self {
        JsonValue::Seq(value.into())
    }
}

impl From<Seq<JsonValue>> for JsonValue {
    fn from(value: Seq<JsonValue>) -> Self {
        JsonValue::Seq(value)
    }
}

impl From<Map<Str, JsonValue>> for JsonValue {
    fn from(value: Map<Str, JsonValue>) -> Self {
        JsonValue::Map(value)
    }
}

impl<K: Into<Str>, V: Into<JsonValue>> FromIterator<(K, V)> for JsonValue {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        JsonValue::Map(
            iter.into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}
