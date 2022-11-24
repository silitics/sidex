use std::{borrow::Cow, marker::PhantomData};

use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Serialize};

/// Any
#[derive(Debug, Clone)]
pub enum BorrowedAny<'v> {
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

    List(Vec<Self>),
    Map(Vec<(Self, Self)>),
}

impl<'v> Serialize for BorrowedAny<'v> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BorrowedAny::Bool(v) => serializer.serialize_bool(*v),
            BorrowedAny::Idx(_) => todo!(),
            BorrowedAny::U8(_) => todo!(),
            BorrowedAny::U16(_) => todo!(),
            BorrowedAny::U32(_) => todo!(),
            BorrowedAny::U64(_) => todo!(),
            BorrowedAny::I8(_) => todo!(),
            BorrowedAny::I16(_) => todo!(),
            BorrowedAny::I32(_) => todo!(),
            BorrowedAny::I64(_) => todo!(),
            BorrowedAny::F32(_) => todo!(),
            BorrowedAny::F64(_) => todo!(),
            BorrowedAny::Char(_) => todo!(),
            BorrowedAny::String(_) => todo!(),
            BorrowedAny::Bytes(_) => todo!(),
            BorrowedAny::None => todo!(),
            BorrowedAny::Some(_) => todo!(),
            BorrowedAny::Unit => todo!(),
            BorrowedAny::List(v) => {
                let mut s = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    s.serialize_element(e)?;
                }
                s.end()
            }
            BorrowedAny::Map(_) => todo!(),
        }
    }
}

struct AnyVisitor<'v> {
    _lifetime: PhantomData<&'v ()>,
}

impl<'de> Visitor<'de> for AnyVisitor<'de> {
    type Value = BorrowedAny<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BorrowedAny::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Signed(v),
            &self,
        ))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Unsigned(v),
            &self,
        ))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_f64(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Float(v),
            &self,
        ))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BorrowedAny::Char(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Str(v),
            &self,
        ))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BorrowedAny::String(Cow::Borrowed(v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let _ = v;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Bytes(v),
            &self,
        ))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(&v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = deserializer;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Unit,
            &self,
        ))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = deserializer;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::NewtypeStruct,
            &self,
        ))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let _ = seq;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Seq,
            &self,
        ))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let _ = map;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Map,
            &self,
        ))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let _ = data;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Enum,
            &self,
        ))
    }

    fn __private_visit_untagged_option<D>(self, _: D) -> Result<Self::Value, ()>
    where
        D: serde::Deserializer<'de>,
    {
        Err(())
    }
}

impl<'de> Deserialize<'de> for BorrowedAny<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

// impl<'v> ToOwned for [BorrowedAny<'v>] {
//     type Owned = Vec<BorrowedAny<'v>>;

//     fn to_owned(&self) -> Self::Owned {
//         todo!()
//     }
// }

type Any = BorrowedAny<'static>;

impl<'v> BorrowedAny<'v> {
    pub fn into_owned(self) -> Any {
        match self {
            BorrowedAny::Bool(v) => BorrowedAny::Bool(v),
            BorrowedAny::Idx(_) => todo!(),
            BorrowedAny::U8(_) => todo!(),
            BorrowedAny::U16(_) => todo!(),
            BorrowedAny::U32(_) => todo!(),
            BorrowedAny::U64(_) => todo!(),
            BorrowedAny::I8(_) => todo!(),
            BorrowedAny::I16(_) => todo!(),
            BorrowedAny::I32(_) => todo!(),
            BorrowedAny::I64(_) => todo!(),
            BorrowedAny::F32(_) => todo!(),
            BorrowedAny::F64(_) => todo!(),
            BorrowedAny::Char(_) => todo!(),
            BorrowedAny::String(v) => BorrowedAny::String(Cow::Owned(v.into_owned())),
            BorrowedAny::Bytes(v) => BorrowedAny::Bytes(Cow::Owned(v.into_owned())),
            BorrowedAny::None => todo!(),
            BorrowedAny::Some(_) => todo!(),
            BorrowedAny::Unit => todo!(),
            BorrowedAny::List(v) => {
                BorrowedAny::List(v.into_iter().map(BorrowedAny::into_owned).collect())
            }
            BorrowedAny::Map(v) => {
                BorrowedAny::Map(
                    v.into_iter()
                        .map(|(k, v)| (k.into_owned(), v.into_owned()))
                        .collect(),
                )
            } /* BorrowedAny::BorrowedSequence(v) => {
               *     BorrowedAny::Sequence(v.iter().cloned().map(BorrowedAny::into_owned).
               * collect()) }
               * BorrowedAny::BorrowedMap(v) => {
               *     BorrowedAny::Map(
               *         v.iter()
               *             .cloned()
               *             .map(|(k, v)| (k.into_owned(), v.into_owned()))
               *             .collect(),
               *     )
               * } */
        }
    }
}

// impl<'v> ToAny<'v> for bool {
//     fn to_any(&self) -> BorrowedAny<'v> {
//         BorrowedAny::Bool(*self)
//     }
// }

// impl<'v> ToAny<'v> for &'v str {
//     fn to_any(&self) -> BorrowedAny<'v> {
//         BorrowedAny::String(Cow::Borrowed(self))
//     }
// }

// impl<'v> ToAny<'v> for &'v [u8] {
//     fn to_any(&self) -> BorrowedAny<'v> {
//         BorrowedAny::Bytes(Cow::Borrowed(self))
//     }
// }

// impl<'v, T: ToAny<'v>> ToAny<'v> for &'v Vec<T> {
//     fn to_any(&self) -> BorrowedAny<'v> {
//         BorrowedAny::Sequence(self.iter().map(ToAny::to_any).collect())
//     }
// }
