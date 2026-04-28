//! Typed attribute schemas for the Rust codegen target.
//!
//! These types mirror [`lib/rust/schemas/attrs.sidex`](https://github.com/silitics/sidex/blob/main/lib/rust/schemas/attrs.sidex).
//! The compiler validates source `#[rust(...)]` attributes against the
//! schemas at IR build time and stores the result in
//! `typed_attrs["rust"]`. The wrapper types in this module read off that
//! map and adapt the wire shapes into ergonomic codegen-friendly forms
//! (`Visibility` with a `ToTokens` impl, `Wrapper` with a Rust path,
//! `TokenStream` derives and attributes).

mod generated;

use std::collections::HashMap;
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use serde_json::Value;
use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Result;
use sidex_ir as ir;

pub use generated::attrs as raw;

const PLUGIN: &str = "rust";

/// Pull the typed `rust` attributes off any IR node's `typed_attrs` map and
/// deserialize them into the generated struct `T`. Returns `Ok(None)` if
/// the node has no `rust` attrs.
pub fn extract<T>(typed_attrs: &HashMap<String, Value>) -> Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    match typed_attrs.get(PLUGIN) {
        Some(value) => serde_json::from_value(value.clone())
            .map(Some)
            .map_err(|err| Diagnostic::error(format!("Invalid `rust` attributes: {err}"))),
        None => Ok(None),
    }
}

/// Visibility of a record field as expressed by `#[rust(...)]`.
#[derive(Clone, Copy, Debug, Default)]
pub enum Visibility {
    /// `#[rust(pub)]` — the default for record fields.
    #[default]
    Pub,
    /// `#[rust(pub(crate))]` — restrict to the current crate.
    Crate,
    /// `#[rust(pub(super))]` — restrict to the parent module.
    Super,
    /// `#[rust(private)]` — explicitly private.
    Private,
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Visibility::Pub => tokens.extend(quote! { pub }),
            Visibility::Crate => tokens.extend(quote! { pub(crate) }),
            Visibility::Super => tokens.extend(quote! { pub(super) }),
            Visibility::Private => {
                // No tokens — private by default.
            }
        }
    }
}

/// A Rust wrapper applied to a field's type (e.g. `Box<T>`).
#[derive(Clone, Debug)]
pub struct Wrapper {
    /// Fully-qualified path to the wrapper type.
    pub wrapper: String,
}

impl Wrapper {
    fn new(path: impl Into<String>) -> Self {
        Self {
            wrapper: path.into(),
        }
    }
}

/// Rust attributes for a record field.
#[derive(Clone, Debug, Default)]
pub struct FieldAttrs {
    pub visibility: Visibility,
    pub wrappers: Vec<Wrapper>,
    pub name: Option<String>,
}

/// Read the `FieldAttrs` for `field` from its `typed_attrs["rust"]`.
pub fn field_attrs(field: &ir::Field) -> Result<FieldAttrs> {
    let Some(raw) = extract::<raw::FieldAttrs>(&field.typed_attrs)? else {
        return Ok(FieldAttrs::default());
    };
    let is_private = raw.is_private.unwrap_or(false);
    let visibility = match (raw.is_pub, is_private) {
        (Some(pub_vis), false) => {
            if pub_vis.is_crate.unwrap_or(false) {
                Visibility::Crate
            } else if pub_vis.is_super.unwrap_or(false) {
                Visibility::Super
            } else {
                Visibility::Pub
            }
        }
        (None, true) => Visibility::Private,
        (None, false) => Visibility::Pub,
        (Some(_), true) => {
            return Err(Diagnostic::error(
                "`#[rust(pub)]` and `#[rust(private)]` cannot both apply to the same field.",
            ));
        }
    };
    let mut wrappers = Vec::new();
    if raw.is_box.unwrap_or(false) {
        wrappers.push(Wrapper::new("::std::boxed::Box"));
    }
    if raw.is_arc.unwrap_or(false) {
        wrappers.push(Wrapper::new("::std::sync::Arc"));
    }
    if raw.is_rc.unwrap_or(false) {
        wrappers.push(Wrapper::new("::std::rc::Rc"));
    }
    if let Some(custom) = raw.wrap {
        wrappers.push(Wrapper::new(custom));
    }
    Ok(FieldAttrs {
        visibility,
        wrappers,
        name: raw.name,
    })
}

/// A `#[rust(type = "...")]` override.
#[derive(Clone, Debug)]
pub struct Type {
    pub path: String,
}

/// Extra `#[derive(...)]` traits.
#[derive(Clone, Debug, Default)]
pub struct Derive {
    pub positive: Vec<TokenStream>,
}

/// Rust attributes for any definition.
#[derive(Clone, Debug, Default)]
pub struct TypeAttrs {
    pub typ: Option<Type>,
    pub derive: Derive,
    pub attrs: Vec<TokenStream>,
}

/// Read the `TypeAttrs` for `def` from its `typed_attrs["rust"]`.
pub fn type_attrs(def: &ir::Def) -> Result<TypeAttrs> {
    let Some(raw) = extract::<raw::TypeAttrs>(&def.typed_attrs)? else {
        return Ok(TypeAttrs::default());
    };
    let derive = Derive {
        positive: raw
            .derive
            .unwrap_or_default()
            .iter()
            .map(|s| TokenStream::from_str(s))
            .collect::<std::result::Result<_, _>>()
            .map_err(|err| {
                Diagnostic::error(format!("Invalid token stream in `#[rust(derive(...))]`: {err}"))
            })?,
    };
    let attrs = raw
        .attr
        .unwrap_or_default()
        .iter()
        .map(|s| TokenStream::from_str(s))
        .collect::<std::result::Result<_, _>>()
        .map_err(|err| {
            Diagnostic::error(format!("Invalid token stream in `#[rust(attr(...))]`: {err}"))
        })?;
    Ok(TypeAttrs {
        typ: raw.typ.map(|path| Type { path }),
        derive,
        attrs,
    })
}
