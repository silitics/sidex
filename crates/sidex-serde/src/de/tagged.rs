//! Deserialization helpers for tagged variant types used by generated code.

use std::marker::PhantomData;

use serde::{
    de::{DeserializeSeed, Visitor},
    Deserialize, Deserializer,
};

use super::{
    content::{content_str_eq, deserialize_content, Content, ContentVisitor},
    sanitize_size_hint,
};

enum KeyTagOrContent<'de> {
    Tag,
    Content(Content<'de>),
}

struct KeyTagOrContentVisitor {
    name: &'static str,
}

impl KeyTagOrContentVisitor {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

macro_rules! impl_key_tag_or_content_visit_content {
    ($visit:ident, $ty:ty, $variant:ident) => {
        fn $visit<E>(self, v: $ty) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            ContentVisitor.$visit(v).map(KeyTagOrContent::Content)
        }
    };
}

impl<'de> Visitor<'de> for KeyTagOrContentVisitor {
    type Value = KeyTagOrContent<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("tag or any value")
    }

    impl_key_tag_or_content_visit_content!(visit_bool, bool, Bool);

    impl_key_tag_or_content_visit_content!(visit_i8, i8, I8);
    impl_key_tag_or_content_visit_content!(visit_i16, i16, I16);
    impl_key_tag_or_content_visit_content!(visit_i32, i32, I32);
    impl_key_tag_or_content_visit_content!(visit_i64, i64, I64);

    impl_key_tag_or_content_visit_content!(visit_u8, u8, U8);
    impl_key_tag_or_content_visit_content!(visit_u16, u16, U16);
    impl_key_tag_or_content_visit_content!(visit_u32, u32, U32);
    impl_key_tag_or_content_visit_content!(visit_u64, u64, U64);

    impl_key_tag_or_content_visit_content!(visit_f32, f32, F32);
    impl_key_tag_or_content_visit_content!(visit_f64, f64, F64);

    impl_key_tag_or_content_visit_content!(visit_char, char, Char);

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.name == v {
            Ok(KeyTagOrContent::Tag)
        } else {
            ContentVisitor.visit_str(v).map(KeyTagOrContent::Content)
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.name == v {
            Ok(KeyTagOrContent::Tag)
        } else {
            ContentVisitor
                .visit_borrowed_str(v)
                .map(KeyTagOrContent::Content)
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.name == v {
            Ok(KeyTagOrContent::Tag)
        } else {
            ContentVisitor.visit_string(v).map(KeyTagOrContent::Content)
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.name.as_bytes() == v {
            Ok(KeyTagOrContent::Tag)
        } else {
            ContentVisitor.visit_bytes(v).map(KeyTagOrContent::Content)
        }
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.name.as_bytes() == v {
            Ok(KeyTagOrContent::Tag)
        } else {
            ContentVisitor
                .visit_borrowed_bytes(v)
                .map(KeyTagOrContent::Content)
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.name.as_bytes() == v {
            Ok(KeyTagOrContent::Tag)
        } else {
            ContentVisitor
                .visit_byte_buf(v)
                .map(KeyTagOrContent::Content)
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor.visit_none().map(KeyTagOrContent::Content)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        ContentVisitor
            .visit_some(deserializer)
            .map(KeyTagOrContent::Content)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ContentVisitor.visit_unit().map(KeyTagOrContent::Content)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        ContentVisitor
            .visit_newtype_struct(deserializer)
            .map(KeyTagOrContent::Content)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        ContentVisitor.visit_seq(seq).map(KeyTagOrContent::Content)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        ContentVisitor.visit_map(map).map(KeyTagOrContent::Content)
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        ContentVisitor
            .visit_enum(data)
            .map(KeyTagOrContent::Content)
    }
}

impl<'de> DeserializeSeed<'de> for KeyTagOrContentVisitor {
    type Value = KeyTagOrContent<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

/// For internally and adjacently tagged variant types.
pub struct TaggedVariant<'de, T> {
    pub tag: T,
    pub content: Content<'de>,
}

impl<'de, T> TaggedVariant<'de, T> {
    pub fn deserialize_internally_tagged<V: Deserialize<'de>, E: 'de + serde::de::Error>(
        self,
    ) -> Result<V, E> {
        deserialize_content(self.content)
    }

    pub fn deserialize_adjacently_tagged<V: Deserialize<'de>, E: 'de + serde::de::Error>(
        self,
        content_field: &'static str,
    ) -> Result<V, E> {
        match self.content {
            Content::Newtype(value) => deserialize_content(*value),
            Content::Seq(mut seq) if seq.len() == 1 => {
                deserialize_content(
                    seq.pop()
                        .expect("We just made sure that there is an element."),
                )
            }
            Content::Map(mut map) if map.len() == 1 => {
                let (key, value) = map
                    .pop()
                    .expect("We just made sure that there is an element.");
                if content_str_eq(&key, content_field) {
                    deserialize_content(value)
                } else {
                    Err(E::missing_field(content_field))
                }
            }
            _ => Err(E::missing_field(content_field)),
        }
    }
}

pub fn deserialize_tagged_variant<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
    tag_field: &'static str,
) -> Result<TaggedVariant<'de, T>, D::Error> {
    let visitor = TaggedVariantVisitor::new(tag_field);
    deserializer.deserialize_any(visitor)
}

struct TaggedVariantVisitor<'de, T> {
    tag_field: &'static str,
    phantom_type: PhantomData<TaggedVariant<'de, T>>,
}

impl<'de, T> TaggedVariantVisitor<'de, T> {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag_field: tag,
            phantom_type: PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for TaggedVariantVisitor<'de, T> {
    type Value = TaggedVariant<'de, T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("enum")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let tag = seq
            .next_element::<T>()?
            .ok_or_else(|| serde::de::Error::missing_field(self.tag_field))?;
        let mut elements = Vec::with_capacity(sanitize_size_hint(seq.size_hint()));
        while let Some(element) = seq.next_element::<Content<'de>>()? {
            elements.push(element)
        }
        Ok(TaggedVariant {
            tag,
            content: Content::Seq(elements),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tag = None;
        let mut entries = Vec::with_capacity(sanitize_size_hint(map.size_hint()));
        while let Some(key) = map.next_key_seed(KeyTagOrContentVisitor::new(self.tag_field))? {
            match key {
                KeyTagOrContent::Tag => {
                    if tag.is_some() {
                        return Err(serde::de::Error::duplicate_field(self.tag_field));
                    }
                    tag = Some(map.next_value::<T>()?)
                }
                KeyTagOrContent::Content(key) => {
                    let value = map.next_value::<Content<'de>>()?;
                    entries.push((key, value))
                }
            }
        }
        Ok(TaggedVariant {
            tag: tag.ok_or_else(|| serde::de::Error::missing_field(self.tag_field))?,
            content: Content::Map(entries),
        })
    }
}
