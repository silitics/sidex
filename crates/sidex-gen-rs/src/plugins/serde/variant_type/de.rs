use proc_macro2::TokenStream;
use quote::quote;
use sidex_attrs_json::{
    atoms::JsonTaggedAttr, JsonFieldAttrs, JsonRecordTypeAttrs, JsonVariantTypeAttrs,
};
use sidex_gen::{
    attrs::TryFromAttrs,
    diagnostics::{Diagnostic, Result},
    ir,
};

use crate::{
    context::{RustTy, SchemaCtx},
    plugins::serde::{
        identifier_enum::{gen_identifier_enum, IdentifierKind},
        SerdeVariant,
    },
};

pub(crate) fn gen_deserialize_body(
    ctx: &SchemaCtx,
    def: &ir::Def,
    ty: &RustTy,
    ty_json_attrs: &JsonVariantTypeAttrs,
    variants: &[SerdeVariant],
) -> Result<TokenStream> {
    let (identifiers, identifier_enum) = gen_identifier_enum(
        variants
            .iter()
            .map(|variant| ty_json_attrs.variant_name(&variant.variant, &variant.json_attrs)),
        IdentifierKind::Variant,
    );

    let variants_array_const = gen_variants_array_const(variants);

    let non_human_readable_body = gen_externally_tagged_body(ctx, def, ty, variants, &identifiers);

    let ty_ident = &ty.ident;

    let human_readable_match_arms =
        variants
            .iter()
            .zip(identifiers.iter())
            .map(|(variant, de_ident)| {
                let ident = &variant.rust_variant.ident;
                let constructor = if let Some(ty) = &variant.rust_variant.ty {
                    let content_field = ty_json_attrs.content_field_name(&variant.json_attrs);
                    match &ty_json_attrs.tagged {
                        JsonTaggedAttr::Adjacently => {
                            quote! {
                                #ty_ident::#ident(__tagged.deserialize_adjacently_tagged::<#ty, __D::Error>(#content_field)?)
                            }
                        }
                        JsonTaggedAttr::Internally => {
                            let is_record = variant.variant.typ.as_ref().map(|typ| ctx.bundle_ctx.unit.resolve_aliases(&typ)).map(|typ| ctx.bundle_ctx.unit.record_type(&typ).is_some()).unwrap_or(false);
                            if is_record && variant.json_attrs.content.is_none() {
                                quote! {
                                    #ty_ident::#ident(__tagged.deserialize_internally_tagged::<#ty, __D::Error>()?)
                                }
                            } else {
                                quote! {
                                    #ty_ident::#ident(__tagged.deserialize_adjacently_tagged::<#ty, __D::Error>(#content_field)?)
                                }
                            }
                        }
                        _ => TokenStream::default(),
                    }
                } else {
                    quote! {
                        // Ignore content without any warning.
                        #ty_ident::#ident
                    }
                };

                quote! {
                    __Identifier::#de_ident => {
                        ::core::result::Result::Ok(#constructor)
                    }
                }
            });

    let body = if !matches!(ty_json_attrs.tagged, JsonTaggedAttr::Externally) {
        let tag_field = ty_json_attrs.tag_field_name();
        quote! {
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<__Identifier, __D>(__deserializer, #tag_field)?;
                match __tagged.tag {
                    #(#human_readable_match_arms,)*
                }
            } else {
                #non_human_readable_body
            }
        }
    } else {
        non_human_readable_body
    };

    Ok(quote! {
        #identifier_enum
        #variants_array_const
        #body
    })
}

fn gen_externally_tagged_body(
    ctx: &SchemaCtx,
    def: &ir::Def,
    ty: &RustTy,
    variants: &[SerdeVariant],
    identifiers: &[syn::Ident],
) -> TokenStream {
    let ty_ident = &ty.ident;
    let ty_name = &ty.name;

    let match_arms = variants
        .iter()
        .zip(identifiers.iter())
        .map(|(variant, de_ident)| {
            let ident = &variant.rust_variant.ident;
            let constructor = if let Some(ty) = &variant.rust_variant.ty {
                quote! {
                    let __value = __serde::de::VariantAccess::newtype_variant::<#ty>(__variant)?;
                    ::core::result::Result::Ok(#ty_ident::#ident(__value))
                }
            } else {
                quote! {
                    __serde::de::VariantAccess::unit_variant(__variant)?;
                    ::core::result::Result::Ok(#ty_ident::#ident)
                }
            };

            quote! {
                (__Identifier::#de_ident, __variant) => {
                    #constructor
                }
            }
        });

    let match_arms_str =
        variants
            .iter()
            .zip(identifiers.iter())
            .filter_map(|(variant, de_ident)| {
                let ident = &variant.rust_variant.ident;
                if variant.rust_variant.ty.is_some() {
                    return None;
                }
                Some(quote! {
                    __Identifier::#de_ident => {
                        ::core::result::Result::Ok(#ty_ident::#ident)
                    }
                })
            });

    let vars = ctx.generic_type_vars(def);

    let vars_with_bounds =
        ctx.generic_type_vars_with_bounds(def, quote! { __serde::Deserialize<'de> });

    let enum_type_name = format!("enum {}", ty_name);

    quote! {
        #[doc(hidden)]
        struct __Visitor < #vars > {
            __phantom_vars: ::core::marker::PhantomData<fn(&( #vars ))>,
        }

        impl <'de, #vars_with_bounds > __serde::de::Visitor<'de> for __Visitor < #vars > {
            type Value = #ty_ident < #vars >;

            fn expecting(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    __formatter,
                    #enum_type_name,
                )
            }

            #[inline]
            fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
            where
                __E: __serde::de::Error,
            {
                let __identifier = __IdentifierVisitor.visit_str(__value)?;

                #[allow(unreachable_patterns)]
                match __identifier {
                    #(#match_arms_str),*
                    _ => Err(__E::invalid_value(__serde::de::Unexpected::Str(__value), &self))
                }
            }

            #[inline]
            fn visit_enum<__A>(self, __data: __A) -> ::core::result::Result<Self::Value, __A::Error>
            where
                __A: __serde::de::EnumAccess<'de>,
            {
                match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                    #(#match_arms,)*
                }
            }
        }

        __serde::Deserializer::deserialize_enum(
            __deserializer,
            #ty_name,
            __VARIANTS,
            __Visitor { __phantom_vars: ::core::marker::PhantomData }
        )
    }
}

/// Generates the `__VARIANTS` array constant with the variant names.
fn gen_variants_array_const(variants: &[SerdeVariant]) -> TokenStream {
    let names = variants.iter().map(|variant| &variant.name);
    quote! {
        #[doc(hidden)]
        const __VARIANTS: &'static [&'static str] = &[#(#names,)*];
    }
}
