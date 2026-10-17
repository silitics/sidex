//! Generation of marker traits for groups.
//!
//! For every group selected with the `group_traits` setting, the generator
//! defines a marker trait at the root of the bundle module and implements it
//! for every record, variant, and wrapper belonging to the group. Code can then
//! use the trait as a bound to accept only members of the group. Aliases and
//! opaque types do not implement the traits because they do not define Rust
//! types of their own.

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;
use sidex_gen::rename::to_pascal_case;

use crate::context::BundleCtx;
use crate::context::SchemaCtx;
use crate::rstyir::RsType;
use crate::rstyir::RsTypeKind;

/// Defines the marker traits of the selected groups.
pub fn define_traits(ctx: &BundleCtx) -> Result<TokenStream> {
    let mut traits = TokenStream::new();
    let mut group_by_trait = HashMap::new();
    for name in &ctx.cfg.group_traits {
        let Some(group) = ctx.bundle.groups.get(name) else {
            return Err(Box::new(
                Diagnostic::error(format!(
                    "Rust setting `group_traits` names group `{name}`, which bundle `{}` does not \
                     declare.",
                    ctx.bundle.metadata.name
                ))
                .with_help(format!(
                    "Declare the group with a `[groups.{name}]` table in `sidex.toml`."
                )),
            ));
        };
        let ident = trait_ident(name)?;
        if let Some(other) = group_by_trait.insert(ident.to_string(), name) {
            return Err(Box::new(Diagnostic::error(format!(
                "Rust setting `group_traits` maps groups `{other}` and `{name}` to the same trait \
                 `{ident}`."
            ))));
        }
        let docs = format!(
            "Implemented by the records, variants, and wrappers of group `{name}`.\n\n{}",
            group.description.trim()
        );
        traits.extend(quote! {
            #[doc = #docs]
            pub trait #ident {}
        });
    }
    Ok(traits)
}

/// Implements the marker traits of the selected groups that `def` belongs to.
pub fn impl_traits(ctx: &SchemaCtx, def: &ir::Def, ty: &RsType) -> Result<TokenStream> {
    let mut impls = TokenStream::new();
    if matches!(ty.kind, RsTypeKind::Alias(_)) {
        return Ok(impls);
    }
    let ident = &ty.ident;
    let vars = &ty.vars;
    for name in &ctx.bundle_ctx.cfg.group_traits {
        if !def.groups.contains(name) {
            continue;
        }
        let trait_ident = trait_ident(name)?;
        impls.extend(quote! {
            #[automatically_derived]
            impl<#(#vars),*> super::#trait_ident for #ident<#(#vars),*> {}
        });
    }
    Ok(impls)
}

/// Name of the marker trait of group `name`, e.g., `MobileGroup` for `mobile`.
fn trait_ident(name: &str) -> Result<syn::Ident> {
    let ident = format!("{}Group", to_pascal_case(name));
    syn::parse_str(&ident).map_err(|_| {
        Box::new(Diagnostic::error(format!(
            "Group `{name}` has no valid Rust trait name: `{ident}` is not an identifier."
        )))
    })
}
