//! Expression AST and the recognizer's typed rule output.

use std::fmt;

/// Comparison operators in the v1 grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmpOp {
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,
}

impl CmpOp {
    pub fn as_str(self) -> &'static str {
        match self {
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Eq => "==",
            CmpOp::Ne => "!=",
            CmpOp::Ge => ">=",
            CmpOp::Gt => ">",
        }
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A leaf literal in an expression. Numbers stay as strings to preserve the
/// user's spelling and avoid lossy parsing — codegens emit them verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Number(String),
    String(String),
}

/// The expression AST. Lossless w.r.t. the grammar; the recognizer narrows
/// shapes into typed [`Rule`]s for codegen.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// The placeholder value — `_`.
    Underscore,
    /// `_.<a>.<b>...` — accessor chain rooted at `_`.
    Member { path: Vec<String> },
    /// A literal leaf.
    Lit(Literal),
    /// `name(arg1, arg2, ...)`. Arguments are themselves expressions.
    Call { name: String, args: Vec<Expr> },
    /// `a cmp_op b cmp_op c ...`. Always at least two operands and one op.
    Compare {
        operands: Vec<Expr>,
        ops: Vec<CmpOp>,
    },
}

/// Recognized shapes — the union of patterns the v1 recognizer maps to
/// specific runtime helpers. Anything that doesn't match a recognized
/// shape becomes [`Rule::Predicate`] (codegens emit it as a verbatim
/// boolean expression with `code: "predicate"`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// `n <= _` / `_ <= n` / `_.length <= n` etc. Already normalized to
    /// "accessor op threshold" — the recognizer flips operands when
    /// needed so codegens can lower mechanically.
    Compare {
        accessor: Accessor,
        helper: HelperKind,
        threshold: Literal,
        /// Stable error code derived from the helper and accessor.
        code: &'static str,
    },
    /// `matches(_, "<pattern>")`.
    Regex { accessor: Accessor, pattern: String },
    /// Fallback — the expression is emitted verbatim and reported as
    /// `code: "predicate"` if it evaluates false.
    Predicate { source: String },
}

/// Which runtime comparison helper a [`Rule::Compare`] lowers to. Codegens
/// translate this 1:1 into a function name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelperKind {
    MinInclusive,
    MinExclusive,
    MaxInclusive,
    MaxExclusive,
    Eq,
    Ne,
}

/// What the rule operates on. All v1 accessors are pure functions of the
/// validated value; codegens emit them as either a direct reference (`Self_`,
/// `Field`) or a runtime call (`CharCount`, `ByteCount`, `ItemCount`,
/// `EntryCount`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Accessor {
    /// `_` itself.
    Self_,
    /// `_.size` on a string — Unicode code-point count.
    CharCount,
    /// `_.size` on `bytes` — byte count.
    ByteCount,
    /// `_.size` on a sequence — element count.
    ItemCount,
    /// `_.size` on a map — entry count.
    EntryCount,
    /// `_.<field>` — record field access. Multi-segment paths
    /// (`_.a.b.c`) collapse to a dotted string.
    Field(String),
}
