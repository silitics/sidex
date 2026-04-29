use std::collections::HashMap;

use crate::bundle::BundleSource;
use crate::bundle::{self};
use crate::transformer::Transformer;

// The `core`, `py`, `json`, and `rust` Sidex bundles are read at compile
// time and embedded into the binary, so they must live inside the
// `sidex-core` crate directory — otherwise `cargo publish` / vendored
// downstream consumers (e.g. Nix-built apps) wouldn't ship them.
macro_rules! read_lib_file {
    ($dir:literal, $($path:tt)*) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/lib/",
            $dir,
            "/",
            $($path)*
        ))
    };
}

macro_rules! lib_bundle_schemas {
    ($transformer:expr, $dir:literal, [ $($name:literal $(,)?)* ]) => {{
        let mut schemas = HashMap::new();
        $(
            let source_id = $transformer.insert_source(
                read_lib_file!($dir, concat!("schemas/", $name, ".sidex")).to_owned(),
                None,
            );
            schemas.insert($name.to_owned(), source_id);
        )*
        schemas
    }};
}

pub fn std_bundle(transformer: &mut Transformer) -> BundleSource {
    let manifest = bundle::try_parse_manifest(read_lib_file!("core", "sidex.toml"))
        .expect("Manifest of Sidex standard library should be valid.");
    let schemas = lib_bundle_schemas!(transformer, "core", ["builtins", "attrs"]);

    BundleSource {
        manifest,
        schemas,
        path: None,
    }
}

/// Plugin attribute bundles that the compiler auto-loads alongside every user
/// bundle so that the typed-attrs parser can validate `#[<plugin>(...)]`
/// attributes against a known schema.
///
/// These bundles are flagged [`is_internal`](sidex_ir::Bundle::is_internal)
/// in the IR. Code generators should skip internal bundles since their defs
/// are an implementation detail of the compiler, not part of the user's API
/// surface.
pub fn plugin_attrs_bundles(transformer: &mut Transformer) -> Vec<BundleSource> {
    let py_manifest = bundle::try_parse_manifest(read_lib_file!("py", "sidex.toml"))
        .expect("Manifest of py-attrs bundle should be valid.");
    let py_schemas = lib_bundle_schemas!(transformer, "py", ["attrs"]);
    let json_manifest = bundle::try_parse_manifest(read_lib_file!("json", "sidex.toml"))
        .expect("Manifest of json-attrs bundle should be valid.");
    let json_schemas = lib_bundle_schemas!(transformer, "json", ["attrs"]);
    let rust_manifest = bundle::try_parse_manifest(read_lib_file!("rust", "sidex.toml"))
        .expect("Manifest of rust-attrs bundle should be valid.");
    let rust_schemas = lib_bundle_schemas!(transformer, "rust", ["attrs"]);
    vec![
        BundleSource {
            manifest: py_manifest,
            schemas: py_schemas,
            path: None,
        },
        BundleSource {
            manifest: json_manifest,
            schemas: json_schemas,
            path: None,
        },
        BundleSource {
            manifest: rust_manifest,
            schemas: rust_schemas,
            path: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_load_std_bundle() {
        let mut transformer = Transformer::new();
        let bundle = std_bundle(&mut transformer);
        transformer.insert_bundle(bundle).unwrap();
    }
}
