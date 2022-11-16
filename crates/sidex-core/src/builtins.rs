use std::collections::HashMap;

use sidex_syntax::source::SourceStorage;

use crate::bundle::{self, BundleSource};

macro_rules! read_std_bundle_file {
    ($($path:tt)*) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../lib/std/",
            $($path)*
        ))
    };
}

macro_rules! std_bundle_schemas {
    ($storage:expr, [ $($name:literal $(,)?)* ]) => {{
        let mut schemas = HashMap::new();
        $(
            let source_id = $storage.insert(
                read_std_bundle_file!(concat!("schemas/", $name, ".sidex")).to_owned(),
                None,
            );
            schemas.insert($name.to_owned(), source_id);
        )*
        schemas
    }};
}

pub fn std_bundle(storage: &mut SourceStorage) -> BundleSource {
    let manifest = bundle::try_parse_manifest(read_std_bundle_file!("sidex.toml"))
        .expect("Manifest of Sidex standard library should be valid.");
    let schemas = std_bundle_schemas!(storage, ["any", "builtins", "option", "result"]);

    BundleSource {
        manifest,
        schemas,
        path: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformer::Transformer;

    #[test]
    pub fn test_load_std_bundle() {
        let mut transformer = Transformer::new();
        let bundle = std_bundle(&mut transformer.storage);
        transformer.insert_bundle(bundle).unwrap();
    }
}
