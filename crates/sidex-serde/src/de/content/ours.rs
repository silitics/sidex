//! Our own implementation of the private Serde `Content` enum.

use std::marker::PhantomData;

use serde::{
    de::{Unexpected, Visitor},
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Deserializer, Serialize,
};

#[cfg(not(feature = "serde-private-content"))]
use super::ContentVisitor;

/// An arbitrary value produced by deserialization.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-private-content", allow(dead_code))]
pub enum Content<'de> {
    /// A boolean.
    Bool(bool),

    /// An unsigned 8-bit integer.
    U8(u8),
    /// An unsigned 16-bit integer.
    U16(u16),
    /// An unsigned 32-bit integer.
    U32(u32),
    /// An unsigned 64-bit integer.
    U64(u64),

    /// A signed 8-bit integer.
    I8(i8),
    /// A signed 16-bit integer.
    I16(i16),
    /// A signed 32-bit integer.
    I32(i32),
    /// A signed 64-bit integer.
    I64(i64),

    /// A 32-bit floating-point number.
    F32(f32),
    /// A 64-bit floating-point number.
    F64(f64),

    /// A character.
    Char(char),
    /// A string which has been borrowed.
    Str(&'de str),
    /// An owned string.
    String(String),
    /// A sequence of bytes which has been borrowed.
    Bytes(&'de [u8]),
    /// An owned sequence of bytes.
    ByteBuf(Vec<u8>),

    /// Nothing.
    None,
    /// Some value.
    Some(Box<Self>),

    /// A unit value.
    Unit,

    /// A newtype value.
    Newtype(Box<Self>),

    /// A sequence.
    Seq(Vec<Self>),
    /// A map.
    Map(Vec<(Self, Self)>),
}

impl<'de> Content<'de> {
    /// Converts the content into a [`Deserializer`].
    pub fn into_deserializer<E: serde::de::Error>(self) -> impl Deserializer<'de, Error = E> {
        ContentDeserializer::new(self)
    }
}

#[cfg(not(feature = "serde-private-content"))]
impl<'de> Deserialize<'de> for Content<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 🐌 This is unnecessarily slow and copies around stuff when we are trying
        // to deserialize `Content` with a `ContentDeserializer`. Unfortunately, this
        // will actually happen quite frequently with tagged variant types. Thus, it
        // is much better to use Serde's private content implementation.
        deserializer.deserialize_any(ContentVisitor)
    }
}

impl<'de> Serialize for Content<'de> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Content::Bool(v) => serializer.serialize_bool(*v),
            Content::U8(v) => serializer.serialize_u8(*v),
            Content::U16(v) => serializer.serialize_u16(*v),
            Content::U32(v) => serializer.serialize_u32(*v),
            Content::U64(v) => serializer.serialize_u64(*v),
            Content::I8(v) => serializer.serialize_i8(*v),
            Content::I16(v) => serializer.serialize_i16(*v),
            Content::I32(v) => serializer.serialize_i32(*v),
            Content::I64(v) => serializer.serialize_i64(*v),
            Content::F32(v) => serializer.serialize_f32(*v),
            Content::F64(v) => serializer.serialize_f64(*v),
            Content::Char(v) => serializer.serialize_char(*v),
            Content::String(v) => serializer.serialize_str(v),
            Content::Str(v) => serializer.serialize_str(v),
            Content::Bytes(v) => serializer.serialize_bytes(v),
            Content::ByteBuf(v) => serializer.serialize_bytes(v),
            Content::None => serializer.serialize_none(),
            Content::Some(v) => serializer.serialize_some(v),
            Content::Unit => serializer.serialize_unit(),
            Content::Newtype(v) => serializer.serialize_newtype_struct("Content", v),
            Content::Seq(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for element in v {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
            Content::Map(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for (key, value) in v {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

/// An implementation of [`Deserializer`] from [`Content`].
pub(crate) struct ContentDeserializer<'de, E> {
    content: Content<'de>,
    _phantom_error: PhantomData<E>,
}

impl<'de, E> ContentDeserializer<'de, E> {
    pub(crate) fn new(content: Content<'de>) -> Self {
        Self {
            content,
            _phantom_error: PhantomData,
        }
    }
}

/// Generates a `deserialize` method delegating to `deserialize_any`.
macro_rules! impl_content_deserializer_delegate_to_any {
    ($deserialize:ident) => {
        fn $deserialize<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_any(visitor)
        }
    };
}

impl<'de, E: serde::de::Error> Deserializer<'de> for ContentDeserializer<'de, E> {
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Bool(v) => visitor.visit_bool(v),
            Content::U8(v) => visitor.visit_u8(v),
            Content::U16(v) => visitor.visit_u16(v),
            Content::U32(v) => visitor.visit_u32(v),
            Content::U64(v) => visitor.visit_u64(v),
            Content::I8(v) => visitor.visit_i8(v),
            Content::I16(v) => visitor.visit_i16(v),
            Content::I32(v) => visitor.visit_i32(v),
            Content::I64(v) => visitor.visit_i64(v),
            Content::F32(v) => visitor.visit_f32(v),
            Content::F64(v) => visitor.visit_f64(v),
            Content::Char(v) => visitor.visit_char(v),
            Content::String(v) => visitor.visit_string(v),
            Content::Str(v) => visitor.visit_borrowed_str(v),
            Content::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Content::ByteBuf(v) => visitor.visit_byte_buf(v),
            Content::None => visitor.visit_none(),
            Content::Some(v) => visitor.visit_some(v.into_deserializer()),
            Content::Unit => visitor.visit_unit(),
            Content::Newtype(v) => visitor.visit_newtype_struct(v.into_deserializer()),
            Content::Seq(v) => visitor.visit_seq(ContentSeqAccess::new(v)),
            Content::Map(v) => visitor.visit_map(ContentMapAccess::new(v)),
        }
    }

    impl_content_deserializer_delegate_to_any!(deserialize_bool);

    impl_content_deserializer_delegate_to_any!(deserialize_i8);
    impl_content_deserializer_delegate_to_any!(deserialize_i16);
    impl_content_deserializer_delegate_to_any!(deserialize_i32);
    impl_content_deserializer_delegate_to_any!(deserialize_i64);

    impl_content_deserializer_delegate_to_any!(deserialize_u8);
    impl_content_deserializer_delegate_to_any!(deserialize_u16);
    impl_content_deserializer_delegate_to_any!(deserialize_u32);
    impl_content_deserializer_delegate_to_any!(deserialize_u64);

    impl_content_deserializer_delegate_to_any!(deserialize_f32);
    impl_content_deserializer_delegate_to_any!(deserialize_f64);

    impl_content_deserializer_delegate_to_any!(deserialize_char);

    impl_content_deserializer_delegate_to_any!(deserialize_str);
    impl_content_deserializer_delegate_to_any!(deserialize_string);
    impl_content_deserializer_delegate_to_any!(deserialize_bytes);
    impl_content_deserializer_delegate_to_any!(deserialize_byte_buf);

    impl_content_deserializer_delegate_to_any!(deserialize_option);

    impl_content_deserializer_delegate_to_any!(deserialize_unit);

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            // Deserialize unit structs from empty maps and sequences.
            Content::Seq(seq) if seq.is_empty() => visitor.visit_unit(),
            Content::Map(entries) if entries.is_empty() => visitor.visit_unit(),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Newtype(v) => visitor.visit_newtype_struct(v.into_deserializer()),
            _ => self.deserialize_any(visitor),
        }
    }

    impl_content_deserializer_delegate_to_any!(deserialize_seq);

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Seq(mut seq) if !seq.is_empty() && seq.len() <= 2 => {
                let content = if seq.len() == 2 { seq.pop() } else { None };
                let variant = seq
                    .pop()
                    .expect("We just made sure that there is an element.");
                visitor.visit_enum(ContentEnumAccess::new(variant, content))
            }
            Content::Map(mut map) if map.len() == 1 => {
                let (variant, content) = map
                    .pop()
                    .expect("We just made sure that there is an element.");
                visitor.visit_enum(ContentEnumAccess::new(variant, Some(content)))
            }
            Content::Bytes(_) | Content::String(_) => {
                visitor.visit_enum(ContentEnumAccess::new(self.content, None))
            }
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
    }
}

/// Implementation of [`serde::de::SeqAccess`] for [`Content`].
struct ContentSeqAccess<'de, E> {
    remaining: std::vec::IntoIter<Content<'de>>,
    _phantom_error: PhantomData<E>,
}

impl<'de, E> ContentSeqAccess<'de, E> {
    /// Creates a new [`ContentSeqAccess`].
    fn new(seq: Vec<Content<'de>>) -> Self {
        Self {
            remaining: seq.into_iter(),
            _phantom_error: PhantomData,
        }
    }
}

impl<'de, E: serde::de::Error> serde::de::SeqAccess<'de> for ContentSeqAccess<'de, E> {
    type Error = E;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.remaining
            .next()
            .map(|element| seed.deserialize(element.into_deserializer()))
            .transpose()
    }
}

/// Implementation of [`serde::de::MapAccess`] for [`Content`].
struct ContentMapAccess<'de, E> {
    remaining: std::vec::IntoIter<(Content<'de>, Content<'de>)>,
    value: Option<Content<'de>>,
    _phantom_error: PhantomData<E>,
}

impl<'de, E> ContentMapAccess<'de, E> {
    /// Creates a new [`ContentMapAccess`].
    fn new(seq: Vec<(Content<'de>, Content<'de>)>) -> Self {
        Self {
            remaining: seq.into_iter(),
            value: None,
            _phantom_error: PhantomData,
        }
    }
}

impl<'de, E: serde::de::Error> serde::de::MapAccess<'de> for ContentMapAccess<'de, E> {
    type Error = E;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        self.remaining
            .next()
            .map(|(key, value)| {
                self.value = Some(value);
                seed.deserialize(key.into_deserializer())
            })
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let value = self.value.take().expect(
            "Calling `next_value_seed` before `next_key_seed` is incorrect and allowed to panic.",
        );
        seed.deserialize(value.into_deserializer())
    }
}

/// Implementation of [`serde::de::EnumAccess`] for [`Content`].
struct ContentEnumAccess<'de, E> {
    variant: Content<'de>,
    content: Option<Content<'de>>,
    _phantom_error: PhantomData<E>,
}

impl<'de, E> ContentEnumAccess<'de, E> {
    /// Creates a new [`ContentEnumAccess`].
    pub fn new(variant: Content<'de>, content: Option<Content<'de>>) -> Self {
        Self {
            variant,
            content,
            _phantom_error: PhantomData,
        }
    }
}

impl<'de, E: serde::de::Error> serde::de::EnumAccess<'de> for ContentEnumAccess<'de, E> {
    type Error = E;

    type Variant = ContentVariantAccess<'de, E>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.variant.into_deserializer())
            .map(|variant| (variant, ContentVariantAccess::new(self.content)))
    }
}

/// Implementation of [`serde::de::VariantAccess`] for [`Content`].
struct ContentVariantAccess<'de, E> {
    content: Option<Content<'de>>,
    _phantom_error: PhantomData<E>,
}

impl<'de, E> ContentVariantAccess<'de, E> {
    /// Creates a new [`ContentVariantAccess`].
    pub fn new(content: Option<Content<'de>>) -> Self {
        Self {
            content,
            _phantom_error: PhantomData,
        }
    }
}

impl<'de, E: serde::de::Error> serde::de::VariantAccess<'de> for ContentVariantAccess<'de, E> {
    type Error = E;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.content {
            Some(content) => <()>::deserialize(content.into_deserializer()),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.content {
            Some(content) => seed.deserialize(content.into_deserializer()),
            None => {
                Err(serde::de::Error::invalid_type(
                    Unexpected::UnitVariant,
                    &"newtype variant",
                ))
            }
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Some(value) => value.into_deserializer().deserialize_any(visitor),
            None => {
                Err(serde::de::Error::invalid_type(
                    Unexpected::UnitVariant,
                    &"tuple variant",
                ))
            }
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Some(value) => value.into_deserializer().deserialize_any(visitor),
            None => {
                Err(serde::de::Error::invalid_type(
                    Unexpected::UnitVariant,
                    &"struct variant",
                ))
            }
        }
    }
}

impl<'c, 'de> From<&'c Content<'de>> for Unexpected<'c> {
    fn from(content: &'c Content<'de>) -> Self {
        match content {
            Content::Bool(v) => Unexpected::Bool(*v),
            Content::U8(v) => Unexpected::Unsigned(u64::from(*v)),
            Content::U16(v) => Unexpected::Unsigned(u64::from(*v)),
            Content::U32(v) => Unexpected::Unsigned(u64::from(*v)),
            Content::U64(v) => Unexpected::Unsigned(*v),
            Content::I8(v) => Unexpected::Signed(i64::from(*v)),
            Content::I16(v) => Unexpected::Signed(i64::from(*v)),
            Content::I32(v) => Unexpected::Signed(i64::from(*v)),
            Content::I64(v) => Unexpected::Signed(*v),
            Content::F32(v) => Unexpected::Float(f64::from(*v)),
            Content::F64(v) => Unexpected::Float(*v),
            Content::Char(v) => Unexpected::Char(*v),
            Content::String(v) => Unexpected::Str(v.as_str()),
            Content::Str(v) => Unexpected::Str(v),
            Content::Bytes(v) => Unexpected::Bytes(v),
            Content::ByteBuf(v) => Unexpected::Bytes(v.as_slice()),
            Content::None => Unexpected::Option,
            Content::Some(_) => Unexpected::Option,
            Content::Unit => Unexpected::Unit,
            Content::Newtype(_) => Unexpected::NewtypeStruct,
            Content::Seq(_) => Unexpected::Seq,
            Content::Map(_) => Unexpected::Map,
        }
    }
}
