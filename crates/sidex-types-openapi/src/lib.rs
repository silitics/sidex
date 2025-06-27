use indexmap::IndexMap;
pub use sidex_types_json_schema as schema;

mod generated;

pub use generated::openapi::*;

impl Markdown {
    pub fn new(string: String) -> Self {
        Self(string)
    }
}

impl Responses {
    pub fn new(map: IndexMap<String, MaybeRef<Response>>) -> Self {
        Self(map)
    }
}

impl Paths {
    pub fn new(map: IndexMap<String, PathItem>) -> Self {
        Self(map)
    }
}
