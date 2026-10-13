//! Formats representing typed values.
//!
//! A format defines how the values of Sidex types are represented in a target
//! syntax and converts them to and from that syntax.
//!
//! - [`json`]: JSON. Its representation is the cross-language contract of Sidex, which
//!   the Rust, TypeScript, and Python backends all follow. JSON documents can also be
//!   held in memory as JSON values (see [`json::to_value`]).
//! - `toml`: TOML, using the representation of JSON. Requires the `toml` feature.
//!
//! Reading builds a [`JsonValue`] before decoding it. Configuration files and API
//! messages are small enough for this to be insignificant. Writing streams
//! without building a [`JsonValue`].
//!
//! [`JsonValue`]: crate::types::JsonValue

#[cfg(any(feature = "json", feature = "toml"))]
pub(crate) mod bridge;
pub mod json;
#[cfg(feature = "toml")]
pub mod toml;
mod view;
