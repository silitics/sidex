use std::path::Path;

use color_eyre::Help;
use eyre::Result;
use sidex_core::{ir, transformer::Transformer};
use sidex_derive_mutation::derive_mutations;

pub fn load_unit_and_bundle(
    directory: Option<&Path>,
) -> Result<(ir::Unit, ir::BundleIdx, Transformer)> {
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

        Ok((transformer.transform(), idx))
    });

    ctx.report().eprint(&transformer.storage);

    result.map(|(mut unit, bundle_idx)| {
        derive_mutations(&mut unit);
        (unit, bundle_idx, transformer)
    })
}
