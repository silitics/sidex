//! Rust types of Sidex's built-in types.
//!
//! Scalars map to Rust's primitives, for instance, `u64` to [`u64`]. `uuid`,
//! `timestamp`, and `date` map to [`Uuid`], [`Timestamp`], and [`Date`].
//!
//! Strings, sequences, maps, bytes, and JSON values are [`Str`], [`Seq`],
//! [`Map`], [`Bytes`], and [`Json`]. They borrow their data and are `Copy`.
//! Decoded values borrow from the input, which is not copied. Applications
//! create values to encode from data they own, for instance, from a slice with
//! [`Seq::new`].

use core::fmt;

pub use self::bytes::Bytes;
pub use self::json::Json;
pub use self::map::Map;
pub use self::seq::Seq;
pub use self::str::Chars;
pub use self::str::Str;
pub use self::time::Date;
pub use self::time::Timestamp;
pub use self::uuid::Uuid;

/// Implements the traits for a type with a text representation of at most
/// `$capacity` bytes, using its `Display` and `FromStr` implementations.
macro_rules! impl_text {
    ($ty:ty, $expected:literal, $capacity:literal) => {
        impl crate::Decode<'_> for $ty {
            fn decode(reader: &mut crate::json::Reader<'_>) -> Result<Self, crate::DecodeError> {
                let error = reader.error(crate::DecodeErrorKind::Unexpected($expected));
                let text = reader.string_as($expected)?;
                <Self as crate::MapKey<'_>>::decode_key(text).map_err(|_| error)
            }
        }

        impl crate::Encode for $ty {
            fn encode(
                &self,
                writer: &mut crate::json::Writer<'_>,
            ) -> Result<(), crate::EncodeError> {
                writer.fmt_str(format_args!("{self}"))
            }
        }

        impl crate::MapKey<'_> for $ty {
            fn decode_key(key: crate::types::Str<'_>) -> Result<Self, crate::DecodeErrorKind> {
                key.with_text::<$capacity, _>(|text| text.parse().ok())
                    .flatten()
                    .ok_or(crate::DecodeErrorKind::Unexpected($expected))
            }

            fn encode_key(
                &self,
                writer: &mut crate::json::Writer<'_>,
            ) -> Result<(), crate::EncodeError> {
                crate::Encode::encode(self, writer)
            }
        }
    };
}

pub mod bytes;
mod json;
pub mod map;
mod scalars;
pub mod seq;
pub(crate) mod str;
mod time;
mod uuid;

/// Error parsing a built-in type from its text representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError {
    expected: &'static str,
}

impl ParseError {
    const fn new(expected: &'static str) -> Self {
        Self { expected }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}", self.expected)
    }
}

impl core::error::Error for ParseError {}
