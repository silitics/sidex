//! Rust types of Sidex's built-in types.
//!
//! This module provides the Rust types of Sidex's built-in types and implements
//! [`Shaped`], [`Encode`], and [`Decode`] for them. Scalars map to Rust's
//! primitives, for instance, `u64` to [`u64`]. For types without a standard
//! counterpart, such as `timestamp`, the module provides its own, such as
//! [`Timestamp`].
//!
//! Strings, sequences, maps, and bytes are [`Str`], [`Seq`], [`Map`], and
//! [`Bytes`]. Clones share their data, which makes cloning large values cheap,
//! and mutation copies shared data first. Maps preserve the insertion order of
//! their entries. The traits are also implemented for [`String`], [`Vec`],
//! [`HashMap`](std::collections::HashMap), and
//! [`BTreeMap`](std::collections::BTreeMap).
//!
//! This crate does not expose third-party crates. Compatibility features add
//! conversions between the types of this module and those of popular crates.
//! Each feature is named after the crate and its major version:
//!
//! - `indexmap-2`: [`Map`] and `indexmap::IndexMap`.
//! - `jiff-02`: [`Timestamp`] and `jiff::Timestamp`, [`Date`] and `jiff::civil::Date`.
//! - `uuid-1`: [`Uuid`] and `uuid::Uuid`.
//!
//! [`Shaped`]: crate::Shaped
//! [`Encode`]: crate::Encode
//! [`Decode`]: crate::Decode

use std::fmt;

pub use self::bytes::Bytes;
pub use self::map::Map;
pub use self::seq::Seq;
pub use self::string::Str;
pub use self::time::Date;
pub use self::time::Timestamp;
pub use self::uuid::Uuid;

/// Implements the traits for a `Copy` type representing a scalar.
macro_rules! impl_scalar {
    ($ty:ty, $name:literal, $scalar:ident, $method:ident) => {
        impl $crate::Shaped for $ty {
            const SHAPE: &'static $crate::Shape = &$crate::Shape::new(
                concat!("::core::builtins::", $name),
                $crate::shape::ShapeKind::Scalar($crate::shape::Scalar::$scalar),
            );
        }

        impl $crate::Encode for $ty {
            fn encode(&self) -> $crate::View<'_> {
                $crate::View::$scalar(*self)
            }
        }

        impl $crate::Decode for $ty {
            fn decode(decoder: &mut dyn $crate::Decoder) -> Result<Self, $crate::DecodeError> {
                decoder.$method()
            }
        }
    };
}

mod bytes;
pub mod map;
mod seq;
mod standard;
mod string;
mod time;
mod uuid;

/// Error parsing a built-in type from its text representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    expected: &'static str,
}

impl ParseError {
    fn new(expected: &'static str) -> Self {
        Self { expected }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}", self.expected)
    }
}

impl std::error::Error for ParseError {}
