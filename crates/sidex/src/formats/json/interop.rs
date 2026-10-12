//! Conversions between freeform values and `serde_json` values.

use base64::Engine;

use super::repr::BASE64;
use super::repr::non_finite;
use crate::Value;
use crate::types::Map;
use crate::types::Str;

/// Converts the value like the JSON format writes it.
///
/// Bytes become Base64 strings and non-finite floats become strings. Integers
/// beyond the 64-bit ranges become floats.
impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        Self::from(&value)
    }
}

impl From<&Value> for serde_json::Value {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(value) => Self::Bool(*value),
            Value::Int(value) => {
                if let Ok(value) = i64::try_from(*value) {
                    Self::from(value)
                } else if let Ok(value) = u64::try_from(*value) {
                    Self::from(value)
                } else {
                    float(*value as f64)
                }
            }
            Value::Float(value) => float(*value),
            Value::String(value) => Self::String(value.to_string()),
            Value::Bytes(value) => Self::String(BASE64.encode(value)),
            Value::Seq(items) => Self::Array(items.iter().map(Self::from).collect()),
            Value::Map(entries) => {
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

fn float(value: f64) -> serde_json::Value {
    serde_json::Number::from_f64(value)
        .map(serde_json::Value::Number)
        .unwrap_or_else(|| serde_json::Value::String(non_finite(value).to_owned()))
}

/// Converts the value like the JSON format reads it.
impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(value) => Value::Bool(value),
            serde_json::Value::Number(number) => {
                if let Some(value) = number.as_i64() {
                    Value::Int(value.into())
                } else if let Some(value) = number.as_u64() {
                    Value::Int(value.into())
                } else {
                    Value::Float(number.as_f64().unwrap_or(f64::NAN))
                }
            }
            serde_json::Value::String(value) => Value::String(value.into()),
            serde_json::Value::Array(items) => {
                Value::Seq(items.into_iter().map(Value::from).collect())
            }
            serde_json::Value::Object(entries) => {
                Value::Map(
                    entries
                        .into_iter()
                        .map(|(key, value)| (Str::from(key), Value::from(value)))
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
        let value = Value::from(json.clone());
        assert_eq!(
            value,
            Value::from_iter([
                ("name", Value::from("device")),
                ("count", Value::from(u64::MAX)),
                ("ratio", Value::from(0.5)),
                (
                    "tags",
                    Value::from(vec![Value::from("a"), Value::Null, Value::from(true)])
                ),
            ])
        );
        assert_eq!(serde_json::Value::from(value), json);
        assert_eq!(serde_json::Value::from(Value::Float(f64::NAN)), "NaN");
    }
}
