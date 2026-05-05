//! Expression grammar + recognizer for Sidex's validation extension.
//!
//! This crate is target-agnostic: it parses the body text of a
//! `#[validate(expr = { ... })]` attribute into an expression AST and
//! recognizes the v1 set of common shapes (numeric ranges, length ranges,
//! regex matches) into a small typed [`Rule`] enum that codegens lower
//! into per-target validator calls.
//!
//! The plugin owns its lexer/parser entirely — the Sidex compiler core
//! captures the body verbatim and hands it off as text. See
//! `crates/sidex-validate-expr/src/lib.rs` for the grammar.
//!
//! ## Grammar (v1)
//!
//! ```text
//! expr     = primary (cmp_op primary)+        // chained comparison
//!          | call                              // top-level predicate call
//!          | primary                           // bare value (rare)
//! primary  = "_" ("." ident)*
//!          | number
//!          | string
//!          | call
//! call     = ident "(" arg ("," arg)* ")"
//! arg      = primary | string
//! cmp_op   = "<" | "<=" | "==" | "!=" | ">=" | ">"
//! ```
//!
//! No `&&`, `||`, `!` — multiple `#[validate(expr = { ... })]` attributes
//! accumulate. No arithmetic. Functions are looked up against a small
//! built-in table; v1 ships only `matches(_, "<pattern>")`.

#![forbid(unsafe_code)]

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod recognizer;

pub use ast::Accessor;
pub use ast::CmpOp;
pub use ast::Expr;
pub use ast::HelperKind;
pub use ast::Literal;
pub use ast::Rule;
pub use parser::ParseError;
pub use parser::parse;
pub use recognizer::RecognizeError;
pub use recognizer::RecognizeOutput;
pub use recognizer::Target;
pub use recognizer::recognize;
