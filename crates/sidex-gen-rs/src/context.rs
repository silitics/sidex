//! Code generation context.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
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
        match typ {
            ir::Type::TypeVar(var) => {
                let var = format_ident!("{}", def[var.idx].name);
                quote! { #var }
            }
            ir::Type::Instance(instance) => {
                let bundle = &self.bundle_ctx.unit[instance.bundle];
                let schema = &bundle[instance.schema];
                let def = &schema[instance.def];

                let qualified_path =
                    format!("::{}::{}::{}", bundle.metadata.name, schema.name, def.name);

                let typ = match qualified_path.as_str() {
                    "::std::builtins::string" => quote! { String },
                    "::std::builtins::bytes" => quote! { Vec<u8> },
                    "::std::builtins::i8" => quote! { i8 },
                    "::std::builtins::i16" => quote! { i16 },
                    "::std::builtins::i32" => quote! { i32 },
                    "::std::builtins::i64" => quote! { i64 },
                    "::std::builtins::u8" => quote! { u8 },
                    "::std::builtins::u16" => quote! { u16 },
                    "::std::builtins::u32" => quote! { u32 },
                    "::std::builtins::u64" => quote! { u64 },
                    "::std::builtins::idx" => quote! { usize },
                    "::std::builtins::bool" => quote! { bool },
                    "::std::builtins::Sequence" => quote! { ::std::vec::Vec },
                    "::std::builtins::Map" => quote! { ::std::collections::HashMap },
                    _ => {
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
