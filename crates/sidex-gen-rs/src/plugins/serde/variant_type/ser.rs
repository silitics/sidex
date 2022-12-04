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
    plugins::serde::SerdeVariant,
};

pub(crate) fn gen_serialize_body(
    ctx: &SchemaCtx,
    ty: &RustTy,
    ty_json_attrs: &JsonVariantTypeAttrs,
    variants: &[SerdeVariant],
) -> Result<TokenStream> {
    let human_readable_body = match &ty_json_attrs.tagged {
        JsonTaggedAttr::Externally => {
            // Externally tagged is the default in Serde. Hence, we do not need to
            // generate a separate body for human readable interchange formats.
            return Ok(gen_externally_tagged_body(ty, variants));
        }
        JsonTaggedAttr::Adjacently => gen_adjacently_tagged_body(ty, ty_json_attrs, variants),
        JsonTaggedAttr::Internally => gen_internally_tagged_body(ctx, ty, ty_json_attrs, variants)?,
    };

    let non_human_readable_body = gen_externally_tagged_body(ty, variants);

    Ok(quote! {
        if ::serde::Serializer::is_human_readable(&__serializer) {
            #human_readable_body
        } else {
            #non_human_readable_body
        }
    })
}

fn gen_adjacently_tagged_body(
    ty: &RustTy,
    ty_json_attrs: &JsonVariantTypeAttrs,
    variants: &[SerdeVariant],
) -> TokenStream {
    let ty_name = &ty.name;

    let ty_tag_field = ty_json_attrs.tag_field_name();

    let match_arms = variants.iter().map(|variant| {
        let ident = &variant.rust_variant.ident;
        let name = &variant.name;

        let content_field = ty_json_attrs.content_field_name(&variant.json_attrs);

        let serialize_tag = quote! {
            ::serde::ser::SerializeStruct::serialize_field(&mut __struct, #ty_tag_field, #name)?;
        };

        if variant.rust_variant.ty.is_some() {
            quote! {
                Self::#ident(__value) => {
                    let mut __struct = ::serde::Serializer::serialize_struct(__serializer, #ty_name, 2)?;
                    #serialize_tag
                    ::serde::ser::SerializeStruct::serialize_field(&mut __struct, #content_field, __value)?;
                    ::serde::ser::SerializeStruct::end(__struct)
                }
            }
        } else {
            quote! {
                Self::#ident => {
                    let mut __struct = ::serde::Serializer::serialize_struct(__serializer, #ty_name, 1)?;
                    #serialize_tag
                    ::serde::ser::SerializeStruct::end(__struct)
                }
            }
        }
    });

    quote! {
        match self {
            #(#match_arms),*
        }
    }
}

fn gen_internally_tagged_body(
    ctx: &SchemaCtx,
    ty: &RustTy,
    ty_json_attrs: &JsonVariantTypeAttrs,
    variants: &[SerdeVariant],
) -> Result<TokenStream> {
    let ty_name = &ty.name;

    let ty_tag_field = ty_json_attrs.tag_field_name();

    let match_arms = variants.iter().map(|variant| {
        let ident = &variant.rust_variant.ident;
        let name = &variant.name;

        let serialize_tag = quote! {
            ::serde::ser::SerializeStruct::serialize_field(&mut __struct, #ty_tag_field, #name)?;
        };

        if let Some(typ) = &variant.variant.typ {
            // In contrast to Serde, we have all type information here. Hence, we do not
            // have to resort to runtime errors and can simply query Sidex for the fields.

            let resolved = ctx.bundle_ctx.unit.resolve_aliases(typ);

            if let Some(inner_record) = ctx.bundle_ctx.unit.record_type(&resolved) {
                let inner_def = ctx.bundle_ctx.unit.type_def(&resolved).expect("We already established that the type is a record type, hence, there must be a definition.");

                let inner_json_attrs = JsonRecordTypeAttrs::try_from_attrs(&inner_def.attrs)?;

                let num_fields = inner_record.fields.len() + 1;

                let serialize_fields = inner_record.fields.iter().map(|field| {
                    let rust_field = ctx.field(&inner_def, field);
                    let json_field_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
                    let ident = &rust_field.ident;
                    let name = inner_json_attrs.field_name(field, &json_field_attrs);
                    Ok(quote!{
                        ::serde::ser::SerializeStruct::serialize_field(&mut __struct, #name, &__value.#ident)?;
                    })
                }).collect::<Result<Vec<_>>>()?;

                Ok(quote! {
                    Self::#ident(__value) => {
                        let mut __struct = ::serde::Serializer::serialize_struct(__serializer, #ty_name, #num_fields)?;
                        #serialize_tag
                        #(#serialize_fields)*
                        ::serde::ser::SerializeStruct::end(__struct)
                    }
                })
            } else {
                let content_name = ty_json_attrs.content_field_name(&variant.json_attrs);
                Ok(quote! {
                    Self::#ident(__value) => {
                        let mut __struct = ::serde::Serializer::serialize_struct(__serializer, #ty_name, 2)?;
                        #serialize_tag
                        ::serde::ser::SerializeStruct::serialize_field(&mut __struct, #content_name, &__value)?;
                        ::serde::ser::SerializeStruct::end(__struct)
                    }
                })
            }
        } else {
            Ok(quote! {
                Self::#ident => {
                    let mut __struct = ::serde::Serializer::serialize_struct(__serializer, #ty_name, 1)?;
                    #serialize_tag
                    ::serde::ser::SerializeStruct::end(__struct)
                }
            })
        }
    }).collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        match self {
            #(#match_arms),*
        }
    })
}

fn gen_externally_tagged_body(ty: &RustTy, variants: &[SerdeVariant]) -> TokenStream {
    let ty_name = &ty.name;

    let match_arms = variants.iter().enumerate().map(|(idx, variant)| {
        let idx = idx as u32;
        let ident = &variant.rust_variant.ident;
        let name = &variant.name;
        if variant.rust_variant.ty.is_some() {
            quote! {
                Self::#ident(__value) => {
                    ::serde::Serializer::serialize_newtype_variant(
                        __serializer, #ty_name, #idx, #name, __value
                    )
                }
            }
        } else {
            quote! {
                Self::#ident => {
                    ::serde::Serializer::serialize_unit_variant(
                        __serializer, #ty_name, #idx, #name
                    )
                }
            }
        }
    });

    quote! {
        match self {
            #(#match_arms),*
        }
    }
}
