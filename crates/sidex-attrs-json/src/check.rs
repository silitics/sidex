//! Schema checks for JSON attributes after type references are resolved.

use std::collections::HashMap;

use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Label;
use sidex_diagnostics::Result;
use sidex_ir as ir;

use crate::JsonTaggedAttr;
use crate::field_attrs;
use crate::record_type_attrs;
use crate::variant_attrs;
use crate::variant_type_attrs;

/// Rejects invalid inline fields and duplicate names in flattened JSON objects.
pub fn check(unit: &ir::Ir) {
    for (idx, def) in unit.defs.iter().enumerate() {
        let result = match &def.kind {
            ir::DefKind::RecordType(record) => check_record(unit, ir::DefIdx::from(idx), record),
            ir::DefKind::VariantType(variant) => check_variant(unit, def, variant),
            _ => continue,
        };
        if let Err(error) = result {
            error.emit();
        }
    }
}

/// Checks inline restrictions locally and names across the entire flattened record.
fn check_record(unit: &ir::Ir, idx: ir::DefIdx, record: &ir::RecordTypeDef) -> Result<()> {
    for field in &record.fields {
        let attrs = field_attrs(field).map_err(|error| error.with_span(field.span.clone()))?;
        if !attrs.inline {
            continue;
        }
        if field.is_optional {
            field_error(
                field,
                "`#[json(inline)]` requires a nonoptional field.",
                "optional inline field",
            );
        }
        if resolve_record(unit, &field.typ).is_none() {
            field_error(
                field,
                "`#[json(inline)]` requires a record type, or an alias or wrapper of a record.",
                "not a record",
            );
        }
    }

    let fields = flattened_fields(unit, idx, &[], unit[idx].name.as_str(), &mut Vec::new())?;
    let mut names = HashMap::<String, JsonField>::new();
    for field in fields {
        if let Some(other) = names.get(&field.name) {
            // Duplicates within one inline field are reported when checking its record.
            if field.introducer != other.introducer {
                duplicate_name(&field, other, field.introducer_span.clone());
            }
        } else {
            names.insert(field.name.clone(), field);
        }
    }
    Ok(())
}

/// Reserves the tag name beside each internally tagged record payload.
fn check_variant(unit: &ir::Ir, def: &ir::Def, variant: &ir::VariantTypeDef) -> Result<()> {
    let attrs = variant_type_attrs(def).map_err(|error| error.with_span(def.span.clone()))?;
    if attrs.tagged != JsonTaggedAttr::Internally {
        return Ok(());
    }
    for case in &variant.variants {
        let case_attrs = variant_attrs(case).map_err(|error| error.with_span(case.span.clone()))?;
        if case_attrs.content.is_some() {
            continue;
        }
        let Some(instance) = case.typ.as_ref().and_then(|typ| resolve_record(unit, typ)) else {
            continue;
        };
        let path = format!("{}::{}", def.name.as_str(), case.name.as_str());
        let tag = JsonField {
            name: attrs.tag_field_name(),
            path: format!("{path} tag"),
            span: case.span.clone(),
            introducer: 0,
            introducer_span: case.span.clone(),
        };
        for field in flattened_fields(
            unit,
            instance.def.def,
            &instance.subst,
            &path,
            &mut Vec::new(),
        )? {
            if field.name == tag.name {
                duplicate_name(&field, &tag, field.span.clone());
            }
        }
    }
    Ok(())
}

/// One wire key, its path through inline fields, and the field introducing it.
struct JsonField {
    name: String,
    path: String,
    span: Option<ir::Span>,
    introducer: usize,
    introducer_span: Option<ir::Span>,
}

/// Collects wire keys in declaration order, substituting inline record arguments.
fn flattened_fields(
    unit: &ir::Ir,
    idx: ir::DefIdx,
    subst: &[ir::Type],
    path: &str,
    active: &mut Vec<ir::DefIdx>,
) -> Result<Vec<JsonField>> {
    // Repeated records still contribute their own keys, but following their
    // inline edges again would never terminate.
    let recursive = active.contains(&idx);
    let def = &unit[idx];
    let ir::DefKind::RecordType(record) = &def.kind else {
        unreachable!("callers resolve record types");
    };
    let attrs = record_type_attrs(def).map_err(|error| error.with_span(def.span.clone()))?;
    active.push(idx);
    let mut fields = Vec::new();
    for (introducer, field) in record.fields.iter().enumerate() {
        let field_attrs =
            field_attrs(field).map_err(|error| error.with_span(field.span.clone()))?;
        let field_path = format!("{path}.{}", field.name.as_str());
        if field_attrs.inline {
            if recursive {
                continue;
            }
            let typ = unit.apply_subst(&field.typ, subst);
            if let Some(instance) = resolve_record(unit, &typ) {
                for mut nested in
                    flattened_fields(unit, instance.def.def, &instance.subst, &field_path, active)?
                {
                    nested.introducer = introducer;
                    nested.introducer_span = field.span.clone();
                    fields.push(nested);
                }
            }
        } else {
            fields.push(JsonField {
                name: attrs.field_name(field, &field_attrs),
                path: field_path,
                span: field.span.clone(),
                introducer,
                introducer_span: field.span.clone(),
            });
        }
    }
    active.pop();
    Ok(fields)
}

/// Resolves aliases and wrappers, substituting arguments at each step.
fn resolve_record(unit: &ir::Ir, typ: &ir::Type) -> Option<ir::InstanceType> {
    let ir::TypeKind::Instance(instance) = &typ.kind else {
        return None;
    };
    let inner = match &unit[instance.def].kind {
        ir::DefKind::RecordType(_) => return Some(instance.clone()),
        ir::DefKind::TypeAlias(alias) => &alias.aliased,
        ir::DefKind::WrapperType(wrapper) => &wrapper.wrapped,
        _ => return None,
    };
    resolve_record(unit, &unit.apply_subst(inner, &instance.subst))
}

/// Reports an inline restriction at the field.
fn field_error(field: &ir::Field, message: &str, label: &str) {
    let mut diagnostic = Diagnostic::error(message).with_span(field.span.clone());
    if let Some(span) = &field.span {
        diagnostic = diagnostic.with_label(Label::new(span.clone(), label));
    }
    diagnostic.emit();
}

/// Reports both sources of a wire key at the field introducing the duplicate.
fn duplicate_name(field: &JsonField, other: &JsonField, span: Option<ir::Span>) {
    let mut diagnostic = Diagnostic::error(format!(
        "JSON field name `{}` occurs more than once: `{}` conflicts with `{}`.",
        field.name, field.path, other.path,
    ))
    .with_span(span.clone());
    if let Some(span) = span {
        diagnostic = diagnostic.with_label(Label::new(
            span,
            format!(
                "duplicate JSON field `{}` from `{}`",
                field.name, field.path
            ),
        ));
    }
    if let Some(span) = &other.introducer_span {
        diagnostic = diagnostic.with_label(Label::new(
            span.clone(),
            format!("first JSON field `{}` from `{}`", other.name, other.path),
        ));
    }
    diagnostic.emit();
}
