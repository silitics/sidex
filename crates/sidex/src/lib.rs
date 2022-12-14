#![doc = include_str!("../README.md")]

#[cfg(feature = "macros")]
pub use sidex_macros::*;
#[cfg(feature = "serde")]
pub use sidex_serde as serde;
