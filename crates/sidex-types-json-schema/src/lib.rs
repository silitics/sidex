//! Sidex-generated Rust types for [JSON Schema](https://json-schema.org).

mod generated;

pub use generated::schema::*;

impl SchemaRef {
    pub fn new(string: String) -> Self {
        Self(string)
    }
}

impl Pattern {
    pub fn new(string: String) -> Self {
        Self(string)
    }
}

impl From<SchemaObject> for Schema {
    fn from(value: SchemaObject) -> Self {
        Schema::Object(value)
    }
}

impl From<bool> for Schema {
    fn from(value: bool) -> Self {
        Schema::Bool(value)
    }
}

impl<T> From<T> for MaybeArray<T> {
    fn from(value: T) -> Self {
        MaybeArray::Single(value)
    }
}

impl<T> From<Vec<T>> for MaybeArray<T> {
    fn from(value: Vec<T>) -> Self {
        MaybeArray::Array(value)
    }
}

#[cfg(test)]
mod tests {
    use sidex::formats::json;

    use super::SchemaObject;

    /// Required inline keyword groups preserve empty and populated JSON Schema objects.
    #[test]
    fn keyword_groups_roundtrip() {
        for source in [
            "{}",
            r#"{"title":"Sample","allOf":[true],"minimum":1,"pattern":"x","items":true,"required":["x"]}"#,
        ] {
            let schema: SchemaObject = json::from_str(source).unwrap();
            assert_eq!(json::to_string(&schema).unwrap(), source);
            let schema: SchemaObject = serde_json::from_str(source).unwrap();
            assert_eq!(serde_json::to_string(&schema).unwrap(), source);
        }
    }
}
