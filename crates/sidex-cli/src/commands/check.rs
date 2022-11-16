//! The `check` command.

use eyre::Result;

use crate::utils::load_cwd_unit_and_bundle;

pub fn exec() -> Result<()> {
    load_cwd_unit_and_bundle()?;
    println!("✅ Looks good!");
    Ok(())
}
