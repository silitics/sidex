//! Autoformatter for Sidex schemas.
//!
//! Walks the lossless [`sidex_syntax::cst`] produced by the parser and emits
//! a normalized, comment-preserving rendering. Records and variants are
//! always expanded one item per line; attributes use width-aware reflow.
//! Imports are sorted: external bundles first (alphabetically), then internal
//! schema imports (alphabetically).

use sidex_diagnostics::DiagnosticCtx;
use sidex_ir as ir;

mod convert;
mod doc;

pub use doc::{Doc, LayoutOptions};

/// Configuration for the formatter.
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// Maximum line width before width-sensitive groups break.
    pub max_width: usize,
    /// Number of spaces per indent level.
    pub indent: usize,
    /// Names of bundles considered external (their imports are grouped first
    /// and separated from internal-schema imports by a blank line).
    ///
    /// `std` is always treated as external.
    pub external_bundles: Vec<String>,
    /// Fully-rendered import paths to drop from the output. Used by
    /// `sidex check --fix` to remove unused imports. The string format
    /// matches what the formatter would emit, e.g. `"::other::Foo"` or
    /// `"types::Color"`.
    pub excluded_imports: std::collections::HashSet<String>,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            max_width: 80,
            indent: 4,
            external_bundles: vec!["std".to_owned()],
            excluded_imports: std::collections::HashSet::new(),
        }
    }
}

/// Errors that can be returned by [`format`].
#[derive(Debug)]
pub enum FormatError {
    /// The source could not be parsed.
    Parse,
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::Parse => f.write_str("parse error"),
        }
    }
}

impl std::error::Error for FormatError {}

/// Formats `source` with default options.
pub fn format(source: &str) -> Result<String, FormatError> {
    format_with(source, &FormatOptions::default())
}

/// Formats `source` with the given options.
pub fn format_with(source: &str, opts: &FormatOptions) -> Result<String, FormatError> {
    let mut storage = ir::SourceStorage::new();
    let id = storage.insert(source.to_owned(), None);
    let ctx = DiagnosticCtx::new();
    let parsed = ctx.exec(|| sidex_syntax::parse_full(&storage[id]));
    let report = ctx.report();
    if report.has_error() {
        return Err(FormatError::Parse);
    }
    let parsed = parsed.ok_or(FormatError::Parse)?;
    Ok(convert::format_schema(&parsed.cst, source, opts))
}
