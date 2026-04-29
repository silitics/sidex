//! Generator for the [`serde::Serialize`] implementation of record types.

use proc_macro2::TokenStream;
use quote::quote;

use crate::rstyir::RsType;
use crate::rstyir::RsTypeRecord;

/// Generates the body of [`serialize`][serde::Serialize::serialize].
pub(crate) fn gen_serialize_body(ty: &RsType, record_ty: &RsTypeRecord) -> TokenStream {
    let ty_name = &ty.name;
    let num_fields = record_ty.fields.len();
    let serialize_fields = record_ty.fields.iter().map(|field| {
        let ident = &field.ident;
        let name = &field.json_name;
        let field_ty = &field.ty;
        let inner_ty = &field.inner_ty;
        let encoding = &field.encoding;
        let inner_encoding = &field.inner_encoding;

        if field.json_attrs.inline {
            quote! {
                __record.serialize_inlined_field(
                    #name,
                    &__sidex_serde::SerializeAsWrap::<#field_ty, #encoding>::new(&self.#ident),
                )?;
            }
        } else if field.is_optional {
            // `serialize_optional_field` reads the outer `Option` to decide
            // whether to skip the field on the wire; the inner value is
            // wrapped with the field's encoding for the wire form.
            quote! {
                {
                    let __wrapped = ::core::option::Option::map(
                        ::core::option::Option::as_ref(&self.#ident),
                        |__v| __sidex_serde::SerializeAsWrap::<#inner_ty, #inner_encoding>::new(__v),
                    );
                    __record.serialize_optional_field(
                        #name,
                        ::core::option::Option::as_ref(&__wrapped),
                    )?;
                }
            }
        } else {
            quote! {
                __record.serialize_field(
                    #name,
                    &__sidex_serde::SerializeAsWrap::<#field_ty, #encoding>::new(&self.#ident),
                )?;
            }
        }
    });
    quote! {
        let mut __record = __sidex_serde::ser::RecordSerializer::new(__serializer, #ty_name, #num_fields)?;
        #(#serialize_fields)*
        __record.end()
    }
}
