#![doc = include_str!("../README.md")]

/// Re-export the _Sidex Intermediate Representation_ (SIR).
pub use sidex_ir as ir;

pub mod model;
pub mod transformer;
