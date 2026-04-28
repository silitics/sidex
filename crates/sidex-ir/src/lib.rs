#![doc = include_str!("../README.md")]
//!
//! The root structure of the Sidex IR is [`Ir`], a flat collection of
//! [`Bundle`]s, [`Schema`]s, [`Def`]s, and [`Source`]s. References between
//! entities are typed indexes ([`BundleIdx`], [`SchemaIdx`], [`DefIdx`]) and
//! cross-bundle references are [`DefRef`] triples.
//!
//! Note that the data structures in this crate are generated with Sidex itself
//! from [`lib/meta/schemas/ir.sidex`](https://github.com/silitics/sidex/blob/main/lib/meta/schemas/ir.sidex).

use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

mod generated;

pub use generated::ir::*;

pub const STD_BUNDLE_IDX: BundleIdx = BundleIdx(0);

// --- Index <-> usize -----------------------------------------------------

macro_rules! idx_conversions {
    ($($idx:ident),*) => {
        $(
            impl From<usize> for $idx {
                fn from(idx: usize) -> Self { Self(idx) }
            }
            impl $idx {
                pub fn idx(&self) -> usize { self.0 }
            }
        )*
    };
}

idx_conversions!(SourceIdx, BundleIdx, SchemaIdx, DefIdx, TypeVarIdx);

// --- Indexing into Ir's flat arenas --------------------------------------

impl Index<BundleIdx> for Ir {
    type Output = Bundle;
    fn index(&self, idx: BundleIdx) -> &Bundle { &self.bundles[idx.0] }
}
impl IndexMut<BundleIdx> for Ir {
    fn index_mut(&mut self, idx: BundleIdx) -> &mut Bundle { &mut self.bundles[idx.0] }
}

impl Index<SchemaIdx> for Ir {
    type Output = Schema;
    fn index(&self, idx: SchemaIdx) -> &Schema { &self.schemas[idx.0] }
}
impl IndexMut<SchemaIdx> for Ir {
    fn index_mut(&mut self, idx: SchemaIdx) -> &mut Schema { &mut self.schemas[idx.0] }
}

impl Index<DefIdx> for Ir {
    type Output = Def;
    fn index(&self, idx: DefIdx) -> &Def { &self.defs[idx.0] }
}
impl IndexMut<DefIdx> for Ir {
    fn index_mut(&mut self, idx: DefIdx) -> &mut Def { &mut self.defs[idx.0] }
}

impl Index<DefRef> for Ir {
    type Output = Def;
    fn index(&self, def_ref: DefRef) -> &Def { &self.defs[def_ref.def.0] }
}
impl IndexMut<DefRef> for Ir {
    fn index_mut(&mut self, def_ref: DefRef) -> &mut Def { &mut self.defs[def_ref.def.0] }
}

impl Index<SourceIdx> for Ir {
    type Output = Source;
    fn index(&self, idx: SourceIdx) -> &Source { &self.sources[idx.0] }
}

impl Index<TypeVarIdx> for Def {
    type Output = TypeVar;
    fn index(&self, idx: TypeVarIdx) -> &TypeVar { &self.vars[idx.0] }
}
impl IndexMut<TypeVarIdx> for Def {
    fn index_mut(&mut self, idx: TypeVarIdx) -> &mut TypeVar { &mut self.vars[idx.0] }
}

// --- IR construction helpers ---------------------------------------------

impl Ir {
    /// Append a source to the IR and return its index.
    pub fn insert_source(&mut self, text: Option<String>, origin: Option<String>) -> SourceIdx {
        let idx = SourceIdx(self.sources.len());
        self.sources
            .push(Source::new().with_text(text).with_origin(origin));
        idx
    }

    /// Append a bundle to the IR and return its index.
    pub fn insert_bundle(&mut self, bundle: Bundle) -> BundleIdx {
        let idx = BundleIdx(self.bundles.len());
        self.bundles.push(bundle);
        idx
    }

    /// Append a schema to the IR and return its index. Also records the
    /// schema in its bundle's schema list.
    pub fn insert_schema(&mut self, schema: Schema) -> SchemaIdx {
        let idx = SchemaIdx(self.schemas.len());
        let bundle = schema.bundle;
        self.schemas.push(schema);
        self.bundles[bundle.0].schemas.push(idx);
        idx
    }

    /// Append a definition to the IR and return its index. Also records the
    /// definition in its schema's def list.
    pub fn insert_def(&mut self, def: Def) -> DefIdx {
        let idx = DefIdx(self.defs.len());
        let schema = def.schema;
        self.defs.push(def);
        self.schemas[schema.0].defs.push(idx);
        idx
    }

    /// Iterate the schemas belonging to `bundle`, paired with their global indices.
    pub fn schemas_of(&self, bundle: BundleIdx) -> impl Iterator<Item = (SchemaIdx, &Schema)> {
        self.bundles[bundle.0]
            .schemas
            .iter()
            .map(move |&idx| (idx, &self.schemas[idx.0]))
    }

    /// Iterate the definitions belonging to `schema`, paired with their global indices.
    pub fn defs_of(&self, schema: SchemaIdx) -> impl Iterator<Item = (DefIdx, &Def)> {
        self.schemas[schema.0]
            .defs
            .iter()
            .map(move |&idx| (idx, &self.defs[idx.0]))
    }
}

// --- Type operations -----------------------------------------------------

impl Ir {
    pub fn is_concrete(&self, typ: &Type) -> bool {
        match &typ.kind {
            TypeKind::TypeVar(_) => false,
            TypeKind::Instance(instance) => instance.subst.iter().all(|t| self.is_concrete(t)),
        }
    }

    pub fn is_alias_free(&self, typ: &Type) -> bool {
        match &typ.kind {
            TypeKind::TypeVar(_) => true,
            TypeKind::Instance(instance) => {
                let def = &self[instance.def];
                match &def.kind {
                    DefKind::TypeAlias(_) => false,
                    _ => instance.subst.iter().all(|t| self.is_alias_free(t)),
                }
            }
        }
    }

    pub fn apply_subst(&self, typ: &Type, subst: &[Type]) -> Type {
        let substitutions = subst
            .iter()
            .enumerate()
            .map(|(i, s)| (TypeVarIdx::from(i), s.clone()))
            .collect();
        typ.substitute(&substitutions)
    }

    pub fn resolve_aliases(&self, typ: &Type) -> Type {
        match &typ.kind {
            TypeKind::TypeVar(_) => typ.clone(),
            TypeKind::Instance(instance) => {
                let def = &self[instance.def];
                match &def.kind {
                    DefKind::TypeAlias(alias) => {
                        let aliased = self.apply_subst(&alias.aliased, &instance.subst);
                        self.resolve_aliases(&aliased)
                    }
                    _ => Type {
                        kind: TypeKind::Instance(InstanceType {
                            subst: instance
                                .subst
                                .iter()
                                .map(|t| self.resolve_aliases(t))
                                .collect(),
                            ..instance.clone()
                        }),
                        ..typ.clone()
                    },
                }
            }
        }
    }

    pub fn type_def(&self, typ: &Type) -> Option<&Def> {
        self.type_def_ref(typ).map(|def_ref| &self[def_ref])
    }

    pub fn type_def_ref(&self, typ: &Type) -> Option<DefRef> {
        let typ = self.resolve_aliases(typ);
        match typ.kind {
            TypeKind::TypeVar(_) => None,
            TypeKind::Instance(instance) => Some(instance.def),
        }
    }

    pub fn record_type(&self, typ: &Type) -> Option<&RecordTypeDef> {
        self.type_def(typ).and_then(|def| match &def.kind {
            DefKind::TypeAlias(alias) => self.record_type(&alias.aliased),
            DefKind::RecordType(record) => Some(record),
            _ => None,
        })
    }
}

impl Type {
    pub fn substitute(&self, substitutions: &HashMap<TypeVarIdx, Type>) -> Type {
        match &self.kind {
            TypeKind::TypeVar(var) => substitutions.get(&var.idx).unwrap_or(self).clone(),
            TypeKind::Instance(instance) => Type {
                kind: TypeKind::Instance(InstanceType {
                    subst: instance
                        .subst
                        .iter()
                        .map(|t| t.substitute(substitutions))
                        .collect(),
                    ..instance.clone()
                }),
                ..self.clone()
            },
        }
    }
}

// --- Source helpers ------------------------------------------------------

impl Source {
    /// Span pointing at the position past the last character of the source.
    pub fn end_span(&self, idx: SourceIdx) -> Span {
        let pos = self.text.as_ref().map(|t| t.chars().count()).unwrap_or(0);
        Span::new(idx, pos, pos + 1)
    }

    /// One-character span at the given character offset.
    pub fn span_at(&self, idx: SourceIdx, pos: usize) -> Span {
        Span::new(idx, pos, pos + 1)
    }
}

impl Ident {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

impl Docs {
    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

// --- Spanned -------------------------------------------------------------

pub trait Spanned {
    fn span(&self) -> Option<Span>;
}

impl Spanned for Span {
    fn span(&self) -> Option<Span> {
        Some(self.clone())
    }
}

impl Spanned for Attr {
    fn span(&self) -> Option<Span> {
        self.span.clone()
    }
}

// --- AttrValue accessors -------------------------------------------------

impl AttrValue {
    /// Returns the contained string if this is a [`AttrValue::String`].
    pub fn as_string(&self) -> Option<&str> {
        match self {
            AttrValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the contained path if this is a [`AttrValue::Path`].
    pub fn as_path(&self) -> Option<&str> {
        match self {
            AttrValue::Path(p) => Some(p),
            _ => None,
        }
    }

    /// Returns the contained boolean if this is a [`AttrValue::Bool`].
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AttrValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the contained number (as a string) if this is a [`AttrValue::Number`].
    pub fn as_number(&self) -> Option<&str> {
        match self {
            AttrValue::Number(n) => Some(n),
            _ => None,
        }
    }
}

// --- Display for attributes ---------------------------------------------

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AttrKind::Path(path) => f.write_str(path),
            AttrKind::List(list) => {
                f.write_str(&list.path)?;
                f.write_str("(")?;
                for (idx, arg) in list.args.iter().enumerate() {
                    if idx != 0 {
                        f.write_str(", ")?;
                    }
                    std::fmt::Display::fmt(arg, f)?;
                }
                f.write_str(")")
            }
            AttrKind::Assign(assign) => {
                f.write_str(&assign.path)?;
                f.write_str(" = ")?;
                std::fmt::Display::fmt(&assign.value, f)
            }
        }
    }
}

impl std::fmt::Display for AttrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrValue::Bool(b) => write!(f, "{}", b),
            AttrValue::Number(n) => f.write_str(n),
            AttrValue::String(s) => write!(f, "{:?}", s),
            AttrValue::Path(p) => f.write_str(p),
        }
    }
}
