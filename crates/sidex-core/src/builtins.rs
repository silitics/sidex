use std::collections::HashMap;

use crate::bundle::BundleSource;
use crate::bundle::{self};
use crate::transformer::Transformer;

macro_rules! read_std_bundle_file {
    ($($path:tt)*) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../lib/core/",
            $($path)*
        ))
    };
}

macro_rules! std_bundle_schemas {
    ($transformer:expr, [ $($name:literal $(,)?)* ]) => {{
        let mut schemas = HashMap::new();
        $(
            let source_id = $transformer.insert_source(
                read_std_bundle_file!(concat!("schemas/", $name, ".sidex")).to_owned(),
                None,
            );
            schemas.insert($name.to_owned(), source_id);
        )*
        schemas
    }};
}

pub fn std_bundle(transformer: &mut Transformer) -> BundleSource {
    let manifest = bundle::try_parse_manifest(read_std_bundle_file!("sidex.toml"))
        .expect("Manifest of Sidex standard library should be valid.");
    let schemas = std_bundle_schemas!(transformer, ["builtins"]);

    BundleSource {
        manifest,
        schemas,
        path: None,
    }
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
