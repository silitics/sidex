use proc_macro2::TokenStream;
use sidex_gen::ir;

use crate::context::{BundleCtx, SchemaCtx};

pub mod data_types;

pub trait Plugin {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<TokenStream, ()>;

    #[allow(unused_variables)]
    fn visit_bundle(&self, ctx: &BundleCtx) -> Result<TokenStream, ()> {
        Ok(Default::default())
    }

    #[allow(unused_variables)]
    fn visit_schema(&self, ctx: &SchemaCtx) -> Result<TokenStream, ()> {
        Ok(Default::default())
    }
}
