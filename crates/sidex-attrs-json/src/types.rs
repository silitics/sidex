use std::{collections::HashSet, str::FromStr};

use sidex_gen::{
    attrs::{AttrConvertExt, TryFromAttr},
    diagnostics, ir,
};

/// A JSON type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonType {
    /// JSON number type.
    Number,
    /// JSON boolean type.
    Boolean,
    /// JSON string type.
    String,
    /// JSON null type.
    Null,
    /// JSON object type.
    Object,
    /// JSON array type.
    Array,
    /// JSON any type.
    Any,
}

impl FromStr for JsonType {
    type Err = diagnostics::Diagnostic;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "number" => Ok(Self::Number),
            "boolean" => Ok(Self::Boolean),
            "string" => Ok(Self::String),
            "null" => Ok(Self::Null),
            "object" => Ok(Self::Object),
            "array" => Ok(Self::Array),
            "any" => Ok(Self::Any),
            _ => {
                Err(diagnostics::Diagnostic::error(format!(
                    "`{string}` is not a valid JSON type."
                )).with_help(
                    "A JSON type must be either `number`, `boolean`, `string`, `null`, `object`, `array`, or `any`."
                ))
            }
        }
    }
}

impl TryFromAttr for JsonType {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        attr.expect_from_string()
    }
}

/// A union over JSON types.
#[derive(Debug, Clone)]
pub struct JsonUnionType {
    /// The JSON types.
    pub types: HashSet<JsonType>,
}

impl FromStr for JsonUnionType {
    type Err = diagnostics::Diagnostic;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            types: string
                .split("|")
                .map(str::trim)
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFromAttr for JsonUnionType {
    fn try_from_attr(attr: &ir::Attr) -> diagnostics::Result<Self> {
        attr.expect_from_string()
    }
}
