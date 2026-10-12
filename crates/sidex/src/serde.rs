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
//! Deserialization uses the shape of a type as a hint for the deserializer,
//! so that formats representing everything as strings, like query strings, can
//! parse scalars. Variants and opaque types require self-describing formats.

use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::Serialize;
use ::serde::Serializer;
use ::serde::de::DeserializeOwned;
use ::serde::de::DeserializeSeed;
use ::serde::de::Error as _;
use ::serde::de::MapAccess;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;
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
use crate::formats::json::text::ValueVisitor;
use crate::shape::FieldShape;
use crate::shape::OpaqueShape;
use crate::shape::RecordShape;
use crate::shape::Scalar;
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
pub fn deserialize<'de, T: Shaped + Decode, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    let value = Guided(T::SHAPE).deserialize(deserializer)?;
    json::from_value(&value).map_err(D::Error::custom)
}

/// Reads a freeform value, using the shape as a hint for the deserializer.
struct Guided(&'static Shape);

impl<'de> DeserializeSeed<'de> for Guided {
    type Value = Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        match &self.0.kind {
            ShapeKind::Scalar(scalar) => {
                match scalar {
                    Scalar::Unit => deserializer.deserialize_unit(ValueVisitor),
                    Scalar::Bool => deserializer.deserialize_bool(ValueVisitor),
                    Scalar::I8 | Scalar::I16 | Scalar::I32 => {
                        deserializer.deserialize_i64(ValueVisitor)
                    }
                    Scalar::U8 | Scalar::U16 | Scalar::U32 | Scalar::Idx => {
                        deserializer.deserialize_u64(ValueVisitor)
                    }
                    Scalar::String | Scalar::Uuid | Scalar::Timestamp | Scalar::Date => {
                        deserializer.deserialize_string(ValueVisitor)
                    }
                    // Their representations are numbers or strings.
                    Scalar::I64 | Scalar::U64 | Scalar::F32 | Scalar::F64 | Scalar::Bytes => {
                        deserializer.deserialize_any(ValueVisitor)
                    }
                }
            }
            ShapeKind::Sequence(item) => deserializer.deserialize_seq(Items(item())),
            ShapeKind::Map(_, value) => deserializer.deserialize_map(Entries::Map(value())),
            ShapeKind::Record(record) => deserializer.deserialize_map(Entries::Record(record)),
            ShapeKind::Wrapper(inner) => Guided(inner()).deserialize(deserializer),
            ShapeKind::Variant(_) | ShapeKind::Opaque(_) | ShapeKind::Value => {
                deserializer.deserialize_any(ValueVisitor)
            }
        }
    }
}

/// Reads an optional field, whose absence `null` may represent.
struct Optional(&'static Shape);

impl<'de> DeserializeSeed<'de> for Optional {
    type Value = Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_option(self)
    }
}

impl<'de> Visitor<'de> for Optional {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "an optional value of `{}`", self.0)
    }

    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        Guided(self.0).deserialize(deserializer)
    }
}

/// Reads the items of a sequence.
struct Items(&'static Shape);

impl<'de> Visitor<'de> for Items {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a sequence of `{}`", self.0)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(item) = seq.next_element_seed(Guided(self.0))? {
            items.push(item);
        }
        Ok(Value::Seq(items.into()))
    }
}

/// Reads the entries of a map or the fields of a record.
enum Entries {
    Map(&'static Shape),
    Record(&'static RecordShape),
}

impl<'de> Visitor<'de> for Entries {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut entries = Map::with_capacity(map.size_hint().unwrap_or(0));
        while let Some(key) = map.next_key::<String>()? {
            let value = match &self {
                Entries::Map(value) => map.next_value_seed(Guided(value))?,
                Entries::Record(record) => {
                    match find_field(record, &key) {
                        Some(field) if field.optional => {
                            map.next_value_seed(Optional((field.shape)()))?
                        }
                        Some(field) => map.next_value_seed(Guided((field.shape)()))?,
                        // Unknown fields are kept, so that decoding can reject them.
                        None => map.next_value::<DeserializeValue>()?.0,
                    }
                }
            };
            entries.insert(Str::from(key), value);
        }
        Ok(Value::Map(entries))
    }
}

/// Finds the field with the given JSON name, also among inline records.
fn find_field(record: &'static RecordShape, name: &str) -> Option<&'static FieldShape> {
    record.fields.iter().find_map(|field| {
        if field.json.inline {
            (field.shape)()
                .as_record()
                .and_then(|inner| find_field(inner, name))
        } else {
            (field.json.name == name).then_some(field)
        }
    })
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
    [T: Shaped + Encode + Decode] Seq<T>,
    [K: Shaped + Encode + Decode + Eq + std::hash::Hash, V: Shaped + Encode + Decode] Map<K, V>,
}
