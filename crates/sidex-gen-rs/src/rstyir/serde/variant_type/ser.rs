use proc_macro2::TokenStream;
use quote::quote;
use sidex_attrs_json::JsonTaggedAttr;

use crate::rstyir::RsType;
use crate::rstyir::RsTypeVariant;

pub(crate) fn gen_serialize_body(ty: &RsType, variant_ty: &RsTypeVariant) -> TokenStream {
    let ty_name = &ty.name;

    let tag_field = variant_ty.json_attrs.tag_field_name();

    let match_arms = variant_ty
        .variants
        .iter()
        .enumerate()
        .map(|(variant_idx, variant)| {
            use JsonTaggedAttr::*;

            let ident = &variant.ident;
            let variant_tag = &variant.json_name;
            let variant_idx = variant_idx as u32;

            let value_field = variant_ty
                .json_attrs
                .content_field_name(&variant.json_attrs);

            // For variants carrying data, wrap the value with the encoding
            // so the underlying serializer drives the override-aware wire form.
            let wrapped_value =
                variant
                    .ty
                    .as_ref()
                    .zip(variant.encoding.as_ref())
                    .map(|(ty, encoding)| {
                        quote! {
                            &__sidex_serde::SerializeAsWrap::<#ty, #encoding>::new(__value)
                        }
                    });

            let adjacently_tagged = if let Some(wrapped) = wrapped_value.as_ref() {
                quote! {
                    Self::#ident(__value) => {
                        __serializer.serialize_adjacently_tagged(
                            #tag_field, #value_field, #variant_tag, #variant_idx, #wrapped
                        )
                    }
                }
            } else {
                quote! {
                    Self::#ident => {
                        __serializer.serialize_adjacent_tag(
                            #tag_field, #variant_tag, #variant_idx
                        )
                    }
                }
            };

            match variant_ty.json_attrs.tagged {
                Adjacently => adjacently_tagged,
                Externally => {
                    if let Some(wrapped) = wrapped_value.as_ref() {
                        quote! {
                            Self::#ident(__value) => {
                                __serializer.serialize_externally_tagged(
                                    #variant_tag, #variant_idx, #wrapped
                                )
                            }
                        }
                    } else {
                        quote! {
                            Self::#ident => {
                                __serializer.serialize_tag(
                                    #variant_tag, #variant_idx
                                )
                            }
                        }
                    }
                }
                Internally => {
                    if variant.json_attrs.content.is_some() {
                        adjacently_tagged
                    } else if let Some(wrapped) = wrapped_value.as_ref() {
                        if variant.is_record {
                            quote! {
                                Self::#ident(__value) => {
                                    __serializer.serialize_internally_tagged(
                                        #tag_field, #variant_tag, #variant_idx, #wrapped
                                    )
                                }
                            }
                        } else {
                            adjacently_tagged
                        }
                    } else {
                        quote! {
                            Self::#ident => {
                                __serializer.serialize_internal_tag(
                                    #tag_field, #variant_tag, #variant_idx
                                )
                            }
                        }
                    }
                }
                Implicitly => {
                    if variant.json_attrs.content.is_some() {
                        adjacently_tagged
                    } else if let Some(wrapped) = wrapped_value.as_ref() {
                        quote! {
                            Self::#ident(__value) => {
                                __serializer.serialize_implicitly_tagged(
                                    #variant_tag, #variant_idx, #wrapped
                                )
                            }
                        }
                    } else {
                        quote! {
                            Self::#ident => {
                                __serializer.serialize_tag(
                                    #variant_tag, #variant_idx
                                )
                            }
                        }
                    }
                }
            }
        });

    quote! {
        let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, #ty_name);
        match self {
            #(#match_arms),*
        }
    }
}
