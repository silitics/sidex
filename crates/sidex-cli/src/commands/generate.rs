//! The `gen` command.

use clap::Parser;
use eyre::Context;
// use sidex_core::elaborate::load;

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    pub backend: String,
}

pub fn exec(_gen_args: &GenerateArgs) -> eyre::Result<()> {
    let cwd = std::env::current_dir()?;

    let manifest_path = sidex_core::model::try_locate_manifest(&cwd)
        .context("Make sure to be in a (child) directory of a Sidex definition.")?;

    // let (unit, root_id) = load(manifest_path)?;

    // println!("{}", sidex_gen_rs::generate_code(&unit, root_id));

    Ok(())
}
