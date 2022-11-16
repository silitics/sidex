use quote::{format_ident, quote};
use sidex_gen::ir::{Def, DefKind};

use super::Plugin;
use crate::context::SchemaCtx;

pub struct Types;

impl Plugin for Types {
    fn visit_def(&self, ctx: &SchemaCtx, def: &Def) -> Result<proc_macro2::TokenStream, ()> {
        let name = format_ident!("{}", def.name);
        let docs = &def.docs;
        let vars = ctx.generic_type_vars(def);
        let vis = quote! { pub };
        match &def.kind {
            DefKind::TypeAlias(alias) => {
                let aliased = ctx.resolve_type(def, &alias.aliased);
                Ok(quote! {
                    #[doc = #docs]
                    #vis type #name #vars = #aliased;
                })
            }
            DefKind::OpaqueType(_) => Ok(quote! { type #name #vars = (); }),
            DefKind::RecordType(typ) => {
                let fields = typ
                    .fields
                    .iter()
                    .map(|field| {
                        let name = format_ident!("{}", &field.name);
                        let docs = &field.docs;
                        let mut typ = ctx.resolve_type(def, &field.typ);
                        if field.is_optional {
                            typ = quote! { ::std::option::Option<#typ> };
                        }
                        quote! {
                            #[doc = #docs]
                            #vis #name: #typ,
                        }
                    })
                    .collect::<Vec<_>>();
                Ok(quote! {
                    #[doc = #docs]
                    #vis struct #name #vars {
                        #(#fields)*
                    }
                })
            }
            DefKind::VariantType(typ) => {
                let variants = typ
                    .variants
                    .iter()
                    .map(|variant| {
                        let name = format_ident!("{}", &variant.name);
                        let docs = &variant.docs;
                        if let Some(typ) = &variant.typ {
                            let typ = ctx.resolve_type(def, typ);
                            quote! {
                                #[doc = #docs]
                                #name(#typ),
                            }
                        } else {
                            quote! {
                                #[doc = #docs]
                                #name,
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                Ok(quote! {
                    #[doc = #docs]
                    #vis enum #name #vars {
                        #(#variants)*
                    }
                })
            }
            DefKind::WrapperType(typ) => {
                let wrapped = ctx.resolve_type(def, &typ.wrapped);
                Ok(quote! {
                    #[doc = #docs]
                    #vis struct #name #vars (#wrapped);
                })
            }
            DefKind::Service(_) => {
                // Service definitions are handled by a separate plugin.
                Ok(Default::default())
            }
        }
    }
}
