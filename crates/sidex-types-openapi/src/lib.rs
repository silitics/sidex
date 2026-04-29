//! Sidex-generated Rust types for the [OpenAPI](https://spec.openapis.org)
//! specification.
//!
//! The generated code is produced at build time from `lib/openapi/` via
//! [`sidex_build_rs`]; nothing is checked in, so the encoding of the
//! generated types tracks the current `sidex-gen-rs` codegen automatically.

use indexmap::IndexMap;
pub use sidex_types_json_schema as schema;

sidex::include_bundle!(openapi as generated);

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
