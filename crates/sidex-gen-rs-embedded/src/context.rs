//! Resolution of Sidex types to Rust types.
//!
//! Generated types take the lifetime `'a` if they borrow, that is, if they
//! contain a type borrowing from the input, such as a string. Type variables
//! used as keys of maps need the `MapKey` bound. Both properties propagate
//! through the definitions using a type, so they are computed for all
//! definitions up front.

use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use sidex_attrs_json::opaque_type_attrs;
use sidex_attrs_json::types::JsonShape;
use sidex_attrs_json::types::JsonType;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

use crate::config::Config;

/// Returns the Rust identifier for a name, which is raw if the name is a
/// keyword, for instance, `r#type`.
///
/// Fails for names that are not identifiers and for the keywords that cannot be
/// raw identifiers, such as `self`.
pub(crate) fn rust_ident(name: &str) -> Result<Ident> {
    if let Ok(ident) = syn::parse_str::<Ident>(name) {
        return Ok(ident);
    }
    if !matches!(name, "self" | "Self" | "super" | "crate" | "_")
        && let Ok(ident) = syn::parse_str::<Ident>(&format!("r#{name}"))
    {
        return Ok(ident);
    }
    Err(Box::new(Diagnostic::error(format!(
        "The `rust-embedded` backend cannot use `{name}` as a Rust identifier."
    ))))
}

/// Path of the runtime crate in generated code.
pub(crate) fn runtime() -> TokenStream {
    quote! { ::sidex_embedded }
}

/// Representation of an opaque type without a configured Rust type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpaqueRepr {
    /// String.
    Str,
    /// Boolean.
    Bool,
    /// Any JSON value.
    Json,
}

impl OpaqueRepr {
    /// Returns the representation implied by the opaque type's JSON type.
    pub(crate) fn of(def: &ir::Def) -> Result<Self> {
        let Some(union) = opaque_type_attrs(def)?.typ else {
            return Ok(Self::Json);
        };
        Ok(match union.classify() {
            JsonShape::Single(JsonType::String) => Self::Str,
            JsonShape::Single(JsonType::Boolean) => Self::Bool,
            _ => Self::Json,
        })
    }

    pub(crate) fn rust_type(self) -> TokenStream {
        let rt = runtime();
        match self {
            Self::Str => quote! { #rt::types::Str<'a> },
            Self::Bool => quote! { bool },
            Self::Json => quote! { #rt::types::Json<'a> },
        }
    }
}

/// Context for generating the code of a bundle.
pub(crate) struct Ctx<'cx> {
    pub(crate) cfg: &'cx Config,
    pub(crate) unit: &'cx ir::Ir,
    pub(crate) bundle: ir::BundleIdx,
    /// Indicates for each definition whether its Rust type borrows.
    borrows: Vec<bool>,
    /// Type variables of each definition that are used as keys of maps.
    keyed: Vec<BTreeSet<usize>>,
}

impl<'cx> Ctx<'cx> {
    pub(crate) fn new(cfg: &'cx Config, unit: &'cx ir::Ir, bundle: ir::BundleIdx) -> Result<Self> {
        let mut ctx = Self {
            cfg,
            unit,
            bundle,
            borrows: vec![false; unit.defs.len()],
            keyed: vec![BTreeSet::new(); unit.defs.len()],
        };
        // Borrowing only ever becomes true and the sets of keyed variables
        // only grow, so iterating until nothing changes reaches a fixed point.
        loop {
            let mut changed = false;
            for (idx, def) in unit.defs.iter().enumerate() {
                if !ctx.borrows[idx] && ctx.compute_borrows(idx, def)? {
                    ctx.borrows[idx] = true;
                    changed = true;
                }
                let mut keyed = ctx.keyed[idx].clone();
                for typ in types_of(def) {
                    ctx.collect_keyed(typ, &mut keyed);
                }
                if keyed != ctx.keyed[idx] {
                    ctx.keyed[idx] = keyed;
                    changed = true;
                }
            }
            if !changed {
                return Ok(ctx);
            }
        }
    }

    /// Indicates whether the Rust type of the definition borrows.
    pub(crate) fn def_borrows(&self, def: ir::DefRef) -> bool {
        self.borrows[def.def.idx()]
    }

    /// Indicates whether the Rust type of a type borrows.
    pub(crate) fn type_borrows(&self, typ: &ir::Type) -> bool {
        match &typ.kind {
            ir::TypeKind::TypeVar(_) => false,
            ir::TypeKind::Instance(instance) => {
                self.def_borrows(instance.def)
                    || instance.subst.iter().any(|typ| self.type_borrows(typ))
            }
        }
    }

    /// Indices of the definition's type variables used as keys of maps.
    pub(crate) fn keyed_vars(&self, def: ir::DefIdx) -> &BTreeSet<usize> {
        &self.keyed[def.idx()]
    }

    /// Returns the fully qualified Sidex path of a definition.
    pub(crate) fn path(&self, def: ir::DefRef) -> String {
        format!(
            "::{}::{}::{}",
            self.unit[def.bundle].metadata.name,
            self.unit[def.schema].name,
            self.unit[def].name.as_str()
        )
    }

    /// Indicates whether the definition is the built-in with the given name.
    pub(crate) fn is_builtin(&self, def: ir::DefRef, name: &str) -> bool {
        def.bundle == ir::STD_BUNDLE_IDX && self.unit[def].name.as_str() == name
    }

    /// Returns the configured Rust type of a definition.
    pub(crate) fn configured_type(&self, def: ir::DefRef) -> Option<&'cx str> {
        self.cfg
            .types
            .table
            .get(&self.path(def))
            .map(String::as_str)
    }

    /// Parses the configured Rust type of a definition.
    pub(crate) fn parse_configured_type(&self, def: ir::DefRef) -> Result<Option<syn::Type>> {
        let Some(rust_type) = self.configured_type(def) else {
            return Ok(None);
        };
        syn::parse_str(rust_type).map(Some).map_err(|error| {
            Box::new(Diagnostic::error(format!(
                "Invalid Rust type `{rust_type}` for `{}`: {error}.",
                self.path(def)
            )))
        })
    }

    /// Resolves a type used in `def` of `schema` to its Rust type.
    pub(crate) fn resolve(
        &self,
        schema: ir::SchemaIdx,
        def: &ir::Def,
        typ: &ir::Type,
    ) -> Result<TokenStream> {
        let instance = match &typ.kind {
            ir::TypeKind::TypeVar(var) => {
                let ident = rust_ident(def[var.idx].name.as_str())?;
                return Ok(quote! { #ident });
            }
            ir::TypeKind::Instance(instance) => instance,
        };
        let args = instance
            .subst
            .iter()
            .map(|typ| self.resolve(schema, def, typ))
            .collect::<Result<Vec<_>>>()?;
        let lifetime = self.def_borrows(instance.def).then(|| quote! { 'a, });
        let generics = if lifetime.is_some() || !args.is_empty() {
            quote! { <#lifetime #(#args,)*> }
        } else {
            quote! {}
        };
        if let Some(rust_type) = self.parse_configured_type(instance.def)? {
            return Ok(if args.is_empty() {
                quote! { #rust_type }
            } else {
                quote! { #rust_type<#(#args,)*> }
            });
        }
        if instance.def.bundle == ir::STD_BUNDLE_IDX {
            return builtin(self.unit[instance.def].name.as_str(), &args);
        }
        let target = &self.unit[instance.def];
        let ident = rust_ident(target.name.as_str())?;
        let schema_ident = rust_ident(&self.unit[instance.def.schema].name)?;
        if instance.def.bundle == self.bundle {
            return Ok(if instance.def.schema == schema {
                quote! { #ident #generics }
            } else {
                quote! { super::#schema_ident::#ident #generics }
            });
        }
        let bundle_name = &self.unit[instance.def.bundle].metadata.name;
        let Some(external) = self.cfg.external.get(bundle_name) else {
            return Err(Box::new(
                Diagnostic::error(format!(
                    "No Rust module configured for bundle `{bundle_name}`, which `{}` belongs to.",
                    self.path(instance.def)
                ))
                .with_help(format!(
                    "Add `{bundle_name}` to the `external` setting of the `rust-embedded` backend."
                )),
            ));
        };
        let external = syn::parse_str::<syn::Path>(external).map_err(|error| {
            Diagnostic::error(format!(
                "Invalid Rust path `{external}` for bundle `{bundle_name}`: {error}."
            ))
        })?;
        Ok(quote! { #external::#schema_ident::#ident #generics })
    }

    /// Indicates whether the Rust type of a type is known to implement
    /// `MapKey`, which configured types are not.
    pub(crate) fn implements_map_key(&self, typ: &ir::Type) -> Result<bool> {
        let instance = match &self.unit.resolve_aliases(typ).kind {
            ir::TypeKind::TypeVar(_) => return Ok(true),
            ir::TypeKind::Instance(instance) => instance.clone(),
        };
        if self.configured_type(instance.def).is_some() {
            return Ok(false);
        }
        let def = &self.unit[instance.def];
        if instance.def.bundle == ir::STD_BUNDLE_IDX {
            return Ok(KEY_BUILTINS.contains(&def.name.as_str()));
        }
        Ok(match &def.kind {
            ir::DefKind::WrapperType(wrapper) if def.vars.is_empty() => {
                self.implements_map_key(&wrapper.wrapped)?
            }
            ir::DefKind::OpaqueType(_) => OpaqueRepr::of(def)? == OpaqueRepr::Str,
            _ => false,
        })
    }

    fn compute_borrows(&self, idx: usize, def: &ir::Def) -> Result<bool> {
        let def_ref = ir::DefRef {
            bundle: self.unit.schemas[def.schema.idx()].bundle,
            schema: def.schema,
            def: ir::DefIdx::from(idx),
        };
        if let Some(rust_type) = self.configured_type(def_ref) {
            return Ok(rust_type.contains("'a"));
        }
        if def_ref.bundle == ir::STD_BUNDLE_IDX {
            return Ok(BORROWING_BUILTINS.contains(&def.name.as_str()));
        }
        if let ir::DefKind::OpaqueType(_) = def.kind {
            return Ok(OpaqueRepr::of(def)? != OpaqueRepr::Bool);
        }
        Ok(types_of(def).any(|typ| self.type_borrows(typ)))
    }

    /// Adds the type variables that `typ` uses as keys of maps.
    fn collect_keyed(&self, typ: &ir::Type, keyed: &mut BTreeSet<usize>) {
        let ir::TypeKind::Instance(instance) = &typ.kind else {
            return;
        };
        let var = |index: usize| {
            match instance.subst.get(index).map(|typ| &typ.kind) {
                Some(ir::TypeKind::TypeVar(var)) => Some(var.idx.idx()),
                _ => None,
            }
        };
        if self.is_builtin(instance.def, "Map") {
            keyed.extend(var(0));
        }
        for index in &self.keyed[instance.def.def.idx()] {
            keyed.extend(var(*index));
        }
        for typ in &instance.subst {
            self.collect_keyed(typ, keyed);
        }
    }
}

/// Built-in types whose Rust types borrow.
const BORROWING_BUILTINS: &[&str] = &["string", "bytes", "json", "Sequence", "Map"];

/// Built-in types that can be keys of maps.
const KEY_BUILTINS: &[&str] = &[
    "string",
    "i8",
    "i16",
    "i32",
    "i64",
    "u8",
    "u16",
    "u32",
    "u64",
    "idx",
    "uuid",
    "timestamp",
    "date",
];

/// Returns the Rust type of a built-in type.
fn builtin(name: &str, args: &[TokenStream]) -> Result<TokenStream> {
    let rt = runtime();
    Ok(match name {
        "string" => quote! { #rt::types::Str<'a> },
        "bytes" => quote! { #rt::types::Bytes<'a> },
        "json" => quote! { #rt::types::Json<'a> },
        "i8" => quote! { i8 },
        "i16" => quote! { i16 },
        "i32" => quote! { i32 },
        "i64" => quote! { i64 },
        "u8" => quote! { u8 },
        "u16" => quote! { u16 },
        "u32" => quote! { u32 },
        "u64" => quote! { u64 },
        "idx" => quote! { usize },
        "f32" => quote! { f32 },
        "f64" => quote! { f64 },
        "bool" => quote! { bool },
        "uuid" => quote! { #rt::types::Uuid },
        "timestamp" => quote! { #rt::types::Timestamp },
        "date" => quote! { #rt::types::Date },
        "Sequence" => quote! { #rt::types::Seq<'a, #(#args),*> },
        "Map" => quote! { #rt::types::Map<'a, #(#args),*> },
        _ => {
            return Err(Box::new(Diagnostic::error(format!(
                "The `rust-embedded` backend does not support the built-in type `{name}`."
            ))));
        }
    })
}

/// Returns the types that a definition uses directly.
fn types_of(def: &ir::Def) -> Box<dyn Iterator<Item = &ir::Type> + '_> {
    match &def.kind {
        ir::DefKind::TypeAlias(alias) => Box::new(std::iter::once(&alias.aliased)),
        ir::DefKind::OpaqueType(_) => Box::new(std::iter::empty()),
        ir::DefKind::RecordType(record) => Box::new(record.fields.iter().map(|field| &field.typ)),
        ir::DefKind::VariantType(variant) => {
            Box::new(variant.variants.iter().filter_map(|case| case.typ.as_ref()))
        }
        ir::DefKind::WrapperType(wrapper) => Box::new(std::iter::once(&wrapper.wrapped)),
    }
}

#[cfg(test)]
mod tests {
    use super::rust_ident;

    /// Escapes keywords as raw identifiers and rejects names that cannot be
    /// identifiers.
    #[test]
    fn test_rust_ident() {
        assert_eq!(rust_ident("value").unwrap().to_string(), "value");
        assert_eq!(rust_ident("type").unwrap().to_string(), "r#type");
        for name in ["self", "Self", "crate", "super", "_", "display-name"] {
            assert!(rust_ident(name).is_err(), "{name}");
        }
    }
}
