//! JSON text.

use std::fmt;

use serde::Deserialize;
use serde::Deserializer;
use serde::de;

use crate::Decode;
use crate::DecodeError;
use crate::Encode;
use crate::EncodeError;
use crate::Value;
use crate::formats::bridge;
use crate::formats::bridge::SerializeView;
use crate::types::Map;
use crate::types::Str;

/// Writes `value` as compact JSON.
pub fn to_string<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, |view: &SerializeView<'_>| {
        serde_json::to_string(view)
    })
}

/// Writes `value` as indented JSON.
pub fn to_string_pretty<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, |view: &SerializeView<'_>| {
        serde_json::to_string_pretty(view)
    })
}

/// Reads a value from JSON text.
pub fn from_str<T: Decode>(text: &str) -> Result<T, DecodeError> {
    let DeserializeValue(value) = serde_json::from_str(text).map_err(DecodeError::new)?;
    super::from_value(&value)
}

/// Reads a value from UTF-8 encoded JSON.
pub fn from_slice<T: Decode>(bytes: &[u8]) -> Result<T, DecodeError> {
    let DeserializeValue(value) = serde_json::from_slice(bytes).map_err(DecodeError::new)?;
    super::from_value(&value)
}

/// [`Value`] read with Serde.
pub(crate) struct DeserializeValue(pub(crate) Value);

impl<'de> Deserialize<'de> for DeserializeValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueVisitor).map(Self)
    }
}

pub(crate) struct ValueVisitor;

impl<'de> de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Int(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Int(value.into()))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Value, E> {
        Ok(Value::Int(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.into()))
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value.into()))
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Value, E> {
        Ok(Value::Bytes(value.into()))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Value, E> {
        Ok(Value::Bytes(value.into()))
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(DeserializeValue(item)) = seq.next_element()? {
            items.push(item);
        }
        Ok(Value::Seq(items.into()))
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut entries = Map::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((key, DeserializeValue(item))) = map.next_entry::<String, _>()? {
            entries.insert(Str::from(key), item);
        }
        Ok(Value::Map(entries))
    }
}
