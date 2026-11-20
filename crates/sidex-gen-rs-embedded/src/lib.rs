//! Sidex code generator for Rust on embedded targets without an allocator.
//!
//! The `rust-embedded` backend generates Rust types for `no_std` targets
//! without an allocator, such as microcontrollers. The generated types use the
//! `sidex-embedded` runtime crate and implement its `Decode` and `Encode`
//! traits for Sidex's JSON representation. Types containing strings,
//! sequences, maps, bytes, or JSON values borrow them from the input and take
//! the lifetime `'a`.
//!
//! The backend is configured in `sidex.toml` (see [`Config`]):
//!
//! ```toml
//! [backend.rust-embedded]
//! # Only generate the definitions of these groups.
//! groups = ["device"]
//! # Derive these traits in addition to `Clone` and `Copy`.
//! derive = ["Debug", "PartialEq"]
//! ```
//!
//! [`EmbeddedRustGenerator::generate_bundle`] returns the code of a bundle,
//! which [`sidex_gen::Generator::generate`] writes to `mod.rs`.

use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use sidex_gen::Generator;
use sidex_gen::Job;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

pub use crate::config::Config;
pub use crate::config::TypesConfig;
use crate::context::Ctx;
use crate::context::rust_ident;

mod config;
mod context;
mod defs;

/// Implements [`Generator`] for Rust on embedded targets.
#[derive(Debug, Clone, Default)]
pub struct EmbeddedRustGenerator;

impl EmbeddedRustGenerator {
    /// Returns the code of a bundle.
    pub fn generate_bundle(
        &self,
        config: &Config,
        unit: &ir::Ir,
        bundle: ir::BundleIdx,
    ) -> Result<TokenStream> {
        let ctx = Ctx::new(config, unit, bundle)?;
        let groups = config.groups.iter().collect::<BTreeSet<_>>();
        for group in &groups {
            if !unit[bundle].groups.contains_key(*group) {
                return Err(Box::new(
                    Diagnostic::error(format!(
                        "The `rust-embedded` backend selects group `{group}`, which bundle `{}` \
                         does not declare.",
                        unit[bundle].metadata.name
                    ))
                    .with_help(format!(
                        "Declare the group with a `[groups.{group}]` table in `sidex.toml`."
                    )),
                ));
            }
        }
        let mut schemas = unit.schemas_of(bundle).collect::<Vec<_>>();
        schemas.sort_by(|(_, a), (_, b)| a.name.cmp(&b.name));
        let mut modules = Vec::new();
        for (schema_idx, schema) in schemas {
            let defs = unit
                .defs_of(schema_idx)
                .filter(|(_, def)| {
                    groups.is_empty() || def.groups.iter().any(|group| groups.contains(group))
                })
                .map(|(def_idx, _)| {
                    defs::generate(
                        &ctx,
                        ir::DefRef {
                            bundle,
                            schema: schema_idx,
                            def: def_idx,
                        },
                    )
                })
                .collect::<Result<Vec<_>>>()?;
            if defs.is_empty() {
                continue;
            }
            let ident = rust_ident(&schema.name)?;
            let docs = schema
                .docs
                .as_ref()
                .map(|docs| docs.as_str())
                .unwrap_or_default();
            modules.push(quote! {
                pub mod #ident {
                    #![doc = #docs]
                    // Generated code may not use every definition, and it
                    // implements the traits of deprecated definitions.
                    #![allow(
                        clippy::all,
                        clippy::pedantic,
                        clippy::nursery,
                        dead_code,
                        deprecated,
                    )]

                    #(#defs)*
                }
            });
        }
        Ok(quote! { #(#modules)* })
    }
}

impl Generator for EmbeddedRustGenerator {
    fn generate(&self, job: Job) -> Result<()> {
        let config = Option::<Config>::deserialize(job.config)
            .map_err(|error| {
                Diagnostic::error(format!(
                    "Invalid configuration of the `rust-embedded` backend: {error}."
                ))
            })?
            .unwrap_or_default();
        let code = self.generate_bundle(&config, job.unit, job.bundle)?;
        std::fs::write(
            job.output.join("mod.rs"),
            format!("/* GENERATED WITH SIDEX. DO NOT MODIFY! */\n\n{code}"),
        )?;
        Ok(())
    }
}
