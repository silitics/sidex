//! Helper types for parsing the string-shaped attribute values exposed by
//! the JSON codegen attribute schemas.

use std::collections::HashSet;
use std::str::FromStr;

use sidex_diagnostics::Diagnostic;

/// A primitive JSON type tag.
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
    type Err = Diagnostic;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "number" => Ok(Self::Number),
            "boolean" => Ok(Self::Boolean),
            "string" => Ok(Self::String),
            "null" => Ok(Self::Null),
            "object" => Ok(Self::Object),
            "array" => Ok(Self::Array),
            "any" => Ok(Self::Any),
            _ => Err(Diagnostic::error(format!(
                "`{string}` is not a valid JSON type."
            ))
            .with_help(
                "A JSON type must be either `number`, `boolean`, `string`, `null`, `object`, `array`, or `any`."
            )),
        }
    }
}

/// A union over JSON types, parsed from a `|`-separated source string.
#[derive(Debug, Clone)]
pub struct JsonUnionType {
    /// The constituent JSON types.
    pub types: HashSet<JsonType>,
}

impl FromStr for JsonUnionType {
    type Err = Diagnostic;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            types: string
                .split('|')
                .map(str::trim)
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}
