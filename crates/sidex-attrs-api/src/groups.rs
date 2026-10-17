//! Group membership of definitions.
//!
//! A bundle declares groups in its manifest, and `#[group(name = "...")]`
//! assigns a definition to one of them. A group's promise covers every type
//! its definitions use, so membership extends to all definitions of the same
//! bundle that a member uses: field types, variant payloads, wrapped and
//! aliased types, type arguments, and the response of a `#[request(...)]`.
//! The response is the only type reference in a typed attribute that membership
//! follows because it is the only one describing data a member's consumers
//! exchange. Membership stops at bundle boundaries because group declarations
//! belong to a bundle.

use std::collections::BTreeSet;
use std::collections::HashMap;

use serde::Deserialize;
use sidex::formats::json;
use sidex::types::JsonValue;
use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Label;
use sidex_ir as ir;

use crate::raw;

const PLUGIN_GROUP: &str = "group";
const PLUGIN_REQUEST: &str = "request";

/// Set [`ir::Def::groups`] for every definition in `ir`.
///
/// Reports an error for each `#[group(...)]` naming a group that the
/// definition's bundle does not declare.
pub fn assign_groups(ir: &mut ir::Ir) {
    let mut groups = vec![BTreeSet::new(); ir.defs.len()];
    for (idx, def) in ir.defs.iter().enumerate() {
        let bundle = ir.schemas[def.schema.idx()].bundle;
        for name in assigned_groups(&def.typed_attrs) {
            if !ir[bundle].groups.contains_key(&name) {
                report_undeclared_group(&ir[bundle], def, &name);
                continue;
            }
            extend_membership(ir, ir::DefIdx::from(idx), bundle, &name, &mut groups);
        }
    }
    for (def, groups) in ir.defs.iter_mut().zip(groups) {
        def.groups = groups.into_iter().collect();
    }
}

/// Render a Markdown paragraph for each group of `def`, in name order,
/// suitable for prepending to the rendered doc comment. Returns `""` when
/// `def` belongs to no group.
pub fn render_group_prelude(ir: &ir::Ir, def: &ir::Def) -> String {
    let declared = &ir[ir.schemas[def.schema.idx()].bundle].groups;
    let mut out = String::new();
    for name in &def.groups {
        let group = declared
            .get(name)
            .expect("`assign_groups` only assigns groups declared by the definition's bundle");
        out.push_str(&format!(
            "**Group `{name}`:** {}\n\n",
            group.description.trim()
        ));
    }
    out
}

/// The part of a `#[request(...)]` attribute that membership follows.
#[derive(Deserialize)]
struct RequestResponse {
    #[serde(with = "sidex::serde")]
    response: ir::DefRef,
}

/// Report that `def` names the group `name`, which `bundle` does not declare.
fn report_undeclared_group(bundle: &ir::Bundle, def: &ir::Def, name: &str) {
    let mut diagnostic = Diagnostic::error(format!(
        "Group `{name}` is not declared in the manifest of bundle `{}`.",
        bundle.metadata.name
    ))
    .with_help(format!(
        "Declare the group with a `[groups.{name}]` table in `sidex.toml`."
    ));
    if let Some(span) = &def.span {
        diagnostic = diagnostic
            .with_span(Some(span.clone()))
            .with_label(Label::new(span.clone(), "assigned to an undeclared group"));
    }
    diagnostic.emit();
}

/// Names of the groups assigned with `#[group(...)]`.
fn assigned_groups(typed_attrs: &HashMap<String, JsonValue>) -> Vec<String> {
    typed_attrs
        .get(PLUGIN_GROUP)
        .and_then(|value| json::from_value::<Vec<raw::DefGroup>>(value).ok())
        .into_iter()
        .flatten()
        .map(|group| group.name)
        .collect()
}

/// Add `name` to `root` and every definition of `bundle` reachable from it.
fn extend_membership(
    ir: &ir::Ir,
    root: ir::DefIdx,
    bundle: ir::BundleIdx,
    name: &str,
    groups: &mut [BTreeSet<String>],
) {
    let mut pending = vec![root];
    while let Some(idx) = pending.pop() {
        if !groups[idx.idx()].insert(name.to_owned()) {
            continue;
        }
        pending.extend(
            used_defs(&ir[idx])
                .into_iter()
                .filter(|def_ref| def_ref.bundle == bundle)
                .map(|def_ref| def_ref.def),
        );
    }
}

/// Collect the definitions that `def` uses directly.
fn used_defs(def: &ir::Def) -> Vec<ir::DefRef> {
    let mut used = Vec::new();
    match &def.kind {
        ir::DefKind::RecordType(record) => {
            for field in &record.fields {
                collect_type_defs(&field.typ, &mut used);
            }
        }
        ir::DefKind::VariantType(variant) => {
            for typ in variant.variants.iter().filter_map(|case| case.typ.as_ref()) {
                collect_type_defs(typ, &mut used);
            }
        }
        ir::DefKind::WrapperType(wrapper) => collect_type_defs(&wrapper.wrapped, &mut used),
        ir::DefKind::TypeAlias(alias) => collect_type_defs(&alias.aliased, &mut used),
        ir::DefKind::OpaqueType(_) => {}
    }
    if let Some(request) = def.typed_attrs.get(PLUGIN_REQUEST)
        && let Ok(request) =
            serde_json::from_value::<RequestResponse>(serde_json::Value::from(request))
    {
        used.push(request.response);
    }
    used
}

/// Collect the definition of `typ` and of its type arguments.
fn collect_type_defs(typ: &ir::Type, out: &mut Vec<ir::DefRef>) {
    if let ir::TypeKind::Instance(instance) = &typ.kind {
        out.push(instance.def);
        for arg in &instance.subst {
            collect_type_defs(arg, out);
        }
    }
}
