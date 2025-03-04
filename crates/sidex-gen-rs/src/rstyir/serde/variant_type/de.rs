use proc_macro2::TokenStream;
use quote::quote;
use sidex_attrs_json::atoms::JsonTaggedAttr;

use crate::rstyir::{
    RsType, RsTypeVariant, RsVariant,
    serde::identifier_enum::{IdentifierKind, gen_identifier_enum},
};

pub(crate) fn gen_deserialize_body(ty: &RsType, variant_ty: &RsTypeVariant) -> TokenStream {
    let (identifiers, identifier_enum) = gen_identifier_enum(
        variant_ty.variants.iter().map(|variant| {
            variant_ty
                .json_attrs
                .variant_name_from_str(&variant.name, &variant.json_attrs)
        }),
        IdentifierKind::Variant,
    );

    let variants_array_const = gen_variants_array_const(&variant_ty.variants);

    let non_human_readable_body = gen_externally_tagged_body(ty, variant_ty, &identifiers);

    let ty_ident = &ty.ident;

    let body = if !matches!(variant_ty.json_attrs.tagged, JsonTaggedAttr::Externally) {
        let human_readable_body = if matches!(
            variant_ty.json_attrs.tagged,
            JsonTaggedAttr::Implicitly
        ) {
            let try_variants = variant_ty.variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                if let Some(ty) = &variant.ty {
                    quote! {
                        match __sidex_serde::de::content::deserialize_content_ref::<#ty, __D::Error>(&__content) {
                            Ok(__value) => return Ok(#ty_ident::#ident(__value)),
                            Err(_) => { /* ignore */ },
                        };
                    }
                } else {
                    todo!()
                }
            });
            quote! {
                let __content = __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                #(#try_variants)*
                Err(<__D::Error as __serde::de::Error>::custom("no matching variant found"))
            }
        } else {
            let human_readable_match_arms = variant_ty.variants
            .iter()
            .zip(identifiers.iter())
            .map(|(variant, de_ident)| {
                let ident = &variant.ident;
                let constructor = if let Some(ty) = &variant.ty {
                    let content_field = variant_ty.json_attrs.content_field_name(&variant.json_attrs);
                    match &variant_ty.json_attrs.tagged {
                        JsonTaggedAttr::Adjacently => {
                            quote! {
                                #ty_ident::#ident(__tagged.deserialize_adjacently_tagged::<#ty, __D::Error>(#content_field)?)
                            }
                        }
                        JsonTaggedAttr::Internally => {
                            let is_record = variant.is_record;
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
            let tag_field = variant_ty.json_attrs.tag_field_name();
            quote! {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<__Identifier, __D>(__deserializer, #tag_field)?;
                match __tagged.tag {
                    #(#human_readable_match_arms,)*
                }
            }
        };
        quote! {
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                #human_readable_body
            } else {
                #non_human_readable_body
            }
        }
    } else {
        non_human_readable_body
    };

    quote! {
        #identifier_enum
        #variants_array_const
        #body
    }
}

fn gen_externally_tagged_body(
    ty: &RsType,
    variant_ty: &RsTypeVariant,
    identifiers: &[syn::Ident],
) -> TokenStream {
    let ty_ident = &ty.ident;
    let ty_name = &ty.name;

    let match_arms = variant_ty.variants.iter().zip(identifiers.iter()).map(
        |(variant, de_ident)| {
            let ident = &variant.ident;
            let constructor = if let Some(ty) = &variant.ty {
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
        },
    );

    let match_arms_str = variant_ty
        .variants
        .iter()
        .zip(identifiers.iter())
        .filter_map(|(variant, de_ident)| {
            let ident = &variant.ident;
            if variant.ty.is_some() {
                return None;
            }
            Some(quote! {
                __Identifier::#de_ident => {
                    ::core::result::Result::Ok(#ty_ident::#ident)
                }
            })
        });

    let type_generics = ty.type_generics();
    let impl_generics = ty.type_generics_with_bounds(&quote! { __serde::Deserialize<'de> });

    let enum_type_name = format!("enum {}", ty_name);

    quote! {
        #[doc(hidden)]
        struct __Visitor <#type_generics> {
            __phantom_vars: ::core::marker::PhantomData<fn(&( #type_generics ))>,
        }

        impl <'de, #impl_generics> __serde::de::Visitor<'de> for __Visitor <#type_generics> {
            type Value = #ty_ident <#type_generics>;

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
fn gen_variants_array_const(variants: &[RsVariant]) -> TokenStream {
    let names = variants.iter().map(|variant| &variant.json_name);
    quote! {
        #[doc(hidden)]
        const __VARIANTS: &'static [&'static str] = &[#(#names,)*];
    }
}
