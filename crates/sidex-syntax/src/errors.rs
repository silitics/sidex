//! Error types produced by the lexer and parser.

use thiserror::Error;

use crate::source::Span;

/// A syntax error.
#[derive(Debug, Clone, Error)]
#[error("{0}")]
pub struct SyntaxError(pub(crate) chumsky::error::Simple<String, Span>);
