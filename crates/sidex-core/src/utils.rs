use std::path::Path;

use crate::{bundle, ir, transformer::Transformer};

pub fn load_unit_and_bundle(path: &Path) -> eyre::Result<(ir::Unit, ir::BundleIdx, Transformer)> {
    let bundle_path = bundle::try_locate_bundle(path)?
        .ok_or_else(|| eyre::eyre!("Unable to locate Sidex bundle directory."))?;
    // .suggestion("Make sure to be in a (child) directory of a Sidex bundle.")?;

    let mut transformer = Transformer::new();
    let idx = transformer.load_bundle_recursive(&bundle_path)?;

    Ok((transformer.transform(), idx, transformer))
}
