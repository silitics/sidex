use std::{error::Error, fs, path::Path};

use sidex_core::transformer::Transformer;
use sidex_derive_mutation::derive_mutations;
use sidex_gen::Generator;
use sidex_gen_rs::RustGenerator;

pub fn build(bundle_path: &Path, output_path: &Path) {
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let bundle_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    // let bundle_path = Path::new(&bundle_dir);

    let ctx = sidex_core::diagnostics::DiagnosticCtx::new();

    let mut transformer = Transformer::new();

    let result: Result<_, Box<dyn Error>> = ctx.exec(|| {
        let idx = transformer.load_bundle_recursive(&bundle_path)?;

        Ok((transformer.transform(), idx))
    });

    ctx.report().eprint(&transformer.storage);

    let (mut unit, bundle) = result.unwrap();

    derive_mutations(&mut unit);

    // let out_path = Path::new(&out_dir).join("generated");
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

        println!("cargo:rerun-if-changed={}", entry.path().to_string_lossy());
    }

    println!("cargo:rerun-if-changed={}", schemas_dir.to_string_lossy());
    println!(
        "cargo:rerun-if-changed={}",
        bundle_path.join("sidex.toml").to_string_lossy()
    );

    println!("cargo:rerun-if-changed=build.rs");
}
