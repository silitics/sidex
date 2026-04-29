#![doc = include_str!("../README.md")]
//!
//! **ℹ️ Note:** Use this crate as a starting point for customizing Sidex's Rust code
//! generator to your needs.

use std::sync::Arc;

use config::Config;
use context::BundleCtx;
use context::SchemaCtx;
use plugins::Plugin;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use serde::Deserialize;
use serde::de::IntoDeserializer;
use sidex_gen::Generator;
use sidex_gen::Job;
use sidex_gen::diagnostics;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

pub mod config;
pub mod context;
pub mod plugins;
pub mod rstyir;

/// Implements [`Generator`] for Rust.
#[derive(Clone)]
pub struct RustGenerator {
    plugins: Vec<Arc<dyn 'static + Plugin + Sync>>,
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RustGenerator {
    pub fn new() -> Self {
        Self {
            plugins: vec![Arc::new(plugins::types::Types)],
        }
    }

    pub fn generate_macro(
        &self,
        config: &Config,
        unit: &ir::Ir,
        bundle: ir::BundleIdx,
    ) -> diagnostics::Result<TokenStream> {
        let mut config = config.clone();

        let mut generator = self.clone();

        let all_plugins = plugins::plugins();
        for plugin in &config.plugins {
            generator
                .plugins
                .push(all_plugins.get(plugin).unwrap().clone());
        }

        config.types.populate_table_with_builtins();

        generator.generate_bundle_inner(&config, unit, bundle)
    }

    pub fn generate_bundle_inner(
        &self,
        cfg: &Config,
        unit: &ir::Ir,
        bundle_idx: ir::BundleIdx,
    ) -> diagnostics::Result<TokenStream> {
        let bundle = &unit[bundle_idx];
        let bundle_ctx = BundleCtx {
            cfg,
            unit,
            bundle_idx,
            bundle,
        };
        let bundle_preambles = self
            .plugins
            .iter()
            .map(|plugin| plugin.visit_bundle(&bundle_ctx))
            .collect::<Result<Vec<_>>>()?;
        let mut schemas: Vec<(ir::SchemaIdx, &ir::Schema)> = unit.schemas_of(bundle_idx).collect();
        schemas.sort_by(|(_, a), (_, b)| a.name.cmp(&b.name));
        let schemas = schemas
            .iter()
            .map(|(schema_idx, schema)| {
                let schema_ctx = SchemaCtx {
                    bundle_ctx: bundle_ctx.clone(),
                    schema_idx: *schema_idx,
                    schema,
                };
                let name = format_ident!("{}", &schema.name);
                let schema_preambles = self
                    .plugins
                    .iter()
                    .map(|plugin| plugin.visit_schema(&schema_ctx))
                    .collect::<Result<Vec<_>>>()?;
                let defs = unit
                    .defs_of(*schema_idx)
                    .map(|(_, def)| {
                        let parts = self
                            .plugins
                            .iter()
                            .map(|plugin| plugin.visit_def(&schema_ctx, def))
                            .collect::<Result<Vec<_>>>()?;
                        Ok(quote! {
                            #(#parts)*
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                let docs = schema
                    .docs
                    .as_ref()
                    .map(|docs| docs.as_str())
                    .unwrap_or_default();
                Ok(quote! {
                    pub mod #name {
                        #![doc = #docs]
                        // Generated code is emitted mechanically and uses
                        // shapes (fully-qualified paths, while-let-Some
                        // loops, redundant patterns) that clippy doesn't
                        // love, and may declare types or fields a given
                        // consumer never reaches. Silence clippy + dead
                        // code at the module boundary; the rest of rustc
                        // stays on so real codegen bugs surface.
                        #![allow(
                            clippy::all,
                            clippy::pedantic,
                            clippy::nursery,
                            clippy::cargo,
                            dead_code,
                        )]

                        #(#schema_preambles)*
                        #(#defs)*
                    }
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            #(#bundle_preambles)*
            #(#schemas)*
        })
    }
}

impl Generator for RustGenerator {
    fn generate(&self, job: Job) -> diagnostics::Result<()> {
        let mod_path = job.output.join("mod.rs");

        let mut config = Option::<Config>::deserialize(job.config.clone().into_deserializer())
            .unwrap()
            .unwrap_or_default();

        let mut generator = self.clone();

        let all_plugins = plugins::plugins();
        for plugin in &config.plugins {
            generator
                .plugins
                .push(all_plugins.get(plugin).unwrap().clone());
        }

        config.types.populate_table_with_builtins();

        let mut mod_code = String::new();
        mod_code.push_str("/* GENERATED WITH SIDEX. DO NOT MODIFY! */\n\n");
        mod_code.push_str(
            &generator
                .generate_bundle_inner(&config, job.unit, job.bundle)?
                .to_string(),
        );

        std::fs::write(&mod_path, mod_code)?;

        Ok(())
    }
}
