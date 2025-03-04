//! Generator for the [`serde::Serialize`] implementation of record types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::rstyir::{RsType, RsTypeRecord};

/// Generates the body of [`serialize`][serde::Serialize::serialize].
pub(crate) fn gen_serialize_body(ty: &RsType, record_ty: &RsTypeRecord) -> TokenStream {
    let ty_name = &ty.name;
    let num_fields = record_ty.fields.len();
    let serialize_fields = record_ty.fields.iter().map(|field| {
        let ident = &field.ident;
        let name = &field.json_name;

        if field.json_attrs.inline {
            quote! {
                __record.serialize_inlined_field(#name, &self.#ident)?;
            }
        } else if field.is_optional {
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
