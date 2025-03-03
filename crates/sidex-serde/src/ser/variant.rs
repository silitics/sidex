use serde::{Serialize, Serializer, ser::SerializeMap};

use super::record::InlineSerializer;

pub struct VariantSerializer<S: Serializer> {
    serializer: S,
    type_name: &'static str,
}

impl<S: Serializer> VariantSerializer<S> {
    pub fn new(serializer: S, type_name: &'static str) -> Self {
        Self {
            serializer,
            type_name,
        }
    }

    pub fn serialize_tag(
        self,
        variant_tag: &'static str,
        variant_idx: u32,
    ) -> Result<S::Ok, S::Error> {
        self.serializer
            .serialize_unit_variant(self.type_name, variant_idx, variant_tag)
    }

    pub fn serialize_adjacent_tag(
        self,
        tag_field: &'static str,
        variant_tag: &'static str,
        variant_idx: u32,
    ) -> Result<S::Ok, S::Error> {
        if self.serializer.is_human_readable() {
            let mut map = self.serializer.serialize_map(Some(1))?;
            map.serialize_entry(tag_field, variant_tag)?;
            map.end()
        } else {
            self.serializer
                .serialize_unit_variant(self.type_name, variant_idx, variant_tag)
        }
    }

    pub fn serialize_internal_tag(
        self,
        tag_field: &'static str,
        variant_tag: &'static str,
        variant_idx: u32,
    ) -> Result<S::Ok, S::Error> {
        if self.serializer.is_human_readable() {
            let mut map = self.serializer.serialize_map(Some(1))?;
            map.serialize_entry(tag_field, variant_tag)?;
            map.end()
        } else {
            self.serializer
                .serialize_unit_variant(self.type_name, variant_idx, variant_tag)
        }
    }

    pub fn serialize_externally_tagged<T: ?Sized + Serialize>(
        self,
        variant_tag: &'static str,
        variant_idx: u32,
        value: &T,
    ) -> Result<S::Ok, S::Error> {
        if self.serializer.is_human_readable() {
            let mut map = self.serializer.serialize_map(Some(1))?;
            map.serialize_entry(variant_tag, value)?;
            map.end()
        } else {
            self.serializer.serialize_newtype_variant(
                self.type_name,
                variant_idx,
                variant_tag,
                value,
            )
        }
    }

    pub fn serialize_internally_tagged<T: ?Sized + Serialize>(
        self,
        tag_field: &'static str,
        variant_tag: &'static str,
        variant_idx: u32,
        value: &T,
    ) -> Result<S::Ok, S::Error> {
        if self.serializer.is_human_readable() {
            let mut map = self.serializer.serialize_map(None)?;
            map.serialize_entry(tag_field, variant_tag)?;
            value.serialize(InlineSerializer::new(&mut map))?;
            map.end()
        } else {
            self.serializer.serialize_newtype_variant(
                self.type_name,
                variant_idx,
                variant_tag,
                value,
            )
        }
    }

    pub fn serialize_adjacently_tagged<T: ?Sized + Serialize>(
        self,
        tag_field: &'static str,
        value_field: &'static str,
        variant_tag: &'static str,
        variant_idx: u32,
        value: &T,
    ) -> Result<S::Ok, S::Error> {
        if self.serializer.is_human_readable() {
            let mut map = self.serializer.serialize_map(Some(2))?;
            map.serialize_entry(tag_field, variant_tag)?;
            map.serialize_entry(value_field, value)?;
            map.end()
        } else {
            self.serializer.serialize_newtype_variant(
                self.type_name,
                variant_idx,
                variant_tag,
                value,
            )
        }
    }

    pub fn serialize_implicitly_tagged<T: ?Sized + Serialize>(
        self,
        variant_tag: &'static str,
        variant_idx: u32,
        value: &T,
    ) -> Result<S::Ok, S::Error> {
        if self.serializer.is_human_readable() {
            value.serialize(self.serializer)
        } else {
            self.serializer.serialize_newtype_variant(
                self.type_name,
                variant_idx,
                variant_tag,
                value,
            )
        }
    }
}
