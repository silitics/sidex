//! Error types produced by the lexer and parser.

use crate::source::Span;

/// A syntax error.
#[derive(Debug, Clone)]
pub struct SyntaxError(pub(crate) chumsky::error::Simple<String, Span>);
