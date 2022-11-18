#![doc = include_str!("../README.md")]

pub use sidex_diagnostics as diagnostics;
pub use sidex_ir as ir;

pub mod builtins;
pub mod bundle;
pub mod transformer;
pub mod utils;
