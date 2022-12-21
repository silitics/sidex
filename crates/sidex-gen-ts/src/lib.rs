use std::sync::Arc;

use config::Config;
use context::{BundleCtx, SchemaCtx};
use plugins::Plugin;
use quote::quote;
use sidex_gen::{diagnostics, Generator};

pub mod config;
pub mod context;
pub mod plugins;

/// Implements [`Generator`] for Rust.
#[derive(Clone)]
pub struct TsGenerator {
    plugins: Vec<Arc<dyn 'static + Plugin + Sync>>,
}

impl TsGenerator {
    pub fn new() -> Self {
        Self {
            plugins: vec![Arc::new(plugins::types::Types)],
        }
    }
}

impl Generator for TsGenerator {
    fn generate(&self, job: sidex_gen::Job) -> sidex_gen::diagnostics::Result<()> {
        let cfg = serde_json::from_value::<Option<Config>>(job.config.clone())
            .unwrap()
            .unwrap_or_default();

        let bundle_ctx = BundleCtx {
            cfg: &cfg,
            unit: job.unit,
            bundle: &job.unit[job.bundle],
        };

        let index_preamble = self
            .plugins
            .iter()
            .map(|plugin| plugin.visit_bundle(&bundle_ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let index = quote! { #(#index_preamble)* }.to_string();

        std::fs::write(&job.output.join("index.ts"), index)?;

        for schema in &bundle_ctx.bundle.schemas {
            let schema_ctx = SchemaCtx {
                bundle_ctx: bundle_ctx.clone(),
                schema,
            };
            let preambles = self
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
                        .collect::<diagnostics::Result<Vec<_>>>()?;
                    Ok(quote! {
                        #(#parts)*
                    })
                })
                .collect::<diagnostics::Result<Vec<_>>>()?;

            let source = quote! {
                #(#preambles)*
                #(#defs)*
            }
            .to_string();

            std::fs::write(&job.output.join(format!("{}.ts", schema.name)), source)?;
        }

        Ok(())
    }
}
