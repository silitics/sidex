#![doc = include_str!("../README.md")]
//!
//! ## Example
//!
//! A typical `build.rs` build script may look like this:
//!
//! ```ignore
//! fn main() {
//!     sidex_build_rs::configure()
//!         .with_bundle("./path-to-bundle")
//!         .generate()
//!         .expect("Failed to generate code for Sidex bundles.");
//! }
//! ```
//!
//! It configures a [`Generator`] to generate code for the bundle located at
//! `./path-to-bundle` relative to the `Cargo.toml` manifest of the crate. The generated
//! code can then be included with the [`include_bundle!`] macro.
//!
//! [`include_bundle!`]: https://docs.rs/sidex/latest/sidex/macro.include_bundle.html

use std::{
    error::Error,
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

use sidex_core::transformer::Transformer;
use sidex_gen_rs::RustGenerator;

mod utils;

/// Starts the configuration of a [`Generator`].
pub fn configure() -> Generator {
    Generator::new()
}

/// Interface for generating Rust code from Sidex bundles.
#[must_use]
pub struct Generator {
    /// The output path where to place the generated code.
    output_path: PathBuf,
    /// The root path with respect to which bundle paths are resolved.
    root_path: PathBuf,
    /// The `emit_rerun_if_changed` option.
    emit_rerun_if_changed: bool,
    /// The `format_generated_code` option.
    format_generated_code: bool,
    /// The bundles for which to generate code.
    bundles: Vec<Bundle>,
}

impl Generator {
    /// Constructs a new [`Generator`] with default settings.
    fn new() -> Self {
        Self {
            output_path: utils::default_output_path(),
            root_path: utils::default_root_path(),
            emit_rerun_if_changed: true,
            format_generated_code: true,
            bundles: Vec::new(),
        }
    }

    /// Sets the `emit_rerun_if_changed` option.
    ///
    /// If enabled, `cargo:rerun-if-changed=` directives will be emitted for all relevant
    /// files.
    ///
    /// By default, this option is enabled.
    pub fn emit_rerun_if_changed(mut self, enabled: bool) -> Self {
        self.emit_rerun_if_changed = enabled;
        self
    }

    /// Sets the `format_generated_code` option.
    ///
    /// If enabled, the generated code is formatted by calling `rustfmt`.
    ///
    /// By default, this option is enabled.
    pub fn format_generated_code(mut self, enabled: bool) -> Self {
        self.format_generated_code = enabled;
        self
    }

    /// Adds a bundle for which to generate code.
    pub fn with_bundle<B: Into<Bundle>>(mut self, bundle: B) -> Self {
        self.bundles.push(bundle.into());
        self
    }

    /// Generates code for all bundles.
    pub fn generate(self) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(&self.output_path)?;

        let ctx = sidex_core::diagnostics::DiagnosticCtx::new();

        let mut transformer = Transformer::new();

        let bundle_indices = match self
            .bundles
            .iter()
            .map(|bundle| {
                ctx.exec(|| transformer.load_bundle_recursive(&self.root_path.join(&bundle.path)))
            })
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(indices) => indices,
            Err(err) => {
                ctx.report().eprint(&transformer.storage);
                return Err(Box::new(err));
            }
        };

        let unit = transformer.transform();

        for (bundle_idx, bundle) in bundle_indices.iter().zip(&self.bundles) {
            let null = serde_json::Value::Null;
            let bundle_path = self.root_path.join(&bundle.path);
            let config = transformer
                .get_bundle_manifest(*bundle_idx)
                .backend
                .get("rust")
                .unwrap_or(&null);

            let bundle_name = &transformer.get_bundle_manifest(*bundle_idx).metadata.name;

            let output_path = self.output_path.join(bundle_name);

            std::fs::create_dir_all(&output_path).unwrap();

            let job = sidex_gen::Job {
                unit: &unit,
                bundle: *bundle_idx,
                output: &output_path,
                config,
            };

            let generator = RustGenerator::new();
            sidex_gen::Generator::generate(&generator, job).unwrap();

            if self.format_generated_code {
                utils::run_rustfmt(output_path.join("mod.rs"))?;
            }

            if self.emit_rerun_if_changed {
                let schemas_dir = bundle_path.join("schemas");
                for entry in fs::read_dir(&schemas_dir).unwrap() {
                    let entry = entry.unwrap();

                    utils::emit_rerun_if_changed(entry.path());
                }
                utils::emit_rerun_if_changed(&schemas_dir);
                utils::emit_rerun_if_changed(bundle_path.join("sidex.toml"));
            }
        }

        if self.emit_rerun_if_changed {
            utils::emit_rerun_if_changed("build.rs");
        }

        Ok(())
    }
}

/// A bundle for which to generate code.
pub struct Bundle {
    path: PathBuf,
}

impl Bundle {
    /// Creates a new [`Bundle`] with the specified path.
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// Helper macro for implementing the [`From`] trait for [`Bundle`].
macro_rules! impl_from_for_bundle {
    ($from:ty) => {
        impl From<$from> for Bundle {
            fn from(value: $from) -> Self {
                Bundle::new(value.into())
            }
        }
    };
}

impl_from_for_bundle!(&str);
impl_from_for_bundle!(String);
impl_from_for_bundle!(&Path);
impl_from_for_bundle!(PathBuf);
impl_from_for_bundle!(&OsStr);
impl_from_for_bundle!(OsString);
