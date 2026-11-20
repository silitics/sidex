//! The `json` type.

use core::fmt;

use crate::Decode;
use crate::Encode;
use crate::error::DecodeError;
use crate::error::DecodeErrorKind;
use crate::error::EncodeError;
use crate::json;
use crate::json::Reader;
use crate::json::Writer;

/// Any JSON value.
///
/// Decoded values hold their JSON text. Decode them as a type with
/// [`decode`](Self::decode). To encode a value, create it from JSON text with
/// [`parse`](Self::parse) or from any value implementing [`Encode`] with
/// [`from_value`](Self::from_value).
///
/// ```
/// use sidex_embedded::json;
/// use sidex_embedded::types::{Json, Seq, Str};
///
/// let value: Json<'_> = json::from_str(r#"{"enabled": true}"#).unwrap();
/// assert_eq!(value.as_text(), Some(r#"{"enabled": true}"#));
///
/// let names = Json::parse(r#"["a", "b"]"#).unwrap();
/// let names: Seq<'_, Str<'_>> = names.decode().unwrap();
/// assert_eq!(names.get(1).unwrap(), "b");
///
/// let value = Json::from_value(&42_u32);
/// let mut buffer = [0; 8];
/// let len = json::to_slice(&value, &mut buffer).unwrap();
/// assert_eq!(&buffer[..len], b"42");
/// ```
#[derive(Clone, Copy)]
pub struct Json<'a> {
    repr: Repr<'a>,
}

#[derive(Clone, Copy)]
enum Repr<'a> {
    /// JSON text that the reader has checked.
    Text(&'a str),
    Value(&'a dyn Encode),
}

impl<'a> Json<'a> {
    /// The `null` value.
    pub const NULL: Json<'static> = Json {
        repr: Repr::Text("null"),
    };

    /// Creates a value from JSON text.
    pub fn parse(text: &'a str) -> Result<Self, DecodeError> {
        json::from_str(text)
    }

    /// Creates a value that encodes as the given value.
    ///
    /// Such values cannot be decoded.
    pub const fn from_value(value: &'a dyn Encode) -> Self {
        Self {
            repr: Repr::Value(value),
        }
    }

    /// Returns the JSON text of the value, unless it was created with
    /// [`from_value`](Self::from_value).
    pub fn as_text(&self) -> Option<&'a str> {
        match self.repr {
            Repr::Text(text) => Some(text),
            Repr::Value(_) => None,
        }
    }

    /// Decodes the value as the given type.
    ///
    /// Fails for values created with [`from_value`](Self::from_value).
    pub fn decode<T: Decode<'a>>(&self) -> Result<T, DecodeError> {
        match self.repr {
            Repr::Text(text) => json::from_str(text),
            Repr::Value(_) => {
                Err(DecodeError::new(
                    DecodeErrorKind::Unexpected("JSON text"),
                    0,
                ))
            }
        }
    }
}

impl Default for Json<'_> {
    fn default() -> Self {
        Json::NULL
    }
}

impl fmt::Debug for Json<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Text(text) => f.write_str(text),
            Repr::Value(_) => f.write_str("Json(..)"),
        }
    }
}

impl<'a> Decode<'a> for Json<'a> {
    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        Ok(Self {
            repr: Repr::Text(reader.skip()?),
        })
    }
}

impl Encode for Json<'_> {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        match self.repr {
            Repr::Text(text) => writer.raw(text),
            Repr::Value(value) => value.encode(writer),
        }
    }
}
