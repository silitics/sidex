use std::{collections::HashMap, sync::Arc};

use proc_macro2::TokenStream;
use sidex_gen::{diagnostics::Result, ir};

use crate::context::{BundleCtx, SchemaCtx};

pub mod builder;
pub mod interfaces;
pub mod serde;
pub mod types;

pub trait Plugin {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<TokenStream>;

    #[allow(unused_variables)]
    fn visit_bundle(&self, ctx: &BundleCtx) -> Result<TokenStream> {
        Ok(Default::default())
    }

    #[allow(unused_variables)]
    fn visit_schema(&self, ctx: &SchemaCtx) -> Result<TokenStream> {
        Ok(Default::default())
    }
}

pub fn plugins() -> HashMap<String, Arc<dyn 'static + Plugin + Sync>> {
    let mut plugins: HashMap<String, Arc<dyn 'static + Plugin + Sync>> = HashMap::new();
    plugins.insert("types".to_owned(), Arc::new(types::Types));
    plugins.insert("builder".to_owned(), Arc::new(builder::Builder));
    plugins.insert("serde".to_owned(), Arc::new(serde::Serde));
    plugins.insert("interfaces".to_owned(), Arc::new(interfaces::Interfaces));
    plugins
}
