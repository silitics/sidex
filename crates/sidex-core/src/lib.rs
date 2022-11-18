#![doc = include_str!("../README.md")]

pub use sidex_ir as ir;

pub mod builtins;
pub mod bundle;
pub mod diagnostics;
pub mod transformer;
pub mod utils;
