//! Code generation context.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_gen::ir;

#[derive(Clone)]
pub struct BundleCtx<'cx> {
    pub unit: &'cx ir::Unit,
    pub bundle: &'cx ir::Bundle,
}

#[derive(Clone)]
pub struct SchemaCtx<'cx> {
    pub bundle_ctx: BundleCtx<'cx>,
    pub schema: &'cx ir::Schema,
}

impl<'cx> SchemaCtx<'cx> {
    pub fn generic_type_vars(&self, def: &ir::Def) -> TokenStream {
        let vars = def.vars.iter().map(|var| format_ident!("{}", var.name));

        quote! { < #(#vars , )* > }
    }

    pub fn resolve_type(&self, _typ: &ir::Type) {
        todo!()
    }
}
