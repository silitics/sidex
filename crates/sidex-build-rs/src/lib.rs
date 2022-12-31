#![doc = include_str!("../README.md")]

use std::{error::Error, fs, path::Path};

use sidex_core::transformer::Transformer;
use sidex_derive_mutation::derive_mutations;
use sidex_gen::Generator;
use sidex_gen_rs::RustGenerator;

mod utils;

pub fn build(bundle_path: &Path, output_path: &Path) {
    let ctx = sidex_core::diagnostics::DiagnosticCtx::new();

    let mut transformer = Transformer::new();

    let result: Result<_, Box<dyn Error>> = ctx.exec(|| {
        let idx = transformer.load_bundle_recursive(&bundle_path)?;

        Ok((transformer.transform(), idx))
    });

    ctx.report().eprint(&transformer.storage);

    let (mut unit, bundle) = result.unwrap();

    derive_mutations(&mut unit);

    fs::create_dir_all(&output_path).unwrap();

    let null = serde_json::Value::Null;
    let config = transformer
        .get_bundle_manifest(bundle)
        .backend
        .get("rust")
        .unwrap_or(&null);

    let job = sidex_gen::Job {
        unit: &unit,
        bundle,
        output: &output_path,
        config,
    };

    let generator = RustGenerator::new();
    generator.generate(job).unwrap();

    let schemas_dir = bundle_path.join("schemas");

    for entry in fs::read_dir(&schemas_dir).unwrap() {
        let entry = entry.unwrap();

        utils::emit_cargo_rerun_if_changed(entry.path());
    }

    utils::emit_cargo_rerun_if_changed(&schemas_dir);
    utils::emit_cargo_rerun_if_changed(bundle_path.join("sidex.toml"));
    utils::emit_cargo_rerun_if_changed("build.rs");
}
