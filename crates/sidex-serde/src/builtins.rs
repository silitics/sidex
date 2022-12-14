//! Helpers for serializing and deserializing Sidex's builtin types with Serde.
//!
//! Some of the builtins like floating-point numbers are serialized differently than they
//! normally are with Serde. For instance, in case of floating-point numbers, for
//! human-readable formats, they are serialized as strings if they are not finite.

use serde::{Deserialize, Serialize};

/// Constant string for representing `NaN`.
pub const FLOAT_NAN: &'static str = "NaN";
/// Constant string for representing positive infinity.
pub const FLOAT_POSITIVE_INFINITY: &'static str = "+Infinity";
/// Constant string for representing negative infinity.
pub const FLOAT_NEGATIVE_INFINITY: &'static str = "-Infinity";

/// Abstraction for serializing and deserializing floating-point numbers.
pub trait Float: Serialize + for<'de> Deserialize<'de> {
    /// Indicates whether the number is finite, i.e., neither `NaN` nor infinity.
    fn is_finite(&self) -> bool;
    /// Indicates whether the number is `NaN`.
    fn is_nan(&self) -> bool;
    /// Indicates whether the number is positive.
    fn is_positive(&self) -> bool;
}

/// Helper macro for implementing [`Float`].
macro_rules! impl_float {
    ($float_type:ty) => {
        impl Float for $float_type {
            #[inline]
            fn is_finite(&self) -> bool {
                <$float_type>::is_finite(*self)
            }

            #[inline]
            fn is_nan(&self) -> bool {
                <$float_type>::is_nan(*self)
            }

            #[inline]
            fn is_positive(&self) -> bool {
                <$float_type>::is_sign_positive(*self)
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

/// Serialize a floating-point number.
pub fn serialize_float<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Float,
    S: serde::Serializer,
{
    if serializer.is_human_readable() && !value.is_finite() {
        // The float is either NaN or infinity and the serialization format is human-readable.
        // Therefore, it should be serialized as a string.
        if value.is_nan() {
            serializer.serialize_str(FLOAT_NAN)
        } else if value.is_positive() {
            serializer.serialize_str(FLOAT_POSITIVE_INFINITY)
        } else {
            // If the float is neither NaN nor positive, then it must be negative infinity.
            serializer.serialize_str(FLOAT_NEGATIVE_INFINITY)
        }
    } else {
        value.serialize(serializer)
    }
}

/// Serialize bytes.
pub fn serialize_bytes<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: serde::Serializer,
{
    if serializer.is_human_readable() {
        let encoded = base64::encode(value);
        serializer.serialize_str(&encoded)
    } else {
        serializer.serialize_bytes(value.as_ref())
    }
}
