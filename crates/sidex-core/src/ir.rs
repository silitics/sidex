//! The _Sidex Intermediate Representation_ (SIR) used for code generation.
//!
//! This module re-exports the [`sidex-ir`](https://docs.rs/sidex-ir/latest/sidex-ir/) crate.
//!
//! ```plain
//! pub use sidex_ir::*;
//! ```

// This crate will eventually re-export the `sidex-ir` crate. For now, however, we define
// the IR here because it enables a direct construction.

// === ⚠️ CAUTION ⚠️ ===
// This module must not use anything else defined in this crate because we want to isolate
// it from `sidex_core` and put everything into `sidex-ir` later.
// === ⚠️ CAUTION ⚠️ ===

use std::ops::{Index, IndexMut};

#[doc(hidden)]
mod generated;

// Do not document the IR here.
#[doc(hidden)]
pub use generated::reflect::*;

impl Unit {
    /// Create a new empty unit.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Metadata {
    /// Create a new metadata struct with the given name and version.
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: Default::default(),
            authors: Default::default(),
        }
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
