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
//! conversions between the types of this module and those of popular crates,
//! or implement their traits. Each feature is named after the crate and its
//! major version:
//!
//! - `indexmap-2`: [`Map`] and `indexmap::IndexMap`.
//! - `jiff-02`: [`Timestamp`] and `jiff::Timestamp`, [`Date`] and `jiff::civil::Date`.
//! - `uuid-1`: [`Uuid`] and `uuid::Uuid`.
//! - `sqlx-08`: [`Str`] in SQL databases, stored like `String`.
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

/// Implements [`Shaped`], [`Encode`], and [`Decode`] for a type represented as a
/// string, using its [`Display`] and [`FromStr`] implementations.
///
/// The shape of the type is opaque, with a path made of the module path and the
/// name of the type. In JSON, values are strings.
///
/// ```
/// use std::fmt;
/// use std::str::FromStr;
///
/// /// Version of the form `MAJOR.MINOR`.
/// #[derive(Debug, PartialEq)]
/// struct Version(u32, u32);
///
/// impl fmt::Display for Version {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "{}.{}", self.0, self.1)
///     }
/// }
///
/// impl FromStr for Version {
///     type Err = &'static str;
///
///     fn from_str(text: &str) -> Result<Self, Self::Err> {
///         let (major, minor) = text.split_once('.').ok_or("expected `MAJOR.MINOR`")?;
///         Ok(Self(
///             major.parse().map_err(|_| "invalid major version")?,
///             minor.parse().map_err(|_| "invalid minor version")?,
///         ))
///     }
/// }
///
/// sidex::types::impl_text!(Version);
///
/// let text = sidex::formats::json::to_string(&Version(1, 2)).unwrap();
/// assert_eq!(text, r#""1.2""#);
/// assert_eq!(sidex::formats::json::from_str::<Version>(&text).unwrap(), Version(1, 2));
/// ```
///
/// [`Shaped`]: crate::Shaped
/// [`Encode`]: crate::Encode
/// [`Decode`]: crate::Decode
/// [`Display`]: std::fmt::Display
/// [`FromStr`]: std::str::FromStr
#[doc(inline)]
pub use crate::__impl_text as impl_text;

/// Implementation of [`impl_text`].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_text {
    ($ty:ty) => {
        impl $crate::Shaped for $ty {
            const SHAPE: &'static $crate::Shape = &$crate::Shape::new(
                ::std::concat!("::", ::std::module_path!(), "::", ::std::stringify!($ty)),
                $crate::shape::ShapeKind::Opaque(
                    $crate::shape::OpaqueShape::new()
                        .json_types(&[$crate::formats::json::Type::String]),
                ),
            );
        }

        impl $crate::Encode for $ty {
            fn encode(&self) -> $crate::View<'_> {
                $crate::View::Value(::std::borrow::Cow::Owned($crate::Value::String(
                    ::std::string::ToString::to_string(self).into(),
                )))
            }
        }

        impl $crate::Decode for $ty {
            fn decode(
                decoder: &mut dyn $crate::Decoder,
            ) -> ::std::result::Result<Self, $crate::DecodeError> {
                ::std::str::FromStr::from_str(&decoder.string()?).map_err($crate::DecodeError::new)
            }
        }
    };
}

mod bytes;
pub mod map;
mod seq;
#[cfg(feature = "sqlx-08")]
mod sqlx;
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
