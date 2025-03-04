//! Plugin generating implementations of [`serde::Serialize`] and [`serde::Deserialize`]
//! for Sidex types.

use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use sidex_gen::{diagnostics::Result, ir};

use super::Plugin;
use crate::{
    context::SchemaCtx,
    rstyir::{self, rs_type_from_def},
};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SerdePluginConfig {
    pub sidex_serde_path: String,
}

impl Default for SerdePluginConfig {
    fn default() -> Self {
        Self {
            sidex_serde_path: "::sidex::serde".to_owned(),
        }
    }
}

/// The [`Serde`] plugin.
pub struct Serde;

impl Plugin for Serde {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<TokenStream> {
        let Some(ty) = rs_type_from_def(ctx, def)? else {
            return Ok(Default::default());
        };
        Ok(rstyir::serde::rs_type_impl_serde(&ty))
    }

    fn visit_schema(&self, ctx: &SchemaCtx) -> Result<TokenStream> {
        let plugin_cfg = ctx
            .bundle_ctx
            .get_plugin_config::<SerdePluginConfig>("serde");
        let sidex_serde_path: syn::Path = syn::parse_str(&plugin_cfg.sidex_serde_path).unwrap();
        Ok(quote! {
            #[allow(unused)]
            use #sidex_serde_path as __sidex_serde;
            #[allow(unused)]
            use ::serde as __serde;
        })
    }
}
