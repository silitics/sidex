//! Generation of marker traits for groups.
//!
//! For every group selected with the `group_traits` setting, the generator
//! defines a marker trait at the root of the bundle module and implements it
//! for every record, variant, and wrapper belonging to the group. The trait is
//! named `<Group>GroupMarker` unless `group_trait_names` overrides it. Code can
//! then use the trait as a bound to accept only members of the group. Aliases
//! and opaque types do not implement the traits because they do not define
//! Rust types of their own.

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;
use sidex_gen::rename::to_pascal_case;

use crate::config::Config;
use crate::context::BundleCtx;
use crate::context::SchemaCtx;
use crate::rstyir::RsType;
use crate::rstyir::RsTypeKind;

/// Defines the marker traits of the selected groups.
pub fn define_traits(ctx: &BundleCtx) -> Result<TokenStream> {
    if let Some(name) = ctx
        .cfg
        .group_trait_names
        .keys()
        .find(|name| !ctx.cfg.group_traits.contains(name))
    {
        return Err(Box::new(
            Diagnostic::error(format!(
                "Rust setting `group_trait_names` names group `{name}`, which `group_traits` does \
                 not select."
            ))
            .with_help(format!("Add `{name}` to `group_traits`.")),
        ));
    }
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
        let ident = trait_ident(ctx.cfg, name)?;
        if let Some(other) = group_by_trait.insert(ident.to_string(), name) {
            return Err(Box::new(Diagnostic::error(format!(
                "Rust groups `{other}` and `{name}` have the same trait name `{ident}`."
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
        let trait_ident = trait_ident(ctx.bundle_ctx.cfg, name)?;
        impls.extend(quote! {
            #[automatically_derived]
            impl<#(#vars),*> super::#trait_ident for #ident<#(#vars),*> {}
        });
    }
    Ok(impls)
}

/// Name of the marker trait of group `name`, e.g., `MobileGroupMarker` for
/// `mobile` unless overridden.
fn trait_ident(cfg: &Config, name: &str) -> Result<syn::Ident> {
    let ident = cfg
        .group_trait_names
        .get(name)
        .cloned()
        .unwrap_or_else(|| format!("{}GroupMarker", to_pascal_case(name)));
    syn::parse_str(&ident).map_err(|_| {
        Box::new(
            Diagnostic::error(format!(
                "Rust trait name `{ident}` of group `{name}` is not an identifier."
            ))
            .with_help("Set a valid name in `group_trait_names`."),
        )
    })
}
