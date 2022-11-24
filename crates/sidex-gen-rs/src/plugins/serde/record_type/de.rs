//! Generator for the [`serde::Deserialize`] implementation of record types.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_attrs_json::JsonRecordTypeAttrs;

use crate::{
    context::RustTy,
    plugins::serde::{identifier_enum::gen_identifier_enum, SerdeField},
};

/// Generates the body of [`deserialize`][serde::Deserialize::deserialize].
pub(crate) fn gen_deserialize_body(
    ty: &RustTy,
    ty_json_attrs: &JsonRecordTypeAttrs,
    fields: &[SerdeField],
) -> TokenStream {
    // Generate the `__Visitor` for deserialization.
    let visitor = gen_visitor(ty, ty_json_attrs, fields);

    // Generate the `__FIELDS` array constant with the field names.
    let fields_array_const = gen_fields_array_const(fields);

    let ty_name = &ty.name;

    quote! {
        #visitor
        #fields_array_const
        ::serde::Deserializer::deserialize_struct(
            __deserializer,
            #ty_name,
            __FIELDS,
            __Visitor,
        )
    }
}

/// Generates the `__Visitor` for deserialization.
fn gen_visitor(
    ty: &RustTy,
    ty_json_attrs: &JsonRecordTypeAttrs,
    fields: &[SerdeField],
) -> TokenStream {
    let ty_ident = &ty.ident;

    let num_fields = fields.len();

    let (field_variants, field_identifiers_enum) = gen_identifier_enum(
        fields
            .iter()
            .map(|field| ty_json_attrs.field_name(&field.field, &field.json_attrs)),
    );

    let field_indices = (0..num_fields).collect::<Vec<_>>();

    let field_idents = fields
        .iter()
        .map(|field| &field.rust_field.ident)
        .collect::<Vec<_>>();

    let field_vars = (0..num_fields)
        .map(|idx| format_ident!("__field{idx}"))
        .collect::<Vec<_>>();

    let field_tys = fields
        .iter()
        .map(|field| &field.rust_field.ty)
        .collect::<Vec<_>>();

    let field_inner_tys = fields
        .iter()
        .map(|field| &field.rust_field.inner_ty)
        .collect::<Vec<_>>();

    let field_names = fields.iter().map(|field| &field.name).collect::<Vec<_>>();

    let expecting = format!("record {}", ty.name);

    let expected_length = format!("record with {num_fields} fields");

    let extract_values = fields
        .iter()
        .zip(field_vars.iter())
        .map(|(field, var)| {
            if !field.field.is_optional {
                let name = &field.name;
                quote! {
                    let #var = match #var {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as ::serde::de::Error>::missing_field(#name)
                            );
                        }
                    };
                }
            } else {
                TokenStream::default()
            }
        })
        .collect::<Vec<_>>();

    let constructor = quote! {
        ::core::result::Result::Ok(#ty_ident {
            #(
                #field_idents: #field_vars,
            )*
        })
    };

    quote! {
        #field_identifiers_enum

        struct __Visitor;

        impl<'de> ::serde::de::Visitor<'de> for __Visitor {
            type Value = #ty_ident;

            fn expecting(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(__formatter, #expecting)
            }

            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> ::core::result::Result<Self::Value, __A::Error>
            where
                __A: ::serde::de::SeqAccess<'de>,
            {
                #(
                    // Optional fields are deserialized as `Option<T>` and, hence, need no special treatment.
                    let #field_vars = match ::serde::de::SeqAccess::next_element::<#field_tys>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                ::serde::de::Error::invalid_length(#field_indices, &#expected_length)
                            );
                        }
                    };
                )*

                #constructor
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> ::core::result::Result<Self::Value, __A::Error>
            where
                __A: ::serde::de::MapAccess<'de>,
            {
                #(
                    // We use the inner type here so optional fields end up with only one level of `Option`.
                    let mut #field_vars: ::core::option::Option<#field_inner_tys> = ::core::option::Option::None;
                )*
                while let ::core::option::Option::Some(__key) = ::serde::de::MapAccess::next_key::<__Identifier>(&mut __map)? {
                    match __key {
                        #(
                            __Identifier::#field_variants => {
                                if ::core::option::Option::is_some(&#field_vars) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as ::serde::de::Error>::duplicate_field(#field_names)
                                    );
                                }
                                #field_vars = ::core::option::Option::Some(
                                    ::serde::de::MapAccess::next_value::<#field_inner_tys>(&mut __map)?
                                );
                            },
                        )*
                        _ => {
                            // Unknown fields are simply ignored.
                            //
                            // 🔮 At a later point in time, we may want attributes to disallow unknown fields.
                            ::serde::de::MapAccess::next_value::<::serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                };

                // Extract the values of non-optional fields.
                #(#extract_values)*

                #constructor
            }
        }
    }
}

/// Generates the `__FIELDS` array constant with the field names.
fn gen_fields_array_const(fields: &[SerdeField]) -> TokenStream {
    let names = fields.iter().map(|field| &field.name);
    quote! {
        const __FIELDS: &'static [&'static str] = &[#(#names,)*];
    }
}
