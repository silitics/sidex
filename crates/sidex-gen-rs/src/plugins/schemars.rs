//! Plugin implementing `schemars::JsonSchema` for Sidex types.
//!
//! The schemas describe the JSON representation and come from the types' shapes
//! through `sidex::formats::json::schemars_schema`, which requires the
//! `schemars-1` feature of the `sidex` crate.

use proc_macro2::TokenStream;
use quote::quote;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

use super::Plugin;
use crate::context::SchemaCtx;
use crate::rstyir::RsTypeKind;
use crate::rstyir::rs_type_from_def;

/// The [`Schemars`] plugin.
pub struct Schemars;

impl Plugin for Schemars {
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
            impl<#(#vars: ::sidex::Shaped),*> ::schemars::JsonSchema for #ident<#(#vars),*> {
                fn schema_name() -> ::std::borrow::Cow<'static, str> {
                    ::std::borrow::Cow::Owned(<Self as ::sidex::Shaped>::SHAPE.to_string())
                }

                fn json_schema(generator: &mut ::schemars::SchemaGenerator) -> ::schemars::Schema {
                    ::sidex::formats::json::schemars_schema(<Self as ::sidex::Shaped>::SHAPE, generator)
                }
            }
        })
    }
}
