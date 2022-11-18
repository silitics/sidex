use std::{collections::HashMap, sync::Arc};

use proc_macro2::TokenStream;
use sidex_gen::ir;

use crate::context::{BundleCtx, SchemaCtx};

pub mod builder;
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

pub fn plugins() -> HashMap<String, Arc<dyn 'static + Plugin + Sync>> {
    let mut plugins: HashMap<String, Arc<dyn 'static + Plugin + Sync>> = HashMap::new();
    plugins.insert("data-types".to_owned(), Arc::new(data_types::Types));
    plugins.insert("builder".to_owned(), Arc::new(builder::Builder));
    plugins
}
