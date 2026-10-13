use std::path::PathBuf;

use clap::Parser;
use sidex::formats::json;
use sidex_core::ir;

#[derive(Parser, Debug)]
pub struct ValidateIrArgs {
    pub file: PathBuf,
}

pub fn exec(args: &ValidateIrArgs) -> eyre::Result<()> {
    let src = std::fs::read_to_string(&args.file)?;
    let unit = json::from_str::<ir::Ir>(&src)?;

    assert_eq!(src, json::to_string(&unit)?);

    Ok(())
}
