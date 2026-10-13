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
use ::serde::de::IntoDeserializer;
use ::serde::de::MapAccess;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;
use ::serde::de::value::MapAccessDeserializer;
use ::serde::de::value::MapDeserializer;
use ::serde::de::value::SeqDeserializer;
use ::serde::ser::Error as _;

use crate::Decode;
use crate::DecodeError;
use crate::Decoder;
use crate::Encode;
use crate::Shape;
use crate::Shaped;
use crate::View;
use crate::formats::bridge;
use crate::formats::bridge::SerializeView;
use crate::formats::bridge::Syntax;
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
use crate::types::JsonValue;
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
    let result = bridge::encode(value, Syntax::Json, |view: &SerializeView<'_, '_>| {
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

/// Reads a JSON value, using the shape as a hint for the deserializer.
struct Guided(&'static Shape);

impl<'de> DeserializeSeed<'de> for Guided {
    type Value = JsonValue;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<JsonValue, D::Error> {
        match &self.0.kind {
            ShapeKind::Scalar(scalar) => {
                match scalar {
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
            ShapeKind::Variant(_) | ShapeKind::Opaque(_) | ShapeKind::Json => {
                deserializer.deserialize_any(ValueVisitor)
            }
        }
    }
}

/// Reads an optional field, whose absence `null` may represent.
struct Optional(&'static Shape);

impl<'de> DeserializeSeed<'de> for Optional {
    type Value = JsonValue;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<JsonValue, D::Error> {
        deserializer.deserialize_option(self)
    }
}

impl<'de> Visitor<'de> for Optional {
    type Value = JsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "an optional value of `{}`", self.0)
    }

    fn visit_none<E>(self) -> Result<JsonValue, E> {
        Ok(JsonValue::Null)
    }

    fn visit_unit<E>(self) -> Result<JsonValue, E> {
        Ok(JsonValue::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<JsonValue, D::Error> {
        Guided(self.0).deserialize(deserializer)
    }
}

/// Reads the items of a sequence.
struct Items(&'static Shape);

impl<'de> Visitor<'de> for Items {
    type Value = JsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a sequence of `{}`", self.0)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<JsonValue, A::Error> {
        let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(item) = seq.next_element_seed(Guided(self.0))? {
            items.push(item);
        }
        Ok(JsonValue::Seq(items.into()))
    }
}

/// Reads the entries of a map or the fields of a record.
enum Entries {
    Map(&'static Shape),
    Record(&'static RecordShape),
}

impl<'de> Visitor<'de> for Entries {
    type Value = JsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<JsonValue, A::Error> {
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
        Ok(JsonValue::Map(entries))
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
        encode_serde(&self.0)
    }
}

impl<T: DeserializeOwned> Decode for Serde<T> {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decode_serde(decoder).map(Self)
    }
}

/// Implements [`Shaped`], [`Encode`], and [`Decode`] for a Serde type.
///
/// Like with [`Serde`], values are represented as Serde serializes them with
/// [`serde_json`](https://docs.rs/serde_json), and the shape is opaque, with
/// a path made of the module path and the name of the type. Use this for the
/// Rust types of opaque types, so that their values need no wrapper.
///
/// ```
/// use serde::Deserialize;
/// use serde::Serialize;
///
/// /// Reading of a sensor, either a number or a fault description.
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// #[serde(untagged)]
/// enum Reading {
///     Value(f64),
///     Fault(String),
/// }
///
/// sidex::serde::impl_opaque!(Reading);
///
/// let text = sidex::formats::json::to_string(&Reading::Value(1.5)).unwrap();
/// assert_eq!(text, "1.5");
/// let reading: Reading = sidex::formats::json::from_str(r#""overheated""#).unwrap();
/// assert_eq!(reading, Reading::Fault("overheated".to_owned()));
/// ```
///
/// # Panics
///
/// Encoding panics if a value cannot be serialized as JSON, for instance,
/// because a map has non-string keys.
#[doc(inline)]
pub use crate::__impl_serde_opaque as impl_opaque;

/// Implementation of [`impl_opaque`].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_serde_opaque {
    ($ty:ty) => {
        impl $crate::Shaped for $ty {
            const SHAPE: &'static $crate::Shape = &$crate::Shape::new(
                ::std::concat!("::", ::std::module_path!(), "::", ::std::stringify!($ty)),
                $crate::shape::ShapeKind::Opaque($crate::shape::OpaqueShape::new()),
            );
        }

        impl $crate::Encode for $ty {
            fn encode(&self) -> $crate::View<'_> {
                $crate::serde::encode_serde(self)
            }
        }

        impl $crate::Decode for $ty {
            fn decode(
                decoder: &mut dyn $crate::Decoder,
            ) -> ::std::result::Result<Self, $crate::DecodeError> {
                $crate::serde::decode_serde(decoder)
            }
        }
    };
}

/// Encodes a Serde value as Serde serializes it with `serde_json`.
#[doc(hidden)]
pub fn encode_serde<T: Serialize + ?Sized>(value: &T) -> View<'static> {
    let value = serde_json::to_value(value)
        .and_then(DeserializeValue::deserialize)
        .unwrap_or_else(|error| panic!("cannot serialize value as JSON: {error}"));
    View::Json(Cow::Owned(value.0))
}

/// Decodes a Serde value from its `serde_json` representation.
#[doc(hidden)]
pub fn decode_serde<T: DeserializeOwned>(decoder: &mut dyn Decoder) -> Result<T, DecodeError> {
    let value = decoder.json()?;
    T::deserialize(ValueDeserializer(&value)).map_err(DecodeError::new)
}

/// Deserializer reading a JSON value like a self-describing format.
///
/// Unlike `serde_json::Value`, it passes values to the visitor whatever type
/// the deserialized type requests, like TOML's deserializer does. Hence, types
/// requesting an integer may also accept strings, for instance, byte sizes
/// like `"4GiB"` in configuration files.
struct ValueDeserializer<'v>(&'v JsonValue);

impl<'de> Deserializer<'de> for ValueDeserializer<'_> {
    type Error = ::serde::de::value::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            JsonValue::Null => visitor.visit_unit(),
            JsonValue::Bool(value) => visitor.visit_bool(*value),
            JsonValue::Number(number) => {
                if let Some(value) = number.as_u64() {
                    visitor.visit_u64(value)
                } else if let Some(value) = number.as_i64() {
                    visitor.visit_i64(value)
                } else {
                    visitor.visit_f64(number.as_f64())
                }
            }
            JsonValue::String(value) => visitor.visit_str(value),
            JsonValue::Seq(items) => {
                let mut seq = SeqDeserializer::new(items.iter().map(ValueDeserializer));
                let value = visitor.visit_seq(&mut seq)?;
                seq.end()?;
                Ok(value)
            }
            JsonValue::Map(entries) => {
                let mut map = MapDeserializer::new(
                    entries
                        .iter()
                        .map(|(key, value)| (key.as_str(), ValueDeserializer(value))),
                );
                let value = visitor.visit_map(&mut map)?;
                map.end()?;
                Ok(value)
            }
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.0 {
            JsonValue::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    /// Reads a unit variant from a string and other variants from a map with a
    /// single entry, like `serde_json` does.
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match self.0 {
            JsonValue::String(name) => visitor.visit_enum(name.as_str().into_deserializer()),
            JsonValue::Map(entries) if entries.len() == 1 => {
                visitor.visit_enum(MapAccessDeserializer::new(MapDeserializer::new(
                    entries
                        .iter()
                        .map(|(key, value)| (key.as_str(), ValueDeserializer(value))),
                )))
            }
            _ => {
                Err(::serde::de::Error::custom(
                    "expected a string or a map with a single entry",
                ))
            }
        }
    }

    ::serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
        byte_buf unit unit_struct seq tuple tuple_struct map struct identifier ignored_any
    }
}

impl IntoDeserializer<'_, ::serde::de::value::Error> for ValueDeserializer<'_> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
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
    [] JsonValue,
    [] Str,
    [] Bytes,
    [] Uuid,
    [] Timestamp,
    [] Date,
    [T: Shaped + Encode + Decode] Seq<T>,
    [K: Shaped + Encode + Decode + Eq + std::hash::Hash, V: Shaped + Encode + Decode] Map<K, V>,
}
