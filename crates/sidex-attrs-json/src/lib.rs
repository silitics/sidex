//! Typed attribute schemas for the JSON codegen target.
//!
//! These types mirror [`lib/json/schemas/attrs.sidex`](https://github.com/silitics/sidex/blob/main/lib/json/schemas/attrs.sidex).
//! The generated wire-shape structs in [`generated::attrs`] are read off the
//! IR's `typed_attrs["json"]` map and adapted into the higher-level wrapper
//! types in this module — the wrappers parse string-shaped fields (e.g.
//! `rename_all = "PascalCase"`) into the [`RenameFunction`] / [`JsonUnionType`]
//! / [`JsonTaggedAttr`] enums codegens want to consume.

mod generated;

use std::collections::HashMap;

use serde_json::Value;
use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Result;
use sidex_gen::rename::RenameFunction;
use sidex_ir as ir;

pub mod types;

pub use generated::attrs as raw;

use crate::types::JsonUnionType;

const PLUGIN: &str = "json";

/// Pull the typed `json` attributes off any IR node's `typed_attrs` map and
/// deserialize them into the generated struct `T`. Returns `Ok(None)` if the
/// node has no `json` attrs.
pub fn extract<T>(typed_attrs: &HashMap<String, Value>) -> Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    match typed_attrs.get(PLUGIN) {
        Some(value) => serde_json::from_value(value.clone()).map(Some).map_err(|err| {
            Box::new(Diagnostic::error(format!("Invalid `json` attributes: {err}")))
        }),
        None => Ok(None),
    }
}

/// JSON attributes that may apply to an `opaque` definition.
#[derive(Debug, Clone, Default)]
pub struct JsonOpaqueTypeAttrs {
    /// JSON union type expression (e.g. `string|null`).
    pub typ: Option<JsonUnionType>,
    /// JSON Schema path emitted as `$ref` for the opaque type.
    pub schema: Option<String>,
}

/// Read the `JsonOpaqueTypeAttrs` for `def` from its `typed_attrs["json"]`.
pub fn opaque_type_attrs(def: &ir::Def) -> Result<JsonOpaqueTypeAttrs> {
    let Some(raw) = extract::<raw::OpaqueTypeAttrs>(&def.typed_attrs)? else {
        return Ok(JsonOpaqueTypeAttrs::default());
    };
    Ok(JsonOpaqueTypeAttrs {
        typ: raw.typ.map(|s| s.parse()).transpose()?,
        schema: raw.schema,
    })
}

/// JSON attributes that may apply to a `record` definition.
#[derive(Debug, Clone)]
pub struct JsonRecordTypeAttrs {
    /// Default rename function applied to each field name.
    pub rename_all: RenameFunction,
}

impl Default for JsonRecordTypeAttrs {
    fn default() -> Self {
        Self {
            rename_all: RenameFunction::CamelCase,
        }
    }
}

impl JsonRecordTypeAttrs {
    /// The JSON key for `field`, taking record-level and field-level overrides
    /// into account (in order: explicit `name`, field-level `rename`,
    /// record-level `rename_all`).
    pub fn field_name(&self, field: &ir::Field, field_attrs: &JsonFieldAttrs) -> String {
        if let Some(name) = &field_attrs.name {
            name.clone()
        } else if let Some(rename) = &field_attrs.rename {
            rename.apply_to(field.name.as_str())
        } else {
            self.rename_all.apply_to(field.name.as_str())
        }
    }
}

/// Read the `JsonRecordTypeAttrs` for `def` from its `typed_attrs["json"]`.
pub fn record_type_attrs(def: &ir::Def) -> Result<JsonRecordTypeAttrs> {
    let Some(raw) = extract::<raw::RecordTypeAttrs>(&def.typed_attrs)? else {
        return Ok(JsonRecordTypeAttrs::default());
    };
    Ok(JsonRecordTypeAttrs {
        rename_all: parse_rename(raw.rename_all)?.unwrap_or(RenameFunction::CamelCase),
    })
}

/// JSON attributes that may apply to a record `field`.
#[derive(Debug, Clone, Default)]
pub struct JsonFieldAttrs {
    /// Override the JSON key with an explicit string.
    pub name: Option<String>,
    /// Apply a rename function to the field name.
    pub rename: Option<RenameFunction>,
    /// Whether to inline the field's contents into the enclosing object.
    pub inline: bool,
}

/// Read the `JsonFieldAttrs` for `field` from its `typed_attrs["json"]`.
pub fn field_attrs(field: &ir::Field) -> Result<JsonFieldAttrs> {
    let Some(raw) = extract::<raw::FieldAttrs>(&field.typed_attrs)? else {
        return Ok(JsonFieldAttrs::default());
    };
    Ok(JsonFieldAttrs {
        name: raw.name,
        rename: parse_rename(raw.rename)?,
        inline: raw.inline.unwrap_or(false),
    })
}

/// JSON attributes that may apply to a `variant` definition.
#[derive(Debug, Clone)]
pub struct JsonVariantTypeAttrs {
    /// Default rename function applied to each variant case name.
    pub rename_all: RenameFunction,
    /// The variant tagging strategy.
    pub tagged: JsonTaggedAttr,
    /// Field name carrying the tag value (for adjacently / internally
    /// tagging modes).
    pub tag: String,
    /// Field name carrying the variant payload (for adjacently tagging).
    pub content: String,
}

impl Default for JsonVariantTypeAttrs {
    fn default() -> Self {
        Self {
            rename_all: RenameFunction::PascalCase,
            tagged: JsonTaggedAttr::Internally,
            tag: "tag".to_owned(),
            content: "content".to_owned(),
        }
    }
}

impl JsonVariantTypeAttrs {
    pub fn tag_field_name(&self) -> String {
        self.tag.clone()
    }

    pub fn content_field_name(&self, variant_attrs: &JsonVariantAttrs) -> String {
        variant_attrs
            .content
            .clone()
            .unwrap_or_else(|| self.content.clone())
    }

    pub fn variant_name(&self, variant: &ir::Variant, variant_attrs: &JsonVariantAttrs) -> String {
        self.variant_name_from_str(variant.name.as_str(), variant_attrs)
    }

    pub fn variant_name_from_str(&self, variant: &str, variant_attrs: &JsonVariantAttrs) -> String {
        if let Some(name) = &variant_attrs.name {
            name.clone()
        } else if let Some(rename) = &variant_attrs.rename {
            rename.apply_to(variant)
        } else {
            self.rename_all.apply_to(variant)
        }
    }
}

/// Read the `JsonVariantTypeAttrs` for `def` from its `typed_attrs["json"]`.
pub fn variant_type_attrs(def: &ir::Def) -> Result<JsonVariantTypeAttrs> {
    let defaults = JsonVariantTypeAttrs::default();
    let Some(raw) = extract::<raw::VariantTypeAttrs>(&def.typed_attrs)? else {
        return Ok(defaults);
    };
    Ok(JsonVariantTypeAttrs {
        rename_all: parse_rename(raw.rename_all)?.unwrap_or(defaults.rename_all),
        tagged: parse_tagged(raw.tagged)?.unwrap_or(defaults.tagged),
        tag: raw.tag.unwrap_or(defaults.tag),
        content: raw.content.unwrap_or(defaults.content),
    })
}

/// JSON attributes that may apply to a variant case.
#[derive(Debug, Clone, Default)]
pub struct JsonVariantAttrs {
    /// Override the JSON tag value with an explicit string.
    pub name: Option<String>,
    /// Apply a rename function to the case name.
    pub rename: Option<RenameFunction>,
    /// Override the per-case content field name.
    pub content: Option<String>,
}

/// Read the `JsonVariantAttrs` for `variant` from its `typed_attrs["json"]`.
pub fn variant_attrs(variant: &ir::Variant) -> Result<JsonVariantAttrs> {
    let Some(raw) = extract::<raw::VariantAttrs>(&variant.typed_attrs)? else {
        return Ok(JsonVariantAttrs::default());
    };
    Ok(JsonVariantAttrs {
        name: raw.name,
        rename: parse_rename(raw.rename)?,
        content: raw.content,
    })
}

/// The variant tagging strategies exposed by the JSON codegen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum JsonTaggedAttr {
    Adjacently,
    Externally,
    #[default]
    Internally,
    Implicitly,
}

fn parse_rename(value: Option<String>) -> Result<Option<RenameFunction>> {
    value
        .map(|s| s.parse::<RenameFunction>().map_err(Into::into))
        .transpose()
}

fn parse_tagged(value: Option<String>) -> Result<Option<JsonTaggedAttr>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let parsed = match value.as_str() {
        "adjacently" => JsonTaggedAttr::Adjacently,
        "externally" => JsonTaggedAttr::Externally,
        "internally" => JsonTaggedAttr::Internally,
        "implicitly" => JsonTaggedAttr::Implicitly,
        _ => {
            return Err(Box::new(Diagnostic::error(format!(
                "Unknown `tagged` value `{value}` — expected one of: \
                 adjacently, externally, internally, implicitly."
            ))));
        }
    };
    Ok(Some(parsed))
}
