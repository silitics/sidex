//! Evaluation of rules against views.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::PoisonError;
use std::sync::RwLock;

use regex::Regex;

use super::CmpOp;
use super::Expr;
use super::Operand;
use super::Rule;
use super::ValidationError;
use super::ValidationReport;
use crate::Encode;
use crate::Shape;
use crate::Shaped;
use crate::View;
use crate::decode::PathSegment;
use crate::encode::RecordView;
use crate::encode::VariantView;
use crate::formats::json::Tagging;
use crate::shape::RecordShape;
use crate::shape::ShapeKind;
use crate::types::JsonValue;
use crate::types::Str;

/// Checks the rules of `value` and of the values nested in it.
pub fn validate<T: Shaped + Encode + ?Sized>(value: &T) -> Result<(), ValidationReport> {
    let mut walker = Walker::default();
    walker.check(&value.encode(), T::SHAPE);
    if walker.errors.is_empty() {
        Ok(())
    } else {
        Err(ValidationReport {
            errors: walker.errors,
        })
    }
}

/// Segment of the path to the value being checked.
enum Segment {
    Field(&'static str),
    Index(usize),
    Key(Str),
}

/// Walks a value and its shape, collecting violations.
#[derive(Default)]
struct Walker {
    path: Vec<Segment>,
    errors: Vec<ValidationError>,
}

impl Walker {
    fn check(&mut self, view: &View<'_>, shape: &'static Shape) {
        self.apply(shape.rules, view);
        match (&shape.kind, view) {
            (ShapeKind::Wrapper(inner), _) => self.check(view, inner()),
            (ShapeKind::Record(record), View::Record(fields)) => self.record(*fields, record),
            (ShapeKind::Variant(_), View::Variant(variant)) => self.variant(*variant),
            (ShapeKind::Sequence(item), View::Seq(seq)) => {
                let mut index = 0;
                let result = seq.each(&mut |element| {
                    self.within(Segment::Index(index), |walker| {
                        walker.check(&element, item());
                    });
                    index += 1;
                    Ok(())
                });
                if let Err(error) = result {
                    self.report("invalid", Cow::Owned(error.to_string()));
                }
            }
            (ShapeKind::Map(_, value), View::Map(map)) => {
                let result = map.each(&mut |key, entry| {
                    self.within(Segment::Key(key_text(&key)), |walker| {
                        walker.check(&entry, value());
                    });
                    Ok(())
                });
                if let Err(error) = result {
                    self.report("invalid", Cow::Owned(error.to_string()));
                }
            }
            _ => {}
        }
    }

    fn record(&mut self, fields: &dyn RecordView, record: &'static RecordShape) {
        for (index, field) in record.fields.iter().enumerate() {
            let Some(value) = fields.field(index) else {
                continue;
            };
            let check = |walker: &mut Walker| {
                walker.apply(field.rules, &value);
                walker.check(&value, (field.shape)());
            };
            if field.json.inline {
                check(self);
            } else {
                self.within(Segment::Field(field.json.name), check);
            }
        }
    }

    fn variant(&mut self, variant: &dyn VariantView) {
        let shape = variant.shape();
        let (Some(variant_shape), Some(payload)) = (shape.as_variant(), variant.payload()) else {
            return;
        };
        let Some(payload_shape) = variant_shape
            .cases
            .get(variant.case())
            .and_then(|case| case.payload.map(|payload| (case, payload())))
        else {
            return;
        };
        let (case, payload_shape) = payload_shape;
        // The segment follows the JSON representation of the variant.
        let segment = match variant_shape.json {
            Tagging::External => Some(case.json.name),
            Tagging::Implicit => None,
            Tagging::Internal { .. } | Tagging::Adjacent { .. } => {
                case.json.content_field(variant_shape.json, payload_shape)
            }
        };
        match segment {
            Some(name) => {
                self.within(Segment::Field(name), |walker| {
                    walker.check(&payload, payload_shape);
                })
            }
            None => self.check(&payload, payload_shape),
        }
    }

    fn apply(&mut self, rules: &'static [Rule], view: &View<'_>) {
        for rule in rules {
            match evaluate(&rule.expr, view) {
                Ok(None) => {}
                Ok(Some(code)) => {
                    let code = rule.code.unwrap_or(code);
                    let message = rule.message.unwrap_or_else(|| default_message(code));
                    self.report(code, Cow::Borrowed(message));
                }
                Err(reason) => {
                    let message = format!("rule `{}` cannot be evaluated: {reason}", rule.source);
                    self.report("invalid_rule", Cow::Owned(message));
                }
            }
        }
    }

    fn within(&mut self, segment: Segment, f: impl FnOnce(&mut Walker)) {
        self.path.push(segment);
        f(self);
        self.path.pop();
    }

    fn report(&mut self, code: &'static str, message: Cow<'static, str>) {
        let path = self
            .path
            .iter()
            .map(|segment| {
                match segment {
                    Segment::Field(name) => PathSegment::Key(Str::from_static(name)),
                    Segment::Index(index) => PathSegment::Index(*index),
                    Segment::Key(key) => PathSegment::Key(key.clone()),
                }
            })
            .collect();
        self.errors.push(ValidationError {
            path,
            code,
            message,
        });
    }
}

/// Evaluates an expression and returns the code of the violation, if any.
fn evaluate(expr: &Expr, view: &View<'_>) -> Result<Option<&'static str>, String> {
    match expr {
        Expr::Compare { operands, ops } => {
            if operands.len() != ops.len() + 1 {
                return Err("the number of operands does not match the operators".to_owned());
            }
            let Some(mut left) = resolve(&operands[0], view)? else {
                return Ok(None);
            };
            for (index, op) in ops.iter().enumerate() {
                let Some(right) = resolve(&operands[index + 1], view)? else {
                    return Ok(None);
                };
                if !compare(&left, *op, &right)? {
                    let left = (&operands[index], &left);
                    let right = (&operands[index + 1], &right);
                    return Ok(Some(code(left, *op, right)));
                }
                left = right;
            }
            Ok(None)
        }
        Expr::Matches { operand, pattern } => {
            match resolve(operand, view)? {
                None => Ok(None),
                Some(Term {
                    value: Scalar::String(text),
                    ..
                }) => Ok((!regex(pattern)?.is_match(&text)).then_some("regex")),
                Some(_) => Err("`matches` requires a string".to_owned()),
            }
        }
    }
}

/// Resolved operand.
struct Term<'v> {
    value: Scalar<'v>,
    /// Indicates whether the term is the size of a value.
    size: bool,
}

enum Scalar<'v> {
    Number(Number),
    String(Cow<'v, str>),
}

#[derive(Clone, Copy)]
enum Number {
    Int(i128),
    Float(f64),
}

impl Number {
    fn partial_cmp(self, other: Number) -> Option<Ordering> {
        match (self, other) {
            (Number::Int(left), Number::Int(right)) => Some(left.cmp(&right)),
            (left, right) => left.as_f64().partial_cmp(&right.as_f64()),
        }
    }

    fn as_f64(self) -> f64 {
        match self {
            // Precision loss is inherent to comparisons with floats.
            Number::Int(value) => value as f64,
            Number::Float(value) => value,
        }
    }
}

/// Resolves an operand, or returns `None` if it refers to an absent field.
fn resolve<'v>(operand: &Operand, view: &View<'v>) -> Result<Option<Term<'v>>, String> {
    let value = match operand {
        Operand::Value => return term(view).map(Some),
        Operand::Member(path) => return member(view, path),
        Operand::Int(value) => Scalar::Number(Number::Int(*value)),
        Operand::Float(value) => Scalar::Number(Number::Float(*value)),
        Operand::String(value) => Scalar::String(Cow::Borrowed(value)),
    };
    Ok(Some(Term { value, size: false }))
}

fn member<'v>(view: &View<'v>, path: &[&str]) -> Result<Option<Term<'v>>, String> {
    let Some((first, rest)) = path.split_first() else {
        return term(view).map(Some);
    };
    match view {
        View::Record(record) => {
            let record: &'v dyn RecordView = *record;
            let shape = record.shape();
            let index = shape
                .as_record()
                .and_then(|fields| fields.fields.iter().position(|field| field.name == *first))
                .ok_or_else(|| format!("`{shape}` has no field `{first}`"))?;
            match record.field(index) {
                Some(field) => member(&field, rest),
                None => Ok(None),
            }
        }
        _ if *first == "size" && rest.is_empty() => {
            let size = size(view)?;
            Ok(Some(Term {
                value: Scalar::Number(Number::Int(
                    i128::try_from(size).expect("every `usize` fits into `i128`"),
                )),
                size: true,
            }))
        }
        _ => Err(format!("{} has no member `{first}`", kind(view))),
    }
}

fn term<'v>(view: &View<'v>) -> Result<Term<'v>, String> {
    let int = |value: i128| Scalar::Number(Number::Int(value));
    let value = match view {
        View::I8(value) => int((*value).into()),
        View::I16(value) => int((*value).into()),
        View::I32(value) => int((*value).into()),
        View::I64(value) => int((*value).into()),
        View::U8(value) => int((*value).into()),
        View::U16(value) => int((*value).into()),
        View::U32(value) => int((*value).into()),
        View::U64(value) => int((*value).into()),
        View::Idx(value) => int(i128::try_from(*value).expect("every `usize` fits into `i128`")),
        View::F32(value) => Scalar::Number(Number::Float((*value).into())),
        View::F64(value) => Scalar::Number(Number::Float(*value)),
        View::String(value) => Scalar::String(Cow::Borrowed(value)),
        View::Json(Cow::Borrowed(value)) => value_scalar(value)?,
        View::Json(Cow::Owned(value)) => {
            match value_scalar(value)? {
                Scalar::String(text) => Scalar::String(Cow::Owned(text.into_owned())),
                Scalar::Number(number) => Scalar::Number(number),
            }
        }
        _ => return Err(format!("{} cannot be compared", kind(view))),
    };
    Ok(Term { value, size: false })
}

fn value_scalar(value: &JsonValue) -> Result<Scalar<'_>, String> {
    match value {
        JsonValue::Number(number) => {
            Ok(Scalar::Number(match number.as_i128() {
                Some(value) => Number::Int(value),
                None => Number::Float(number.as_f64()),
            }))
        }
        JsonValue::String(value) => Ok(Scalar::String(Cow::Borrowed(value))),
        other => Err(format!("{} cannot be compared", other.kind())),
    }
}

/// Size of a string, bytes, sequence, or map.
fn size(view: &View<'_>) -> Result<usize, String> {
    match view {
        View::String(value) => Ok(value.chars().count()),
        View::Bytes(value) => Ok(value.len()),
        View::Seq(seq) => Ok(seq.len()),
        View::Map(map) => Ok(map.len()),
        View::Json(value) => {
            match value.as_ref() {
                JsonValue::String(value) => Ok(value.chars().count()),
                JsonValue::Seq(items) => Ok(items.len()),
                JsonValue::Map(entries) => Ok(entries.len()),
                other => Err(format!("{} has no size", other.kind())),
            }
        }
        _ => Err(format!("{} has no size", kind(view))),
    }
}

fn compare(left: &Term<'_>, op: CmpOp, right: &Term<'_>) -> Result<bool, String> {
    let ordering = match (&left.value, &right.value) {
        (Scalar::Number(left), Scalar::Number(right)) => left.partial_cmp(*right),
        (Scalar::String(left), Scalar::String(right)) => Some(left.cmp(right)),
        _ => return Err("a number cannot be compared with a string".to_owned()),
    };
    Ok(match ordering {
        Some(ordering) => {
            match op {
                CmpOp::Lt => ordering == Ordering::Less,
                CmpOp::Le => ordering != Ordering::Greater,
                CmpOp::Eq => ordering == Ordering::Equal,
                CmpOp::Ne => ordering != Ordering::Equal,
                CmpOp::Ge => ordering != Ordering::Less,
                CmpOp::Gt => ordering == Ordering::Greater,
            }
        }
        // `NaN` is unequal to everything and unordered.
        None => op == CmpOp::Ne,
    })
}

/// Code of a violated comparison of two operands and their terms.
fn code(left: (&Operand, &Term<'_>), op: CmpOp, right: (&Operand, &Term<'_>)) -> &'static str {
    let is_literal = |operand: &Operand| {
        matches!(
            operand,
            Operand::Int(_) | Operand::Float(_) | Operand::String(_)
        )
    };
    // Normalize to `subject op literal`.
    let ((subject, term), op) = match (is_literal(left.0), is_literal(right.0)) {
        (false, true) => (left, op),
        (true, false) => {
            let flipped = match op {
                CmpOp::Lt => CmpOp::Gt,
                CmpOp::Le => CmpOp::Ge,
                CmpOp::Ge => CmpOp::Le,
                CmpOp::Gt => CmpOp::Lt,
                other => other,
            };
            (right, flipped)
        }
        _ => return "invalid",
    };
    match (subject, term.size, op) {
        (Operand::Value, _, CmpOp::Gt | CmpOp::Ge) => "min",
        (Operand::Value, _, CmpOp::Lt | CmpOp::Le) => "max",
        (Operand::Value, _, CmpOp::Eq) => "eq",
        (Operand::Value, _, CmpOp::Ne) => "ne",
        (Operand::Member(_), true, CmpOp::Gt | CmpOp::Ge) => "min_size",
        (Operand::Member(_), true, CmpOp::Lt | CmpOp::Le) => "max_size",
        (Operand::Member(_), true, CmpOp::Eq) => "eq_size",
        (Operand::Member(_), true, CmpOp::Ne) => "ne_size",
        _ => "invalid",
    }
}

fn default_message(code: &str) -> &'static str {
    match code {
        "min" => "JsonValue is below the minimum.",
        "max" => "JsonValue is above the maximum.",
        "eq" => "JsonValue does not match the expected value.",
        "ne" => "JsonValue matches a forbidden value.",
        "min_size" => "JsonValue is below the minimum size.",
        "max_size" => "JsonValue exceeds the maximum size.",
        "eq_size" => "JsonValue does not have the expected size.",
        "ne_size" => "JsonValue has a forbidden size.",
        "regex" => "JsonValue does not match the required pattern.",
        _ => "JsonValue is not valid.",
    }
}

/// Returns the compiled regular expression for `pattern`.
fn regex(pattern: &'static str) -> Result<Regex, String> {
    static CACHE: OnceLock<RwLock<HashMap<&'static str, Regex>>> = OnceLock::new();
    let cache = CACHE.get_or_init(RwLock::default);
    if let Some(regex) = cache
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .get(pattern)
    {
        return Ok(regex.clone());
    }
    let regex = Regex::new(pattern).map_err(|error| error.to_string())?;
    cache
        .write()
        .unwrap_or_else(PoisonError::into_inner)
        .insert(pattern, regex.clone());
    Ok(regex)
}

/// Text of a map key for paths.
fn key_text(key: &View<'_>) -> Str {
    match key {
        View::String(key) => (*key).into(),
        View::I8(key) => key.to_string().into(),
        View::I16(key) => key.to_string().into(),
        View::I32(key) => key.to_string().into(),
        View::I64(key) => key.to_string().into(),
        View::U8(key) => key.to_string().into(),
        View::U16(key) => key.to_string().into(),
        View::U32(key) => key.to_string().into(),
        View::U64(key) => key.to_string().into(),
        View::Idx(key) => key.to_string().into(),
        View::Uuid(key) => key.to_string().into(),
        View::Timestamp(key) => key.to_string().into(),
        View::Date(key) => key.to_string().into(),
        other => Str::from_static(kind(other)),
    }
}

/// Short description of a view's kind for error messages.
fn kind(view: &View<'_>) -> &'static str {
    match view {
        View::Bool(_) => "a boolean",
        View::I8(_)
        | View::I16(_)
        | View::I32(_)
        | View::I64(_)
        | View::U8(_)
        | View::U16(_)
        | View::U32(_)
        | View::U64(_)
        | View::Idx(_) => "an integer",
        View::F32(_) | View::F64(_) => "a float",
        View::String(_) => "a string",
        View::Bytes(_) => "bytes",
        View::Uuid(_) => "a UUID",
        View::Timestamp(_) => "a timestamp",
        View::Date(_) => "a date",
        View::Seq(_) => "a sequence",
        View::Map(_) => "a map",
        View::Record(_) => "a record",
        View::Variant(_) => "a variant",
        View::Json(value) => value.kind(),
    }
}
