#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod builtins;
pub mod de;
pub mod ser;

// fn default_serialize<T: serde::Serialize, S: serde::Serializer>(
//     value: &T,
//     serializer: S,
// ) -> Result<S::Ok, S::Error> { // Use Serde's default `serialize` implementation but
//   wrap the serializer such // that inner values are serialized as required by Sidex.
//   serde::Serialize::serialize(value, Sidex(serializer))
// }

// pub trait Serialize: serde::Serialize {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer;
// }

// impl<T: serde::Serialize> Serialize for T {
//     default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         default_serialize(self, serializer)
//     }
// }

// impl<T: serde::Serialize, E: serde::Serialize> Serialize for Result<T, E> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         if serializer.is_human_readable() {
//             let mut serializer = serializer.serialize_struct("Result", 2)?;
//             match self {
//                 Ok(value) => {
//                     serializer.serialize_field("result", "Ok")?;
//                     serializer.serialize_field("value", &Sidex(value))?;
//                 }
//                 Err(error) => {
//                     serializer.serialize_field("result", "Err")?;
//                     serializer.serialize_field("error", &Sidex(error))?;
//                 }
//             }
//             serializer.end()
//         } else {
//             default_serialize(self, serializer)
//         }
//     }
// }

// #[repr(transparent)]
// struct Sidex<S>(S);

// impl<T: serde::Serialize> serde::Serialize for Sidex<T> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         Serialize::serialize(&self, serializer)
//     }
// }

// impl<S: serde::Serializer> serde::Serializer for Sidex<S> {
//     type Ok = S::Ok;
//     type Error = S::Error;

//     type SerializeSeq = Sidex<S::SerializeSeq>;
//     type SerializeTuple = S::SerializeTuple;
//     type SerializeTupleStruct = S::SerializeTupleStruct;
//     type SerializeTupleVariant = S::SerializeTupleVariant;
//     type SerializeMap = S::SerializeMap;
//     type SerializeStruct = S::SerializeStruct;
//     type SerializeStructVariant = S::SerializeStructVariant;

//     fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
//     where
//         T: serde::Serialize,
//     {
//         todo!()
//     }

//     fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }

//     fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>
// {         todo!()
//     }

//     fn serialize_unit_variant(
//         self,
//         name: &'static str,
//         variant_index: u32,
//         variant: &'static str,
//     ) -> Result<Self::Ok, Self::Error> { todo!()
//     }

//     fn serialize_newtype_struct<T: ?Sized>(
//         self,
//         name: &'static str,
//         value: &T,
//     ) -> Result<Self::Ok, Self::Error>
//     where
//         T: serde::Serialize,
//     {
//         todo!()
//     }

//     fn serialize_newtype_variant<T: ?Sized>(
//         self,
//         name: &'static str,
//         variant_index: u32,
//         variant: &'static str,
//         value: &T,
//     ) -> Result<Self::Ok, Self::Error>
//     where
//         T: serde::Serialize,
//     {
//         todo!()
//     }

//     fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq,
// Self::Error> {         todo!()
//     }

//     fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
//         todo!()
//     }

//     fn serialize_tuple_struct(
//         self,
//         name: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeTupleStruct, Self::Error> { todo!()
//     }

//     fn serialize_tuple_variant(
//         self,
//         name: &'static str,
//         variant_index: u32,
//         variant: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeTupleVariant, Self::Error> { todo!()
//     }

//     fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap,
// Self::Error> {         todo!()
//     }

//     fn serialize_struct(
//         self,
//         name: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeStruct, Self::Error> { todo!()
//     }

//     fn serialize_struct_variant(
//         self,
//         name: &'static str,
//         variant_index: u32,
//         variant: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeStructVariant, Self::Error> { todo!()
//     }
// }

// impl<S: serde::ser::SerializeSeq> serde::ser::SerializeSeq for Sidex<S> {
//     type Ok = S::Ok;
//     type Error = S::Error;

//     fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
//     where
//         T: serde::Serialize,
//     {
//         self.0.serialize_element(&Sidex(value))
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         self.0.end()
//     }
// }
