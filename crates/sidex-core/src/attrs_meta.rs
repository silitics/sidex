//! Discovery of typed attribute schemas (`#[attrs(plugin, target)]`-marked defs)
//! and the resulting plugin registry consumed by the typed-attrs parser.
//!
//! At IR build time the compiler walks every loaded definition, reads its
//! `#[attrs(...)]` meta-attribute (if any), and records the def in a
//! `(plugin, target) -> DefRef` registry. The typed-attrs parser later uses
//! this registry to route each source attribute to the correct schema.
//!
//! `AttrTarget` mirrors `core::attrs::AttrTarget`. Since the meta-attrs are
//! parsed by the compiler before any typed-attrs machinery exists, the
//! parsing of `#[attrs(...)]` itself is hand-coded here — a small bootstrap.

use std::collections::HashMap;

use sidex_ir as ir;

/// IR positions that a plugin's attribute schema can target. Mirrors the
/// `core::attrs::AttrTarget` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttrTarget {
    Schema,
    Def,
    Alias,
    Opaque,
    Record,
    Variant,
    Wrapper,
    Field,
    VariantCase,
}

impl AttrTarget {
    fn from_path(path: &str) -> Option<Self> {
        match path {
            "Schema" | "schema" => Some(Self::Schema),
            "Def" | "def" => Some(Self::Def),
            "Alias" | "alias" => Some(Self::Alias),
            "Opaque" | "opaque" => Some(Self::Opaque),
            "Record" | "record" => Some(Self::Record),
            "Variant" | "variant" => Some(Self::Variant),
            "Wrapper" | "wrapper" => Some(Self::Wrapper),
            "Field" | "field" => Some(Self::Field),
            "VariantCase" | "variant_case" => Some(Self::VariantCase),
            _ => None,
        }
    }
}

/// The parsed contents of a `#[attrs(plugin = ..., target = ...)]` annotation.
#[derive(Debug, Clone)]
pub struct AttrsMeta {
    pub plugin: String,
    pub target: AttrTarget,
}

/// Extract the `#[attrs(...)]` meta-attribute from a list of attributes.
///
/// Returns `None` if no `attrs` outer list is present, or if the list is
/// missing required fields. This is intentionally lenient at the discovery
/// stage; the typed-attrs parser will emit precise diagnostics later when
/// it tries to use a malformed registry entry.
pub fn extract_attrs_meta(attrs: &[ir::Attr]) -> Option<AttrsMeta> {
    for attr in attrs {
        let ir::AttrKind::List(list) = &attr.kind else {
            continue;
        };
        if list.path != "attrs" {
            continue;
        }
        let mut plugin: Option<String> = None;
        let mut target: Option<AttrTarget> = None;
        for arg in &list.args {
            let ir::AttrKind::Assign(assign) = &arg.kind else {
                continue;
            };
            match assign.path.as_str() {
                "plugin" => {
                    if let ir::AttrValue::String(s) = &assign.value {
                        plugin = Some(s.clone());
                    }
                }
                "target" => {
                    if let ir::AttrValue::Path(p) = &assign.value {
                        target = AttrTarget::from_path(p);
                    }
                }
                _ => {}
            }
        }
        if let (Some(plugin), Some(target)) = (plugin, target) {
            return Some(AttrsMeta { plugin, target });
        }
    }
    None
}

/// Registry mapping a plugin's typed-attrs schema for each IR position.
///
/// Built once after the IR is assembled; the typed-attrs parser consults it
/// to find the schema def for `(plugin, target)` while processing source
/// attributes.
#[derive(Debug, Default, Clone)]
pub struct PluginRegistry {
    schemas: HashMap<(String, AttrTarget), ir::DefRef>,
}

impl PluginRegistry {
    /// Build a registry by walking every def in `ir` for `#[attrs(...)]`.
    pub fn build(ir: &ir::Ir) -> Self {
        let mut schemas: HashMap<(String, AttrTarget), ir::DefRef> = HashMap::new();
        for (idx, def) in ir.defs.iter().enumerate() {
            let Some(meta) = extract_attrs_meta(&def.attrs) else {
                continue;
            };
            let schema = &ir.schemas[def.schema.idx()];
            let def_ref = ir::DefRef::new(schema.bundle, def.schema, ir::DefIdx::from(idx));
            schemas.insert((meta.plugin, meta.target), def_ref);
        }
        Self { schemas }
    }

    /// Look up the schema def for a `(plugin, target)` pair.
    pub fn get(&self, plugin: &str, target: AttrTarget) -> Option<ir::DefRef> {
        self.schemas.get(&(plugin.to_owned(), target)).copied()
    }

    /// Iterate over every registered `(plugin, target, def_ref)` triple.
    pub fn iter(&self) -> impl Iterator<Item = (&str, AttrTarget, ir::DefRef)> {
        self.schemas
            .iter()
            .map(|((plugin, target), def_ref)| (plugin.as_str(), *target, *def_ref))
    }

    /// Number of registered plugin schemas.
    pub fn len(&self) -> usize {
        self.schemas.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.schemas.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformer::Transformer;

    /// The auto-loaded plugin attribute bundles (currently `py`) register
    /// themselves under their plugin name and target via the `#[attrs(...)]`
    /// meta attribute. A fresh transformer with no user bundle should
    /// already have these in the registry — no user import required.
    #[test]
    fn registry_finds_attrs_marked_defs() {
        let transformer = Transformer::new();
        let ir = transformer.transform(ir::STD_BUNDLE_IDX);
        let registry = PluginRegistry::build(&ir);
        assert!(
            registry.get("py", AttrTarget::Opaque).is_some(),
            "py-attrs bundle should register an opaque-target schema"
        );
    }
}
