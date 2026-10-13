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
