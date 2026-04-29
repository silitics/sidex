//! Sidex-generated Rust types for the [OpenAPI](https://spec.openapis.org)
//! specification.
//!
//! The generated code is produced at build time from the bundle at
//! `lib/openapi/` (alongside its `json_schema` dep at `lib/json-schema/`)
//! via [`sidex_build_rs`]; nothing is checked in, so the encoding of the
//! generated types tracks the current `sidex-gen-rs` codegen automatically.
//! Both bundles are kept inside this crate so a downstream consumer
//! vendoring only the published `sidex-types-openapi` package still has
//! everything the build script needs.

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
