#![doc = include_str!("../README.md")]
//!
//! The root structure of the IR is a transformation unit ([`Unit`]) consisting of
//! multiple bundles ([`Bundle`]). These bundles, in turn, consist of multiple schemas
//! ([`Schema`]) containing the actual type and service definitions ([`Def`]).
//!
//! A code generator usually takes a [`Unit`] and generates code for a bundle identified
//! by a given [`BundleIdx`].

// Re-export the IR from `sidex_core`.
//
// In the future, the IR will be defined here together with an instance of the builder
// pattern for constructing it. In `sidex_core` we will then use this builder. For now,
// however, we define the IR in `sidex_core` itself which enables a direct construction
// without the necessity for a stable builder interface.
pub use sidex_core::ir::*;
