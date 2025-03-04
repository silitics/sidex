//! Generator for the [`serde::Deserialize`] implementation of record types.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::rstyir::{
    RsField, RsType, RsTypeRecord,
    serde::identifier_enum::{IdentifierKind, gen_identifier_enum},
};

/// Generates the body of [`deserialize`][serde::Deserialize::deserialize].
pub(crate) fn gen_deserialize_body(ty: &RsType, record_ty: &RsTypeRecord) -> TokenStream {
    // Generate the `__Visitor` for deserialization.
    let visitor = gen_visitor(ty, record_ty);

    // Generate the `__FIELDS` array constant with the field names.
    let fields_array_const = gen_fields_array_const(&record_ty.fields);

    let ty_name = &ty.name;

    quote! {
        #visitor
        #fields_array_const
        __serde::Deserializer::deserialize_struct(
            __deserializer,
            #ty_name,
            __FIELDS,
            __Visitor { __phantom_vars: ::core::marker::PhantomData },
        )
    }
}

/// Generates the `__Visitor` for deserialization.
fn gen_visitor(ty: &RsType, record_ty: &RsTypeRecord) -> TokenStream {
    let ty_ident = &ty.ident;

    let num_fields = record_ty.fields.len();

    let (field_variants, field_identifiers_enum) = gen_identifier_enum(
        record_ty.fields.iter().map(|field| &field.json_name),
        IdentifierKind::Field,
    );

    let field_indices = (0..num_fields).collect::<Vec<_>>();

    let field_idents = record_ty
        .fields
        .iter()
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    let field_vars = (0..num_fields)
        .map(|idx| format_ident!("__field{idx}"))
        .collect::<Vec<_>>();

    let field_tys = record_ty
        .fields
        .iter()
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

    let field_names = record_ty
        .fields
        .iter()
        .map(|field| &field.json_name)
        .collect::<Vec<_>>();

    let expecting = format!("record {}", ty.name);

    let expected_length = format!("record with {num_fields} fields");

    let extract_values = record_ty
        .fields
        .iter()
        .zip(field_vars.iter())
        .map(|(field, var)| {
            if !field.is_optional {
                let name = &field.json_name;
                quote! {
                    let #var = match #var {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field(#name)
                            );
                        }
                    };
                }
            } else {
                quote! {
                    let #var = match #var {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                }
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

    let type_generics = ty.type_generics();
    let impl_generics = ty.type_generics_with_bounds(&quote! { __serde::Deserialize<'de> });

    quote! {
        #field_identifiers_enum

        #[doc(hidden)]
        struct __Visitor <#type_generics> {
            __phantom_vars: ::core::marker::PhantomData<fn(&( #type_generics ))>,
        }

        impl <'de, #impl_generics> __serde::de::Visitor<'de> for __Visitor <#type_generics> {
            type Value = #ty_ident <#type_generics>;

            fn expecting(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(__formatter, #expecting)
            }

            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> ::core::result::Result<Self::Value, __A::Error>
            where
                __A: __serde::de::SeqAccess<'de>,
            {
                #(
                    // Optional fields are deserialized as `Option<T>` and, hence, need no special treatment.
                    let #field_vars = match __serde::de::SeqAccess::next_element::<#field_tys>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(#field_indices, &#expected_length)
                            );
                        }
                    };
                )*

                #constructor
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> ::core::result::Result<Self::Value, __A::Error>
            where
                __A: __serde::de::MapAccess<'de>,
            {
                #(
                    // We use the inner type here so optional fields end up with only one level of `Option`.
                    let mut #field_vars: ::core::option::Option<#field_tys> = ::core::option::Option::None;
                )*
                while let ::core::option::Option::Some(__key) = __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)? {
                    match __key {
                        #(
                            __Identifier::#field_variants => {
                                if ::core::option::Option::is_some(&#field_vars) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(#field_names)
                                    );
                                }
                                #field_vars = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<#field_tys>(&mut __map)?
                                );
                            },
                        )*
                        _ => {
                            // Unknown fields are simply ignored.
                            //
                            // 🔮 At a later point in time, we may want attributes to disallow unknown fields.
                            __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(&mut __map)?;
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
fn gen_fields_array_const(fields: &[RsField]) -> TokenStream {
    let names = fields.iter().map(|field| &field.json_name);
    quote! {
        #[doc(hidden)]
        const __FIELDS: &'static [&'static str] = &[#(#names,)*];
    }
}
