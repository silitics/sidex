//! The `check` command.

use std::path::PathBuf;

use clap::Parser;
use eyre::Result;
use sidex_core::lints;
use sidex_diagnostics::DiagnosticCtx;

use crate::utils::load_unit_and_bundle;

#[derive(Parser, Debug)]
pub struct CheckArgs {
    pub bundle_dir: Option<PathBuf>,
}

pub fn exec(args: &CheckArgs) -> Result<()> {
    let (_unit, bundle_idx, transformer) =
        load_unit_and_bundle(args.bundle_dir.as_ref().map(AsRef::as_ref))?;

    let ctx = DiagnosticCtx::new();
    ctx.exec(|| {
        lints::lint_unused_imports(&transformer, bundle_idx);
    });
    let report = ctx.report();
    report.eprint(&transformer.storage);

    if report.has_error() {
        std::process::exit(1);
    }

    println!("✅ Looks good!");
    Ok(())
}
