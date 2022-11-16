#![doc = include_str!("../README.md")]

use config::Config;
use context::{BundleCtx, SchemaCtx};
use plugins::Plugin;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::{de::IntoDeserializer, Deserialize};
use sidex_gen::{ir, Generator, Job};

pub mod config;
pub mod context;
pub mod plugins;

/// Implements [`Generator`] for Rust.
pub struct RustGenerator {
    plugins: Vec<Box<dyn Plugin>>,
}

impl RustGenerator {
    pub fn new() -> Self {
        Self {
            plugins: vec![Box::new(plugins::data_types::Types)],
        }
    }

    pub fn generate_bundle_inner(
        &self,
        cfg: &Config,
        unit: &ir::Collection,
        bundle: ir::BundleIdx,
    ) -> Result<TokenStream, ()> {
        let bundle = &unit[bundle];
        let bundle_ctx = BundleCtx { cfg, unit, bundle };
        let bundle_preambles = self
            .plugins
            .iter()
            .map(|plugin| plugin.visit_bundle(&bundle_ctx))
            .collect::<Result<Vec<_>, _>>()?;
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
                    .collect::<Result<Vec<_>, _>>()?;
                let defs = schema
                    .defs
                    .iter()
                    .map(|def| {
                        let parts = self
                            .plugins
                            .iter()
                            .map(|plugin| plugin.visit_def(&schema_ctx, def))
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(quote! {
                            #(#parts)*
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(quote! {
                    pub mod #name {
                        #(#schema_preambles)*
                        #(#defs)*
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(quote! {
            #(#bundle_preambles)*
            #(#schemas)*
        })
    }
}

impl Generator for RustGenerator {
    fn generate(&self, job: Job) -> Result<(), Box<dyn std::error::Error>> {
        let mod_path = job.output.join("mod.rs");

        let config = Option::<Config>::deserialize(job.config.clone().into_deserializer())
            .unwrap()
            .unwrap_or_default();

        let mut mod_code = String::new();
        mod_code.push_str("/* GENERATED WITH SIDEX. DO NOT MODIFY! */\n\n");
        mod_code.push_str(
            &self
                .generate_bundle_inner(&config, job.unit, job.bundle)
                .map_err(|err| format!("{err:?}"))?
                .to_string(),
        );

        std::fs::write(&mod_path, mod_code)?;

        Ok(())
    }
}
