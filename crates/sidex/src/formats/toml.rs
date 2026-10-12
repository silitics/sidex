//! The TOML format.
//!
//! TOML uses the representation of the JSON format (see [`json`]).
//!
//! [`json`]: crate::formats::json
//!
//! TOML has no `null`, so values containing `unit` or freeform `null` cannot be
//! written. TOML's integers are signed 64-bit integers. Reading accepts TOML's
//! date-times, which the JSON representation treats as strings, so a `timestamp` may be
//! written either as `2026-10-12T08:30:00Z` or as `"2026-10-12T08:30:00Z"`.
//!
//! ```
//! use sidex::Value;
//!
//! let value: Value = sidex::formats::toml::from_str("at = 2026-10-12T08:30:00Z").unwrap();
//! assert_eq!(value, Value::from_iter([("at", "2026-10-12T08:30:00Z")]));
//! ```

use super::bridge;
use super::bridge::SerializeView;
use crate::Decode;
use crate::DecodeError;
use crate::Encode;
use crate::EncodeError;
use crate::Value;
use crate::formats::json;

/// Writes `value` as TOML.
pub fn to_string<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, |view: &SerializeView<'_>| toml::to_string(view))
}

/// Writes `value` as TOML with arrays spread over multiple lines.
pub fn to_string_pretty<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, |view: &SerializeView<'_>| {
        toml::to_string_pretty(view)
    })
}

/// Reads a value from TOML text.
pub fn from_str<T: Decode>(text: &str) -> Result<T, DecodeError> {
    let table: toml::Table = toml::from_str(text).map_err(DecodeError::new)?;
    json::from_value(&from_toml(toml::Value::Table(table)))
}

/// Converts a TOML value into a freeform value.
fn from_toml(value: toml::Value) -> Value {
    match value {
        toml::Value::String(value) => Value::String(value.into()),
        toml::Value::Integer(value) => Value::Int(value.into()),
        toml::Value::Float(value) => Value::Float(value),
        toml::Value::Boolean(value) => Value::Bool(value),
        toml::Value::Datetime(value) => Value::String(value.to_string().into()),
        toml::Value::Array(items) => Value::Seq(items.into_iter().map(from_toml).collect()),
        toml::Value::Table(entries) => {
            Value::Map(
                entries
                    .into_iter()
                    .map(|(key, value)| (key.into(), from_toml(value)))
                    .collect(),
            )
        }
    }
}
