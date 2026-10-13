//! Plugin implementing [`serde::Serialize`] and [`serde::Deserialize`] for Sidex
//! types.
//!
//! The implementations use the JSON representation through `sidex::serde`, which
//! requires the `serde-1` feature of the `sidex` crate.

use proc_macro2::TokenStream;
use quote::quote;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

use super::Plugin;
use crate::context::SchemaCtx;
use crate::rstyir::RsTypeKind;
use crate::rstyir::rs_type_from_def;

/// The [`Serde`] plugin.
pub struct Serde;

impl Plugin for Serde {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<TokenStream> {
        let Some(ty) = rs_type_from_def(ctx, def)? else {
            return Ok(Default::default());
        };
        if matches!(ty.kind, RsTypeKind::Alias(_)) {
            return Ok(Default::default());
        }
        let ident = &ty.ident;
        let vars = &ty.vars;
        Ok(quote! {
            #[automatically_derived]
            impl<#(#vars: ::sidex::Shaped + ::sidex::Encode),*> ::serde::Serialize for #ident<#(#vars),*> {
                fn serialize<__S: ::serde::Serializer>(
                    &self,
                    serializer: __S,
                ) -> ::std::result::Result<__S::Ok, __S::Error> {
                    ::sidex::serde::serialize(self, serializer)
                }
            }

            #[automatically_derived]
            impl<'de, #(#vars: ::sidex::Shaped + ::sidex::Decode),*> ::serde::Deserialize<'de> for #ident<#(#vars),*> {
                fn deserialize<__D: ::serde::Deserializer<'de>>(
                    deserializer: __D,
                ) -> ::std::result::Result<Self, __D::Error> {
                    ::sidex::serde::deserialize(deserializer)
                }
            }
        })
    }
}
