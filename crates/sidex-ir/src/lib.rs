#![doc = include_str!("../README.md")]

use std::ops::{Index, IndexMut};

mod generated;

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

impl Index<ModelIdx> for Unit {
    type Output = Model;

    fn index(&self, index: ModelIdx) -> &Self::Output {
        &self.models[index.0]
    }
}

impl IndexMut<ModelIdx> for Unit {
    fn index_mut(&mut self, index: ModelIdx) -> &mut Self::Output {
        &mut self.models[index.0]
    }
}

impl Index<InstanceType> for Unit {
    type Output = Def;

    fn index(&self, index: InstanceType) -> &Self::Output {
        &self[index.model][index.schema][index.def]
    }
}

impl IndexMut<InstanceType> for Unit {
    fn index_mut(&mut self, index: InstanceType) -> &mut Self::Output {
        &mut self[index.model][index.schema][index.def]
    }
}

impl Index<SchemaIdx> for Model {
    type Output = Schema;

    fn index(&self, index: SchemaIdx) -> &Self::Output {
        &self.schemas[index.0]
    }
}

impl IndexMut<SchemaIdx> for Model {
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
