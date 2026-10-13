use std::path::Path;

use color_eyre::Help;
use eyre::Result;
use sidex_core::ir;
use sidex_core::transformer::Transformer;

/// Error indicating that errors have been reported as diagnostics.
#[derive(Debug)]
pub struct ReportedErrors;

impl std::fmt::Display for ReportedErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("The Sidex bundle has errors.")
    }
}

impl std::error::Error for ReportedErrors {}

pub fn load_unit_and_bundle(
    directory: Option<&Path>,
) -> Result<(ir::Ir, ir::BundleIdx, Transformer)> {
    let cwd = if let Some(directory) = directory {
        directory.to_owned()
    } else {
        std::env::current_dir()?
    };

    let ctx = sidex_diagnostics::DiagnosticCtx::new();

    let mut transformer = Transformer::new();

    let result = ctx.exec(|| {
        let bundle_path = sidex_core::bundle::try_locate_bundle(&cwd)?
            .ok_or_else(|| eyre::eyre!("Unable to locate Sidex bundle directory."))
            .suggestion("Make sure to be in a (child) directory of a Sidex bundle.")?;

        let idx = transformer.load_bundle_recursive(&bundle_path)?;
        let unit = transformer.transform(idx);
        Ok::<_, eyre::Report>((unit, idx))
    });

    let report = ctx.report();
    report.eprint(&transformer.sources);
    let (unit, bundle_idx) = result?;
    if report.has_error() {
        return Err(ReportedErrors.into());
    }
    Ok((unit, bundle_idx, transformer))
}
