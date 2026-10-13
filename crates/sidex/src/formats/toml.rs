//! The TOML format.
//!
//! TOML uses the representation of the JSON format (see [`json`]).
//!
//! [`json`]: crate::formats::json
//!
//! TOML has no `null` and only signed 64-bit integers. Writing omits map entries
//! whose value is `null`, fails for `null` in arrays, and writes larger integers
//! as decimal strings. Reading accepts TOML's
//! date-times, which the JSON representation treats as strings, so a `timestamp` may be
//! written either as `2026-10-12T08:30:00Z` or as `"2026-10-12T08:30:00Z"`.
//!
//! ```
//! use sidex::JsonValue;
//!
//! let value: JsonValue = sidex::formats::toml::from_str("at = 2026-10-12T08:30:00Z").unwrap();
//! assert_eq!(value, JsonValue::from_iter([("at", "2026-10-12T08:30:00Z")]));
//! ```

use super::bridge;
use super::bridge::SerializeView;
use super::bridge::Syntax;
use crate::Decode;
use crate::DecodeError;
use crate::Encode;
use crate::EncodeError;
use crate::JsonValue;
use crate::formats::json;

/// Writes `value` as TOML.
pub fn to_string<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, Syntax::Toml, |view: &SerializeView<'_, '_>| {
        toml::to_string(view)
    })
}

/// Writes `value` as TOML with arrays spread over multiple lines.
pub fn to_string_pretty<T: Encode + ?Sized>(value: &T) -> Result<String, EncodeError> {
    bridge::encode(value, Syntax::Toml, |view: &SerializeView<'_, '_>| {
        toml::to_string_pretty(view)
    })
}

/// Reads a value from TOML text.
pub fn from_str<T: Decode>(text: &str) -> Result<T, DecodeError> {
    json::from_value(&parse(text)?)
}

/// Reads a value from TOML text and rejects unknown fields (see
/// [`json::from_value_strict`]).
pub fn from_str_strict<T: Decode>(text: &str) -> Result<T, DecodeError> {
    json::from_value_strict(&parse(text)?)
}

/// Parses TOML text as a JSON value.
fn parse(text: &str) -> Result<JsonValue, DecodeError> {
    let table: toml::Table = toml::from_str(text).map_err(DecodeError::new)?;
    Ok(from_toml(toml::Value::Table(table)))
}

/// Converts a TOML value into a JSON value.
fn from_toml(value: toml::Value) -> JsonValue {
    match value {
        toml::Value::String(value) => JsonValue::String(value.into()),
        toml::Value::Integer(value) => JsonValue::Int(value.into()),
        toml::Value::Float(value) => JsonValue::Float(value),
        toml::Value::Boolean(value) => JsonValue::Bool(value),
        toml::Value::Datetime(value) => JsonValue::String(value.to_string().into()),
        toml::Value::Array(items) => JsonValue::Seq(items.into_iter().map(from_toml).collect()),
        toml::Value::Table(entries) => {
            JsonValue::Map(
                entries
                    .into_iter()
                    .map(|(key, value)| (key.into(), from_toml(value)))
                    .collect(),
            )
        }
    }
}
