use proc_macro2::TokenStream;
use quote::quote;
use sidex_attrs_json::{
    atoms::JsonTaggedAttr, JsonFieldAttrs, JsonRecordTypeAttrs, JsonVariantTypeAttrs,
};
use sidex_gen::{
    attrs::TryFromAttrs,
    diagnostics::{Diagnostic, Result},
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

    let non_human_readable_body = gen_externally_tagged_body(ty, variants, &identifiers);

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
                            quote! {
                                #ty_ident::#ident(__tagged.deserialize_internally_tagged::<#ty, __D::Error>()?)
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
            if ::serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = ::sidex_serde::de::tagged::deserialize_tagged_variant::<__Identifier, __D>(__deserializer, #tag_field)?;
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
                    let __value = ::serde::de::VariantAccess::newtype_variant::<#ty>(__variant)?;
                    ::core::result::Result::Ok(#ty_ident::#ident(__value))
                }
            } else {
                quote! {
                    ::serde::de::VariantAccess::unit_variant(__variant)?;
                    ::core::result::Result::Ok(#ty_ident::#ident)
                }
            };

            quote! {
                (__Identifier::#de_ident, __variant) => {
                    #constructor
                }
            }
        });

    quote! {
        struct __Visitor;

        impl<'de> ::serde::de::Visitor<'de> for __Visitor {
            type Value = #ty_ident;

            fn expecting(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    __formatter,
                    "enum ExternallyTagged",
                )
            }

            #[inline]
            fn visit_enum<__A>(self, __data: __A) -> ::core::result::Result<Self::Value, __A::Error>
            where
                __A: ::serde::de::EnumAccess<'de>,
            {
                match ::serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                    #(#match_arms,)*
                }
            }
        }

        ::serde::Deserializer::deserialize_enum(
            __deserializer,
            #ty_name,
            __VARIANTS,
            __Visitor
        )
    }
}

/// Generates the `__VARIANTS` array constant with the variant names.
fn gen_variants_array_const(variants: &[SerdeVariant]) -> TokenStream {
    let names = variants.iter().map(|variant| &variant.name);
    quote! {
        const __VARIANTS: &'static [&'static str] = &[#(#names,)*];
    }
}
