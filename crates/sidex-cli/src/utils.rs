use color_eyre::Help;
use eyre::Result;
use sidex_core::{ir, transformer::Transformer};

pub fn load_cwd_unit_and_bundle() -> Result<(ir::Unit, ir::BundleIdx, Transformer)> {
    let cwd = std::env::current_dir()?;

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

    result.map(|(x, y)| (x, y, transformer))
}
