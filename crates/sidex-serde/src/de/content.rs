//! Data structure for holding intermediate deserialized values of unknown type.

use serde::{de::Visitor, Deserialize, Deserializer};

use super::sanitize_size_hint;

mod ours;
mod theirs;

#[cfg(not(feature = "serde-private-content"))]
/// Alias to the `Content` enum selected via the `serde-private-content` feature.
pub(crate) type Content<'de> = ours::Content<'de>;

#[cfg(feature = "serde-private-content")]
/// Alias to the `Content` enum selected via the `serde-private-content` feature.
pub(crate) type Content<'de> = theirs::Content<'de>;

/// Deserializes [`Content`] into `T`.
pub(crate) fn deserialize_content<'de, T: Deserialize<'de>, E: 'de + serde::de::Error>(
    content: Content<'de>,
) -> Result<T, E> {
    #[cfg(not(feature = "serde-private-content"))]
    return T::deserialize(ours::ContentDeserializer::new(content));
    #[cfg(feature = "serde-private-content")]
    return T::deserialize(theirs::ContentDeserializer::new(content));
}

/// Compares a [`Content`] and a `&str` for equality.
pub(crate) fn content_str_eq<'de>(content: &Content<'de>, string: &str) -> bool {
    match content {
        Content::String(v) => v.as_str() == string,
        Content::Str(v) => *v == string,
        Content::ByteBuf(v) => v.as_slice() == string.as_bytes(),
        Content::Bytes(v) => *v == string.as_bytes(),
        _ => false,
    }
}

/// Visitor for deserializing into [`Content`].
///
/// Note that this visitor works for Serde's and our content implementation.
pub(crate) struct ContentVisitor;

/// Generates a `visit` method for a primitive type.
macro_rules! impl_visit_primitive {
    ($visit:ident, $ty:ty, $variant:ident) => {
        fn $visit<E>(self, v: $ty) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Content::$variant(v))
        }
    };
}

impl<'de> Visitor<'de> for ContentVisitor {
    type Value = Content<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any value")
    }

    impl_visit_primitive!(visit_bool, bool, Bool);

    impl_visit_primitive!(visit_i8, i8, I8);
    impl_visit_primitive!(visit_i16, i16, I16);
    impl_visit_primitive!(visit_i32, i32, I32);
    impl_visit_primitive!(visit_i64, i64, I64);

    impl_visit_primitive!(visit_u8, u8, U8);
    impl_visit_primitive!(visit_u16, u16, U16);
    impl_visit_primitive!(visit_u32, u32, U32);
    impl_visit_primitive!(visit_u64, u64, U64);

    impl_visit_primitive!(visit_f32, f32, F32);
    impl_visit_primitive!(visit_f64, f64, F64);

    impl_visit_primitive!(visit_char, char, Char);

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::String(v.to_owned()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::Str(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::String(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::ByteBuf(v.to_owned()))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::Bytes(v))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::ByteBuf(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Content::Some(Box::new(Content::deserialize(deserializer)?)))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Content::Unit)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Content::Newtype(Box::new(Content::deserialize(
            deserializer,
        )?)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut elements = Vec::with_capacity(sanitize_size_hint(seq.size_hint()));
        while let Some(element) = seq.next_element::<Self::Value>()? {
            elements.push(element);
        }
        Ok(Content::Seq(elements))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut entries = Vec::with_capacity(sanitize_size_hint(map.size_hint()));
        while let Some(entry) = map.next_entry()? {
            entries.push(entry);
        }
        Ok(Content::Map(entries))
    }
}
