//! Generator for an `__Identifier` enum used when deserializing record and variant types.
//!
//! For deserializing field names and variant discriminants, an `__Identifier` enum is
//! created which has a variant for each field or variant, respectively. In addition, it
//! has an `__Unknown` variant for unknown field names and discriminates. This
//! `__Identifier` enum can then be deserialized from an identifier (based on
//! [`deserialize_identifier`][serde::Deserializer::deserialize_identifier]).
//!
//! The resulting `__Identifier` is used for identifying fields and variants when
//! deserializing record and variant types. Serde's derive macros for structs and enums
//! work in a similar fashion by generating a `__Field` enum.

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

/// Generates an `__Identifier` enum with an implementation of [`serde::Deserialize`].
///
/// Returns a vector of identifiers for the variants of the `__Identifier` enum and the
/// token stream containing the definition of the `__Identifier` enum itself as well as
/// the implementation of [`serde::Deserialize`] for it.
pub(crate) fn gen_identifier_enum<'n, S: AsRef<str>, I: IntoIterator<Item = S>>(
    identifiers: I,
) -> (Vec<syn::Ident>, TokenStream) {
    let names = identifiers.into_iter().collect::<Vec<_>>();

    let names_str = names.iter().map(AsRef::<str>::as_ref).collect::<Vec<_>>();
    let names_bytes = names_str
        .iter()
        .map(|name| Literal::byte_string(name.as_bytes()))
        .collect::<Vec<_>>();

    let indices = (0..names_str.len() as u64).collect::<Vec<_>>();
    let variants = indices
        .iter()
        .map(|idx| format_ident!("__Identifier{}", idx))
        .collect::<Vec<_>>();

    let expecting = format!("an identifier in {:?}", names_str);

    let tokens = quote! {
        #[derive(::core::clone::Clone, ::core::marker::Copy)]
        enum __Identifier {
            #(#variants,)*
            __Unknown,
        }

        struct __IdentifierVisitor;

        impl<'de> ::serde::de::Visitor<'de> for __IdentifierVisitor {
            type Value = __Identifier;

            fn expecting(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(__formatter, #expecting)
            }

            fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
            where
                __E: ::serde::de::Error,
            {
                ::core::result::Result::Ok(
                    match __value {
                        #(
                            #indices => __Identifier::#variants,
                        )*
                        _ => __Identifier::__Unknown,
                    }
                )
            }

            fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
            where
                __E: ::serde::de::Error,
            {
                ::core::result::Result::Ok(
                    match __value {
                        #(
                            #names_str => __Identifier::#variants,
                        )*
                        _ => __Identifier::__Unknown,
                    }
                )
            }

            fn visit_bytes<__E>(self, __value: &[u8]) -> ::core::result::Result<Self::Value, __E>
            where
                __E: ::serde::de::Error,
            {
                ::core::result::Result::Ok(
                    match __value {
                        #(
                            #names_bytes => __Identifier::#variants,
                        )*
                        _ => __Identifier::__Unknown,
                    }
                )
            }
        }

        impl<'de> ::serde::Deserialize<'de> for __Identifier {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
            where
                __D: ::serde::Deserializer<'de>,
            {
                ::serde::Deserializer::deserialize_identifier(__deserializer, __IdentifierVisitor)
            }
        }
    };

    (variants, tokens)
}
