use std::{collections::HashMap, sync::Arc};

use sidex_codegen::Code;
use sidex_gen::{diagnostics::Result, ir};

use crate::context::{BundleCtx, SchemaCtx};

pub mod types;

pub trait Plugin {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<Code>;

    #[allow(unused_variables)]
    fn visit_bundle(&self, ctx: &BundleCtx) -> Result<Code> {
        Ok(Code::new())
    }

    #[allow(unused_variables)]
    fn visit_schema(&self, ctx: &SchemaCtx) -> Result<Code> {
        Ok(Code::new())
    }
}

pub fn plugins() -> HashMap<String, Arc<dyn 'static + Plugin + Sync>> {
    let mut plugins: HashMap<String, Arc<dyn 'static + Plugin + Sync>> = HashMap::new();
    plugins.insert("types".to_owned(), Arc::new(types::Types));
    plugins
}
