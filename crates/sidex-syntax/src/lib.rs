#![doc = include_str!("../README.md")]

mod parser;

pub mod ast;
pub mod errors;
pub mod reporting;
pub mod source;
pub mod tokens;

pub use parser::parse;
