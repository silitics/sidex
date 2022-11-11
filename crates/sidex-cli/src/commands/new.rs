//! The `new` command.

use clap::Parser;
use sidex_core::{
    ir,
    model::{Manifest, MANIFEST_NAME, SCHEMAS_DIR},
};

#[derive(Parser, Debug)]
pub struct NewArgs {
    /// Name of the Sidex definition to create.
    name: String,
}

pub fn exec(new_args: &NewArgs) -> eyre::Result<()> {
    let cwd = std::env::current_dir()?;

    let manifest_dir = cwd.join(&new_args.name);
    let modules_dir = manifest_dir.join(SCHEMAS_DIR);

    std::fs::create_dir_all(modules_dir)?;

    let manifest_path = manifest_dir.join(MANIFEST_NAME);

    let manifest = Manifest::new(ir::Metadata::new(
        new_args.name.to_owned(),
        "0.1.0".to_owned(),
    ));

    std::fs::write(manifest_path, &toml::to_vec(&manifest).unwrap())?;

    Ok(())
}
