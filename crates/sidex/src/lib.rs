//! Runtime support for Sidex types in Rust.
//!
//! Sidex generates Rust types from schemas. This crate provides what those types
//! need at runtime: descriptions of their structure, a dyn-safe way to encode and
//! decode them, JSON values, and formats.
//!
//! # Concepts
//!
//! Every Sidex type has a [`Shape`], a static description of its structure,
//! documentation, attributes, stability, and validation rules. Types provide
//! their shape by implementing [`Shaped`] (see [`shape`]). Rules can be checked
//! at runtime (see [`validation`]).
//!
//! Types implement [`Encode`] by returning a [`View`], which exposes their data
//! in terms of the Sidex data model (see [`encode`]), and [`Decode`] by
//! requesting their data from a [`Decoder`] (see [`decode`]). Both traits are
//! dyn-safe and know nothing about formats.
//!
//! A *format* defines how Sidex types are represented in a target syntax and
//! converts typed values to and from it (see [`formats`]). The JSON format is
//! the cross-language contract of Sidex.
//!
//! A [`JsonValue`] is self-describing data following the JSON data
//! model (see [`value`]). Types use values for data without a schema, and the
//! JSON format can hold documents in values.
//!
//! The [`types`] module provides the Rust types of Sidex's built-in types. Its
//! strings, sequences, maps, and bytes are cheap to clone, and its maps preserve
//! insertion order. This crate does not expose third-party crates in its
//! interfaces. Compatibility features implement the traits for the types of
//! popular crates instead.
//!
//! # Example
//!
//! The following record is written by hand, the way Sidex generates it:
//!
//! ```
//! use sidex::encode::{RecordView, View};
//! use sidex::formats::json;
//! use sidex::shape::{FieldShape, RecordShape, ShapeKind, shape_of};
//! use sidex::{Decode, DecodeError, Decoder, Encode, Shape, Shaped, JsonValue};
//!
//! /// A device.
//! struct Device {
//!     id: u64,
//!     display_name: Option<String>,
//! }
//!
//! impl Shaped for Device {
//!     const SHAPE: &'static Shape = &Shape::new(
//!         "::example::devices::Device",
//!         ShapeKind::Record(RecordShape::new(&[
//!             FieldShape::new("id", shape_of::<u64>),
//!             FieldShape::new("display_name", shape_of::<String>)
//!                 .optional()
//!                 .json_name("displayName"),
//!         ])),
//!     )
//!     .with_docs("A device.");
//! }
//!
//! impl Encode for Device {
//!     fn encode(&self) -> View<'_> {
//!         View::Record(self)
//!     }
//! }
//!
//! impl RecordView for Device {
//!     fn shape(&self) -> &'static Shape {
//!         Self::SHAPE
//!     }
//!
//!     fn field(&self, index: usize) -> Option<View<'_>> {
//!         match index {
//!             0 => Some(self.id.encode()),
//!             1 => self.display_name.as_ref().map(Encode::encode),
//!             _ => None,
//!         }
//!     }
//! }
//!
//! impl Decode for Device {
//!     fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
//!         let (mut id, mut display_name) = (None, None);
//!         decoder.record(Self::SHAPE, &mut |index, field| {
//!             match index {
//!                 0 => id = Some(u64::decode(field)?),
//!                 1 => display_name = Some(String::decode(field)?),
//!                 _ => {}
//!             }
//!             Ok(())
//!         })?;
//!         Ok(Self {
//!             id: id.ok_or_else(|| DecodeError::new("missing field `id`"))?,
//!             display_name,
//!         })
//!     }
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let device = Device { id: 1 << 60, display_name: None };
//! let value = json::to_value(&device)?;
//! assert_eq!(value, JsonValue::from_iter([("id", "1152921504606846976")]));
//!
//! let decoded: Device = json::from_value(&value)?;
//! assert_eq!(decoded.id, device.id);
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - `validation` (default): Evaluation of validation rules (see [`validation`]).
//! - `json` (default): JSON text (see [`formats::json`]).
//! - `toml`: TOML format (see [`formats`]).
//! - `serde-1`: Interoperability with Serde (see `serde`).
//! - `indexmap-2`, `jiff-02`, `uuid-1`: Conversions to and from types of third-party
//!   crates and, for their scalar types, implementations of the traits (see [`types`]).
//! - `serde_json-1`: Conversions between [`JsonValue`] and `serde_json::Value`, which
//!   also implements the traits like [`JsonValue`].
//! - `schemars-1`: Schemas for `schemars` (see [`formats::json`]).
//! - `sqlx-08`, `rusqlite-037`: Storing strings in SQL databases with `sqlx` or
//!   `rusqlite`.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod decode;
pub mod encode;
pub mod formats;
pub mod shape;
pub mod types;
pub mod validation;
pub mod value;

#[cfg(feature = "serde-1")]
pub mod serde;

pub use crate::decode::Decode;
pub use crate::decode::DecodeError;
pub use crate::decode::Decoder;
pub use crate::encode::Encode;
pub use crate::encode::EncodeError;
pub use crate::encode::View;
pub use crate::shape::Shape;
pub use crate::shape::Shaped;
pub use crate::value::JsonValue;

/// Macro for including bundles build with [`sidex_build_rs`].
///
/// [`sidex_build_rs`]: https://docs.rs/sidex_build_rs/latest/sidex_build_rs/
///
///
/// ## Examples
///
/// To include a bundle named `bundle_name` as a module with the same name, use the
/// following macro invocation:
///
/// ```ignore
/// sidex::include_bundle!(pub bundle_name);
/// ```
///
/// Here, `pub` specifies that the generated module should be public. Without it, the
/// generated module would be private.
///
/// Using the `as` keyword, it is also possible to give the generated module a different
/// name than the bundle name. For instance, the following macro invocation includes the
/// bundle named `bundle_name` as a module named `mod_name`:
///
/// ```ignore
/// sidex::include_bundle!(pub bundle_name as mod_name);
/// ```
///
/// Additional attributes for the module can be given using the standard Rust syntax. For
/// instance:
///
/// ```ignore
/// sidex::include_bundle! {
///     #[cfg(feature = "include-bundle")]
///     bundle_name as mod_name
/// }
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
