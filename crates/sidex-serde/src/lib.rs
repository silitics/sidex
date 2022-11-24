//! [Serde](https://serde.rs) compatibility layer for Sidex.

use std::{borrow::Cow, marker::PhantomData};

use serde::{ser::SerializeStruct, Deserialize, Serialize};

struct F32(f32);

impl Serialize for F32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            if self.0.is_nan() {
                serializer.serialize_str("NaN")
            } else if self.0.is_infinite() {
                if self.0.is_sign_negative() {
                    serializer.serialize_str("-Infinity")
                } else {
                    serializer.serialize_str("+Infinity")
                }
            } else {
                serializer.serialize_f32(self.0)
            }
        } else {
            serializer.serialize_f32(self.0)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Content<'v> {
    Bool(bool),

    Idx(usize),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(Cow<'v, str>),
    Bytes(Cow<'v, [u8]>),

    None,
    Some(Box<Self>),

    Unit,

    Sequence(Vec<Self>),
    Map(Vec<(Self, Self)>),
}

impl<'v> Serialize for Content<'v> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Content::Bool(v) => serializer.serialize_bool(*v),
            Content::Idx(v) => serializer.serialize_u64(*v as u64),
            Content::U8(v) => serializer.serialize_u8(*v),
            Content::U16(_) => todo!(),
            Content::U32(_) => todo!(),
            Content::U64(_) => todo!(),
            Content::I8(_) => todo!(),
            Content::I16(_) => todo!(),
            Content::I32(_) => todo!(),
            Content::I64(_) => todo!(),
            Content::F32(_) => todo!(),
            Content::F64(_) => todo!(),
            Content::Char(_) => todo!(),
            Content::String(_) => todo!(),
            Content::Bytes(_) => todo!(),
            Content::None => todo!(),
            Content::Some(_) => todo!(),
            Content::Unit => todo!(),
            Content::Sequence(_) => todo!(),
            Content::Map(_) => todo!(),
        }
    }
}

struct Visitor<'de> {
    lifetime: PhantomData<&'de ()>,
}

impl<'de> Deserialize<'de> for Content<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

pub struct AdjacentlyTagged<'v, T: Serialize> {
    name: &'static str,
    tag_field: &'static str,
    tag: &'static str,
    value_field: &'static str,
    value: &'v T,
}

impl<'v, T: Serialize> Serialize for AdjacentlyTagged<'v, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct(self.name, 2)?;
        s.serialize_field(self.tag_field, self.tag)?;
        s.serialize_field(self.value_field, self.value)?;
        s.end()
    }
}
