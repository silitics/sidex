//! The `gen` command.

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use sidex_gen::Generator;
use sidex_gen_ir::IrGenerator;
use sidex_gen_openapi::OpenApiGenerator;
use sidex_gen_rs::RustGenerator;
use sidex_gen_ts::TsGenerator;

use crate::utils::load_unit_and_bundle;

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    pub backend: String,
    pub output: PathBuf,
}

struct GeneratorRegistry {
    registry: HashMap<String, Box<dyn Generator>>,
}

impl GeneratorRegistry {
    pub fn new() -> Self {
        let mut registry: HashMap<_, Box<dyn Generator>> = HashMap::new();
        registry.insert("ir".to_owned(), Box::new(IrGenerator));
        registry.insert("rust".to_owned(), Box::new(RustGenerator::new()));
        registry.insert("ts".to_owned(), Box::new(TsGenerator::new()));
        registry.insert("openapi".to_owned(), Box::new(OpenApiGenerator));
        Self { registry }
    }
}

pub fn exec(args: &GenerateArgs) -> eyre::Result<()> {
    let (unit, bundle, transformer) = load_unit_and_bundle(None)?;

    let null = serde_json::Value::Null;
    let config = transformer
        .get_bundle_manifest(bundle)
        .backend
        .get(&args.backend)
        .unwrap_or(&null);

    let job = sidex_gen::Job {
        unit: &unit,
        bundle,
        output: &args.output,
        config,
    };

    std::fs::create_dir_all(&args.output)?;

    GeneratorRegistry::new()
        .registry
        .get(&args.backend)
        .unwrap()
        .generate(job)
        .map_err(|err| eyre::eyre!(format!("{err:?}")))?;

    Ok(())
}
