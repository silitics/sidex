//! The `fmt` command — autoformatter for Sidex schemas.

use std::path::{Path, PathBuf};

use clap::Parser;
use eyre::Result;
use sidex_core::bundle;
use sidex_fmt::FormatOptions;

#[derive(Parser, Debug)]
pub struct FmtArgs {
    /// Files or directories to format. When empty, formats every `.sidex`
    /// schema in the current bundle.
    pub paths: Vec<PathBuf>,
    /// Check formatting without writing changes. Exits with status 1 if any
    /// file would change.
    #[clap(long)]
    pub check: bool,
}

pub fn exec(args: &FmtArgs) -> Result<()> {
    let opts = format_options_from_cwd()?;

    let files = collect_files(&args.paths)?;
    if files.is_empty() {
        eyre::bail!("no `.sidex` files to format");
    }

    let mut diff_count = 0;
    let mut error_count = 0;

    for file in &files {
        let src = std::fs::read_to_string(file)?;
        match sidex_fmt::format_with(&src, &opts) {
            Ok(out) => {
                if out != src {
                    diff_count += 1;
                    if args.check {
                        println!("would reformat: {}", file.display());
                    } else {
                        std::fs::write(file, &out)?;
                        println!("formatted: {}", file.display());
                    }
                }
            }
            Err(err) => {
                eprintln!("error: {}: {}", file.display(), err);
                error_count += 1;
            }
        }
    }

    if error_count > 0 {
        eprintln!("{} file(s) failed to format", error_count);
        std::process::exit(2);
    }

    if args.check {
        if diff_count > 0 {
            eprintln!("{} file(s) would be reformatted", diff_count);
            std::process::exit(1);
        }
        println!("✅ {} file(s) already formatted", files.len());
    } else if diff_count == 0 {
        println!("✅ {} file(s) already formatted", files.len());
    }

    Ok(())
}

/// Build [`FormatOptions`] from the current working directory's bundle, if
/// one is reachable. Bundle dependencies populate `external_bundles` so that
/// imports referencing them sub-group correctly.
fn format_options_from_cwd() -> Result<FormatOptions> {
    let mut opts = FormatOptions::default();
    let cwd = std::env::current_dir()?;
    if let Some(bundle_path) = bundle::try_locate_bundle(&cwd)? {
        if let Ok(manifest) = bundle::try_load_manifest(&bundle_path) {
            for (name, _) in manifest.dependencies() {
                opts.external_bundles.push(name.to_owned());
            }
        }
    }
    opts.external_bundles.sort();
    opts.external_bundles.dedup();
    Ok(opts)
}

/// Resolve a list of input paths to a flat list of `.sidex` file paths. When
/// `paths` is empty, falls back to schemas in the enclosing bundle.
fn collect_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    if paths.is_empty() {
        let cwd = std::env::current_dir()?;
        let bundle_path = bundle::try_locate_bundle(&cwd)?
            .ok_or_else(|| eyre::eyre!("no `.sidex` files passed and no bundle found"))?;
        let mut out = Vec::new();
        for entry in bundle::iter_schemas(&bundle_path)? {
            out.push(entry?);
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for path in paths {
        if path.is_dir() {
            walk_dir_collect_sidex(path, &mut out)?;
        } else if path.is_file() {
            out.push(path.clone());
        } else {
            eyre::bail!("path does not exist: {}", path.display());
        }
    }
    Ok(out)
}

fn walk_dir_collect_sidex(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            walk_dir_collect_sidex(&path, out)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("sidex") {
            out.push(path);
        }
    }
    Ok(())
}
