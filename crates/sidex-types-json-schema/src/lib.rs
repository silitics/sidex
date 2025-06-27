//! Sidex-generated Rust types for [JSON Schema](https://json-schema.org).

mod generated;

pub use generated::schema::*;
use serde::Serialize;

impl SchemaRef {
    pub fn new(string: String) -> Self {
        Self(string)
    }
}

impl From<serde_json::Value> for Any {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => todo!(),
            serde_json::Value::Bool(v) => Self::Boolean(v),
            serde_json::Value::Number(_) => {
                todo!()
            }
            serde_json::Value::String(v) => Self::String(v),
            serde_json::Value::Array(v) => Self::Array(v.into_iter().map(Into::into).collect()),
            serde_json::Value::Object(v) => {
                Self::Object(v.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
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

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::Integer(value.into())
    }
}
