//! Generator for the [`serde::Serialize`] implementation of record types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{context::RustTy, plugins::serde::SerdeField};

/// Generates the body of [`serialize`][serde::Serialize::serialize].
pub(crate) fn gen_serialize_body(ty: &RustTy, fields: &[SerdeField]) -> TokenStream {
    let ty_name = &ty.name;
    let num_fields = fields.len();
    let serialize_fields = fields.iter().map(|field| {
        let ident = &field.rust_field.ident;
        let name = &field.name;
        if field.rust_field.is_optional {
            quote! {
                match &self.#ident {
                    ::core::option::Option::Some(__value) => {
                        ::serde::ser::SerializeStruct::serialize_field(&mut __struct, #name, __value)?;
                    },
                    ::core::option::Option::None => {
                        ::serde::ser::SerializeStruct::skip_field(&mut __struct, #name)?;
                    }
                }
            }
        } else {
            quote! {
                ::serde::ser::SerializeStruct::serialize_field(&mut __struct, #name, &self.#ident)?;
            }
        }
    });
    quote! {
        let mut __struct = ::serde::Serializer::serialize_struct(__serializer, #ty_name, #num_fields)?;
        #(#serialize_fields)*
        ::serde::ser::SerializeStruct::end(__struct)
    }
}
