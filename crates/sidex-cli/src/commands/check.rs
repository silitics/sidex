//! The `check` command.

use std::path::PathBuf;

use clap::Parser;
use eyre::Result;
use sidex_core::lints;
use sidex_diagnostics::DiagnosticCtx;
use sidex_fmt::FormatOptions;

use crate::utils::load_unit_and_bundle;

#[derive(Parser, Debug)]
pub struct CheckArgs {
    pub bundle_dir: Option<PathBuf>,
    /// Auto-fix lint findings where possible. Currently removes unused
    /// imports.
    #[clap(long)]
    pub fix: bool,
}

pub fn exec(args: &CheckArgs) -> Result<()> {
    let (_unit, bundle_idx, transformer) =
        load_unit_and_bundle(args.bundle_dir.as_ref().map(AsRef::as_ref))?;

    let findings = lints::collect_unused_imports(&transformer, bundle_idx);

    if args.fix {
        return apply_fixes(&transformer, bundle_idx, findings);
    }

    let ctx = DiagnosticCtx::new();
    ctx.exec(|| lints::lint_unused_imports(&transformer, bundle_idx));
    let report = ctx.report();
    report.eprint(&transformer.storage);

    if report.has_error() {
        std::process::exit(1);
    }

    println!("✅ Looks good!");
    Ok(())
}

fn apply_fixes(
    transformer: &sidex_core::transformer::Transformer,
    bundle_idx: sidex_core::ir::BundleIdx,
    findings: Vec<lints::UnusedImport>,
) -> Result<()> {
    if findings.is_empty() {
        println!("✅ Nothing to fix.");
        return Ok(());
    }

    let by_schema = lints::unused_imports_by_schema(&findings);
    let manifest = transformer.get_bundle_manifest(bundle_idx);
    let mut external_bundles = vec!["std".to_owned()];
    for (name, _) in manifest.dependencies() {
        external_bundles.push(name.to_owned());
    }
    external_bundles.sort();
    external_bundles.dedup();

    let mut fixed_count = 0;
    let mut removed_count = 0;
    for (schema_idx, excluded) in by_schema {
        let source_idx = transformer
            .schema_source_idx(bundle_idx, schema_idx)
            .ok_or_else(|| eyre::eyre!("schema source not found"))?;
        let source = &transformer.storage[source_idx];
        let path = source
            .origin
            .as_ref()
            .ok_or_else(|| eyre::eyre!("schema has no on-disk origin"))?;
        let text = source
            .text
            .as_ref()
            .ok_or_else(|| eyre::eyre!("schema has no source text"))?;
        let opts = FormatOptions {
            external_bundles: external_bundles.clone(),
            excluded_imports: excluded.iter().cloned().collect(),
            ..FormatOptions::default()
        };
        let formatted = sidex_fmt::format_with(text, &opts)
            .map_err(|e| eyre::eyre!("failed to format {}: {}", path, e))?;
        if &formatted != text {
            std::fs::write(path, &formatted)?;
            fixed_count += 1;
            removed_count += excluded.len();
            println!(
                "fixed {}: removed {} unused import(s)",
                path,
                excluded.len()
            );
        }
    }

    if fixed_count == 0 {
        println!("✅ Nothing to fix.");
    } else {
        println!(
            "✅ removed {} unused import(s) across {} file(s)",
            removed_count, fixed_count
        );
    }

    Ok(())
}
