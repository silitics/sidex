//! Code generation context.

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use sidex_gen::ir;

use crate::config::Config;

#[derive(Clone)]
pub struct BundleCtx<'cx> {
    pub cfg: &'cx Config,
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

    pub fn resolve_type(&self, def: &ir::Def, typ: &ir::Type) -> TokenStream {
        match &typ.kind {
            ir::TypeKind::TypeVar(var) => {
                let var = format_ident!("{}", def[var.idx].name);
                quote! { #var }
            }
            ir::TypeKind::Instance(instance) => {
                let bundle = &self.bundle_ctx.unit[instance.bundle];
                let schema = &bundle[instance.schema];
                let def = &schema[instance.def];

                let qualified_path =
                    format!("::{}::{}::{}", bundle.metadata.name, schema.name, def.name);

                let typ = if let Some(path) = self.bundle_ctx.cfg.types.table.get(&qualified_path) {
                    let rust_path = syn::parse_str::<syn::TypePath>(path).unwrap();
                    rust_path.to_token_stream()
                } else {
                    if instance.bundle == self.bundle_ctx.bundle.idx {
                        let def_name = format_ident!("{}", &def.name);
                        if instance.schema == self.schema.idx {
                            quote! { #def_name }
                        } else {
                            let schema_name = format_ident!("{}", &schema.name);
                            quote! { super::#schema_name::#def_name }
                        }
                    } else {
                        eprintln!("{}", qualified_path);
                        todo!()
                    }
                };

                let subst = instance
                    .subst
                    .iter()
                    .map(|typ| self.resolve_type(def, typ))
                    .collect::<Vec<_>>();

                quote! { #typ < #(#subst , )* > }
            }
        }
    }
}
