#![doc = include_str!("../README.md")]
//!
//! **ℹ️ Note:** Use this crate as a starting point for customizing Sidex's Rust code
//! generator to your needs.

use std::sync::Arc;

use config::Config;
use context::{BundleCtx, SchemaCtx};
use plugins::Plugin;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::{de::IntoDeserializer, Deserialize};
use sidex_gen::{diagnostics, diagnostics::Result, ir, Generator, Job};

pub mod config;
pub mod context;
pub mod plugins;

/// Implements [`Generator`] for Rust.
#[derive(Clone)]
pub struct RustGenerator {
    plugins: Vec<Arc<dyn 'static + Plugin + Sync>>,
}

impl RustGenerator {
    pub fn new() -> Self {
        Self {
            plugins: vec![Arc::new(plugins::types::Types)],
        }
    }

    pub fn generate_bundle_inner(
        &self,
        cfg: &Config,
        unit: &ir::Unit,
        bundle: ir::BundleIdx,
    ) -> diagnostics::Result<TokenStream> {
        let bundle = &unit[bundle];
        let bundle_ctx = BundleCtx { cfg, unit, bundle };
        let bundle_preambles = self
            .plugins
            .iter()
            .map(|plugin| plugin.visit_bundle(&bundle_ctx))
            .collect::<Result<Vec<_>>>()?;
        let schemas = bundle
            .schemas
            .iter()
            .map(|schema| {
                let schema_ctx = SchemaCtx {
                    bundle_ctx: bundle_ctx.clone(),
                    schema,
                };
                let name = format_ident!("{}", &schema.name);
                let schema_preambles = self
                    .plugins
                    .iter()
                    .map(|plugin| plugin.visit_schema(&schema_ctx))
                    .collect::<Result<Vec<_>>>()?;
                let defs = schema
                    .defs
                    .iter()
                    .map(|def| {
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
                Ok(quote! {
                    pub mod #name {
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
