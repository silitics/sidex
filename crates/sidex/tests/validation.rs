//! Validating values against the rules of their shapes.

mod common;

use common::record;
use sidex::validation::CmpOp;
use sidex::validation::Expr;
use sidex::validation::Operand;
use sidex::validation::Rule;
use sidex::validation::validate;

const POSITIVE: Rule = Rule::new(
    "_ >= 1",
    Expr::Compare {
        operands: &[Operand::Value, Operand::Int(1)],
        ops: &[CmpOp::Ge],
    },
)
.with_message("Timeout must be positive.");

const RANGE: Rule = Rule::new(
    "1 <= _ <= 10",
    Expr::Compare {
        operands: &[Operand::Int(1), Operand::Value, Operand::Int(10)],
        ops: &[CmpOp::Le, CmpOp::Le],
    },
);

const SHORT: Rule = Rule::new(
    "_.size <= 8",
    Expr::Compare {
        operands: &[Operand::Member(&["size"]), Operand::Int(8)],
        ops: &[CmpOp::Le],
    },
);

const LOWERCASE: Rule = Rule::new(
    r#"matches(_, "^[a-z]+$")"#,
    Expr::Matches {
        operand: Operand::Value,
        pattern: "^[a-z]+$",
    },
);

const ORDERED: Rule = Rule::new(
    "_.start <= _.end",
    Expr::Compare {
        operands: &[Operand::Member(&["start"]), Operand::Member(&["end"])],
        ops: &[CmpOp::Le],
    },
);

const MISTYPED: Rule = Rule::new(
    r#"_ == "one""#,
    Expr::Compare {
        operands: &[Operand::Value, Operand::String("one")],
        ops: &[CmpOp::Eq],
    },
);

record!(Settings {
    timeout_ms: u64 => .json_name("timeoutMs").with_rules(&[POSITIVE]);
    name: String => .optional().with_rules(&[SHORT, LOWERCASE]);
    retries: u8 => .optional().with_rules(&[RANGE]);
});

record!(Cluster {
    nodes: Vec<Settings>;
});

record!(Window => .with_rules(&[ORDERED]) {
    start: u32;
    end: u32 => .optional();
});

record!(Counter {
    count: u32 => .with_rules(&[MISTYPED]);
});

/// Returns `(path, code, message)` of each violation of `value`'s rules.
fn violations<T: sidex::Shaped + sidex::Encode>(value: &T) -> Vec<(String, &'static str, String)> {
    match validate(value) {
        Ok(()) => Vec::new(),
        Err(report) => {
            report
                .into_iter()
                .map(|error| {
                    (
                        error.path().to_string(),
                        error.code(),
                        error.message().to_owned(),
                    )
                })
                .collect()
        }
    }
}

fn settings(timeout_ms: u64, name: Option<&str>, retries: Option<u8>) -> Settings {
    Settings {
        timeout_ms: Some(timeout_ms),
        name: name.map(str::to_owned),
        retries,
    }
}

/// Valid values and absent optional fields pass.
#[test]
fn valid_values() {
    assert!(validate(&settings(30, Some("hub"), Some(3))).is_ok());
    assert!(validate(&settings(30, None, None)).is_ok());
}

/// Violations name the JSON path, the code, and the rule's message.
#[test]
fn field_rules() {
    assert_eq!(
        violations(&settings(0, Some("Hello World"), Some(11))),
        [
            (
                "timeoutMs".to_owned(),
                "min",
                "Timeout must be positive.".to_owned()
            ),
            (
                "name".to_owned(),
                "max_size",
                "JsonValue exceeds the maximum size.".to_owned()
            ),
            (
                "name".to_owned(),
                "regex",
                "JsonValue does not match the required pattern.".to_owned()
            ),
            (
                "retries".to_owned(),
                "max",
                "JsonValue is above the maximum.".to_owned()
            ),
        ]
    );
    assert_eq!(violations(&settings(30, None, Some(0)))[0].1, "min");
}

/// Paths lead through sequences to nested records.
#[test]
fn nested_paths() {
    let cluster = Cluster {
        nodes: Some(vec![settings(30, None, None), settings(0, None, None)]),
    };
    assert_eq!(violations(&cluster)[0].0, "nodes[1].timeoutMs");
}

/// Record rules compare fields and hold if a field is absent.
#[test]
fn record_rules() {
    let window = |start, end| {
        Window {
            start: Some(start),
            end,
        }
    };
    assert!(validate(&window(3, Some(5))).is_ok());
    assert!(validate(&window(5, None)).is_ok());
    assert_eq!(
        violations(&window(5, Some(3))),
        [(
            String::new(),
            "invalid",
            "JsonValue is not valid.".to_owned()
        )]
    );
}

/// Rules that cannot be evaluated are reported instead of passing silently.
#[test]
fn invalid_rules() {
    let violations = violations(&Counter { count: Some(1) });
    assert_eq!(violations[0].1, "invalid_rule");
    assert_eq!(
        violations[0].2,
        r#"rule `_ == "one"` cannot be evaluated: a number cannot be compared with a string"#
    );
}
