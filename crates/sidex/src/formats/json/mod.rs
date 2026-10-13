//! The JSON format.
//!
//! The JSON format defines how typed values are represented in JSON. Its
//! representation is the cross-language contract of Sidex: the Rust,
//! TypeScript, and Python backends all follow it. TOML uses the same
//! representation (see [`formats`]).
//!
//! JSON documents are written and read as text with [`to_string`] and
//! [`from_str`], which require the default `json` feature, and held in memory as
//! JSON values with [`to_value`] and [`from_value`]. Shapes carry the
//! JSON-specific metadata of types as [`Field`], [`Case`], [`Tagging`], and
//! [`Opaque`].
//!
//! ```
//! # #[cfg(feature = "json")]
//! # {
//! let text = sidex::formats::json::to_string(&vec![1_u64, u64::MAX]).unwrap();
//! assert_eq!(text, r#"[1,"18446744073709551615"]"#);
//!
//! let numbers: Vec<u64> = sidex::formats::json::from_str(&text).unwrap();
//! assert_eq!(numbers, [1, u64::MAX]);
//! # }
//! ```
//!
//! # Representation
//!
//! | Sidex type | Representation | Also accepted when decoding |
//! | --- | --- | --- |
//! | `bool` | boolean | |
//! | `i8` to `i32`, `u8` to `u32`, `idx` | integer | |
//! | `i64`, `u64` | integer, or decimal string beyond ±(2^53 − 1) | decimal string |
//! | `f32`, `f64` | number, or `"NaN"`, `"+Infinity"`, and `"-Infinity"` | integer, `"Infinity"` |
//! | `string` | string | |
//! | `bytes` | Base64 string with padding | missing padding |
//! | `uuid` | hyphenated lowercase string | uppercase |
//! | `timestamp` | RFC 3339 string in UTC | any offset |
//! | `date` | `YYYY-MM-DD` string | |
//! | sequence | sequence | |
//! | map | map with keys as text (see below) | |
//! | record | map from JSON field names to values | `null` for absent optional fields, unless `null` is a value of the field's type |
//! | variant | depends on the tagging (see below) | |
//! | wrapper | representation of the wrapped type | |
//! | `json` | the value itself | |
//!
//! JavaScript represents numbers as 64-bit floats, which represent integers
//! exactly only up to a magnitude of 2^53 − 1. Larger `i64` and `u64` values
//! are decimal strings, which keeps them exact in every language. JSON has no
//! representation of non-finite floats, so typed floats use strings for them.
//! JSON values cannot hold non-finite floats.
//!
//! Map keys may be strings, integers, UUIDs, timestamps, and dates. Keys use the
//! text representation of their type, with integers in decimal.
//!
//! Records omit absent optional fields. Decoding ignores unknown fields. Fields
//! marked as inline merge the fields of their record into the enclosing map.
//!
//! Variants use one of four taggings. Internally and adjacently tagged cases
//! store their payload in a content field, `content` by default, unless a case
//! sets another field. Without a field set by the case, internal tagging merges
//! the fields of record payloads into the map holding the tag. With the default
//! fields, the cases of
//!
//! ```sidex
//! variant Position {
//!     Unknown,
//!     Named: string,
//!     Coordinates: Point,
//! }
//! ```
//!
//! are represented as follows:
//!
//! | Tagging | `Unknown` | `Named("home")` | `Coordinates(Point { x: 1, y: 2 })` |
//! | --- | --- | --- | --- |
//! | internal | `{"tag":"Unknown"}` | `{"tag":"Named","content":"home"}` | `{"tag":"Coordinates","x":1,"y":2}` |
//! | adjacent | `{"tag":"Unknown"}` | `{"tag":"Named","content":"home"}` | `{"tag":"Coordinates","content":{"x":1,"y":2}}` |
//! | external | `"Unknown"` | `{"Named":"home"}` | `{"Coordinates":{"x":1,"y":2}}` |
//! | implicit | `"Unknown"` | `"home"` | `{"x":1,"y":2}` |
//!
//! Implicit tagging decodes the first case in declaration order that matches.
//!
//! [`formats`]: crate::formats

use std::cell::RefCell;

use self::decoding::Unknown;
pub use self::meta::Case;
pub use self::meta::Field;
pub use self::meta::Opaque;
pub use self::meta::Tagging;
pub use self::meta::Type;
pub use self::schema::map_key_schema;
pub use self::schema::schema;
#[cfg(feature = "schemars-1")]
pub use self::schema::schemars_schema;
#[cfg(feature = "json")]
pub use self::text::from_slice;
#[cfg(feature = "json")]
pub use self::text::from_str;
#[cfg(feature = "json")]
pub use self::text::from_str_strict;
#[cfg(feature = "json")]
pub use self::text::from_str_with_unknown;
#[cfg(feature = "json")]
pub use self::text::to_string;
#[cfg(feature = "json")]
pub use self::text::to_string_pretty;
use super::view::ValueView;
use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Path;
use crate::encode::Encode;
use crate::encode::EncodeError;
use crate::types::JsonValue;

mod decoding;
mod encoding;
#[cfg(feature = "serde_json-1")]
mod interop;
mod meta;
pub(crate) mod repr;
mod schema;
#[cfg(feature = "json")]
pub(crate) mod text;

/// Encodes `value` and passes a view of the result to `f`.
pub(crate) fn encode<T: Encode + ?Sized>(
    value: &T,
    f: &mut dyn FnMut(ValueView<'_>) -> Result<(), EncodeError>,
) -> Result<(), EncodeError> {
    encoding::view(&value.encode(), f)
}

/// Writes `value` as a JSON document held in a JSON value.
///
/// ```
/// use sidex::types::JsonValue;
///
/// let value = sidex::formats::json::to_value(&vec![1_u64, 2]).unwrap();
/// assert_eq!(value, JsonValue::from(vec![1.into(), 2.into()]));
/// ```
pub fn to_value<T: Encode + ?Sized>(value: &T) -> Result<JsonValue, EncodeError> {
    let mut out = JsonValue::Null;
    encode(value, &mut |view| {
        out = JsonValue::from_view(view)?;
        Ok(())
    })?;
    Ok(out)
}

/// Reads a value from a JSON document held in a JSON value.
///
/// ```
/// use sidex::types::JsonValue;
///
/// let value = JsonValue::from(vec!["1".into(), 2.into()]);
/// let numbers: Vec<u64> = sidex::formats::json::from_value(&value).unwrap();
/// assert_eq!(numbers, [1, 2]);
/// ```
pub fn from_value<T: Decode>(value: &JsonValue) -> Result<T, DecodeError> {
    T::decode(&mut decoding::JsonDecoder::new(value, Unknown::Ignore))
}

/// Reads a value like [`from_value`], but rejects fields unknown to records.
///
/// Use this for documents written by people, such as configuration files, where
/// unknown fields are likely typos.
///
/// ```
/// use sidex::types::JsonValue;
/// use sidex::types::Map;
///
/// let value = JsonValue::from_iter([("thing", JsonValue::from("x"))]);
/// let map: Map<String, String> = sidex::formats::json::from_value_strict(&value).unwrap();
/// assert_eq!(map["thing"], "x");
/// ```
pub fn from_value_strict<T: Decode>(value: &JsonValue) -> Result<T, DecodeError> {
    T::decode(&mut decoding::JsonDecoder::new(value, Unknown::Reject))
}

/// Reads a value like [`from_value`] and also returns the paths of fields
/// unknown to records.
///
/// Use this for documents written by people, such as configuration files, to
/// warn about unknown fields. They may be typos, but they may also be fields
/// that a later version added. The paths only contain keys and indices, so
/// applications can show them without revealing values.
pub fn from_value_with_unknown<T: Decode>(
    value: &JsonValue,
) -> Result<(T, Vec<Path>), DecodeError> {
    let unknown = RefCell::new(Vec::new());
    let decoded = T::decode(&mut decoding::JsonDecoder::new(
        value,
        Unknown::Collect(&unknown),
    ))?;
    Ok((decoded, unknown.into_inner()))
}
