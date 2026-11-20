//! The JSON format.
//!
//! Values are read from JSON text held in a byte slice with [`from_slice`] and
//! [`from_str`] and written to a byte slice with [`to_slice`] or to any
//! [`Sink`] with [`to_sink`]. The representation is Sidex's JSON
//! representation, the cross-language contract of Sidex, which the `sidex`
//! crate documents.
//!
//! Decoding checks the entire input against the type, so errors in the input
//! surface there. Decoded values borrow strings, sequences, maps, bytes, and JSON
//! values from the input (see [`types`](crate::types)).
//!
//! ```
//! use sidex_embedded::json;
//! use sidex_embedded::types::Seq;
//!
//! let numbers: Seq<'_, u64> = json::from_str("[1, 2, \"18446744073709551615\"]").unwrap();
//! assert_eq!(numbers.len(), 3);
//! assert_eq!(numbers.iter().max(), Some(u64::MAX));
//!
//! let mut buffer = [0; 64];
//! let len = json::to_slice(&numbers, &mut buffer).unwrap();
//! assert_eq!(&buffer[..len], b"[1,2,\"18446744073709551615\"]");
//! ```
//!
//! Implement [`Decode`] and [`Encode`] for custom types with a [`Reader`] and a
//! [`Writer`]. Their record and variant helpers are what generated code uses.
//!
//! # Differences from the `sidex` Crate
//!
//! Decoding always ignores unknown fields. Maps keep entries with duplicate
//! keys, which their iterators return in input order. Lookups find the last
//! entry with a key, like decoding a map with the `sidex` crate. Errors carry
//! the byte offset of the offending value instead of a path.

pub use self::reader::Array;
pub use self::reader::DEFAULT_MAX_DEPTH;
pub use self::reader::Kind;
pub use self::reader::Object;
pub use self::reader::Reader;
pub use self::records::External;
pub use self::records::Fields;
pub use self::records::case;
pub use self::writer::ArrayWriter;
use self::writer::CountingSink;
pub use self::writer::ObjectWriter;
pub use self::writer::Sink;
use self::writer::SliceSink;
pub use self::writer::Writer;
use crate::Decode;
use crate::Encode;
use crate::error::DecodeError;
use crate::error::EncodeError;

mod reader;
mod records;
mod writer;

/// Reads a value from JSON text.
pub fn from_slice<'a, T: Decode<'a>>(input: &'a [u8]) -> Result<T, DecodeError> {
    from_reader(Reader::new(input))
}

/// Reads a value from JSON text.
pub fn from_str<'a, T: Decode<'a>>(input: &'a str) -> Result<T, DecodeError> {
    from_slice(input.as_bytes())
}

/// Reads a value with the given reader, which must be at the start of the
/// input, and checks that only whitespace follows.
///
/// Use this to set the maximum depth:
///
/// ```
/// use sidex_embedded::json::{self, Reader};
/// use sidex_embedded::types::Json;
///
/// let reader = Reader::new(b"[[[]]]").with_max_depth(2);
/// assert!(json::from_reader::<Json<'_>>(reader).is_err());
/// ```
pub fn from_reader<'a, T: Decode<'a>>(mut reader: Reader<'a>) -> Result<T, DecodeError> {
    let value = T::decode(&mut reader)?;
    reader.finish()?;
    Ok(value)
}

/// Writes `value` to `buffer` and returns the number of bytes written.
pub fn to_slice<T: Encode + ?Sized>(value: &T, buffer: &mut [u8]) -> Result<usize, EncodeError> {
    let mut sink = SliceSink { buffer, len: 0 };
    to_sink(value, &mut sink)?;
    Ok(sink.len)
}

/// Writes `value` to `sink`.
pub fn to_sink<T: Encode + ?Sized>(value: &T, sink: &mut dyn Sink) -> Result<(), EncodeError> {
    value.encode(&mut Writer::new(sink))
}

/// Returns the number of bytes that writing `value` produces.
///
/// Use this to size buffers or to set the length of HTTP requests.
pub fn encoded_len<T: Encode + ?Sized>(value: &T) -> Result<usize, EncodeError> {
    let mut sink = CountingSink(0);
    to_sink(value, &mut sink)?;
    Ok(sink.0)
}
