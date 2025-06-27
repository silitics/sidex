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

    let expecting = format!("record {}", ty.name);

    let type_generics = ty.type_generics();
    let impl_generics = ty.type_generics_with_bounds(&quote! { __serde::Deserialize<'de> });

    let visit_map_body = gen_visit_map_body(ty, record_ty);
    let visit_seq_body = gen_visit_seq_body(ty, record_ty);

    quote! {
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
                #visit_seq_body
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> ::core::result::Result<Self::Value, __A::Error>
            where
                __A: __serde::de::MapAccess<'de>,
            {
                #visit_map_body
            }
        }
    }
}

fn gen_visit_seq_body(ty: &RsType, record_ty: &RsTypeRecord) -> TokenStream {
    let ty_ident = &ty.ident;

    let num_fields = record_ty.fields.len();

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

    let expected_length = format!("record with {num_fields} fields");

    let constructor = quote! {
        ::core::result::Result::Ok(#ty_ident {
            #(
                #field_idents: #field_vars,
            )*
        })
    };

    quote! {
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
}

fn gen_visit_map_body(ty: &RsType, record_ty: &RsTypeRecord) -> TokenStream {
    let ty_ident = &ty.ident;

    let (field_variants, field_identifiers_enum) = gen_identifier_enum(
        record_ty
            .fields
            .iter()
            .filter(|f| !f.json_attrs.inline)
            .map(|field| &field.json_name),
        IdentifierKind::Field,
    );

    let num_fields = field_variants.len();

    let field_idents = record_ty
        .fields
        .iter()
        .filter(|f| !f.json_attrs.inline)
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    let field_vars = (0..num_fields)
        .map(|idx| format_ident!("__field{idx}"))
        .collect::<Vec<_>>();

    let field_tys = record_ty
        .fields
        .iter()
        .filter(|f| !f.json_attrs.inline)
        .map(|field| &field.ty)
        .collect::<Vec<_>>();

    let field_names = record_ty
        .fields
        .iter()
        .filter(|f| !f.json_attrs.inline)
        .map(|field| &field.json_name)
        .collect::<Vec<_>>();

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

    let inline_field_idents = record_ty
        .fields
        .iter()
        .filter(|f| f.json_attrs.inline)
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    let constructor = quote! {
        ::core::result::Result::Ok(#ty_ident {
            #(
                #field_idents: #field_vars,
            )*
            #(
                #inline_field_idents: __sidex_serde::de::content::deserialize_content_ref(&__content)?,
            )*
        })
    };

    let has_inline_fields = record_ty.fields.iter().any(|f| f.json_attrs.inline);

    if !has_inline_fields {
        quote! {
            #field_identifiers_enum

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
    } else {
        quote! {
            #field_identifiers_enum

            let __content = __sidex_serde::de::content::deserialize_content_map(__map)?;
            #(
                // We use the inner type here so optional fields end up with only one level of `Option`.
                let mut #field_vars: ::core::option::Option<#field_tys> = ::core::option::Option::None;
            )*
            let __sidex_serde::de::content::Content::Map(__map) = &__content else {
                panic!("expected a map");
            };
            for (__key, __value) in __map.iter() {
                let __key: __Identifier = __sidex_serde::de::content::deserialize_content_ref(__key)?;
                match __key {
                    #(
                        __Identifier::#field_variants => {
                            if ::core::option::Option::is_some(&#field_vars) {
                                return ::core::result::Result::Err(
                                    <__A::Error as __serde::de::Error>::duplicate_field(#field_names)
                                );
                            }
                            #field_vars = ::core::option::Option::Some(
                                __sidex_serde::de::content::deserialize_content_ref(__value)?
                            );
                        },
                    )*
                    _ => {
                        // Ignore unknown fields.
                    }
                }
            }
            // Extract the values of non-optional fields.
            #(#extract_values)*

            #constructor
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
