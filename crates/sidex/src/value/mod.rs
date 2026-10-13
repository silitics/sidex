//! JSON values and paths into values.
//!
//! A [`JsonValue`] is a self-describing tree following the JSON data model,
//! with exact integers. Like [`Seq`] and [`Map`], values are cheap to clone.
//! The built-in `json` type uses values for data without a schema. Formats also
//! use values to hold documents in memory, for instance, when reading JSON or
//! TOML (see [`formats`]).
//!
//! [`formats`]: crate::formats
//! [`Seq`]: crate::types::Seq
//! [`Map`]: crate::types::Map

use std::borrow::Cow;

pub use self::path::Path;
pub use self::path::PathSegment;
use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::encode::Encode;
use crate::encode::View;
use crate::shape::Shape;
use crate::shape::ShapeKind;
use crate::shape::Shaped;
use crate::types::Map;
use crate::types::Seq;
use crate::types::Str;

mod path;

/// JSON value.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum JsonValue {
    /// The `null` value.
    #[default]
    Null,
    /// Boolean.
    Bool(bool),
    /// Integer.
    ///
    /// Formats preserve integers exactly within the 64-bit signed and unsigned
    /// ranges.
    Int(i128),
    /// Floating-point number.
    ///
    /// JSON has no representation of non-finite floats, so encoding them fails.
    Float(f64),
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

    /// Returns the integer, if the value is one.
    pub fn as_int(&self) -> Option<i128> {
        match self {
            JsonValue::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the number as a float, if the value is an integer or a float.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Int(value) => Some(*value as f64),
            JsonValue::Float(value) => Some(*value),
            _ => None,
        }
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
            JsonValue::Int(_) => "an integer",
            JsonValue::Float(_) => "a float",
            JsonValue::String(_) => "a string",
            JsonValue::Seq(_) => "a sequence",
            JsonValue::Map(_) => "a map",
        }
    }
}

/// Formats the value as compact JSON text.
#[cfg(feature = "json")]
impl std::fmt::Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = crate::formats::json::to_string(self).map_err(|_| std::fmt::Error)?;
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

macro_rules! impl_from_int {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for JsonValue {
                fn from(value: $ty) -> Self {
                    JsonValue::Int(value.into())
                }
            }
        )*
    };
}

impl_from_int!(i8, i16, i32, i64, u8, u16, u32, u64);

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        JsonValue::Bool(value)
    }
}

impl From<f64> for JsonValue {
    fn from(value: f64) -> Self {
        JsonValue::Float(value)
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
