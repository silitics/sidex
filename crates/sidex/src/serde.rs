//! Interoperability with Serde.
//!
//! Sidex types and Serde types can be nested in each other:
//!
//! - [`serialize`] and [`deserialize`] use the JSON representation of a Sidex type with
//!   any Serde serializer or deserializer. Together, they work with `#[serde(with =
//!   "sidex::serde")]` for fields of Sidex types in Serde types. Sidex generates
//!   `Serialize` and `Deserialize` implementations with them and this crate's types
//!   implement both traits.
//! - [`Serde`] wraps a Serde type for use in Sidex types, for instance, as the Rust type
//!   of an opaque type.
//!
//! Deserializers must be self-describing, like the deserializers of JSON, TOML,
//! or YAML.

use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::Serialize;
use ::serde::Serializer;
use ::serde::de::DeserializeOwned;
use ::serde::de::Error as _;
use ::serde::ser::Error as _;

use crate::Decode;
use crate::DecodeError;
use crate::Decoder;
use crate::Encode;
use crate::Shape;
use crate::Shaped;
use crate::Value;
use crate::View;
use crate::formats::bridge;
use crate::formats::bridge::SerializeView;
use crate::formats::json;
use crate::formats::json::text::DeserializeValue;
use crate::shape::OpaqueShape;
use crate::shape::ShapeKind;
use crate::types::Bytes;
use crate::types::Date;
use crate::types::Map;
use crate::types::Seq;
use crate::types::Str;
use crate::types::Timestamp;
use crate::types::Uuid;

/// Serializes `value` in its JSON representation.
pub fn serialize<T: Encode + ?Sized, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut serializer = Some(serializer);
    let mut failure = None;
    let result = bridge::encode(value, |view: &SerializeView<'_>| {
        let serializer = serializer.take().ok_or("value serialized twice")?;
        view.serialize(serializer).map_err(|error| {
            failure = Some(error);
            "serialization failed"
        })
    });
    match (result, failure) {
        (_, Some(error)) => Err(error),
        (Ok(ok), None) => Ok(ok),
        (Err(error), None) => Err(S::Error::custom(error)),
    }
}

/// Deserializes a value from its JSON representation.
pub fn deserialize<'de, T: Decode, D: Deserializer<'de>>(deserializer: D) -> Result<T, D::Error> {
    let DeserializeValue(value) = DeserializeValue::deserialize(deserializer)?;
    json::from_value(&value).map_err(D::Error::custom)
}

/// Serde type used as a Sidex type.
///
/// The wrapped value is represented as Serde serializes it with
/// [`serde_json`](https://docs.rs/serde_json). Its shape is opaque.
///
/// # Panics
///
/// Encoding panics if the wrapped value cannot be serialized as JSON, for
/// instance, because a map has non-string keys.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Serde<T>(pub T);

impl<T> Serde<T> {
    /// Returns the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Serde<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Serde<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for Serde<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: fmt::Display> fmt::Display for Serde<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Shaped for Serde<T> {
    const SHAPE: &'static Shape = &Shape::new(
        "::sidex::serde::Serde",
        ShapeKind::Opaque(OpaqueShape::new()),
    );
}

impl<T: Serialize> Encode for Serde<T> {
    fn encode(&self) -> View<'_> {
        let value = serde_json::to_value(&self.0)
            .and_then(DeserializeValue::deserialize)
            .unwrap_or_else(|error| panic!("cannot serialize value as JSON: {error}"));
        View::Value(Cow::Owned(value.0))
    }
}

impl<T: DeserializeOwned> Decode for Serde<T> {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        let value = decoder.value()?;
        let value = bridge::encode(&value, |view: &SerializeView<'_>| {
            serde_json::to_value(view)
        })
        .map_err(DecodeError::new)?;
        serde_json::from_value(value)
            .map(Self)
            .map_err(DecodeError::new)
    }
}

impl<T: Serialize> Serialize for Serde<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Serde<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self)
    }
}

/// Implements Serde's traits with the JSON representation.
macro_rules! impl_serde {
    ($([$($params:tt)*] $ty:ty,)*) => {
        $(
            impl<$($params)*> Serialize for $ty {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    serialize(self, serializer)
                }
            }

            impl<'de, $($params)*> Deserialize<'de> for $ty {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    deserialize(deserializer)
                }
            }
        )*
    };
}

impl_serde! {
    [] Value,
    [] Str,
    [] Bytes,
    [] Uuid,
    [] Timestamp,
    [] Date,
    [T: Encode + Decode] Seq<T>,
    [K: Encode + Decode + Eq + std::hash::Hash, V: Encode + Decode] Map<K, V>,
}
