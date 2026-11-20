//! Allocation-free runtime support for Sidex types on embedded targets.
//!
//! Sidex's `rust-embedded` backend generates Rust types for microcontrollers
//! and other targets without an allocator. This crate provides what those
//! types need at runtime: the Rust types of Sidex's built-in types and the
//! JSON format. It is `no_std`, never allocates, and has no dependencies.
//!
//! # Concepts
//!
//! Types implement [`Decode`] by reading their JSON representation with a
//! [`Reader`](json::Reader) and [`Encode`] by writing it with a
//! [`Writer`](json::Writer). Types that can be keys of maps also implement
//! [`MapKey`].
//!
//! Decoding reads from a byte slice holding the entire input. Decoded values
//! borrow strings, sequences, maps, bytes, and JSON values from the input
//! instead of copying them, so the input must outlive them. Their types, such
//! as [`Str`](types::Str) and [`Seq`](types::Seq), keep the input and decode
//! it on access (see [`types`]). Decoding checks the entire input, so
//! iterating over sequences and maps cannot fail.
//!
//! Encoding writes to a byte slice or to a [`Sink`](json::Sink), for instance,
//! a network connection. Values to encode borrow their data from the
//! application.
//!
//! # Example
//!
//! The following record is written by hand, the way Sidex generates it:
//!
//! ```
//! use sidex_embedded::json::{self, Reader, Writer, ObjectWriter};
//! use sidex_embedded::types::Str;
//! use sidex_embedded::{Decode, DecodeError, Encode, EncodeError};
//!
//! /// A device.
//! #[derive(Debug, Clone, Copy, PartialEq)]
//! struct Device<'a> {
//!     id: u64,
//!     display_name: Option<Str<'a>>,
//! }
//!
//! impl<'a> Decode<'a> for Device<'a> {
//!     const IS_RECORD: bool = true;
//!
//!     fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
//!         let fields = reader.record(&[&["id"], &["displayName"]])?;
//!         Ok(Self {
//!             id: fields.required(0)?,
//!             display_name: fields.optional(1)?,
//!         })
//!     }
//! }
//!
//! impl Encode for Device<'_> {
//!     fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
//!         let mut object = writer.object()?;
//!         self.encode_fields(&mut object)?;
//!         object.end()
//!     }
//!
//!     fn is_record(&self) -> bool {
//!         true
//!     }
//!
//!     fn encode_fields(&self, object: &mut ObjectWriter<'_, '_>) -> Result<(), EncodeError> {
//!         self.id.encode(object.field("id")?)?;
//!         if let Some(display_name) = &self.display_name {
//!             display_name.encode(object.field("displayName")?)?;
//!         }
//!         Ok(())
//!     }
//! }
//!
//! let input = br#"{"id": "1152921504606846976", "displayName": "Sensor \"A\""}"#;
//! let device: Device<'_> = json::from_slice(input).unwrap();
//! assert_eq!(device.id, 1 << 60);
//! assert_eq!(device.display_name.unwrap(), "Sensor \"A\"");
//!
//! let mut buffer = [0; 64];
//! let len = json::to_slice(&device, &mut buffer).unwrap();
//! assert_eq!(&buffer[..len], br#"{"id":"1152921504606846976","displayName":"Sensor \"A\""}"#);
//! ```

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
extern crate std;

pub use crate::error::BufferTooSmall;
pub use crate::error::DecodeError;
pub use crate::error::DecodeErrorKind;
pub use crate::error::EncodeError;
use crate::json::ObjectWriter;
use crate::json::Reader;
use crate::json::Writer;
use crate::types::Str;

mod error;
pub mod json;
pub mod types;

/// Types that can be decoded.
///
/// Sequences and maps decode their items again on access and panic if that
/// fails, so decoding must succeed whenever it succeeded for the same input
/// before.
pub trait Decode<'a>: Sized {
    /// Indicates whether the type is a record.
    ///
    /// Internally tagged variants merge the fields of record payloads into the
    /// map holding the tag.
    const IS_RECORD: bool = false;

    /// Reads exactly one value.
    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError>;
}

/// Types that can be encoded.
pub trait Encode {
    /// Writes the value.
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError>;

    /// Indicates whether the value is a record, like [`Decode::IS_RECORD`].
    fn is_record(&self) -> bool {
        false
    }

    /// Writes the fields of a record into an object, which inline fields and
    /// internally tagged variants use.
    ///
    /// Fails with [`EncodeError::NotARecord`] unless the value is a record or
    /// wraps one.
    fn encode_fields(&self, object: &mut ObjectWriter<'_, '_>) -> Result<(), EncodeError> {
        let _ = object;
        Err(EncodeError::NotARecord)
    }
}

/// Types that can be keys of maps.
///
/// Keys are strings in JSON. Integers, UUIDs, timestamps, and dates use their
/// text representation.
pub trait MapKey<'a>: Sized {
    /// Decodes a key from its text.
    ///
    /// Like [`Decode::decode`], decoding must succeed whenever it succeeded for
    /// the same key before.
    fn decode_key(key: Str<'a>) -> Result<Self, DecodeErrorKind>;

    /// Writes the key as a string.
    fn encode_key(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError>;
}

/// Includes a bundle generated by `sidex_build_rs` with its `embedded` option.
///
/// To include a bundle named `bundle_name` as a public module with the same
/// name, use:
///
/// ```ignore
/// sidex_embedded::include_bundle!(pub bundle_name);
/// ```
///
/// With `as`, the module gets a different name than the bundle:
///
/// ```ignore
/// sidex_embedded::include_bundle!(pub bundle_name as mod_name);
/// ```
#[macro_export]
macro_rules! include_bundle {
    ($(#[$meta:meta])* $vis:vis $bundle_name:ident) => {
        $crate::include_bundle!($(#[$meta])* $vis $bundle_name as $bundle_name);
    };
    ($(#[$meta:meta])* $vis:vis $bundle_name:ident as $mod_name:ident) => {
        $(#[$meta])*
        $vis mod $mod_name {
            include!(concat!(
                env!("OUT_DIR"),
                "/sidex-bundles/",
                stringify!($bundle_name),
                "/mod.rs"
            ));
        }
    };
}
