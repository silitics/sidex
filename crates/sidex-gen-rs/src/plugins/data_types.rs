use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_attrs::TryFromAttrs;
use sidex_attrs_rust::{FieldAttrs, TypeAttrs};
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
        let mut derive_traits = ctx
            .bundle_ctx
            .cfg
            .derive
            .iter()
            .map(|path| TokenStream::from_str(path).unwrap())
            .collect::<Vec<_>>();

        let type_attrs = TypeAttrs::try_from(def.attrs.as_slice())?;
        for derive_trait in type_attrs.derive.positive {
            derive_traits.push(derive_trait);
        }

        let derive = if derive_traits.is_empty() {
            quote! {}
        } else {
            quote! (
                #[derive( #(#derive_traits, )* )]
            )
        };
        match &def.kind {
            DefKind::TypeAlias(alias) => {
                let aliased = ctx.resolve_type(def, &alias.aliased);
                Ok(quote! {
                    #[doc = #docs]
                    #vis type #name #vars = #aliased;
                })
            }
            DefKind::OpaqueType(_) => {
                match &type_attrs.typ {
                    Some(typ) => {
                        let typ = TokenStream::from_str(&typ.path).unwrap();
                        Ok(quote! {
                            #[doc = #docs]
                            #vis type #name #vars = #typ;
                        })
                    }
                    None => todo!(),
                }
            }
            DefKind::RecordType(typ) => {
                let fields = typ
                    .fields
                    .iter()
                    .map(|field| {
                        let name = format_ident!("{}", &field.name);
                        let docs = &field.docs;
                        let mut typ = ctx.resolve_type(def, &field.typ);
                        let attrs = FieldAttrs::try_from_attrs(&field.attrs).map_err(|_| ())?;
                        for wrapper in attrs.wrappers {
                            let wrapper = TokenStream::from_str(&wrapper.wrapper).unwrap();
                            typ = quote! { #wrapper < #typ > }
                        }
                        if field.is_optional {
                            typ = quote! { ::std::option::Option< #typ > };
                        }
                        let vis = &attrs.visibility;
                        Ok(quote! {
                            #[doc = #docs]
                            #vis #name: #typ,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(quote! {
                    #[doc = #docs]
                    #derive
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
                    #derive
                    #vis enum #name #vars {
                        #(#variants)*
                    }
                })
            }
            DefKind::WrapperType(typ) => {
                let wrapped = ctx.resolve_type(def, &typ.wrapped);
                Ok(quote! {
                    #[doc = #docs]
                    #derive
                    #vis struct #name #vars (pub(crate) #wrapped);

                    impl ::std::convert::From<#name> for #wrapped {
                        fn from(wrapped: #name) -> Self {
                            wrapped.0
                        }
                    }
                })
            }
            DefKind::Service(_) => {
                // Service definitions are handled by a separate plugin.
                Ok(Default::default())
            }
        }
    }
}
