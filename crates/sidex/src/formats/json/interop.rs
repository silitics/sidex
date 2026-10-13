//! Interoperability between Sidex's and `serde_json`'s JSON values.

use std::borrow::Cow;

use crate::Decode;
use crate::DecodeError;
use crate::Decoder;
use crate::Encode;
use crate::Shape;
use crate::Shaped;
use crate::View;
use crate::types::JsonValue;
use crate::types::Map;
use crate::types::Number;
use crate::types::Str;

impl Shaped for serde_json::Value {
    const SHAPE: &'static Shape = JsonValue::SHAPE;
}

impl Encode for serde_json::Value {
    fn encode(&self) -> View<'_> {
        View::Json(Cow::Owned(JsonValue::from(self.clone())))
    }
}

impl Decode for serde_json::Value {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decoder.json().map(Self::from)
    }
}

/// Converts the value.
impl From<JsonValue> for serde_json::Value {
    fn from(value: JsonValue) -> Self {
        Self::from(&value)
    }
}

impl From<&JsonValue> for serde_json::Value {
    fn from(value: &JsonValue) -> Self {
        match value {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(value) => Self::Bool(*value),
            JsonValue::Number(number) => {
                if let Some(value) = number.as_u64() {
                    Self::from(value)
                } else if let Some(value) = number.as_i64() {
                    Self::from(value)
                } else {
                    serde_json::Number::from_f64(number.as_f64())
                        .map(Self::Number)
                        .expect("numbers are finite")
                }
            }
            JsonValue::String(value) => Self::String(value.to_string()),
            JsonValue::Seq(items) => Self::Array(items.iter().map(Self::from).collect()),
            JsonValue::Map(entries) => {
                Self::Object(
                    entries
                        .iter()
                        .map(|(key, value)| (key.to_string(), Self::from(value)))
                        .collect(),
                )
            }
        }
    }
}

/// Converts the value like the JSON format reads it.
impl From<serde_json::Value> for JsonValue {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => JsonValue::Null,
            serde_json::Value::Bool(value) => JsonValue::Bool(value),
            serde_json::Value::Number(number) => {
                if let Some(value) = number.as_u64() {
                    JsonValue::from(value)
                } else if let Some(value) = number.as_i64() {
                    JsonValue::from(value)
                } else {
                    // With `arbitrary_precision`, numbers may exceed `f64`.
                    number
                        .as_f64()
                        .and_then(Number::from_f64)
                        .map_or_else(|| JsonValue::from(number.to_string()), JsonValue::Number)
                }
            }
            serde_json::Value::String(value) => JsonValue::String(value.into()),
            serde_json::Value::Array(items) => {
                JsonValue::Seq(items.into_iter().map(JsonValue::from).collect())
            }
            serde_json::Value::Object(entries) => {
                JsonValue::Map(
                    entries
                        .into_iter()
                        .map(|(key, value)| (Str::from(key), JsonValue::from(value)))
                        .collect::<Map<_, _>>(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let json = serde_json::json!({
            "name": "device",
            "count": 18446744073709551615u64,
            "ratio": 0.5,
            "tags": ["a", null, true],
        });
        let value = JsonValue::from(json.clone());
        assert_eq!(
            value,
            JsonValue::from_iter([
                ("name", JsonValue::from("device")),
                ("count", JsonValue::from(u64::MAX)),
                ("ratio", JsonValue::Number(Number::from_f64(0.5).unwrap())),
                (
                    "tags",
                    JsonValue::from(vec![
                        JsonValue::from("a"),
                        JsonValue::Null,
                        JsonValue::from(true)
                    ])
                ),
            ])
        );
        assert_eq!(serde_json::Value::from(value), json);
    }

    /// `serde_json` values encode and decode like Sidex's JSON values.
    #[test]
    fn traits() {
        let json = serde_json::json!({ "count": 1, "tags": ["a", null] });
        let text = crate::formats::json::to_string(&json).unwrap();
        assert_eq!(text, r#"{"count":1,"tags":["a",null]}"#);
        let decoded: serde_json::Value = crate::formats::json::from_str(&text).unwrap();
        assert_eq!(decoded, json);
    }
}
