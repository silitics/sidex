#![doc = include_str!("../README.md")]

use sidex_gen::{
    attrs::{AttrConvertExt, TryApplyAttr, accept, reject},
    diagnostics, ir,
    rename::RenameFunction,
};

pub mod atoms;
pub mod types;

/// JSON attributes of an opaque type.
#[derive(Debug, Clone, Default)]
pub struct OpaqueTypeAttrs {
    pub typ: Option<atoms::TypeAttr>,
    pub schema: Option<atoms::SchemaAttr>,
}

impl TryApplyAttr for OpaqueTypeAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> diagnostics::Result<()> {
        if let Ok(list) = attr.expect_list_with("json") {
            list.args
                .iter()
                .map(|arg| {
                    if let Ok(typ) = arg.convert() {
                        self.typ = Some(typ);
                        accept!(())
                    } else if let Ok(schema) = arg.convert() {
                        self.schema = Some(schema);
                        accept!(())
                    } else {
                        eprint!("{arg:?}");
                        reject!(arg, "Expect JSON opaque type attribute.")
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(())
        } else {
            Ok(())
        }
    }
}

/// JSON attributes of a record type.
#[derive(Debug, Clone)]
pub struct JsonRecordTypeAttrs {
    pub rename_all: atoms::RenameAllAttr,
}

impl JsonRecordTypeAttrs {
    pub fn field_name(&self, field: &ir::Field, field_attrs: &JsonFieldAttrs) -> String {
        if let Some(name) = &field_attrs.name {
            name.0.clone()
        } else if let Some(rename) = &field_attrs.rename {
            rename.function.apply_to(field.name.as_str())
        } else {
            self.rename_all.function.apply_to(field.name.as_str())
        }
    }
}

impl Default for JsonRecordTypeAttrs {
    fn default() -> Self {
        Self {
            rename_all: atoms::RenameAllAttr {
                function: RenameFunction::CamelCase,
            },
        }
    }
}

impl TryApplyAttr for JsonRecordTypeAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> diagnostics::Result<()> {
        if let Ok(list) = attr.expect_list_with("json") {
            for arg in &list.args {
                if let Ok(rename_all) = arg.convert() {
                    self.rename_all = rename_all;
                } else {
                    reject!(arg, "Expected a JSON record type attribute.")
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct JsonFieldAttrs {
    pub name: Option<atoms::NameAttr>,
    pub rename: Option<atoms::RenameAttr>,
    pub inline: bool,
}

impl TryApplyAttr for JsonFieldAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> diagnostics::Result<()> {
        if let Ok(list) = attr.expect_list_with("json") {
            for arg in &list.args {
                if let Ok(name) = arg.convert() {
                    self.name = Some(name);
                } else if let Ok(rename) = arg.convert() {
                    self.rename = Some(rename);
                } else if arg.is_path("inline") {
                    self.inline = true;
                } else {
                    reject!(arg, "Expected a JSON field attribute.")
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct JsonVariantTypeAttrs {
    pub rename_all: atoms::RenameAllAttr,
    pub tagged: atoms::JsonTaggedAttr,
    pub tag: atoms::TagAttr,
    pub content: atoms::ContentAttr,
}

impl JsonVariantTypeAttrs {
    pub fn tag_field_name(&self) -> String {
        self.tag.0.clone()
    }

    pub fn content_field_name(&self, variant_attrs: &JsonVariantAttrs) -> String {
        variant_attrs
            .content
            .as_ref()
            .map(|attr| &attr.0)
            .unwrap_or(&self.content.0)
            .clone()
    }

    pub fn variant_name(&self, variant: &ir::Variant, variant_attrs: &JsonVariantAttrs) -> String {
        if let Some(name) = &variant_attrs.name {
            name.0.clone()
        } else if let Some(rename) = &variant_attrs.rename {
            rename.function.apply_to(variant.name.as_str())
        } else {
            self.rename_all.function.apply_to(variant.name.as_str())
        }
    }

    pub fn variant_name_from_str(&self, variant: &str, variant_attrs: &JsonVariantAttrs) -> String {
        if let Some(name) = &variant_attrs.name {
            name.0.clone()
        } else if let Some(rename) = &variant_attrs.rename {
            rename.function.apply_to(variant)
        } else {
            self.rename_all.function.apply_to(variant)
        }
    }
}

impl Default for JsonVariantTypeAttrs {
    fn default() -> Self {
        Self {
            rename_all: RenameFunction::PascalCase.into(),
            tagged: atoms::JsonTaggedAttr::Internally,
            tag: "tag".into(),
            content: "content".into(),
        }
    }
}

impl TryApplyAttr for JsonVariantTypeAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> diagnostics::Result<()> {
        if let Ok(list) = attr.expect_list_with("json") {
            for arg in &list.args {
                if let Ok(rename_all) = arg.convert() {
                    self.rename_all = rename_all;
                } else if let Ok(tagged) = arg.convert() {
                    self.tagged = tagged;
                } else if let Ok(tag) = arg.convert() {
                    self.tag = tag;
                } else if let Ok(content) = arg.convert() {
                    self.content = content;
                } else {
                    reject!(arg, "Expected a JSON variant type attribute.")
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct JsonVariantAttrs {
    pub name: Option<atoms::NameAttr>,
    pub rename: Option<atoms::RenameAttr>,
    pub content: Option<atoms::ContentAttr>,
}

impl TryApplyAttr for JsonVariantAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> diagnostics::Result<()> {
        if let Ok(list) = attr.expect_list_with("json") {
            for arg in &list.args {
                if let Ok(name) = arg.convert() {
                    self.name = Some(name)
                } else if let Ok(rename) = arg.convert() {
                    self.rename = Some(rename)
                } else if let Ok(content) = arg.convert() {
                    self.content = Some(content)
                } else {
                    reject!(arg, "Expected a JSON variant type attribute.")
                }
            }
        }
        Ok(())
    }
}
