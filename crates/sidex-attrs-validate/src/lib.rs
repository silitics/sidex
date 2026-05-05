//! Typed attribute schemas for the validation extension.
//!
//! These types mirror [`lib/validate/schemas/attrs.sidex`](https://github.com/silitics/sidex/blob/main/crates/sidex-core/lib/validate/schemas/attrs.sidex).
//! The compiler validates source `#[validate(...)]` attributes against the
//! schema and stores the result in `typed_attrs["validate"]` as a JSON
//! array (the `validate` plugin runs in *repeated* mode — one element per
//! source attribute).
//!
//! Codegens — Rust and TypeScript — pull the array off any IR node via
//! `field_rules`, `record_rules`, `wrapper_rules`, `opaque_rules`. Each
//! returned `*Rule` carries the expression body verbatim plus optional
//! message / code overrides.

mod generated;

use std::collections::HashMap;

pub use generated::attrs as raw;
use serde_json::Value;
use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Label;
use sidex_diagnostics::Result;
use sidex_ir as ir;
use sidex_validate_expr as expr;

const PLUGIN: &str = "validate";

/// Pull the typed `validate` rules off any IR node's `typed_attrs`. Returns
/// an empty `Vec` when the node carries no `#[validate(...)]` attributes.
fn extract_rules<R>(typed_attrs: &HashMap<String, Value>) -> Result<Vec<R>>
where
    R: serde::de::DeserializeOwned,
{
    let Some(value) = typed_attrs.get(PLUGIN) else {
        return Ok(Vec::new());
    };
    serde_json::from_value(value.clone()).map_err(|err| {
        Box::new(Diagnostic::error(format!(
            "Invalid `validate` attributes: {err}"
        )))
    })
}

/// Read the validation rules attached to `field`.
pub fn field_rules(field: &ir::Field) -> Result<Vec<raw::FieldRule>> {
    extract_rules(&field.typed_attrs)
}

/// Read the validation rules attached to a record / variant / etc. `def`.
pub fn record_rules(def: &ir::Def) -> Result<Vec<raw::RecordRule>> {
    extract_rules(&def.typed_attrs)
}

/// Read the validation rules attached to a wrapper `def`.
pub fn wrapper_rules(def: &ir::Def) -> Result<Vec<raw::WrapperRule>> {
    extract_rules(&def.typed_attrs)
}

/// Read the validation rules attached to an opaque `def`.
pub fn opaque_rules(def: &ir::Def) -> Result<Vec<raw::OpaqueRule>> {
    extract_rules(&def.typed_attrs)
}

// ---- check pass -----------------------------------------------------------

/// Walk every validation rule on `ir` and emit diagnostics for any that
/// fail to parse or recognize. Called by `sidex check` so users see
/// regex-syntax errors and other shape-level problems before any codegen
/// runs.
///
/// The recognizer needs a [`expr::Target`] hint to interpret accessors
/// (`_.length`, `_.<field>`) correctly. We infer the target from the
/// node the rule is attached to — string field, numeric field, record,
/// wrapper, or opaque. Unrecognized patterns silently fall back to the
/// `predicate` rule and are not flagged here.
pub fn check(ir: &ir::Ir) {
    for (idx, def) in ir.defs.iter().enumerate() {
        // Skip internal bundles (the standard library, plugin-attribute
        // bundles auto-loaded by the compiler) — they don't carry user
        // validation rules.
        if ir[ir.schemas[def.schema.idx()].bundle].is_internal {
            continue;
        }
        check_def(ir, ir::DefIdx::from(idx));
    }
}

fn check_def(ir: &ir::Ir, def_idx: ir::DefIdx) {
    let def = &ir.defs[def_idx.idx()];
    match &def.kind {
        ir::DefKind::RecordType(record) => {
            check_rules_at(record_rules(def), record_target_for(def));
            for field in &record.fields {
                let target = target_for_type(ir, &field.typ);
                check_rules_at(field_rules(field), target);
            }
        }
        ir::DefKind::WrapperType(wrapper) => {
            let target = target_for_type(ir, &wrapper.wrapped);
            check_rules_at(wrapper_rules(def), target);
        }
        ir::DefKind::OpaqueType(_) => {
            check_rules_at(opaque_rules(def), expr::Target::Other);
        }
        _ => {}
    }
}

fn check_rules_at<R: HasRule>(rules: Result<Vec<R>>, target: expr::Target) {
    let rules = match rules {
        Ok(r) => r,
        Err(diag) => {
            diag.emit();
            return;
        }
    };
    for rule in rules {
        let body = rule.expr_text();
        let span = rule.expr_span();
        let parsed = match expr::parse(body) {
            Ok(p) => p,
            Err(e) => {
                emit_error(
                    format!("Invalid `#[validate]` expression `{}`: {}", body, e.message),
                    span,
                );
                continue;
            }
        };
        let out = expr::recognize(&parsed, body, target);
        for err in out.errors {
            emit_error(
                format!("Invalid `#[validate]` rule `{}`: {}", body, err.message),
                span.clone(),
            );
        }
    }
}

fn emit_error(message: String, span: Option<ir::Span>) {
    let mut diag = Diagnostic::error(message);
    if let Some(span) = span {
        diag = diag
            .with_span(Some(span.clone()))
            .with_label(Label::new(span, "in this rule"));
    }
    diag.emit();
}

fn record_target_for(_def: &ir::Def) -> expr::Target {
    expr::Target::Record
}

fn target_for_type(ir: &ir::Ir, typ: &ir::Type) -> expr::Target {
    let resolved = ir.resolve_aliases(typ);
    let ir::TypeKind::Instance(instance) = &resolved.kind else {
        return expr::Target::Other;
    };
    let bundle = &ir[instance.def.bundle];
    let schema = &ir[instance.def.schema];
    let def = &ir[instance.def];
    if bundle.metadata.name == "core" && schema.name == "builtins" {
        match def.name.as_str() {
            "string" => return expr::Target::String,
            "bytes" => return expr::Target::Bytes,
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" | "idx" => {
                return expr::Target::Number;
            }
            _ => {}
        }
    }
    match &def.kind {
        ir::DefKind::RecordType(_) => expr::Target::Record,
        ir::DefKind::WrapperType(w) => target_for_type(ir, &w.wrapped),
        _ => expr::Target::Other,
    }
}

/// Common shape across the four `*Rule` records — lets the check pass
/// extract the body text + span without per-variant boilerplate.
trait HasRule {
    fn expr_text(&self) -> &str;
    fn expr_span(&self) -> Option<ir::Span>;
}

macro_rules! impl_has_rule {
    ($($ty:ty),*) => {
        $(
            impl HasRule for $ty {
                fn expr_text(&self) -> &str { &self.expr.text }
                fn expr_span(&self) -> Option<ir::Span> {
                    self.expr.span.as_ref().map(|s| {
                        ir::Span::new(
                            ir::SourceIdx::from(s.src),
                            s.start,
                            s.end,
                        )
                    })
                }
            }
        )*
    };
}

impl_has_rule!(
    raw::FieldRule,
    raw::RecordRule,
    raw::WrapperRule,
    raw::OpaqueRule
);
