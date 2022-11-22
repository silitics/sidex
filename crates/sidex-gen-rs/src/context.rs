//! Code generation context.

use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use sidex_attrs::TryFromAttrs;
use sidex_attrs_rust::{FieldAttrs, Visibility};
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

pub struct TypeInfo {
    pub ident: syn::Ident,
    pub vis: syn::Visibility,
    pub generics: TokenStream,
}

pub struct FieldInfo {
    pub name: syn::Ident,
    pub typ: syn::Type,
    pub vis: Visibility,
}

impl<'cx> SchemaCtx<'cx> {
    pub fn field_info(&self, def: &ir::Def, field: &ir::Field) -> FieldInfo {
        let name = format_ident!("{}", &field.name.as_str());
        let mut typ = self.resolve_type(def, &field.typ);
        let attrs = FieldAttrs::try_from_attrs(&field.attrs)
            .map_err(|_| ())
            .unwrap();
        for wrapper in attrs.wrappers {
            let wrapper = TokenStream::from_str(&wrapper.wrapper).unwrap();
            typ = quote! { #wrapper < #typ > }
        }
        if field.is_optional {
            typ = quote! { ::std::option::Option< #typ > };
        }
        let vis = attrs.visibility.clone();
        FieldInfo {
            name,
            typ: syn::parse2::<syn::Type>(typ).unwrap(),
            vis,
        }
    }

    pub fn type_info(&self, def: &ir::Def) -> TypeInfo {
        let ident = format_ident!("{}", def.name.as_str());
        let generics = self.generic_type_vars(def);
        let vis = syn::parse_str::<syn::Visibility>("pub").unwrap();
        TypeInfo {
            ident,
            vis,
            generics,
        }
    }

    pub fn resolve_type_var(&self, def: &ir::Def, var: &ir::TypeVarType) -> syn::Ident {
        format_ident!("{}", def[var.idx].name.as_str())
    }

    pub fn generic_type_vars(&self, def: &ir::Def) -> TokenStream {
        if def.vars.is_empty() {
            quote! {}
        } else {
            let vars = def
                .vars
                .iter()
                .map(|var| format_ident!("{}", var.name.as_str()));

            quote! { < #(#vars , )* > }
        }
    }

    pub fn fully_qualified_type_name(&self, instance: &ir::InstanceType) -> String {
        let bundle = &self.bundle_ctx.unit[instance.bundle];
        let schema = &bundle[instance.schema];
        let def = &schema[instance.def];

        format!(
            "::{}::{}::{}",
            bundle.metadata.name,
            schema.name,
            def.name.as_str()
        )
    }

    pub fn resolve_type(&self, def: &ir::Def, typ: &ir::Type) -> TokenStream {
        match &typ.kind {
            ir::TypeKind::TypeVar(var) => {
                let var = format_ident!("{}", def[var.idx].name.as_str());
                quote! { #var }
            }
            ir::TypeKind::Instance(instance) => {
                let bundle = &self.bundle_ctx.unit[instance.bundle];
                let schema = &bundle[instance.schema];
                let def = &schema[instance.def];

                let qualified_path = format!(
                    "::{}::{}::{}",
                    bundle.metadata.name,
                    schema.name,
                    def.name.as_str()
                );

                let typ = if let Some(path) = self.bundle_ctx.cfg.types.table.get(&qualified_path) {
                    let rust_path = syn::parse_str::<syn::TypePath>(path).unwrap();
                    rust_path.to_token_stream()
                } else {
                    if instance.bundle == self.bundle_ctx.bundle.idx {
                        let def_name = format_ident!("{}", &def.name.as_str());
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
