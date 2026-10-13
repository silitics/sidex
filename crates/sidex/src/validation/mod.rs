//! Validation rules and their evaluation.
//!
//! Schemas attach rules to definitions and fields with `#[validate(...)]`, for
//! instance:
//!
//! ```sidex
//! #[validate({ _.start <= _.end }, message = "The window must not end before it starts.")]
//! record Window {
//!     start: u32,
//!     end: u32,
//!     #[validate({ _.size <= 64 })]
//!     #[validate(matches(_, "^[a-z-]+$"))]
//!     name: string,
//! }
//! ```
//!
//! Rules of a definition apply to its values, so rules of a record can compare
//! its fields. Rules of a field apply to the field's value.
//!
//! Shapes carry rules as [`Rule`]s (see [`Shape::rules`] and
//! [`FieldShape::rules`]).
//!
//! A rule is an expression over `_`, the value being validated:
//!
//! - A comparison, possibly chained like `1 <= _ <= 100`, holds if each comparison of
//!   adjacent operands holds. Operands are `_`, paths into the value like `_.start`, and
//!   number and string literals. Numbers compare numerically and strings
//!   lexicographically. On strings, bytes, sequences, and maps, `_.size` is the number of
//!   characters, bytes, elements, and entries.
//! - `matches(operand, "pattern")` holds if the operand is a string containing a match of
//!   the regular expression.
//!
//! A rule whose path leads to an absent optional field holds. Rules of an
//! optional field only apply if the field is present.
//!
//! [`validate`] checks every rule of a value and its nested values. Each
//! violated rule yields an error with the path to the value, a code, and a
//! message. Rules may set both. Otherwise, the code depends on the rule:
//!
//! | Rule | Code |
//! | --- | --- |
//! | `_` compared with a literal by `>` or `>=` | `min` |
//! | `_` compared with a literal by `<` or `<=` | `max` |
//! | `_` compared with a literal by `==` or `!=` | `eq` or `ne` |
//! | `_.size` compared with a literal | `min_size`, `max_size`, `eq_size`, or `ne_size` |
//! | `matches` | `regex` |
//! | any other comparison | `invalid` |
//!
//! Evaluation requires the `validation` feature, which is enabled by default.
//!
//! [`Shape::rules`]: crate::Shape::rules
//! [`FieldShape::rules`]: crate::shape::FieldShape::rules

#[cfg(feature = "validation")]
pub use self::evaluate::validate;
#[cfg(feature = "validation")]
pub use self::report::ValidationError;
#[cfg(feature = "validation")]
pub use self::report::ValidationReport;
pub use self::rule::CmpOp;
pub use self::rule::Expr;
pub use self::rule::Operand;
pub use self::rule::Rule;

#[cfg(feature = "validation")]
mod evaluate;
#[cfg(feature = "validation")]
mod report;
mod rule;
