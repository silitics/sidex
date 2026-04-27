#![doc = include_str!("../README.md")]

pub mod ast;
pub mod cst;
pub mod tokens;

pub(crate) mod lexer;
mod parser;

pub use parser::{Parsed, parse, parse_full};
