use proc_macro2::TokenStream;
use quote::quote;
use sidex_attrs_json::atoms::JsonTaggedAttr;

use crate::rstyir::{RsType, RsTypeVariant};

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

            let adjacently_tagged = if variant.ty.is_some() {
                quote! {
                    Self::#ident(__value) => {
                        __serializer.serialize_adjacently_tagged(
                            #tag_field, #value_field, #variant_tag, #variant_idx, __value
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
                    if variant.ty.is_some() {
                        quote! {
                            Self::#ident(__value) => {
                                __serializer.serialize_externally_tagged(
                                    #variant_tag, #variant_idx, __value
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
                    } else if variant.ty.is_some() {
                        if variant.is_record {
                            quote! {
                                Self::#ident(__value) => {
                                    __serializer.serialize_internally_tagged(
                                        #tag_field, #variant_tag, #variant_idx, __value
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
                    } else if variant.ty.is_some() {
                        quote! {
                            Self::#ident(__value) => {
                                __serializer.serialize_implicitly_tagged(
                                    #variant_tag, #variant_idx, __value
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
