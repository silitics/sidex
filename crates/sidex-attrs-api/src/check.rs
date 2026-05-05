//! Lint pass for the api stability attributes.
//!
//! Walks the IR after the typed-attrs parser has run and emits diagnostics
//! for problems that aren't shape-level (those are caught earlier by the
//! attrs parser). Currently:
//!
//! - `remove_in` past the bundle's current version → error.
//! - Non-deprecated def references a deprecated def via a field/variant payload type →
//!   warning.
//! - Stable def references an unstable def → warning.
//! - Field `#[since]` older than the enclosing def's `#[since]` → warning.
//!
//! All version comparisons are best-effort: when either side fails to parse
//! as SemVer, the corresponding lint is silently skipped. Schemas often
//! outlive a single versioning scheme and we don't want to be the one to
//! force users into one.

use std::cmp::Ordering;

use sidex_diagnostics::Diagnostic;
use sidex_ir as ir;

use crate::Deprecated;
use crate::Stability;
use crate::stability_of_def;
use crate::stability_of_field;
use crate::stability_of_variant;

/// Walk every user def in `ir` and emit api-stability diagnostics. Called
/// from `sidex check` after [`sidex_attrs_validate::check`].
pub fn check(ir: &ir::Ir) {
    for (idx, def) in ir.defs.iter().enumerate() {
        let bundle_idx = ir.schemas[def.schema.idx()].bundle;
        if ir[bundle_idx].is_internal {
            continue;
        }
        check_def(ir, ir::DefIdx::from(idx), bundle_idx);
    }
}

fn check_def(ir: &ir::Ir, def_idx: ir::DefIdx, bundle_idx: ir::BundleIdx) {
    let def = &ir.defs[def_idx.idx()];
    let def_stability = stability_of_def(def);
    let bundle_version = ir[bundle_idx].metadata.version.as_str();

    if let Some(dep) = &def_stability.deprecated {
        check_remove_in(dep, &def.name.name, bundle_version, def.span.clone());
    }

    match &def.kind {
        ir::DefKind::RecordType(record) => {
            for field in &record.fields {
                let field_stability = stability_of_field(field);
                if let Some(dep) = &field_stability.deprecated {
                    check_remove_in(
                        dep,
                        &format!("{}::{}", def.name.name, field.name.name),
                        bundle_version,
                        field.span.clone(),
                    );
                }
                check_since_monotonicity(
                    &def_stability,
                    &field_stability,
                    &def.name.name,
                    &field.name.name,
                    field.span.clone(),
                );
                check_referenced_type(
                    ir,
                    &field.typ,
                    &def_stability,
                    &format!("field `{}::{}`", def.name.name, field.name.name),
                    field.span.clone(),
                );
            }
        }
        ir::DefKind::VariantType(variant) => {
            for case in &variant.variants {
                let case_stability = stability_of_variant(case);
                if let Some(dep) = &case_stability.deprecated {
                    check_remove_in(
                        dep,
                        &format!("{}::{}", def.name.name, case.name.name),
                        bundle_version,
                        case.span.clone(),
                    );
                }
                if let Some(typ) = &case.typ {
                    check_referenced_type(
                        ir,
                        typ,
                        &def_stability,
                        &format!("variant `{}::{}`", def.name.name, case.name.name),
                        case.span.clone(),
                    );
                }
            }
        }
        ir::DefKind::WrapperType(wrapper) => {
            check_referenced_type(
                ir,
                &wrapper.wrapped,
                &def_stability,
                &format!("wrapper `{}`", def.name.name),
                def.span.clone(),
            );
        }
        ir::DefKind::TypeAlias(alias) => {
            check_referenced_type(
                ir,
                &alias.aliased,
                &def_stability,
                &format!("alias `{}`", def.name.name),
                def.span.clone(),
            );
        }
        ir::DefKind::OpaqueType(_) => {}
    }
}

fn check_remove_in(
    dep: &Deprecated,
    item_label: &str,
    bundle_version: &str,
    span: Option<ir::Span>,
) {
    let Some(remove_in) = &dep.remove_in else {
        return;
    };
    if matches!(
        compare_versions(bundle_version, remove_in),
        Some(Ordering::Greater | Ordering::Equal)
    ) {
        emit(
            Diagnostic::error(format!(
                "Deprecated item `{}` was scheduled to be removed in {} \
                 (bundle version is now {}), but is still present in the schema. \
                 Remove the item or push back `remove_in`.",
                item_label, remove_in, bundle_version
            )),
            span,
        );
    }
}

fn check_since_monotonicity(
    def_stability: &Stability,
    field_stability: &Stability,
    def_name: &str,
    field_name: &str,
    span: Option<ir::Span>,
) {
    let (Some(def_since), Some(field_since)) = (&def_stability.since, &field_stability.since)
    else {
        return;
    };
    if matches!(
        compare_versions(&field_since.version, &def_since.version),
        Some(Ordering::Less)
    ) {
        emit(
            Diagnostic::warning(format!(
                "Field `{}::{}` is marked `#[since(version = \"{}\")]` but its \
                 enclosing record `{}` is `#[since(version = \"{}\")]`. The \
                 field appears to predate the record — likely a typo.",
                def_name, field_name, field_since.version, def_name, def_since.version
            )),
            span,
        );
    }
}

fn check_referenced_type(
    ir: &ir::Ir,
    typ: &ir::Type,
    referrer: &Stability,
    referrer_label: &str,
    span: Option<ir::Span>,
) {
    let resolved = ir.resolve_aliases(typ);
    let ir::TypeKind::Instance(instance) = &resolved.kind else {
        return;
    };
    let target = &ir[instance.def];
    if ir[ir.schemas[target.schema.idx()].bundle].is_internal {
        return;
    }
    let target_stability = stability_of_def(target);

    if target_stability.deprecated.is_some() && referrer.deprecated.is_none() {
        emit(
            Diagnostic::warning(format!(
                "{} uses deprecated type `{}`. Consider deprecating the \
                 referrer too, or migrating to the recommended replacement.",
                referrer_label, target.name.name
            )),
            span.clone(),
        );
    }
    if target_stability.unstable.is_some() && referrer.unstable.is_none() {
        emit(
            Diagnostic::warning(format!(
                "{} uses unstable type `{}` from a stable item. Either mark \
                 the referrer `#[unstable]` too, or wait until `{}` stabilizes.",
                referrer_label, target.name.name, target.name.name
            )),
            span,
        );
    }
}

fn emit(mut diag: Diagnostic, span: Option<ir::Span>) {
    if let Some(span) = span {
        diag = diag.with_span(Some(span));
    }
    diag.emit();
}

/// Compare two version strings under whichever shape they share. Returns
/// `None` when shapes differ (e.g. SemVer vs ISO date) or either side fails
/// to parse — schemas often outlive a single versioning scheme, so we err
/// toward silently skipping the lint rather than emitting a false positive.
///
/// Supports:
/// - SemVer (`1.4.0`, `2.0.0-beta.1`).
/// - Numeric CalVer (`2025.10`, `2025.10.31`, `25.10`) — any number of dot-separated
///   integer components, compared lexicographically as ints. `2025.10` and `2025.10.0`
///   compare equal; missing components default to 0.
/// - ISO-date CalVer (`2025-10-31`).
fn compare_versions(left: &str, right: &str) -> Option<Ordering> {
    if let (Some(l), Some(r)) = (parse_iso_date(left), parse_iso_date(right)) {
        return Some(l.cmp(&r));
    }
    if let (Some(l), Some(r)) = (
        semver::Version::parse(left).ok(),
        semver::Version::parse(right).ok(),
    ) {
        return Some(l.cmp(&r));
    }
    if let (Some(l), Some(r)) = (
        parse_numeric_components(left),
        parse_numeric_components(right),
    ) {
        return Some(compare_components(&l, &r));
    }
    None
}

fn parse_numeric_components(s: &str) -> Option<Vec<u64>> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    s.split('.').map(|p| p.parse::<u64>().ok()).collect()
}

fn compare_components(left: &[u64], right: &[u64]) -> Ordering {
    let n = left.len().max(right.len());
    for i in 0..n {
        let l = left.get(i).copied().unwrap_or(0);
        let r = right.get(i).copied().unwrap_or(0);
        match l.cmp(&r) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}

fn parse_iso_date(s: &str) -> Option<(u32, u32, u32)> {
    let s = s.trim();
    let mut parts = s.split('-');
    let y = parts.next()?.parse().ok()?;
    let m = parts.next()?.parse().ok()?;
    let d = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    Some((y, m, d))
}
