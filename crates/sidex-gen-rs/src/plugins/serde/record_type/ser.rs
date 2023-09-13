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

        if field.json_attrs.inline {
            quote! {
                __record.serialize_inlined_field(#name, &self.#ident)?;
            }   
        } else if field.rust_field.is_optional {
            quote! {
                __record.serialize_optional_field(#name, ::core::option::Option::as_ref(&self.#ident))?;
            }
        }else {
            quote! {
                __record.serialize_field(#name, &self.#ident)?;
            }
        }
    });
    quote! {
        let mut __record = __sidex_serde::ser::RecordSerializer::new(__serializer, #ty_name, #num_fields)?;
        #(#serialize_fields)*
        __record.end()
    }
}
