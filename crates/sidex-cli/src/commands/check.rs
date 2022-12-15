//! The `check` command.

use std::path::PathBuf;

use clap::Parser;
use eyre::Result;

use crate::utils::load_unit_and_bundle;

#[derive(Parser, Debug)]
pub struct CheckArgs {
    pub bundle_dir: Option<PathBuf>,
}

pub fn exec(args: &CheckArgs) -> Result<()> {
    load_unit_and_bundle(args.bundle_dir.as_ref().map(AsRef::as_ref))?;
    println!("✅ Looks good!");
    Ok(())
}
