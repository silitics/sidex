//! Rules as static data.

/// Validation rule.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Rule {
    /// Source of the expression as written in the schema.
    pub source: &'static str,
    /// Expression that must hold.
    pub expr: Expr,
    /// Message replacing the default message of a violation.
    pub message: Option<&'static str>,
    /// Code replacing the default code of a violation.
    pub code: Option<&'static str>,
}

impl Rule {
    /// Creates a rule from an expression and its source.
    pub const fn new(source: &'static str, expr: Expr) -> Self {
        Self {
            source,
            expr,
            message: None,
            code: None,
        }
    }

    /// Sets the message of violations.
    pub const fn with_message(self, message: &'static str) -> Self {
        Self {
            message: Some(message),
            ..self
        }
    }

    /// Sets the code of violations.
    pub const fn with_code(self, code: &'static str) -> Self {
        Self {
            code: Some(code),
            ..self
        }
    }
}

/// Expression of a [`Rule`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Expr {
    /// Chained comparison, which holds if each comparison of adjacent operands
    /// holds.
    ///
    /// There is one more operand than there are operators.
    Compare {
        /// Operands from left to right.
        operands: &'static [Operand],
        /// Operators between the operands.
        ops: &'static [CmpOp],
    },
    /// Holds if the operand is a string containing a match of the pattern.
    Matches {
        /// String to match.
        operand: Operand,
        /// Regular expression.
        pattern: &'static str,
    },
}

/// Operand of an [`Expr`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand {
    /// The value being validated, `_`.
    Value,
    /// Path into the value being validated, for instance, `_.start` as
    /// `&["start"]`.
    Member(&'static [&'static str]),
    /// Integer literal.
    Int(i128),
    /// Floating-point literal.
    Float(f64),
    /// String literal.
    String(&'static str),
}

/// Comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `>=`
    Ge,
    /// `>`
    Gt,
}
