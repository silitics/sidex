//! Serialization helpers for record types.

use serde::{
    ser::{Error, Impossible, SerializeMap, SerializeStruct},
    Serialize, Serializer,
};

use self::RecordSerializerMethod::*;

enum RecordSerializerMethod<S: Serializer> {
    AsStruct(S::SerializeStruct),
    AsMap(S::SerializeMap),
}

/// Serialization helper for record types.
pub struct RecordSerializer<S: Serializer> {
    method: RecordSerializerMethod<S>,
}

impl<S: Serializer> RecordSerializer<S> {
    pub fn new(serializer: S, name: &'static str, num_fields: usize) -> Result<Self, S::Error> {
        let method = if serializer.is_human_readable() {
            AsMap(serializer.serialize_map(None)?)
        } else {
            AsStruct(serializer.serialize_struct(name, num_fields)?)
        };
        Ok(Self { method })
    }

    pub fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        name: &'static str,
        value: &T,
    ) -> Result<(), S::Error> {
        match &mut self.method {
            AsStruct(s) => s.serialize_field(name, value),
            AsMap(s) => s.serialize_entry(name, value),
        }
    }

    pub fn serialize_optional_field<T: ?Sized + Serialize>(
        &mut self,
        name: &'static str,
        value: Option<&T>,
    ) -> Result<(), S::Error> {
        match (&mut self.method, value) {
            (AsStruct(s), Some(v)) => s.serialize_field(name, v),
            (AsStruct(s), None) => s.skip_field(name),
            (AsMap(s), Some(v)) => s.serialize_entry(name, v),
            (AsMap(_), None) => Ok(()),
        }
    }

    pub fn serialize_inlined_field<T: ?Sized + Serialize>(
        &mut self,
        name: &'static str,
        value: &T,
    ) -> Result<(), S::Error> {
        match &mut self.method {
            AsStruct(s) => s.serialize_field(name, value),
            AsMap(s) => value.serialize(InlineSerializer::new(s)),
        }
    }

    pub fn end(self) -> Result<S::Ok, S::Error> {
        match self.method {
            AsStruct(s) => s.end(),
            AsMap(s) => s.end(),
        }
    }
}

/// Serialize into a [map serializer][SerializeMap].
pub struct InlineSerializer<'map, S> {
    map_serializer: &'map mut S,
}

impl<'map, S> InlineSerializer<'map, S> {
    pub fn new(map: &'map mut S) -> Self {
        Self {
            map_serializer: map,
        }
    }
}

macro_rules! fn_bad_primitive {
    ($fn:ident, $type:ty) => {
        fn $fn(self, _: $type) -> Result<Self::Ok, Self::Error> {
            Err(S::Error::custom(concat!(
                "cannot inline ",
                stringify!($type),
                " into record"
            )))
        }
    };
}

macro_rules! bad_type {
    ($type:literal) => {
        Err(S::Error::custom(concat!(
            "cannot inline ",
            $type,
            " into record"
        )))
    };
}

impl<'map, S: SerializeMap> Serializer for InlineSerializer<'map, S> {
    type Ok = ();
    type Error = S::Error;

    type SerializeSeq = Impossible<Self::Ok, S::Error>;
    type SerializeTuple = Impossible<Self::Ok, S::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, S::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, S::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, S::Error>;

    fn_bad_primitive!(serialize_bool, bool);

    fn_bad_primitive!(serialize_i8, i8);
    fn_bad_primitive!(serialize_i16, i16);
    fn_bad_primitive!(serialize_i32, i32);
    fn_bad_primitive!(serialize_i64, i64);
    fn_bad_primitive!(serialize_i128, i128);

    fn_bad_primitive!(serialize_u8, u8);
    fn_bad_primitive!(serialize_u16, u16);
    fn_bad_primitive!(serialize_u32, u32);
    fn_bad_primitive!(serialize_u64, u64);
    fn_bad_primitive!(serialize_u128, u128);

    fn_bad_primitive!(serialize_f32, f32);
    fn_bad_primitive!(serialize_f64, f64);

    fn_bad_primitive!(serialize_char, char);

    fn_bad_primitive!(serialize_str, &str);
    fn_bad_primitive!(serialize_bytes, &[u8]);

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        bad_type!("unit variant")
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        bad_type!("newtype variant")
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        bad_type!("sequence")
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        bad_type!("tuple")
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        bad_type!("tuple struct")
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        bad_type!("tuple variant")
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        bad_type!("struct variant")
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

impl<'map, S: SerializeMap> SerializeMap for InlineSerializer<'map, S> {
    type Ok = ();
    type Error = S::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map_serializer.serialize_key(key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map_serializer.serialize_value(value)
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        self.map_serializer.serialize_entry(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'map, S: SerializeMap> SerializeStruct for InlineSerializer<'map, S> {
    type Ok = ();
    type Error = S::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map_serializer.serialize_entry(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
