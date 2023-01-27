use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_attrs_rust::{FieldAttrs, TypeAttrs};
use sidex_gen::{
    attrs::TryFromAttrs,
    diagnostics::Result,
    ir::{Def, DefKind},
};

use super::Plugin;
use crate::context::SchemaCtx;

pub struct Types;

impl Plugin for Types {
    fn visit_def(&self, ctx: &SchemaCtx, def: &Def) -> Result<proc_macro2::TokenStream> {
        println!("Generating type definition for {}.", def.name.as_str());
        let name = format_ident!("{}", def.name.as_str());
        let docs = def
            .docs
            .as_ref()
            .map(|docs| docs.as_str())
            .unwrap_or_default();
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
        let meta = type_attrs.attrs.iter().map(|attr| quote! { #[ #attr ] });

        match &def.kind {
            DefKind::TypeAlias(alias) => {
                let aliased = ctx.resolve_type_old(def, &alias.aliased);
                Ok(quote! {
                    #[doc = #docs]
                    #vis type #name < #vars > = #aliased;
                })
            }
            DefKind::OpaqueType(_) => {
                match &type_attrs.typ {
                    Some(typ) => {
                        let typ = TokenStream::from_str(&typ.path).unwrap();
                        Ok(quote! {
                            #[doc = #docs]
                            #vis type #name < #vars > = #typ < #vars >;
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
                        let name = format_ident!("{}", &field.name.as_str());
                        let docs = field
                            .docs
                            .as_ref()
                            .map(|docs| docs.as_str())
                            .unwrap_or_default();
                        let mut typ = ctx.resolve_type_old(def, &field.typ);
                        let attrs = FieldAttrs::try_from_attrs(&field.attrs)?;
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
                    .collect::<Result<Vec<_>>>()?;
                Ok(quote! {
                    #[doc = #docs]
                    #derive
                    #(#meta)*
                    #vis struct #name < #vars > {
                        #(#fields)*
                    }
                })
            }
            DefKind::VariantType(typ) => {
                let variants = typ
                    .variants
                    .iter()
                    .map(|variant| {
                        let name = format_ident!("{}", &variant.name.as_str());
                        let docs = variant
                            .docs
                            .as_ref()
                            .map(|docs| docs.as_str())
                            .unwrap_or_default();
                        if let Some(typ) = &variant.typ {
                            let typ = ctx.resolve_type_old(def, typ);
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
                    #(#meta)*
                    #vis enum #name < #vars > {
                        #(#variants)*
                    }
                })
            }
            DefKind::WrapperType(typ) => {
                let wrapped = ctx.resolve_type_old(def, &typ.wrapped);
                Ok(quote! {
                    #[doc = #docs]
                    #derive
                    #(#meta)*
                    #vis struct #name < #vars > (pub(crate) #wrapped);

                    impl ::std::convert::From<#name> for #wrapped {
                        fn from(wrapped: #name) -> Self {
                            wrapped.0
                        }
                    }
                })
            }
            _ => {
                // Service definitions and derived types are handled separately.
                return Ok(TokenStream::default());
            }
        }
    }
}
