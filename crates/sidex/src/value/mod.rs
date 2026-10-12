//! Freeform values.
//!
//! A [`Value`] is a self-describing tree following a JSON-like data model,
//! extended with exact integers and bytes. Like [`Seq`] and [`Map`], values are
//! cheap to clone. Types use values for data without a
//! schema. Formats also use values to hold documents in memory, for instance,
//! when reading JSON (see [`formats`]).
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
use crate::types::Bytes;
use crate::types::Map;
use crate::types::Seq;
use crate::types::Str;

mod path;

/// Freeform value.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    /// Absence of a value.
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
    Float(f64),
    /// String.
    String(Str),
    /// Sequence of bytes.
    Bytes(Bytes),
    /// Sequence of values.
    Seq(Seq<Value>),
    /// Map from strings to values in insertion order.
    Map(Map<Str, Value>),
}

impl Value {
    /// Returns whether the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns the boolean, if the value is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the integer, if the value is one.
    pub fn as_int(&self) -> Option<i128> {
        match self {
            Value::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the number as a float, if the value is an integer or a float.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(value) => Some(*value as f64),
            Value::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the string, if the value is one.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the bytes, if the value is a sequence of bytes.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the values, if the value is a sequence.
    pub fn as_seq(&self) -> Option<&Seq<Value>> {
        match self {
            Value::Seq(items) => Some(items),
            _ => None,
        }
    }

    /// Returns the entries, if the value is a map.
    pub fn as_map(&self) -> Option<&Map<Str, Value>> {
        match self {
            Value::Map(entries) => Some(entries),
            _ => None,
        }
    }

    /// Returns the value of `key`, if the value is a map with that key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_map()?.get(key)
    }

    /// Short description of the value's kind for error messages.
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "a boolean",
            Value::Int(_) => "an integer",
            Value::Float(_) => "a float",
            Value::String(_) => "a string",
            Value::Bytes(_) => "bytes",
            Value::Seq(_) => "a sequence",
            Value::Map(_) => "a map",
        }
    }
}

impl Shaped for Value {
    const SHAPE: &'static Shape = &Shape::new("::core::builtins::value", ShapeKind::Value);
}

impl Encode for Value {
    fn encode(&self) -> View<'_> {
        View::Value(Cow::Borrowed(self))
    }
}

impl Decode for Value {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decoder.value()
    }
}

macro_rules! impl_from_int {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Value {
                fn from(value: $ty) -> Self {
                    Value::Int(value.into())
                }
            }
        )*
    };
}

impl_from_int!(i8, i16, i32, i64, u8, u16, u32, u64);

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.into())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value.into())
    }
}

impl From<Str> for Value {
    fn from(value: Str) -> Self {
        Value::String(value)
    }
}

impl From<Bytes> for Value {
    fn from(value: Bytes) -> Self {
        Value::Bytes(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Seq(value.into())
    }
}

impl From<Seq<Value>> for Value {
    fn from(value: Seq<Value>) -> Self {
        Value::Seq(value)
    }
}

impl From<Map<Str, Value>> for Value {
    fn from(value: Map<Str, Value>) -> Self {
        Value::Map(value)
    }
}

impl<K: Into<Str>, V: Into<Value>> FromIterator<(K, V)> for Value {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Value::Map(
            iter.into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}
