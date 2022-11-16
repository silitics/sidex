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

// Re-export the `ir` module from `sidex_core`.
//
// In the future, the SIR will be defined here together with an instance of the builder
// pattern for constructing it. In `sidex_core` we will then use this builder. For now,
// however, we define the SIR in `sidex_core` itself which enables a direct construction
// without the necessity for a stable builder interface.
pub use sidex_core::ir::*;
