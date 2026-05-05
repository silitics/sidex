//! Map an expression AST to backend-agnostic [`Rule`]s.
//!
//! The recognizer flattens chained comparisons into per-segment rules and
//! pattern-matches the v1 set of recognized shapes (numeric ranges,
//! length ranges, regex). Anything else collapses to [`Rule::Predicate`]
//! with the source text — codegens emit it verbatim under the `predicate`
//! error code.

use crate::ast::Accessor;
use crate::ast::CmpOp;
use crate::ast::Expr;
use crate::ast::HelperKind;
use crate::ast::Literal;
use crate::ast::Rule;

/// Validation target — informs the recognizer about the type `_` denotes.
/// Used to choose the right accessor for `_.length` (char count for
/// strings, byte count for bytes — same operator surface, different
/// runtime helpers picked by the codegen using this hint).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// `_` is a string. `_.length` → Unicode code-point count.
    String,
    /// `_` is a `bytes` value. `_.length` → byte count.
    Bytes,
    /// `_` is a numeric primitive.
    Number,
    /// `_` is a record. `_.field` accesses are resolved by the codegen.
    Record,
    /// `_` is a wrapper / opaque / something else where the validate
    /// plugin can't infer accessor semantics. The recognizer downgrades
    /// to predicate fallback.
    Other,
}

/// Result of recognizing an expression.
///
/// `rules` are recognized Rules ready for codegen. `errors` carries
/// diagnostics produced *during* recognition — currently only invalid
/// regex patterns; in future versions we may detect more shape-specific
/// problems here. When `errors` is non-empty, codegens should skip
/// emission for the offending rule and surface the error to the user;
/// `sidex check` runs the recognizer over every rule for the same effect
/// at lint time, before any codegen runs.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RecognizeOutput {
    pub rules: Vec<Rule>,
    pub errors: Vec<RecognizeError>,
}

/// One recognition error. Carries a human-readable message; the caller
/// supplies the surrounding span (the body's span, available on the IR's
/// [`TokensValue`]) when emitting a diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecognizeError {
    pub message: String,
}

/// Recognize an expression into a list of rules + errors. A chained
/// comparison like `1 <= _.length <= 254` produces two rules (one min,
/// one max); other shapes produce a single rule. Invalid regex patterns
/// surface in `errors` rather than silently producing a runtime panic.
pub fn recognize(expr: &Expr, source: &str, target: Target) -> RecognizeOutput {
    let mut errors = Vec::new();
    let rules = match expr {
        Expr::Compare { operands, ops } => recognize_compare_chain(operands, ops, source, target),
        Expr::Call { name, args } => {
            match recognize_call(name, args, &mut errors) {
                Some(rule) => vec![rule],
                None => {
                    if errors.is_empty() {
                        vec![predicate_fallback(source)]
                    } else {
                        Vec::new()
                    }
                }
            }
        }
        _ => vec![predicate_fallback(source)],
    };
    RecognizeOutput { rules, errors }
}

fn recognize_compare_chain(
    operands: &[Expr],
    ops: &[CmpOp],
    source: &str,
    target: Target,
) -> Vec<Rule> {
    debug_assert_eq!(operands.len(), ops.len() + 1);
    let mut rules = Vec::new();
    let mut all_recognized = true;
    for (idx, op) in ops.iter().enumerate() {
        let lhs = &operands[idx];
        let rhs = &operands[idx + 1];
        match recognize_compare_segment(lhs, *op, rhs, target) {
            Some(rule) => rules.push(rule),
            None => {
                all_recognized = false;
                break;
            }
        }
    }
    if all_recognized {
        rules
    } else {
        vec![predicate_fallback(source)]
    }
}

fn recognize_compare_segment(lhs: &Expr, op: CmpOp, rhs: &Expr, target: Target) -> Option<Rule> {
    // Always normalize to (accessor op threshold). When the accessor sat on
    // the right (`5 <= _`), flip the operator (`_ >= 5`) so codegens emit
    // the canonical `helper(value, bound, ...)` shape mechanically.
    let (accessor, normalized_op, threshold) = if let (Some(accessor), Some(literal)) =
        (accessor_of(lhs, target), literal_of(rhs))
    {
        (accessor, op, literal)
    } else if let (Some(literal), Some(accessor)) = (literal_of(lhs), accessor_of(rhs, target)) {
        (accessor, flip_op(op), literal)
    } else {
        return None;
    };
    let helper = helper_for(normalized_op);
    Some(Rule::Compare {
        code: code_for(&accessor, helper),
        accessor,
        helper,
        threshold,
    })
}

fn flip_op(op: CmpOp) -> CmpOp {
    match op {
        CmpOp::Lt => CmpOp::Gt,
        CmpOp::Le => CmpOp::Ge,
        CmpOp::Gt => CmpOp::Lt,
        CmpOp::Ge => CmpOp::Le,
        CmpOp::Eq => CmpOp::Eq,
        CmpOp::Ne => CmpOp::Ne,
    }
}

fn helper_for(op: CmpOp) -> HelperKind {
    match op {
        CmpOp::Lt => HelperKind::MaxExclusive,
        CmpOp::Le => HelperKind::MaxInclusive,
        CmpOp::Gt => HelperKind::MinExclusive,
        CmpOp::Ge => HelperKind::MinInclusive,
        CmpOp::Eq => HelperKind::Eq,
        CmpOp::Ne => HelperKind::Ne,
    }
}

fn recognize_call(name: &str, args: &[Expr], errors: &mut Vec<RecognizeError>) -> Option<Rule> {
    if name == "matches" && args.len() == 2 {
        let accessor = match &args[0] {
            Expr::Underscore => Some(Accessor::Self_),
            Expr::Member { path } if path.len() == 1 => Some(Accessor::Field(path[0].clone())),
            _ => None,
        }?;
        let pattern = match &args[1] {
            Expr::Lit(Literal::String(s)) => s.clone(),
            _ => return None,
        };
        // Compile the pattern at recognition time. Catches malformed
        // regex (and features unsupported by Rust's `regex` crate, e.g.
        // backreferences and lookaround) before any codegen runs. The
        // generated runtime uses the same engine, so a pattern that
        // compiles here works there too.
        if let Err(err) = ::regex::Regex::new(&pattern) {
            errors.push(RecognizeError {
                message: format!("Invalid regex pattern `{pattern}`: {err}"),
            });
            return None;
        }
        return Some(Rule::Regex { accessor, pattern });
    }
    None
}

fn accessor_of(expr: &Expr, target: Target) -> Option<Accessor> {
    match expr {
        Expr::Underscore => Some(Accessor::Self_),
        Expr::Member { path } if path.len() == 1 && path[0] == "length" => {
            match target {
                Target::String => Some(Accessor::CharCount),
                Target::Bytes => Some(Accessor::ByteCount),
                _ => None,
            }
        }
        Expr::Member { path } if path.len() == 1 => {
            // `_.<field>` makes sense on records; for non-records the
            // recognizer falls back to predicate.
            match target {
                Target::Record => Some(Accessor::Field(path[0].clone())),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Comparison thresholds in v1 must be numeric. String literals trigger a
/// fallback to predicate so codegens never have to lower string equality
/// (`_ == "hello"`) — that would tangle ownership and type inference for
/// no v1 win.
fn literal_of(expr: &Expr) -> Option<Literal> {
    match expr {
        Expr::Lit(lit @ Literal::Number(_)) => Some(lit.clone()),
        _ => None,
    }
}

fn code_for(accessor: &Accessor, helper: HelperKind) -> &'static str {
    use HelperKind::*;
    match (accessor, helper) {
        (Accessor::Self_, MinInclusive | MinExclusive) => "min",
        (Accessor::Self_, MaxInclusive | MaxExclusive) => "max",
        (Accessor::Self_, Eq) => "eq",
        (Accessor::Self_, Ne) => "ne",
        (Accessor::CharCount | Accessor::ByteCount, MinInclusive | MinExclusive) => "min_length",
        (Accessor::CharCount | Accessor::ByteCount, MaxInclusive | MaxExclusive) => "max_length",
        (Accessor::CharCount | Accessor::ByteCount, Eq) => "eq_length",
        (Accessor::CharCount | Accessor::ByteCount, Ne) => "ne_length",
        // Cross-field record accessors fall back to predicate; codegen
        // shouldn't reach here, but pick a stable code anyway.
        (Accessor::Field(_), _) => "predicate",
    }
}

fn predicate_fallback(source: &str) -> Rule {
    Rule::Predicate {
        source: source.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn rules(input: &str, target: Target) -> Vec<Rule> {
        let expr = parse(input).unwrap();
        let out = recognize(&expr, input, target);
        assert!(out.errors.is_empty(), "unexpected errors: {:?}", out.errors);
        out.rules
    }

    #[test]
    fn recognizes_inclusive_range_on_number() {
        let rs = rules("0 <= _ <= 100", Target::Number);
        assert_eq!(rs.len(), 2);
        match &rs[0] {
            Rule::Compare {
                accessor: Accessor::Self_,
                helper: HelperKind::MinInclusive,
                code,
                ..
            } => {
                assert_eq!(*code, "min");
            }
            _ => panic!("expected min compare, got {:?}", rs[0]),
        }
        match &rs[1] {
            Rule::Compare {
                accessor: Accessor::Self_,
                helper: HelperKind::MaxInclusive,
                code,
                ..
            } => {
                assert_eq!(*code, "max");
            }
            _ => panic!("expected max compare, got {:?}", rs[1]),
        }
    }

    #[test]
    fn recognizes_length_range_on_string() {
        let rs = rules("1 <= _.length <= 254", Target::String);
        assert_eq!(rs.len(), 2);
        match &rs[0] {
            Rule::Compare {
                accessor: Accessor::CharCount,
                helper: HelperKind::MinInclusive,
                code,
                ..
            } => {
                assert_eq!(*code, "min_length");
            }
            _ => panic!("expected min_length compare, got {:?}", rs[0]),
        }
        match &rs[1] {
            Rule::Compare {
                accessor: Accessor::CharCount,
                helper: HelperKind::MaxInclusive,
                code,
                ..
            } => {
                assert_eq!(*code, "max_length");
            }
            _ => panic!("expected max_length compare, got {:?}", rs[1]),
        }
    }

    #[test]
    fn recognizes_strict_inequality_on_number() {
        let rs = rules("0 < _", Target::Number);
        assert_eq!(rs.len(), 1);
        match &rs[0] {
            Rule::Compare {
                accessor: Accessor::Self_,
                helper: HelperKind::MinExclusive,
                code: "min",
                ..
            } => {}
            _ => panic!("expected min_exclusive, got {:?}", rs[0]),
        }
    }

    #[test]
    fn recognizes_regex_call() {
        let rs = rules(r#"matches(_, "^[A-Z]+$")"#, Target::String);
        assert_eq!(rs.len(), 1);
        let Rule::Regex { accessor, pattern } = &rs[0] else {
            panic!("expected regex rule, got {:?}", rs[0]);
        };
        assert_eq!(*accessor, Accessor::Self_);
        assert_eq!(pattern, "^[A-Z]+$");
    }

    #[test]
    fn cross_field_falls_back_to_predicate() {
        // `_.min <= _.max` on a record — both sides are accessors, no
        // literal threshold, so we punt to predicate.
        let rs = rules("_.min <= _.max", Target::Record);
        assert_eq!(rs.len(), 1);
        assert!(matches!(&rs[0], Rule::Predicate { .. }));
    }

    #[test]
    fn unknown_call_falls_back_to_predicate() {
        let rs = rules("unknown_fn(_)", Target::String);
        assert!(matches!(&rs[0], Rule::Predicate { .. }));
    }

    #[test]
    fn length_on_non_string_is_predicate() {
        // `_.length` on a number doesn't make sense; recognizer falls back.
        let rs = rules("0 <= _.length", Target::Number);
        assert!(matches!(&rs[0], Rule::Predicate { .. }));
    }

    #[test]
    fn equality_recognized_on_self() {
        let rs = rules("_ == 5", Target::Number);
        match &rs[0] {
            Rule::Compare {
                code: "eq",
                helper: HelperKind::Eq,
                ..
            } => {}
            _ => panic!("expected eq, got {:?}", rs[0]),
        }
    }

    /// Invalid regex patterns surface as recognition errors instead of
    /// silently emitting code that panics at first call. The caller gets
    /// to decide whether to surface as a `Diagnostic` or skip codegen.
    #[test]
    fn invalid_regex_pattern_yields_recognize_error() {
        let body = r#"matches(_, "^[A-Z+$")"#; // unclosed char class
        let expr = parse(body).unwrap();
        let out = recognize(&expr, body, Target::String);
        assert!(
            out.rules.is_empty(),
            "no rules should be emitted for invalid regex"
        );
        assert_eq!(out.errors.len(), 1);
        assert!(
            out.errors[0].message.contains("Invalid regex"),
            "expected regex error message, got `{}`",
            out.errors[0].message
        );
    }

    /// Patterns the Rust `regex` crate doesn't support (e.g. lookahead)
    /// also fail at recognition time. Cross-target consistency: if it
    /// can't compile here, it'd be silently accepted by TS but produce
    /// divergent results — better to reject it across the board.
    #[test]
    fn unsupported_regex_features_yield_recognize_error() {
        let body = r#"matches(_, "(?=lookahead)")"#;
        let expr = parse(body).unwrap();
        let out = recognize(&expr, body, Target::String);
        assert!(out.rules.is_empty());
        assert_eq!(out.errors.len(), 1);
    }
}
