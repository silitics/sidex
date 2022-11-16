//! The `gen` command.

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use sidex_gen::Generator;
use sidex_gen_ir::IrGenerator;
use sidex_gen_rs::RustGenerator;

use crate::utils::load_cwd_unit_and_bundle;

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
        Self { registry }
    }
}

pub fn exec(args: &GenerateArgs) -> eyre::Result<()> {
    let (unit, bundle) = load_cwd_unit_and_bundle()?;

    let job = sidex_gen::Job {
        unit: &unit,
        bundle,
        output: &args.output,
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
