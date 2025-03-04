use proc_macro2::TokenStream;
use quote::quote;

use super::{RsType, RsTypeKind};

mod identifier_enum;
mod record_type;
mod variant_type;

pub fn rs_type_impl_serde(ty: &RsType) -> TokenStream {
    let ty_ident = &ty.ident;
    let (serialize_body, deserialize_body) = match &ty.kind {
        RsTypeKind::Wrapper(_) => {
            (
                quote! { self.0.serialize(__serializer) },
                quote! { Ok(#ty_ident(__serde::Deserialize::deserialize(__deserializer)?)) },
            )
        }
        RsTypeKind::Alias(_) => {
            // no implementation for type aliases
            return Default::default();
        }
        RsTypeKind::Record(rs_type_record) => {
            (
                record_type::ser::gen_serialize_body(ty, rs_type_record),
                record_type::de::gen_deserialize_body(ty, rs_type_record),
            )
        }
        RsTypeKind::Variant(rs_type_variant) => {
            (
                variant_type::ser::gen_serialize_body(ty, rs_type_variant),
                variant_type::de::gen_deserialize_body(ty, rs_type_variant),
            )
        }
    };

    let ty_ident = &ty.ident;

    let type_generics = ty.type_generics();

    let serialize_impl_generics = ty.type_generics_with_bounds(&quote! { __serde::Serialize });

    let deserialize_impl_generics =
        ty.type_generics_with_bounds(&quote! { __serde::Deserialize<'de> });

    quote! {
        #[automatically_derived]
        impl <#serialize_impl_generics> __serde::Serialize for #ty_ident <#type_generics> {
            fn serialize<__S: __serde::Serializer>(&self, __serializer: __S) -> ::std::result::Result<__S::Ok, __S::Error> {
                #serialize_body
            }
        }
        #[automatically_derived]
        impl <'de, #deserialize_impl_generics> __serde::Deserialize<'de> for #ty_ident <#type_generics> {
            fn deserialize<__D: __serde::Deserializer<'de>>(__deserializer: __D) -> ::std::result::Result<Self, __D::Error> {
                #deserialize_body
            }
        }
    }
}
