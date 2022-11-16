//! The `new` command.

use clap::Parser;
use sidex_core::{
    bundle::{Manifest, MANIFEST_NAME, SCHEMAS_DIR},
    ir,
};

#[derive(Parser, Debug)]
pub struct NewArgs {
    /// Name of the Sidex definition to create.
    name: String,
}

pub fn exec(args: &NewArgs) -> eyre::Result<()> {
    let cwd = std::env::current_dir()?;

    let bundle_dir = cwd.join(&args.name);
    let schemas_dir = bundle_dir.join(SCHEMAS_DIR);

    std::fs::create_dir_all(schemas_dir)?;

    let manifest_path = bundle_dir.join(MANIFEST_NAME);

    let manifest = Manifest::new(ir::Metadata::new(args.name.to_owned(), "0.1.0".to_owned()));

    std::fs::write(manifest_path, &toml::to_vec(&manifest).unwrap())?;

    Ok(())
}
