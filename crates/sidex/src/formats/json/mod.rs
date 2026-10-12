//! The JSON format.
//!
//! The JSON format defines how typed values are represented in JSON. Its
//! representation is the cross-language contract of Sidex: the Rust,
//! TypeScript, and Python backends all follow it. TOML uses the same
//! representation (see [`formats`]).
//!
//! JSON documents are written and read as text with [`to_string`] and
//! [`from_str`], which require the default `json` feature, and held in memory as
//! freeform values with [`to_value`] and [`from_value`]. Shapes carry the
//! JSON-specific metadata of types as [`Field`], [`Case`], [`Tagging`], and
//! [`Opaque`].
//!
//! ```
//! # #[cfg(feature = "json")]
//! # {
//! let text = sidex::formats::json::to_string(&vec![1_u64, 2]).unwrap();
//! assert_eq!(text, r#"["1","2"]"#);
//!
//! let numbers: Vec<u64> = sidex::formats::json::from_str(&text).unwrap();
//! assert_eq!(numbers, [1, 2]);
//! # }
//! ```
//!
//! # Representation
//!
//! | Sidex type | Representation | Also accepted when decoding |
//! | --- | --- | --- |
//! | `unit` | `null` | |
//! | `bool` | boolean | |
//! | `i8` to `i32`, `u8` to `u32`, `idx` | integer | |
//! | `i64`, `u64` | decimal string, for instance, `"42"` | integer |
//! | `f32`, `f64` | number, or `"NaN"`, `"Infinity"`, and `"-Infinity"` | integer, `"+Infinity"` |
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
//! | freeform value | the value itself, with bytes as Base64 strings | |
//!
//! JavaScript represents numbers as 64-bit floats, which cannot represent all
//! 64-bit integers. Representing `i64` and `u64` as strings keeps them exact in
//! every language. JSON has no representation of non-finite floats, so typed
//! floats use strings for them. Non-finite freeform floats cannot be encoded.
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

pub use self::meta::Case;
pub use self::meta::Field;
pub use self::meta::Opaque;
pub use self::meta::Tagging;
pub use self::meta::Type;
#[cfg(feature = "json")]
pub use self::text::from_slice;
#[cfg(feature = "json")]
pub use self::text::from_str;
#[cfg(feature = "json")]
pub use self::text::to_string;
#[cfg(feature = "json")]
pub use self::text::to_string_pretty;
use super::view::ValueView;
use crate::Value;
use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::encode::Encode;
use crate::encode::EncodeError;

mod decoding;
mod encoding;
#[cfg(feature = "serde_json-1")]
mod interop;
mod meta;
mod repr;
#[cfg(feature = "json")]
pub(crate) mod text;

/// Encodes `value` and passes a view of the result to `f`.
pub(crate) fn encode<T: Encode + ?Sized>(
    value: &T,
    f: &mut dyn FnMut(ValueView<'_>) -> Result<(), EncodeError>,
) -> Result<(), EncodeError> {
    encoding::view(&value.encode(), f)
}

/// Writes `value` as a JSON document held in a freeform value.
///
/// ```
/// use sidex::Value;
///
/// let value = sidex::formats::json::to_value(&vec![1_u64, 2]).unwrap();
/// assert_eq!(value, Value::from(vec!["1".into(), "2".into()]));
/// ```
pub fn to_value<T: Encode + ?Sized>(value: &T) -> Result<Value, EncodeError> {
    let mut out = Value::Null;
    encode(value, &mut |view| {
        out = Value::from_view(view)?;
        Ok(())
    })?;
    Ok(out)
}

/// Reads a value from a JSON document held in a freeform value.
///
/// ```
/// use sidex::Value;
///
/// let value = Value::from(vec!["1".into(), 2.into()]);
/// let numbers: Vec<u64> = sidex::formats::json::from_value(&value).unwrap();
/// assert_eq!(numbers, [1, 2]);
/// ```
pub fn from_value<T: Decode>(value: &Value) -> Result<T, DecodeError> {
    T::decode(&mut decoding::JsonDecoder::new(value, false))
}

/// Reads a value like [`from_value`], but rejects fields unknown to records.
///
/// Use this for documents written by people, such as configuration files, where
/// unknown fields are likely typos.
///
/// ```
/// use sidex::Value;
/// use sidex::types::Map;
///
/// let value = Value::from_iter([("thing", Value::from("x"))]);
/// let map: Map<String, String> = sidex::formats::json::from_value_strict(&value).unwrap();
/// assert_eq!(map["thing"], "x");
/// ```
pub fn from_value_strict<T: Decode>(value: &Value) -> Result<T, DecodeError> {
    T::decode(&mut decoding::JsonDecoder::new(value, true))
}
