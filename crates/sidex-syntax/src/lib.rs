#![doc = include_str!("../README.md")]

mod parser;

pub(crate) mod span;

pub mod ast;
pub mod tokens;

pub use parser::parse;
