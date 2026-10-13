//! Schema checks for JSON attributes after type references are resolved.

use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Label;
use sidex_ir as ir;

use crate::field_attrs;

/// Rejects inline fields whose types do not resolve to records.
pub fn check(unit: &ir::Ir) {
    for def in &unit.defs {
        let ir::DefKind::RecordType(record) = &def.kind else {
            continue;
        };
        for field in &record.fields {
            let attrs = match field_attrs(field) {
                Ok(attrs) => attrs,
                Err(error) => {
                    error.with_span(field.span.clone()).emit();
                    continue;
                }
            };
            if attrs.inline && !is_record(unit, &field.typ) {
                let mut diagnostic = Diagnostic::error(
                    "`#[json(inline)]` requires a record type, or an alias or wrapper of a record.",
                )
                .with_span(field.span.clone());
                if let Some(span) = &field.span {
                    diagnostic = diagnostic.with_label(Label::new(span.clone(), "not a record"));
                }
                diagnostic.emit();
            }
        }
    }
}

/// Resolves aliases and wrappers, substituting arguments at each step.
fn is_record(unit: &ir::Ir, typ: &ir::Type) -> bool {
    let ir::TypeKind::Instance(instance) = &typ.kind else {
        return false;
    };
    let inner = match &unit[instance.def].kind {
        ir::DefKind::RecordType(_) => return true,
        ir::DefKind::TypeAlias(alias) => &alias.aliased,
        ir::DefKind::WrapperType(wrapper) => &wrapper.wrapped,
        _ => return false,
    };
    is_record(unit, &unit.apply_subst(inner, &instance.subst))
}
