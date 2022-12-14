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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum IdentifierKind {
    Field,
    Variant,
}

/// Generates an `__Identifier` enum with an implementation of [`serde::Deserialize`].
///
/// Returns a vector of identifiers for the variants of the `__Identifier` enum and the
/// token stream containing the definition of the `__Identifier` enum itself as well as
/// the implementation of [`serde::Deserialize`] for it.
pub(crate) fn gen_identifier_enum<'n, S: AsRef<str>, I: IntoIterator<Item = S>>(
    identifiers: I,
    kind: IdentifierKind,
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

    let unknown = match kind {
        IdentifierKind::Field => quote! { __Unknown },
        IdentifierKind::Variant => TokenStream::default(),
    };

    let match_unknown_str = match kind {
        IdentifierKind::Field => {
            quote! {
                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
            }
        }
        IdentifierKind::Variant => {
            quote! {
                __variant => ::core::result::Result::Err(
                    __serde::de::Error::unknown_variant(__variant, __IDENTIFIERS)
                ),
            }
        }
    };

    let match_unknown_bytes = match kind {
        IdentifierKind::Field => {
            quote! {
                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
            }
        }
        IdentifierKind::Variant => {
            quote! {
                __variant => ::core::result::Result::Err(
                    __serde::de::Error::invalid_value(
                        __serde::de::Unexpected::Bytes(__variant),
                        &__EXPECTING_IDENTIFIERS
                    )
                ),
            }
        }
    };

    let match_unknown_u64 = match kind {
        IdentifierKind::Field => {
            quote! {
                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
            }
        }
        IdentifierKind::Variant => {
            quote! {
                __variant => ::core::result::Result::Err(
                    __serde::de::Error::invalid_value(
                        __serde::de::Unexpected::Unsigned(__variant),
                        &__EXPECTING_IDENTIFIERS
                    )
                ),
            }
        }
    };

    let tokens = quote! {
        const __IDENTIFIERS: &'static [&'static str] = &[#(#names_str,)*];

        const __EXPECTING_IDENTIFIERS: &'static str = #expecting;

        #[derive(::core::clone::Clone, ::core::marker::Copy)]
        enum __Identifier {
            #(#variants,)*
            #unknown
        }

        struct __IdentifierVisitor;

        impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
            type Value = __Identifier;

            fn expecting(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
            }

            fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
            where
                __E: __serde::de::Error,
            {

                match __value {
                    #(
                        #indices => ::core::result::Result::Ok(__Identifier::#variants),
                    )*
                    #match_unknown_u64
                }
            }

            fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
            where
                __E: __serde::de::Error,
            {
                match __value {
                    #(
                        #names_str => ::core::result::Result::Ok(__Identifier::#variants),
                    )*
                    #match_unknown_str
                }
            }

            fn visit_bytes<__E>(self, __value: &[u8]) -> ::core::result::Result<Self::Value, __E>
            where
                __E: __serde::de::Error,
            {
                match __value {
                    #(
                        #names_bytes => ::core::result::Result::Ok(__Identifier::#variants),
                    )*
                    #match_unknown_bytes
                }
            }
        }

        impl<'de> __serde::Deserialize<'de> for __Identifier {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
            where
                __D: __serde::Deserializer<'de>,
            {
                __serde::Deserializer::deserialize_identifier(__deserializer, __IdentifierVisitor)
            }
        }
    };

    (variants, tokens)
}
