//! JSON Schemas of the JSON representation.

use std::collections::HashSet;

use super::Tagging;
use super::Type;
use crate::Shape;
use crate::Value;
use crate::shape::RecordShape;
use crate::shape::Scalar;
use crate::shape::ShapeKind;
use crate::shape::VariantShape;
use crate::types::Map;
use crate::types::Str;

/// Returns a JSON Schema describing the JSON representation of values of the
/// shape.
///
/// The schema follows JSON Schema 2020-12. Records and variants are defined in
/// `$defs` and referenced, so that recursive types have finite schemas. The
/// root is also described in place.
///
/// ```
/// use sidex::Shaped;
///
/// let schema = sidex::formats::json::schema(<Vec<u64>>::SHAPE);
/// assert_eq!(schema.get("type"), Some(&"array".into()));
/// ```
pub fn schema(shape: &'static Shape) -> Value {
    let mut builder = Builder {
        defs: Map::new(),
        visiting: HashSet::from([shape.to_string()]),
        pending: Vec::new(),
    };
    let mut schema = builder.describe(shape);
    builder.visiting.clear();
    // Define recursively referenced shapes, whose construction was underway.
    while let Some(pending) = builder.pending.pop() {
        builder.schema(pending);
    }
    if !builder.defs.is_empty()
        && let Value::Map(entries) = &mut schema
    {
        entries.insert("$defs".into(), Value::Map(builder.defs));
    }
    schema
}

/// Returns a [`schemars`](https://docs.rs/schemars) schema describing the JSON
/// representation of values of the shape.
///
/// Definitions are added to the generator, so that generated Rust types can
/// implement `schemars::JsonSchema` with this function.
#[cfg(feature = "schemars-1")]
pub fn schemars_schema(
    shape: &'static Shape,
    generator: &mut schemars::SchemaGenerator,
) -> schemars::Schema {
    let mut schema = serde_json::Value::from(self::schema(shape));
    let path = generator
        .settings()
        .definitions_path
        .trim_start_matches('#')
        .trim_end_matches('/')
        .to_owned();
    rewrite_refs(&mut schema, &format!("#{path}/"));
    if let Some(serde_json::Value::Object(defs)) = schema
        .as_object_mut()
        .and_then(|schema| schema.remove("$defs"))
    {
        let definitions = generator.definitions_mut();
        for (name, definition) in defs {
            definitions.entry(name).or_insert(definition);
        }
    }
    schemars::Schema::try_from(schema).expect("schemas of shapes are objects")
}

/// Rewrites references to definitions to use the given prefix.
#[cfg(feature = "schemars-1")]
fn rewrite_refs(value: &mut serde_json::Value, prefix: &str) {
    match value {
        serde_json::Value::Object(entries) => {
            for (key, value) in entries.iter_mut() {
                if key == "$ref"
                    && let serde_json::Value::String(reference) = value
                    && let Some(name) = reference.strip_prefix("#/$defs/")
                {
                    *reference = format!("{prefix}{name}");
                } else {
                    rewrite_refs(value, prefix);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                rewrite_refs(item, prefix);
            }
        }
        _ => {}
    }
}

/// Builds a schema and its definitions.
struct Builder {
    defs: Map<Str, Value>,
    /// Names of the definitions under construction.
    visiting: HashSet<String>,
    /// Shapes referenced while under construction.
    pending: Vec<&'static Shape>,
}

impl Builder {
    /// Returns a schema for values of the shape, referencing definitions of
    /// records and variants.
    fn schema(&mut self, shape: &'static Shape) -> Value {
        if !matches!(shape.kind, ShapeKind::Record(_) | ShapeKind::Variant(_)) {
            return self.describe(shape);
        }
        let name = shape.to_string();
        if !self.defs.contains_key(name.as_str()) && !self.visiting.contains(&name) {
            self.visiting.insert(name.clone());
            let definition = self.describe(shape);
            self.visiting.remove(&name);
            self.defs.insert(name.as_str().into(), definition);
        } else if self.visiting.contains(&name) && !self.defs.contains_key(name.as_str()) {
            // A recursive reference, possibly to the root, which needs a
            // definition, too.
            self.pending.push(shape);
        }
        object([("$ref", format!("#/$defs/{}", escape(&name)).into())])
    }

    /// Returns a schema describing the shape in place.
    fn describe(&mut self, shape: &'static Shape) -> Value {
        let mut schema = match &shape.kind {
            ShapeKind::Scalar(scalar) => scalar_schema(*scalar),
            ShapeKind::Sequence(item) => {
                object([("type", "array".into()), ("items", self.schema(item()))])
            }
            ShapeKind::Map(_, value) => {
                object([
                    ("type", "object".into()),
                    ("additionalProperties", self.schema(value())),
                ])
            }
            ShapeKind::Record(record) => self.record(record),
            ShapeKind::Variant(variant) => self.variant(variant),
            ShapeKind::Wrapper(inner) => self.schema(inner()),
            ShapeKind::Opaque(opaque) => {
                match opaque.json.types {
                    Some(types) => {
                        object([(
                            "type",
                            Value::from(types.iter().map(|ty| type_name(*ty)).collect::<Vec<_>>()),
                        )])
                    }
                    None => object([]),
                }
            }
            ShapeKind::Value => object([]),
        };
        if let Value::Map(entries) = &mut schema {
            if let Some(docs) = shape.docs {
                entries.insert("description".into(), docs.trim().into());
            }
            if shape.stability.deprecated.is_some() {
                entries.insert("deprecated".into(), true.into());
            }
        }
        schema
    }

    /// Returns an object schema for the fields of the record.
    fn record(&mut self, record: &'static RecordShape) -> Value {
        let mut properties = Map::new();
        let mut required = Vec::new();
        self.fields(record, &mut properties, &mut required);
        let mut schema = object([("type", "object".into()), ("properties", properties.into())]);
        if !required.is_empty()
            && let Value::Map(entries) = &mut schema
        {
            entries.insert("required".into(), required.into());
        }
        schema
    }

    /// Collects the properties of the record's fields, merging inline records.
    fn fields(
        &mut self,
        record: &'static RecordShape,
        properties: &mut Map<Str, Value>,
        required: &mut Vec<Value>,
    ) {
        for field in record.fields {
            let shape = (field.shape)();
            if field.json.inline
                && let Some(inner) = shape.as_record()
            {
                self.fields(inner, properties, required);
                continue;
            }
            let mut property = self.schema(shape);
            if let Value::Map(entries) = &mut property {
                if let Some(docs) = field.docs {
                    entries.insert("description".into(), docs.trim().into());
                }
                if field.stability.deprecated.is_some() {
                    entries.insert("deprecated".into(), true.into());
                }
            }
            properties.insert(field.json.name.into(), property);
            if !field.optional {
                required.push(field.json.name.into());
            }
        }
    }

    /// Returns a schema with one alternative per case.
    fn variant(&mut self, variant: &'static VariantShape) -> Value {
        let mut alternatives = Vec::new();
        for case in variant.cases {
            let name = Value::from(case.json.name);
            let mut alternative = match (variant.json, case.payload) {
                (Tagging::External | Tagging::Implicit, None) => object([("const", name)]),
                (Tagging::External, Some(payload)) => {
                    object([
                        ("type", "object".into()),
                        (
                            "properties",
                            Value::from_iter([(case.json.name, self.schema(payload()))]),
                        ),
                        ("required", Value::from(vec![name])),
                        ("additionalProperties", false.into()),
                    ])
                }
                (Tagging::Implicit, Some(payload)) => self.schema(payload()),
                (Tagging::Internal { tag, .. } | Tagging::Adjacent { tag, .. }, payload) => {
                    let mut properties = Map::new();
                    let mut required = vec![Value::from(tag)];
                    properties.insert(tag.into(), object([("const", name)]));
                    if let Some(payload) = payload {
                        let payload = payload();
                        match case.json.content_field(variant.json, payload) {
                            Some(content) => {
                                properties.insert(content.into(), self.schema(payload));
                                required.push(content.into());
                            }
                            None => {
                                let record = payload.as_record().expect("only records are merged");
                                self.fields(record, &mut properties, &mut required);
                            }
                        }
                    }
                    object([
                        ("type", "object".into()),
                        ("properties", properties.into()),
                        ("required", required.into()),
                    ])
                }
            };
            if let Value::Map(entries) = &mut alternative {
                if let Some(docs) = case.docs {
                    entries.insert("description".into(), docs.trim().into());
                }
                if case.stability.deprecated.is_some() {
                    entries.insert("deprecated".into(), true.into());
                }
            }
            alternatives.push(alternative);
        }
        let combinator = match variant.json {
            // Decoding takes the first case that matches.
            Tagging::Implicit => "anyOf",
            _ => "oneOf",
        };
        object([(combinator, alternatives.into())])
    }
}

/// Returns the schema of a scalar.
fn scalar_schema(scalar: Scalar) -> Value {
    let integer = |min: i128, max: i128| {
        object([
            ("type", "integer".into()),
            ("minimum", Value::Int(min)),
            ("maximum", Value::Int(max)),
        ])
    };
    let string = |format: &str| object([("type", "string".into()), ("format", format.into())]);
    match scalar {
        Scalar::Unit => object([("type", "null".into())]),
        Scalar::Bool => object([("type", "boolean".into())]),
        Scalar::I8 => integer(i8::MIN.into(), i8::MAX.into()),
        Scalar::I16 => integer(i16::MIN.into(), i16::MAX.into()),
        Scalar::I32 => integer(i32::MIN.into(), i32::MAX.into()),
        Scalar::U8 => integer(0, u8::MAX.into()),
        Scalar::U16 => integer(0, u16::MAX.into()),
        Scalar::U32 => integer(0, u32::MAX.into()),
        Scalar::Idx => object([("type", "integer".into()), ("minimum", Value::Int(0))]),
        // Integers, or decimal strings beyond 2^53 - 1.
        Scalar::I64 => {
            object([
                ("type", Value::from(vec!["integer".into(), "string".into()])),
                ("pattern", "^-?[0-9]+$".into()),
            ])
        }
        Scalar::U64 => {
            object([
                ("type", Value::from(vec!["integer".into(), "string".into()])),
                ("minimum", Value::Int(0)),
                ("pattern", "^[0-9]+$".into()),
            ])
        }
        // Non-finite floats are strings.
        Scalar::F32 | Scalar::F64 => {
            object([
                ("type", Value::from(vec!["number".into(), "string".into()])),
                ("pattern", "^(NaN|-?Infinity)$".into()),
            ])
        }
        Scalar::String => object([("type", "string".into())]),
        Scalar::Bytes => {
            object([
                ("type", "string".into()),
                ("contentEncoding", "base64".into()),
            ])
        }
        Scalar::Uuid => string("uuid"),
        Scalar::Timestamp => string("date-time"),
        Scalar::Date => string("date"),
    }
}

/// Returns the name of a JSON type in JSON Schema.
fn type_name(ty: Type) -> Value {
    match ty {
        Type::Null => "null",
        Type::Boolean => "boolean",
        Type::Number => "number",
        Type::String => "string",
        Type::Array => "array",
        Type::Object => "object",
    }
    .into()
}

/// Creates an object schema from entries.
fn object<const N: usize>(entries: [(&str, Value); N]) -> Value {
    Value::from_iter(entries)
}

/// Escapes a definition name for use in a JSON pointer.
fn escape(name: &str) -> String {
    name.replace('~', "~0").replace('/', "~1")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Shaped;

    /// Scalars describe their canonical representation.
    #[test]
    fn scalars() {
        assert_eq!(
            schema(u64::SHAPE).get("pattern"),
            Some(&Value::from("^[0-9]+$"))
        );
        assert_eq!(schema(u8::SHAPE).get("maximum"), Some(&Value::from(255)));
        assert_eq!(
            schema(<Vec<String>>::SHAPE),
            Value::from_iter([
                ("type", Value::from("array")),
                ("items", Value::from_iter([("type", "string")])),
            ])
        );
    }
}
