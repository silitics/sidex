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

use std::ops::{Index, IndexMut};

mod generated;

pub use generated::reflect::*;

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
