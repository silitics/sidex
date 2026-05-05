//! TypeScript codegen plugin for the validation extension.
//!
//! For every IR node carrying `#[validate(...)]` rules the plugin emits an
//! entry under a per-schema `validators` namespace exporting:
//!
//! - A whole-type `Type` validator function: `validators.User.validate(v)`.
//! - One per-field validator: `validators.User.fields.email(v)`.
//! - For wrappers, a `tryNew` factory that returns a `Result`-shaped union.
//!
//! The plugin shares the expression engine (`sidex-validate-expr`) with the
//! Rust plugin, so `(path, code)` tuples reported by the two backends
//! match byte-for-byte on the same input.

use sidex_attrs_validate as attrs;
use sidex_codegen::Code;
use sidex_codegen::quote;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;
use sidex_validate_expr as expr;

use super::Plugin;
use crate::context::SchemaCtx;
use crate::context::TypeExpr;

pub struct Validate;

impl Plugin for Validate {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<Code> {
        match &def.kind {
            ir::DefKind::RecordType(record) => emit_record(ctx, def, record),
            ir::DefKind::WrapperType(wrapper) => emit_wrapper(ctx, def, wrapper),
            ir::DefKind::OpaqueType(_) => emit_opaque(def),
            _ => Ok(Code::new()),
        }
    }

    fn visit_schema(&self, _ctx: &SchemaCtx) -> Result<Code> {
        // Module-level imports for the runtime. Pulled in unconditionally
        // so per-def emission can reference `__validate.*` without
        // checking whether at least one rule was emitted.
        Ok(quote!(
            r#"
            import * as __validate from "@@sidex/validate";
            "#
        ))
    }
}

// ---- Records ---------------------------------------------------------------

fn emit_record(ctx: &SchemaCtx, def: &ir::Def, record: &ir::RecordTypeDef) -> Result<Code> {
    let record_rules = attrs::record_rules(def)?;
    let any_field_rules = record.fields.iter().any(|f| {
        f.typed_attrs
            .get("validate")
            .and_then(|v| v.as_array())
            .is_some_and(|a| !a.is_empty())
    });
    if record_rules.is_empty() && !any_field_rules {
        return Ok(Code::new());
    }

    let name = def.name.as_str().to_owned();

    // Per-field validators (standalone functions) and the corresponding
    // step inside the whole-record validator. The two share rule bodies
    // but with different value sources / path bindings.
    let mut field_validators: Vec<Code> = Vec::new();
    let mut field_steps: Vec<Code> = Vec::new();
    for field in &record.fields {
        let field_rules = attrs::field_rules(field)?;
        if field_rules.is_empty() {
            continue;
        }
        let field_name = field.name.as_str().to_owned();
        let target = target_for_type(ctx, &field.typ);
        let field_ty = ctx.resolve_type(def, &field.typ);

        // Standalone field validator: rule body operates on `__value`.
        // The field-level entry is itself an object so other plugins can
        // contribute sibling methods (e.g. a future `format` plugin would
        // assign `User.fields.email.format(value)`); the validate plugin
        // owns only the `.validate` slot.
        let stmts_standalone = lower_rules_to_ts(&field_rules, "__value", "__path", target);
        let validator = quote!(
            r#"
            @field_name: {
                validate(__value: @field_ty, __path: __validate.Path = __validate.Path.root()): __validate.ValidationReport {
                    const __report = __validate.ValidationReport.ok();
                    @stmts_standalone
                    return __report;
                },
            },
            "#
        );
        field_validators.push(validator);

        // In-record step: same rule body, but the value source is
        // `__instance.<field_name>` and the path is nested under
        // `__path.field("<field_name>")`.
        let value_var = format!("__instance.{field_name}");
        let stmts_inline = lower_rules_to_ts(&field_rules, &value_var, "__field_path", target);
        let step = if field.is_optional {
            quote!(
                r#"
                if (__instance.@field_name !== undefined && __instance.@field_name !== null) {
                    const __field_path = __path.field("@field_name");
                    @stmts_inline
                }
                "#
            )
        } else {
            quote!(
                r#"
                {
                    const __field_path = __path.field("@field_name");
                    @stmts_inline
                }
                "#
            )
        };
        field_steps.push(step);
    }

    // Record-level rules apply against `__instance` directly.
    let record_steps =
        lower_rules_to_ts(&record_rules, "__instance", "__path", expr::Target::Record);

    // Within the same schema file the type alias is in scope under its
    // bare name (the types plugin emits `export type Foo = ...;`).
    let ty_expr = TypeExpr(Code::from(name.as_str()));

    Ok(quote!(
        r#"
        export const @name = {
            validate(__instance: @ty_expr, __path: __validate.Path = __validate.Path.root()): __validate.ValidationReport {
                const __report = __validate.ValidationReport.ok();
                @(@field_steps)*
                @record_steps
                return __report;
            },
            fields: {
                @(@field_validators)*
            },
        };
        "#
    ))
}

// ---- Wrappers --------------------------------------------------------------

fn emit_wrapper(ctx: &SchemaCtx, def: &ir::Def, wrapper: &ir::WrapperTypeDef) -> Result<Code> {
    let rules = attrs::wrapper_rules(def)?;
    if rules.is_empty() {
        return Ok(Code::new());
    }
    let name = def.name.as_str().to_owned();
    let target = target_for_type(ctx, &wrapper.wrapped);
    let inner_ty = ctx.resolve_type(def, &wrapper.wrapped);
    let stmts = lower_rules_to_ts(&rules, "__value", "__path", target);

    let qualified = Code::from(name.as_str());
    // Standalone validator that operates on `__value`.
    let try_new_stmts = lower_rules_to_ts(&rules, "__value", "__path", target);

    Ok(quote!(
        r#"
        export const @name = {
            validate(__value: @qualified, __path: __validate.Path = __validate.Path.root()): __validate.ValidationReport {
                const __report = __validate.ValidationReport.ok();
                @stmts
                return __report;
            },
            tryNew(__value: @inner_ty): { ok: true, value: @qualified } | { ok: false, report: __validate.ValidationReport } {
                const __path = __validate.Path.root();
                const __report = __validate.ValidationReport.ok();
                @try_new_stmts
                if (__report.isOk()) {
                    return { ok: true, value: __value as unknown as @qualified };
                }
                return { ok: false, report: __report };
            },
        };
        "#
    ))
}

// ---- Opaque ---------------------------------------------------------------

fn emit_opaque(def: &ir::Def) -> Result<Code> {
    let rules = attrs::opaque_rules(def)?;
    if rules.is_empty() {
        return Ok(Code::new());
    }
    // Same caveat as the Rust backend: opaques lower to type aliases that
    // can't host inherent methods. Surface this as a TypeScript comment so
    // users know rules were declared but skipped, then fall through.
    let name = def.name.as_str().to_owned();
    let summaries: Vec<String> = rules
        .iter()
        .map(|r| format!("//   `{}`", r.expr.text))
        .collect();
    let joined = summaries.join("\n");
    Ok(Code::from(format!(
        "// `{name}` carries `#[validate(...)]` rules but opaque codegen lowers to a type alias.\n\
         // Wrap the type in `wrapper` to get an enforced `tryNew`.\n\
         {joined}\n"
    )))
}

// ---- Lowering --------------------------------------------------------------

fn lower_rules_to_ts<R: HasRule>(
    rules: &[R],
    value_var: &str,
    path_var: &str,
    target: expr::Target,
) -> Code {
    let mut steps: Vec<Code> = Vec::new();
    let mut regex_count: usize = 0;
    for rule in rules {
        let tokens = rule.expr();
        let parsed = match expr::parse(&tokens.text) {
            Ok(ast) => ast,
            Err(e) => {
                Diagnostic::error(format!(
                    "Invalid `#[validate]` expression `{}`: {}",
                    tokens.text, e.message
                ))
                .emit();
                continue;
            }
        };
        let recognized = expr::recognize(&parsed, &tokens.text, target);
        for err in &recognized.errors {
            Diagnostic::error(format!(
                "Invalid `#[validate]` rule `{}`: {}",
                tokens.text, err.message
            ))
            .emit();
        }
        for r in &recognized.rules {
            steps.push(lower_rule_to_ts(
                r,
                value_var,
                path_var,
                rule.message(),
                rule.code(),
                &mut regex_count,
            ));
        }
    }
    quote!(r#"@(@steps)*"#)
}

fn lower_rule_to_ts(
    rule: &expr::Rule,
    value_var: &str,
    path_var: &str,
    message_override: Option<&str>,
    code_override: Option<&str>,
    _regex_count: &mut usize,
) -> Code {
    let message_arg: String = match message_override {
        Some(m) => format!("\"{}\"", escape_ts_string(m)),
        None => "null".to_owned(),
    };
    match rule {
        expr::Rule::Compare {
            accessor,
            helper,
            threshold,
            code,
        } => {
            let helper_fn = match helper {
                expr::HelperKind::MinInclusive => "minInclusive",
                expr::HelperKind::MinExclusive => "minExclusive",
                expr::HelperKind::MaxInclusive => "maxInclusive",
                expr::HelperKind::MaxExclusive => "maxExclusive",
                expr::HelperKind::Eq => "eq",
                expr::HelperKind::Ne => "ne",
            };
            let lhs = lower_accessor_ts(accessor, value_var);
            let bound = lower_threshold_ts(threshold);
            let code = code_override.unwrap_or(code);
            Code::from(format!(
                "__report.merge(__validate.{helper_fn}({lhs}, {bound}, \"{code}\", {message_arg}, {path_var}));\n"
            ))
        }
        expr::Rule::Regex { accessor, pattern } => {
            let target_value = match accessor {
                expr::Accessor::Self_ => value_var.to_owned(),
                expr::Accessor::Field(name) => format!("({value_var} as any).{name}"),
                expr::Accessor::CharCount | expr::Accessor::ByteCount => {
                    return Code::from("/* unsupported regex accessor */\n");
                }
            };
            let final_code = code_override.unwrap_or("regex");
            let escaped_pattern = escape_ts_string(pattern);
            Code::from(format!(
                "__report.merge(__validate.regexMatch({target_value}, \"{escaped_pattern}\", \"{final_code}\", {message_arg}, {path_var}));\n"
            ))
        }
        expr::Rule::Predicate { source } => {
            let escaped = escape_ts_string(source);
            Code::from(format!(
                "/* predicate fallback for `{escaped}` — codegen TODO */\n"
            ))
        }
    }
}

fn lower_accessor_ts(accessor: &expr::Accessor, value_var: &str) -> String {
    match accessor {
        expr::Accessor::Self_ => format!("{value_var} as any"),
        expr::Accessor::CharCount => {
            format!("__validate.charCount({value_var} as unknown as string)")
        }
        expr::Accessor::ByteCount => {
            format!("__validate.byteCount({value_var} as unknown as string)")
        }
        expr::Accessor::Field(name) => format!("({value_var} as any).{name}"),
    }
}

fn lower_threshold_ts(literal: &expr::Literal) -> String {
    match literal {
        expr::Literal::Number(n) => n.clone(),
        expr::Literal::String(s) => format!("\"{}\"", escape_ts_string(s)),
    }
}

fn escape_ts_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

// ---- Target inference ------------------------------------------------------

fn target_for_type(ctx: &SchemaCtx, ty: &ir::Type) -> expr::Target {
    let resolved = ctx.bundle_ctx.unit.resolve_aliases(ty);
    let ir::TypeKind::Instance(instance) = &resolved.kind else {
        return expr::Target::Other;
    };
    let bundle = &ctx.bundle_ctx.unit[instance.def.bundle];
    let schema = &ctx.bundle_ctx.unit[instance.def.schema];
    let def = &ctx.bundle_ctx.unit[instance.def];
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
        ir::DefKind::WrapperType(w) => target_for_type(ctx, &w.wrapped),
        _ => expr::Target::Other,
    }
}

// ---- Trait abstracting the four `*Rule` records ---------------------------

trait HasRule {
    fn expr(&self) -> &attrs::raw::TokensValue;
    fn message(&self) -> Option<&str>;
    fn code(&self) -> Option<&str>;
}

macro_rules! impl_has_rule {
    ($($ty:ty),*) => {
        $(
            impl HasRule for $ty {
                fn expr(&self) -> &attrs::raw::TokensValue { &self.expr }
                fn message(&self) -> Option<&str> { self.message.as_deref() }
                fn code(&self) -> Option<&str> { self.code.as_deref() }
            }
        )*
    };
}

impl_has_rule!(
    attrs::raw::FieldRule,
    attrs::raw::RecordRule,
    attrs::raw::WrapperRule,
    attrs::raw::OpaqueRule
);
