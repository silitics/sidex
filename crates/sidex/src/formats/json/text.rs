//! JSON text.

use std::fmt;

use base64::Engine;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;

use super::repr::BASE64;
use crate::Decode;
use crate::DecodeError;
use crate::Encode;
use crate::EncodeError;
use crate::JsonValue;
use crate::formats::bridge;
use crate::formats::bridge::SerializeView;
use crate::formats::bridge::Syntax;
use crate::types::Map;
use crate::types::Str;
use crate::value::Path;

/// Writes `value` as compact JSON.
pub fn to_string<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, Syntax::Json, |view: &SerializeView<'_, '_>| {
        serde_json::to_string(view)
    })
}

/// Writes `value` as indented JSON.
pub fn to_string_pretty<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, Syntax::Json, |view: &SerializeView<'_, '_>| {
        serde_json::to_string_pretty(view)
    })
}

/// Reads a value from JSON text.
pub fn from_str<T: Decode>(text: &str) -> Result<T, DecodeError> {
    let DeserializeValue(value) = serde_json::from_str(text).map_err(DecodeError::new)?;
    super::from_value(&value)
}

/// Reads a value from JSON text and rejects unknown fields (see
/// [`from_value_strict`](super::from_value_strict)).
pub fn from_str_strict<T: Decode>(text: &str) -> Result<T, DecodeError> {
    let DeserializeValue(value) = serde_json::from_str(text).map_err(DecodeError::new)?;
    super::from_value_strict(&value)
}

/// Reads a value from JSON text and also returns the paths of fields unknown to
/// records (see [`from_value_with_unknown`](super::from_value_with_unknown)).
pub fn from_str_with_unknown<T: Decode>(text: &str) -> Result<(T, Vec<Path>), DecodeError> {
    let DeserializeValue(value) = serde_json::from_str(text).map_err(DecodeError::new)?;
    super::from_value_with_unknown(&value)
}

/// Reads a value from UTF-8 encoded JSON.
pub fn from_slice<T: Decode>(bytes: &[u8]) -> Result<T, DecodeError> {
    let DeserializeValue(value) = serde_json::from_slice(bytes).map_err(DecodeError::new)?;
    super::from_value(&value)
}

/// [`JsonValue`] read with Serde.
pub(crate) struct DeserializeValue(pub(crate) JsonValue);

impl<'de> Deserialize<'de> for DeserializeValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueVisitor).map(Self)
    }
}

pub(crate) struct ValueVisitor;

impl<'de> de::Visitor<'de> for ValueVisitor {
    type Value = JsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<JsonValue, E> {
        Ok(JsonValue::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<JsonValue, E> {
        Ok(JsonValue::Int(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<JsonValue, E> {
        Ok(JsonValue::Int(value.into()))
    }

    fn visit_i128<E>(self, value: i128) -> Result<JsonValue, E> {
        Ok(JsonValue::Int(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<JsonValue, E> {
        Ok(JsonValue::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<JsonValue, E> {
        Ok(JsonValue::String(value.into()))
    }

    fn visit_string<E>(self, value: String) -> Result<JsonValue, E> {
        Ok(JsonValue::String(value.into()))
    }

    /// Bytes from other formats become Base64 strings, like in JSON.
    fn visit_bytes<E>(self, value: &[u8]) -> Result<JsonValue, E> {
        Ok(JsonValue::String(BASE64.encode(value).into()))
    }

    fn visit_unit<E>(self) -> Result<JsonValue, E> {
        Ok(JsonValue::Null)
    }

    fn visit_none<E>(self) -> Result<JsonValue, E> {
        Ok(JsonValue::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<JsonValue, D::Error> {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<JsonValue, A::Error> {
        let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(DeserializeValue(item)) = seq.next_element()? {
            items.push(item);
        }
        Ok(JsonValue::Seq(items.into()))
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<JsonValue, A::Error> {
        let mut entries = Map::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((key, DeserializeValue(item))) = map.next_entry::<String, _>()? {
            entries.insert(Str::from(key), item);
        }
        Ok(JsonValue::Map(entries))
    }
}
