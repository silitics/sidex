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

/// A coarse-grained shape classification for a [`JsonUnionType`], suitable
/// for codegen decisions.
///
/// Codegens lowering an opaque type to its JSON-primitive native equivalent
/// only need to distinguish three cases: a single JSON type, a nullable
/// single type (`T | null`), and "anything else" — a multi-type union or
/// `any` that must fall back to the language's catch-all JSON value type.
/// The exact mapping of [`JsonType`] to a native type is left to each codegen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonShape {
    /// Exactly one constituent JSON type.
    Single(JsonType),
    /// Exactly two constituent types, one of which is [`JsonType::Null`].
    NullableSingle(JsonType),
    /// Empty, three-or-more types, or contains [`JsonType::Any`] — best
    /// represented by the language's untyped JSON value.
    Other,
}

impl JsonUnionType {
    /// Constituent types in a deterministic order: `null`, `boolean`, `number`,
    /// `string`, `array`, `object`, `any`. Useful for codegens that emit a
    /// language-level union and need stable output across runs.
    pub fn types_sorted(&self) -> Vec<JsonType> {
        const ORDER: &[JsonType] = &[
            JsonType::Null,
            JsonType::Boolean,
            JsonType::Number,
            JsonType::String,
            JsonType::Array,
            JsonType::Object,
            JsonType::Any,
        ];
        ORDER
            .iter()
            .copied()
            .filter(|t| self.types.contains(t))
            .collect()
    }

    /// Classify this union into a [`JsonShape`] for codegen lowering decisions.
    pub fn classify(&self) -> JsonShape {
        if self.types.contains(&JsonType::Any) {
            return JsonShape::Other;
        }
        let sorted = self.types_sorted();
        match sorted.as_slice() {
            [t] => JsonShape::Single(*t),
            [JsonType::Null, t] | [t, JsonType::Null] => JsonShape::NullableSingle(*t),
            _ => JsonShape::Other,
        }
    }
}
