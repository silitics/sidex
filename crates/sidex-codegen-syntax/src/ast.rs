/// A parsed code template.
#[derive(Debug, Clone)]
pub struct Template {
    pub fragments: Vec<Fragment>,
}

/// A fragment of a parsed template.
#[derive(Debug, Clone)]
pub enum Fragment {
    /// Literal text, emitted verbatim.
    Literal(String),
    /// A newline in the template.
    Newline,
    /// Interpolation of a single variable: `@var`.
    Interpolation { var: String, column: usize },
    /// Iteration block: `@(...)*` or `@(...)+`.
    Iteration {
        body: Vec<Fragment>,
        mode: IterMode,
        separator: String,
        column: usize,
    },
}

/// Iteration mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterMode {
    /// One item per line, indented to column.
    Vertical,
    /// Inline, joined by separator.
    Horizontal,
}
