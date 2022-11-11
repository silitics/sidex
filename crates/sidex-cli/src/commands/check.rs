//! The `check` command.

// use std::ffi::OsStr;

use clap::Parser;
use eyre::Context;
// use sidex_core::elaborate::load;
// use yansi::Paint;

#[derive(Parser, Debug)]
pub struct CheckArgs {}

pub fn exec(_check_args: &CheckArgs) -> eyre::Result<()> {
    let cwd = std::env::current_dir()?;

    println!("{:?}", cwd);

    let manifest_path = sidex_core::model::try_locate_manifest(&cwd)
        .context("Make sure to be in a (child) directory of a Sidex definition.")?;

    //let _unit = load(manifest_path);

    Ok(())
}
