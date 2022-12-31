#![doc = include_str!("../README.md")]
//!
//! The root structure of the SIR is a collection ([`Collection`]) of bundles
//! ([`Bundle`]). These bundles, in turn, consist of multiple schemas ([`Schema`])
//! containing the actual type and service definitions ([`Def`]).
//!
//! A code generator usually takes a [`Collection`] and generates code for a bundle
//! identified by a given [`BundleIdx`].
//!
//! Note that the data structures in this crate have been generated with Sidex itself from
//! the [`reflect`](https://github.com/silitics/sidex/blob/main/lib/meta/schemas/reflect.sidex)
//! schema.

use std::{
    collections::HashMap,
    fmt::Write,
    ops::{Index, IndexMut},
};

mod generated;

pub use generated::ir::*;

impl From<usize> for BundleIdx {
    fn from(idx: usize) -> Self {
        Self(idx)
    }
}

impl Index<BundleIdx> for Unit {
    type Output = Bundle;

    fn index(&self, index: BundleIdx) -> &Self::Output {
        &self.bundles[index.0]
    }
}

impl IndexMut<BundleIdx> for Unit {
    fn index_mut(&mut self, index: BundleIdx) -> &mut Self::Output {
        &mut self.bundles[index.0]
    }
}

impl Index<InstanceType> for Unit {
    type Output = Def;

    fn index(&self, index: InstanceType) -> &Self::Output {
        &self[index.bundle][index.schema][index.def]
    }
}

impl IndexMut<InstanceType> for Unit {
    fn index_mut(&mut self, index: InstanceType) -> &mut Self::Output {
        &mut self[index.bundle][index.schema][index.def]
    }
}

impl From<usize> for SchemaIdx {
    fn from(idx: usize) -> Self {
        Self(idx)
    }
}

impl Index<SchemaIdx> for Bundle {
    type Output = Schema;

    fn index(&self, index: SchemaIdx) -> &Self::Output {
        &self.schemas[index.0]
    }
}

impl IndexMut<SchemaIdx> for Bundle {
    fn index_mut(&mut self, index: SchemaIdx) -> &mut Self::Output {
        &mut self.schemas[index.0]
    }
}

impl From<usize> for DefIdx {
    fn from(idx: usize) -> Self {
        Self(idx)
    }
}

impl Index<DefIdx> for Schema {
    type Output = Def;

    fn index(&self, index: DefIdx) -> &Self::Output {
        &self.defs[index.0]
    }
}

impl IndexMut<DefIdx> for Schema {
    fn index_mut(&mut self, index: DefIdx) -> &mut Self::Output {
        &mut self.defs[index.0]
    }
}

impl From<usize> for TypeVarIdx {
    fn from(idx: usize) -> Self {
        Self(idx)
    }
}

impl Index<TypeVarIdx> for Def {
    type Output = TypeVar;

    fn index(&self, index: TypeVarIdx) -> &Self::Output {
        &self.vars[index.0]
    }
}

impl IndexMut<TypeVarIdx> for Def {
    fn index_mut(&mut self, index: TypeVarIdx) -> &mut Self::Output {
        &mut self.vars[index.0]
    }
}

impl From<usize> for SourceIdx {
    fn from(idx: usize) -> Self {
        Self(idx)
    }
}

impl From<String> for Path {
    fn from(path: String) -> Self {
        Self(path)
    }
}

impl Path {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

pub const STD_BUNDLE_IDX: BundleIdx = BundleIdx(0);

impl BundleIdx {
    pub fn idx(&self) -> usize {
        self.0
    }
}

impl SchemaIdx {
    pub fn idx(&self) -> usize {
        self.0
    }
}

impl DefIdx {
    pub fn idx(&self) -> usize {
        self.0
    }
}

impl SourceIdx {
    pub fn idx(&self) -> usize {
        self.0
    }
}

impl TypeVarIdx {
    pub fn idx(&self) -> usize {
        self.0
    }
}

impl From<Vec<Token>> for TokenStream {
    fn from(tokens: Vec<Token>) -> Self {
        Self(tokens)
    }
}

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            AttrKind::Path(path) => f.write_str(path.as_str()),
            AttrKind::List(list) => {
                f.write_str(list.path.as_str())?;
                f.write_char('(')?;
                for idx in 0..list.args.len() {
                    if idx != 0 {
                        f.write_str(", ")?;
                    }
                    std::fmt::Display::fmt(&list.args[idx], f)?;
                }
                f.write_char(')')
            }
            AttrKind::Assign(assign) => {
                f.write_str(assign.path.as_str())?;
                f.write_str(" = ")?;
                std::fmt::Display::fmt(&assign.value, f)
            }
            AttrKind::Tokens(tokens) => {
                for token in tokens.iter() {
                    match &token.kind {
                        TokenKind::Punctuation(token)
                        | TokenKind::Delimiter(token)
                        | TokenKind::Identifier(token) => {
                            f.write_str(token)?;
                        }
                        TokenKind::Literal(literal) => {
                            match literal {
                                Literal::String(string) => {
                                    f.write_fmt(format_args!("{:?}", string))?;
                                }
                                Literal::Number(string) => f.write_str(string)?,
                                Literal::Bool(boolean) => {
                                    f.write_str(if *boolean { "true" } else { "false" })?
                                }
                            }
                        }
                    }
                    f.write_char(' ')?;
                }
                Ok(())
            }
        }
    }
}

impl SourceStorage {
    /// Inserts a *source* into the storage.
    pub fn insert(&mut self, text: String, origin: Option<String>) -> SourceIdx {
        self.insert_with(|idx| Source::new(idx).with_text(Some(text)).with_origin(origin))
    }
    pub fn insert_with<F: FnOnce(SourceIdx) -> Source>(&mut self, make: F) -> SourceIdx {
        let idx = SourceIdx::from(self.sources.len());
        self.sources.push(make(idx));
        idx
    }
}

impl Index<SourceIdx> for SourceStorage {
    type Output = Source;

    fn index(&self, index: SourceIdx) -> &Self::Output {
        &self.sources[index.0]
    }
}

impl Source {
    /// The end of the source.
    pub fn end(&self) -> Span {
        self.span_at(
            self.text
                .as_ref()
                .map(|text| text.chars().count())
                .unwrap_or(0),
        )
    }

    /// Create a span at the given position.
    pub fn span_at(&self, pos: usize) -> Span {
        Span::new(self.idx, pos, pos + 1)
    }
}

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

impl Identifier {
    pub fn as_str(&self) -> &str {
        self.identifier.as_str()
    }
}

impl Docs {
    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

impl Type {
    pub fn substitute(&self, substitutions: &HashMap<TypeVarIdx, Type>) -> Type {
        match &self.kind {
            TypeKind::TypeVar(var) => substitutions.get(&var.idx).unwrap_or(self).clone(),
            TypeKind::Instance(instance) => {
                Type {
                    kind: TypeKind::Instance(InstanceType {
                        subst: instance
                            .subst
                            .iter()
                            .map(|typ| typ.substitute(substitutions))
                            .collect(),
                        ..instance.clone()
                    }),
                    ..self.clone()
                }
            }
        }
    }
}

impl Unit {
    pub fn is_concrete(&self, typ: &Type) -> bool {
        match &typ.kind {
            TypeKind::TypeVar(_) => false,
            TypeKind::Instance(instance) => instance.subst.iter().all(|typ| self.is_concrete(typ)),
        }
    }

    pub fn is_alias_free(&self, typ: &Type) -> bool {
        match &typ.kind {
            TypeKind::TypeVar(_) => true,
            TypeKind::Instance(instance) => {
                let def = &self[instance.bundle][instance.schema][instance.def];
                match &def.kind {
                    DefKind::TypeAlias(_) => false,
                    _ => instance.subst.iter().all(|typ| self.is_alias_free(typ)),
                }
            }
        }
    }

    pub fn apply_subst(&self, typ: &Type, subst: &[Type]) -> Type {
        let substitutions = subst
            .iter()
            .enumerate()
            .map(|(idx, subst)| (TypeVarIdx::from(idx), subst.clone()))
            .collect();
        typ.substitute(&substitutions)
    }

    pub fn resolve_aliases(&self, typ: &Type) -> Type {
        match &typ.kind {
            TypeKind::TypeVar(_) => typ.clone(),
            TypeKind::Instance(instance) => {
                let def = &self[instance.bundle][instance.schema][instance.def];
                match &def.kind {
                    DefKind::TypeAlias(alias) => {
                        let aliased = self.apply_subst(&alias.aliased, &instance.subst);
                        self.resolve_aliases(&aliased)
                    }
                    _ => {
                        // Recursively resolve aliases.
                        Type {
                            kind: TypeKind::Instance(InstanceType {
                                subst: instance
                                    .subst
                                    .iter()
                                    .map(|typ| self.resolve_aliases(typ))
                                    .collect(),
                                ..instance.clone()
                            }),
                            ..typ.clone()
                        }
                    }
                }
            }
        }
    }

    pub fn type_def(&self, typ: &Type) -> Option<&Def> {
        self.type_def_ref(typ).map(|def_ref| &self[def_ref])
    }

    pub fn type_def_ref(&self, typ: &Type) -> Option<DefRef> {
        let typ = self.resolve_aliases(typ);
        match &typ.kind {
            TypeKind::TypeVar(_) => None,
            TypeKind::Instance(instance) => {
                Some(DefRef {
                    schema: SchemaRef {
                        bundle: instance.bundle,
                        schema: instance.schema,
                    },
                    def: instance.def,
                })
            }
        }
    }

    pub fn record_type(&self, typ: &Type) -> Option<&RecordTypeDef> {
        self.type_def(typ).and_then(|def| {
            match &def.kind {
                DefKind::TypeAlias(alias) => self.record_type(&alias.aliased),
                DefKind::RecordType(record) => Some(record),
                _ => None,
            }
        })
    }

    pub fn resolve_path(&self, root: ItemRef, path: &Path) -> Result<ItemRef, String> {
        let mut item = root;
        for segment in path.segments() {
            match &item {
                ItemRef::Def(_) => return Err("Definition has no items.".to_owned()),
                ItemRef::Schema(schema_ref) => {
                    let schema = &self[schema_ref.bundle][schema_ref.schema];
                    item = schema
                        .resolve_name(segment)
                        .ok_or_else(|| "Unable to resolve name.".to_owned())?;
                }
                ItemRef::Bundle(bundle_ref) => {
                    let bundle = &self[*bundle_ref];
                    item = bundle
                        .resolve_name(segment)
                        .ok_or_else(|| "Unable to resolve name.".to_owned())?;
                }
            }
        }
        Ok(item)
    }
}

impl Path {
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.0.split("::")
    }
}

impl Bundle {
    pub fn resolve_name(&self, name: &str) -> Option<ItemRef> {
        for schema in self.schemas.iter() {
            if schema.name.as_str() == name {
                return Some(ItemRef::Schema(schema.schema_ref()));
            }
        }
        None
    }
}

impl Schema {
    pub fn schema_ref(&self) -> SchemaRef {
        SchemaRef {
            bundle: self.bundle,
            schema: self.idx,
        }
    }

    pub fn resolve_name(&self, name: &str) -> Option<ItemRef> {
        for (def_idx, def) in self.defs.iter().enumerate() {
            if def.name.as_str() == name {
                return Some(ItemRef::Def(DefRef {
                    schema: self.schema_ref(),
                    def: DefIdx(def_idx),
                }));
            }
        }
        self.imports.get(name).cloned()
    }
}

impl Index<SchemaRef> for Unit {
    type Output = Schema;

    fn index(&self, index: SchemaRef) -> &Self::Output {
        &self[index.bundle][index.schema]
    }
}

impl Index<DefRef> for Unit {
    type Output = Def;

    fn index(&self, index: DefRef) -> &Self::Output {
        &self[index.schema][index.def]
    }
}
