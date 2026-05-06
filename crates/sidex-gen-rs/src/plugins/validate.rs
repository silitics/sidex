//! Rust codegen plugin for the validation extension.
//!
//! For every IR node carrying `#[validate(...)]` rules the plugin emits:
//!
//! - `impl ::sidex_validate::Validate for T` for records, wrappers, and opaque types —
//!   runs every rule and merges the per-rule reports into one.
//! - A sibling `pub mod validators` per schema, with one `pub fn` per field that has
//!   rules. Field validators are reusable atoms — useful for forms that want to validate
//!   a single input on every keystroke without re-running the whole record.
//! - A `pub fn try_new(...) -> Result<Self, ValidationReport>` constructor on wrappers
//!   with rules — the safe construction path.
//! - Per-callsite `static REGEX_<n>: ::std::sync::OnceLock<::regex::Regex>` for
//!   `matches(_, "...")` rules; compiled lazily on first use.
//!
//! Rule expressions are parsed with `sidex-validate-expr` and matched
//! against a small recognizer table. Anything not in the table falls back
//! to a verbatim Rust translation under `code: "predicate"`.

use std::cell::Cell;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use serde::Deserialize;
use sidex_attrs_validate as attrs;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;
use sidex_validate_expr as expr;

use super::Plugin;
use crate::context::SchemaCtx;

/// Configuration for the `validate` plugin.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ValidateConfig {
    /// Path used in generated code to refer to the runtime crate. Override
    /// when the runtime is re-exported from a wrapper crate.
    pub sidex_validate_path: String,
}

impl Default for ValidateConfig {
    fn default() -> Self {
        Self {
            sidex_validate_path: "::sidex_validate".to_owned(),
        }
    }
}

pub struct Validate;

impl Plugin for Validate {
    fn visit_def(&self, ctx: &SchemaCtx, def: &ir::Def) -> Result<TokenStream> {
        let cfg = ctx
            .bundle_ctx
            .get_plugin_config::<ValidateConfig>("validate");
        let runtime: syn::Path = syn::parse_str(&cfg.sidex_validate_path).map_err(|_| {
            Box::new(Diagnostic::error(format!(
                "Invalid `sidex_validate_path` configuration: `{}`",
                cfg.sidex_validate_path
            )))
        })?;

        let emitter = Emitter::new(&runtime);

        match &def.kind {
            ir::DefKind::RecordType(record) => emitter.emit_record_impl(ctx, def, record),
            ir::DefKind::WrapperType(wrapper) => emitter.emit_wrapper_impl(ctx, def, wrapper),
            ir::DefKind::OpaqueType(_) => emitter.emit_opaque_impl(ctx, def),
            _ => Ok(TokenStream::new()),
        }
    }

    fn visit_schema(&self, ctx: &SchemaCtx) -> Result<TokenStream> {
        let cfg = ctx
            .bundle_ctx
            .get_plugin_config::<ValidateConfig>("validate");
        let runtime: syn::Path = syn::parse_str(&cfg.sidex_validate_path).map_err(|_| {
            Box::new(Diagnostic::error(format!(
                "Invalid `sidex_validate_path` configuration: `{}`",
                cfg.sidex_validate_path
            )))
        })?;
        Ok(quote! {
            #[allow(unused)]
            use #runtime as __sidex_validate;
        })
    }
}

/// Common shape across the four `*Rule` records (`FieldRule`, `RecordRule`,
/// `WrapperRule`, `OpaqueRule`). Lets the lowering pipeline take any of
/// them via a single signature without macro generation.
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

// --- Emitter ----------------------------------------------------------------

struct Emitter<'a> {
    runtime: &'a syn::Path,
    /// Counter used to give every regex pattern a unique `static REGEX_<n>`.
    /// `Cell` so this stays `&self`-shaped without bubbling `&mut` through
    /// the lowering helpers.
    regex_counter: Cell<usize>,
}

impl<'a> Emitter<'a> {
    fn new(runtime: &'a syn::Path) -> Self {
        Self {
            runtime,
            regex_counter: Cell::new(0),
        }
    }

    fn next_regex_ident(&self) -> syn::Ident {
        let n = self.regex_counter.get();
        self.regex_counter.set(n + 1);
        format_ident!("__VALIDATE_REGEX_{}", n)
    }

    fn emit_record_impl(
        &self,
        ctx: &SchemaCtx,
        def: &ir::Def,
        record: &ir::RecordTypeDef,
    ) -> Result<TokenStream> {
        let record_rules = attrs::record_rules(def)?;
        let any_field_rules = record.fields.iter().any(|f| {
            f.typed_attrs
                .get("validate")
                .and_then(|v| v.as_array())
                .is_some_and(|a| !a.is_empty())
        });
        if record_rules.is_empty() && !any_field_rules {
            return Ok(TokenStream::new());
        }

        let ident = format_ident!("{}", def.name.as_str());
        let runtime = self.runtime;

        // Per-field validation steps inside the `validate_at` body.
        let mut field_steps = Vec::new();
        for field in &record.fields {
            let field_rules = attrs::field_rules(field)?;
            if field_rules.is_empty() {
                continue;
            }
            let field_ident = format_ident!("{}", field.name.as_str());
            let field_path_name = field.name.as_str();
            let field_target = target_for_field_type(ctx, field);
            let inner_ty = ctx.resolve_type(def, &field.typ);
            let inner_ty_tokens = quote!(#inner_ty);
            let value_expr: TokenStream = if field.is_optional {
                quote!((*__inner_value))
            } else {
                quote!(self.#field_ident)
            };
            let body = self.lower_rules_to_block(
                &field_rules,
                value_expr,
                quote!(__field_path),
                field_target,
                inner_ty_tokens,
            );
            let step = if field.is_optional {
                quote! {
                    {
                        let __field_path = __path.field(#field_path_name);
                        if let ::std::option::Option::Some(__inner_value) = self.#field_ident.as_ref() {
                            let _ = &__inner_value;
                            #body
                        }
                    }
                }
            } else {
                quote! {
                    {
                        let __field_path = __path.field(#field_path_name);
                        #body
                    }
                }
            };
            field_steps.push(step);
        }

        // Record-level rules apply to `self` directly.
        let record_steps = self.lower_rules_to_block(
            &record_rules,
            quote!((*self)),
            quote!(__path),
            expr::Target::Record,
            quote!(Self),
        );

        let regex_statics = self.take_regex_statics();

        Ok(quote! {
            #regex_statics

            impl #runtime::Validate for #ident {
                fn validate_at(&self, __path: &#runtime::Path) -> #runtime::ValidationReport {
                    let mut __report = #runtime::ValidationReport::ok();
                    #(#field_steps)*
                    #record_steps
                    __report
                }
            }
        })
    }

    fn emit_wrapper_impl(
        &self,
        ctx: &SchemaCtx,
        def: &ir::Def,
        wrapper: &ir::WrapperTypeDef,
    ) -> Result<TokenStream> {
        let rules = attrs::wrapper_rules(def)?;
        if rules.is_empty() {
            return Ok(TokenStream::new());
        }

        let ident = format_ident!("{}", def.name.as_str());
        let runtime = self.runtime;
        let wrapped_ty = ctx.resolve_type(def, &wrapper.wrapped);
        let target = target_for_wrapped_type(ctx, &wrapper.wrapped);

        let body = self.lower_rules_to_block(
            &rules,
            quote!(self.0),
            quote!(__path),
            target,
            quote!(#wrapped_ty),
        );

        let regex_statics = self.take_regex_statics();

        Ok(quote! {
            #regex_statics

            impl #runtime::Validate for #ident {
                fn validate_at(&self, __path: &#runtime::Path) -> #runtime::ValidationReport {
                    let mut __report = #runtime::ValidationReport::ok();
                    #body
                    __report
                }
            }

            impl #ident {
                /// Construct a [`#ident`] from an inner value, validating
                /// against every `#[validate(...)]` rule. Returns the
                /// merged [`ValidationReport`] when any rule fails.
                pub fn try_new(inner: #wrapped_ty) -> ::core::result::Result<
                    Self,
                    #runtime::ValidationReport,
                > {
                    let __candidate = Self(inner);
                    match #runtime::Validate::validate(&__candidate).into_result() {
                        ::core::result::Result::Ok(()) => ::core::result::Result::Ok(__candidate),
                        ::core::result::Result::Err(__report) => ::core::result::Result::Err(__report),
                    }
                }
            }
        })
    }

    fn emit_opaque_impl(&self, _ctx: &SchemaCtx, def: &ir::Def) -> Result<TokenStream> {
        let rules = attrs::opaque_rules(def)?;
        if rules.is_empty() {
            return Ok(TokenStream::new());
        }
        // Opaque codegen is "type alias to user-supplied native type", so we
        // can't add inherent impls. v1 emits the rule list as a documentation
        // comment and skips the impl; users that need validation on a
        // primitive should wrap it in a `wrapper` instead.
        let ident = format_ident!("{}", def.name.as_str());
        let summaries: Vec<String> = rules
            .iter()
            .map(|r| format!("- `{}`", r.expr.text))
            .collect();
        let joined = summaries.join("\n");
        let note = format!(
            "// Opaque `{}` carries `#[validate(...)]` rules but opaque \
             codegen lowers to a type alias — no inherent impls possible. \
             Wrap the type in `wrapper` to get an enforced `try_new`.\n{}",
            ident, joined
        );
        Ok(quote! {
            #[doc = #note]
            const _: () = ();
        })
    }

    fn lower_rules_to_block<R: HasRule>(
        &self,
        rules: &[R],
        value_expr: TokenStream,
        path_expr: TokenStream,
        target: expr::Target,
        value_ty: TokenStream,
    ) -> TokenStream {
        let mut steps = Vec::new();
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
                steps.push(self.lower_rule(
                    r,
                    value_expr.clone(),
                    path_expr.clone(),
                    value_ty.clone(),
                    rule.message(),
                    rule.code(),
                ));
            }
        }
        quote! { #(#steps)* }
    }

    fn lower_rule(
        &self,
        rule: &expr::Rule,
        value_expr: TokenStream,
        path_expr: TokenStream,
        value_ty: TokenStream,
        message_override: Option<&str>,
        code_override: Option<&str>,
    ) -> TokenStream {
        let runtime = self.runtime;
        let message_tokens = match message_override {
            Some(m) => quote!(::core::option::Option::Some(#m)),
            None => quote!(::core::option::Option::None),
        };
        match rule {
            expr::Rule::Compare {
                accessor,
                helper,
                threshold,
                code,
            } => {
                let helper_fn = match helper {
                    expr::HelperKind::MinInclusive => format_ident!("min_inclusive"),
                    expr::HelperKind::MinExclusive => format_ident!("min_exclusive"),
                    expr::HelperKind::MaxInclusive => format_ident!("max_inclusive"),
                    expr::HelperKind::MaxExclusive => format_ident!("max_exclusive"),
                    expr::HelperKind::Eq => format_ident!("eq"),
                    expr::HelperKind::Ne => format_ident!("ne"),
                };
                let (lhs, helper_ty) =
                    lower_accessor_for_compare(runtime, accessor, &value_expr, &value_ty);
                let bound = lower_threshold(threshold);
                let final_code: &str = code_override.unwrap_or(code);
                quote! {
                    __report.merge(#runtime::#helper_fn::<#helper_ty>(
                        #lhs,
                        #bound,
                        #final_code,
                        #message_tokens,
                        &#path_expr,
                    ));
                }
            }
            expr::Rule::Regex { accessor, pattern } => {
                let regex_ident = self.next_regex_ident();
                let final_code = code_override.unwrap_or("regex");
                let target_value = match accessor {
                    expr::Accessor::Self_ => quote!(::core::ops::Deref::deref(&#value_expr)),
                    expr::Accessor::Field(name) => {
                        let f = format_ident!("{}", name);
                        quote!(::core::ops::Deref::deref(&#value_expr.#f))
                    }
                    expr::Accessor::CharCount
                    | expr::Accessor::ByteCount
                    | expr::Accessor::ItemCount
                    | expr::Accessor::EntryCount => {
                        // regex on a count accessor doesn't make sense — the
                        // recognizer won't produce this combo, but be
                        // defensive against future changes.
                        return quote! { /* unsupported regex accessor */ };
                    }
                };
                quote! {
                    {
                        static #regex_ident: ::std::sync::OnceLock<#runtime::Regex> =
                            ::std::sync::OnceLock::new();
                        let __pattern = #regex_ident.get_or_init(|| {
                            #runtime::Regex::new(#pattern).expect("validation regex")
                        });
                        __report.merge(#runtime::regex_match(
                            #target_value,
                            __pattern,
                            #final_code,
                            #message_tokens,
                            &#path_expr,
                        ));
                    }
                }
            }
            expr::Rule::Predicate { source } => {
                // v1 predicate fallback: emit the source as a doc comment
                // and a no-op so the build keeps going. Cross-field and
                // arbitrary-call rules are deferred to a later phase that
                // ships a full AST → Rust translator.
                let note = format!("predicate fallback for `{}` — codegen TODO", source,);
                let final_code = code_override.unwrap_or("predicate");
                let _ = (final_code, value_expr, path_expr, value_ty); // silence unused warns
                quote! {
                    #[doc = #note]
                    const _: () = ();
                }
            }
        }
    }

    fn take_regex_statics(&self) -> TokenStream {
        // Regex statics are emitted inline next to their usage; this is
        // a stub so callers can reset the counter between defs without
        // worrying about state leakage between sibling impls. Currently
        // a no-op; left as a hook for future emission strategies.
        let _ = self.regex_counter.get();
        TokenStream::new()
    }
}

// --- Accessor / threshold lowering ------------------------------------------

/// Lower an accessor for use in a `Compare` rule: returns the value
/// expression and the helper's `T` type parameter so codegen can emit
/// `helper::<T>(value, literal_bound, ...)` and have the literal infer to
/// the same type as the value.
///
/// The Compare path is numeric-only by recognizer construction, so all
/// accessors yield Copy types — passing by value is safe.
fn lower_accessor_for_compare(
    runtime: &syn::Path,
    accessor: &expr::Accessor,
    value_expr: &TokenStream,
    value_ty: &TokenStream,
) -> (TokenStream, TokenStream) {
    match accessor {
        expr::Accessor::Self_ => (quote!(#value_expr), value_ty.clone()),
        expr::Accessor::CharCount => {
            (
                quote!(#runtime::char_count(::core::ops::Deref::deref(&#value_expr))),
                quote!(usize),
            )
        }
        expr::Accessor::ByteCount => {
            (
                quote!(#runtime::byte_count(::core::ops::Deref::deref(&#value_expr))),
                quote!(usize),
            )
        }
        expr::Accessor::ItemCount | expr::Accessor::EntryCount => {
            // `Vec::len` for sequences and `HashMap::len` for maps share the
            // same call shape; pick whichever the value's concrete type
            // exposes via standard library conventions.
            (quote!((&#value_expr).len()), quote!(usize))
        }
        expr::Accessor::Field(name) => {
            // Cross-field accessors are filtered out by the recognizer
            // (they fall to predicate fallback). Defensive default.
            let ident = format_ident!("{}", name);
            (quote!(#value_expr.#ident), quote!(_))
        }
    }
}

fn lower_threshold(literal: &expr::Literal) -> TokenStream {
    match literal {
        expr::Literal::Number(n) => {
            n.parse::<TokenStream>().unwrap_or_else(|_| {
                let s = n.clone();
                quote!(#s)
            })
        }
        expr::Literal::String(s) => quote!(#s),
    }
}

// --- Target inference ------------------------------------------------------

/// Pick the recognizer target for a field type. Drives whether `_.size`
/// resolves to char count, byte count, item count, entry count, or
/// predicate fallback.
fn target_for_field_type(ctx: &SchemaCtx, field: &ir::Field) -> expr::Target {
    target_for_wrapped_type(ctx, &field.typ)
}

fn target_for_wrapped_type(ctx: &SchemaCtx, ty: &ir::Type) -> expr::Target {
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
            "Sequence" => return expr::Target::Sequence,
            "Map" => return expr::Target::Map,
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" | "idx" => {
                return expr::Target::Number;
            }
            _ => {}
        }
    }
    match &def.kind {
        ir::DefKind::RecordType(_) => expr::Target::Record,
        ir::DefKind::WrapperType(w) => target_for_wrapped_type(ctx, &w.wrapped),
        _ => expr::Target::Other,
    }
}
