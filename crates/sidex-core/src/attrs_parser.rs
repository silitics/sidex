//! Parses raw source attributes against registered plugin attribute schemas
//! into typed JSON values that are stored on each IR node's `typed_attrs`.
//!
//! Scope of the current implementation:
//! - Records of optional / required primitive fields (bool, string, number, path).
//! - Multiple `#[plugin(...)]` instances on a single node merge into one record.
//! - Bare-path `#[plugin]` is accepted only when every field of the schema is
//!   optional, and produces an empty record.
//! - Unknown plugins are silently passed through (raw `attrs` remains the
//!   only representation). Unknown fields against a known schema emit a
//!   warning diagnostic and are dropped from the typed value.
//!
//! Out of scope (future passes):
//! - Variant schemas, sequence fields, nested record fields.
//! - `#[attr(name = "...")]` / `#[attr(flag)]` / `#[attr(path)]` /
//!   `#[attr(from_string)]` per-field knobs.
//! - `core::attrs::TypeRef` resolution.

use std::collections::HashMap;

use serde_json::Value;
use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Label;
use sidex_ir as ir;

use crate::attrs_meta::AttrTarget;
use crate::attrs_meta::PluginRegistry;

/// Walk every attr-bearing node in `ir` and populate its `typed_attrs` from
/// the source attributes. Diagnostics are emitted into the active context.
pub fn populate_typed_attrs(ir: &mut ir::Ir, registry: &PluginRegistry) {
    if registry.is_empty() {
        return;
    }
    // Schema-level attrs.
    for schema_idx in 0..ir.schemas.len() {
        let attrs = ir.schemas[schema_idx].attrs.clone();
        let typed = parse_node_attrs(&attrs, AttrTarget::Schema, registry, ir);
        ir.schemas[schema_idx].typed_attrs = typed;
    }
    // Def / Field / Variant attrs.
    for def_idx in 0..ir.defs.len() {
        let def_attrs = ir.defs[def_idx].attrs.clone();
        let def_target = match &ir.defs[def_idx].kind {
            ir::DefKind::TypeAlias(_) => AttrTarget::Alias,
            ir::DefKind::OpaqueType(_) => AttrTarget::Opaque,
            ir::DefKind::RecordType(_) => AttrTarget::Record,
            ir::DefKind::VariantType(_) => AttrTarget::Variant,
            ir::DefKind::WrapperType(_) => AttrTarget::Wrapper,
        };
        // Both the kind-specific target and the generic Def target apply.
        let mut def_typed =
            parse_node_attrs(&def_attrs, def_target, registry, ir);
        let generic = parse_node_attrs(&def_attrs, AttrTarget::Def, registry, ir);
        for (plugin, value) in generic {
            def_typed.entry(plugin).or_insert(value);
        }
        ir.defs[def_idx].typed_attrs = def_typed;

        // Field / variant-case attrs.
        if let ir::DefKind::RecordType(record) = ir.defs[def_idx].kind.clone() {
            let mut new_fields = record.fields.clone();
            for (field_idx, field) in record.fields.iter().enumerate() {
                let typed = parse_node_attrs(&field.attrs, AttrTarget::Field, registry, ir);
                new_fields[field_idx].typed_attrs = typed;
            }
            ir.defs[def_idx].kind = ir::DefKind::RecordType(
                ir::RecordTypeDef::new().with_fields(new_fields),
            );
        }
        if let ir::DefKind::VariantType(variant) = ir.defs[def_idx].kind.clone() {
            let mut new_variants = variant.variants.clone();
            for (var_idx, var) in variant.variants.iter().enumerate() {
                let typed =
                    parse_node_attrs(&var.attrs, AttrTarget::VariantCase, registry, ir);
                new_variants[var_idx].typed_attrs = typed;
            }
            ir.defs[def_idx].kind = ir::DefKind::VariantType(
                ir::VariantTypeDef::new().with_variants(new_variants),
            );
        }
    }
}

/// Parse the typed attributes for one node (a slice of source attrs at a
/// given target position) into a per-plugin map.
fn parse_node_attrs(
    attrs: &[ir::Attr],
    target: AttrTarget,
    registry: &PluginRegistry,
    ir: &ir::Ir,
) -> HashMap<String, Value> {
    if attrs.is_empty() {
        return HashMap::new();
    }
    // Group source attrs by their outer plugin id (the path of the outermost
    // List or Path attribute).
    let mut by_plugin: HashMap<&str, Vec<&ir::Attr>> = HashMap::new();
    for attr in attrs {
        let Some(plugin) = outer_plugin(attr) else {
            continue;
        };
        // The `attrs` and `attr` meta-attributes are processed by the
        // compiler itself; they don't end up in `typed_attrs`.
        if plugin == "attrs" || plugin == "attr" {
            continue;
        }
        by_plugin.entry(plugin).or_default().push(attr);
    }

    let mut typed = HashMap::new();
    for (plugin, plugin_attrs) in by_plugin {
        let Some(schema_ref) = registry.get(plugin, target) else {
            // Unknown plugin for this target — leave raw attrs as the only
            // representation. This is intentionally silent; tools like
            // `sidex check` could opt in to a stricter mode later.
            continue;
        };
        let schema_def = &ir[schema_ref];
        match parse_against_schema(&plugin_attrs, schema_def, ir) {
            Ok(value) => {
                typed.insert(plugin.to_owned(), value);
            }
            Err(diag) => {
                diag.emit();
            }
        }
    }
    typed
}

/// Return the outer plugin id of an attribute, or `None` for bare assigns.
fn outer_plugin(attr: &ir::Attr) -> Option<&str> {
    match &attr.kind {
        ir::AttrKind::Path(p) => Some(p.as_str()),
        ir::AttrKind::List(l) => Some(l.path.as_str()),
        ir::AttrKind::Assign(_) => None,
    }
}

/// Parse the merged contents of a plugin's attributes on a node against the
/// plugin's schema def, returning a JSON value matching the schema.
fn parse_against_schema(
    attrs: &[&ir::Attr],
    def: &ir::Def,
    ir: &ir::Ir,
) -> Result<Value, Diagnostic> {
    match &def.kind {
        ir::DefKind::RecordType(record) => parse_record(attrs, def, record, ir),
        // TODO(typed-attrs): variant / opaque / alias / wrapper schemas.
        _ => Err(Diagnostic::error(format!(
            "Plugin attribute schema `{}` must be a record. Variants and other shapes are not supported yet.",
            def.name.as_str()
        )).with_span(attrs.first().and_then(|a| a.span.clone()))),
    }
}

/// Parse a list of plugin attributes against a record-shaped schema.
fn parse_record(
    plugin_attrs: &[&ir::Attr],
    def: &ir::Def,
    record: &ir::RecordTypeDef,
    ir: &ir::Ir,
) -> Result<Value, Diagnostic> {
    let mut object = serde_json::Map::new();
    let all_optional = record.fields.iter().all(|f| f.is_optional);

    for plugin_attr in plugin_attrs {
        let args: Vec<&ir::Attr> = match &plugin_attr.kind {
            ir::AttrKind::Path(_) => {
                if all_optional {
                    Vec::new()
                } else {
                    return Err(Diagnostic::error(format!(
                        "`{}` requires arguments — schema `{}` has required fields.",
                        outer_plugin(plugin_attr).unwrap_or("?"),
                        def.name.as_str(),
                    ))
                    .with_span(plugin_attr.span.clone())
                    .with_label(Label::new(
                        plugin_attr
                            .span
                            .clone()
                            .unwrap_or_else(|| ir::Span::new(0.into(), 0, 0)),
                        "expected `(...)` form",
                    )));
                }
            }
            ir::AttrKind::List(list) => list.args.iter().collect(),
            ir::AttrKind::Assign(_) => {
                return Err(Diagnostic::error(
                    "Top-level plugin attribute must be a list, not an assignment.",
                )
                .with_span(plugin_attr.span.clone()));
            }
        };

        for arg in args {
            let Some(name) = arg_name(arg) else {
                return Err(Diagnostic::error(
                    "Attribute argument must have a name.",
                )
                .with_span(arg.span.clone()));
            };
            let Some(field) = record.fields.iter().find(|f| f.name.as_str() == name) else {
                Diagnostic::warning(format!(
                    "Unknown attribute field `{}` for schema `{}`.",
                    name,
                    def.name.as_str()
                ))
                .with_span(arg.span.clone())
                .emit();
                continue;
            };
            let value = parse_field_value(arg, field, ir)?;
            object.insert(name.to_owned(), value);
        }
    }

    Ok(Value::Object(object))
}

/// Extract the name of an attribute argument, regardless of kind.
fn arg_name(arg: &ir::Attr) -> Option<&str> {
    match &arg.kind {
        ir::AttrKind::Path(p) => Some(p.as_str()),
        ir::AttrKind::List(l) => Some(l.path.as_str()),
        ir::AttrKind::Assign(a) => Some(a.path.as_str()),
    }
}

/// Parse a single attribute argument's value into JSON, given the schema
/// field it's targeting. The current implementation handles primitive
/// `AttrValue` leaves and bare-path "flag" args for `bool` fields.
fn parse_field_value(
    arg: &ir::Attr,
    _field: &ir::Field,
    _ir: &ir::Ir,
) -> Result<Value, Diagnostic> {
    match &arg.kind {
        ir::AttrKind::Assign(assign) => Ok(attr_value_to_json(&assign.value)),
        ir::AttrKind::Path(_) => {
            // Bare-path argument — treat as a `bool` flag set to true.
            Ok(Value::Bool(true))
        }
        ir::AttrKind::List(_) => Err(Diagnostic::error(
            "Nested list attributes are not yet supported as field values.",
        )
        .with_span(arg.span.clone())),
    }
}

fn attr_value_to_json(value: &ir::AttrValue) -> Value {
    match value {
        ir::AttrValue::Bool(b) => Value::Bool(*b),
        ir::AttrValue::Number(n) => {
            n.parse::<i64>()
                .map(Value::from)
                .or_else(|_| n.parse::<f64>().map(Value::from))
                .unwrap_or_else(|_| Value::String(n.clone()))
        }
        ir::AttrValue::String(s) => Value::String(s.clone()),
        ir::AttrValue::Path(p) => Value::String(p.clone()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sidex_diagnostics::DiagnosticCtx;

    use super::*;
    use crate::bundle::BundleSource;
    use crate::bundle::Manifest;
    use crate::transformer::Transformer;

    fn build_ir(source: &str) -> ir::Ir {
        let mut transformer = Transformer::new();
        let source_id = transformer.insert_source(source.to_owned(), None);
        let mut schemas: HashMap<String, ir::SourceIdx> = HashMap::new();
        schemas.insert("main".to_owned(), source_id);
        let manifest = Manifest::new(ir::Metadata::new("demo".to_owned(), "0".to_owned()));
        let bundle_idx = transformer
            .insert_bundle(BundleSource {
                manifest,
                path: None,
                schemas,
            })
            .unwrap();
        let ctx = DiagnosticCtx::new();
        ctx.exec(|| transformer.transform(bundle_idx))
    }

    #[test]
    fn registers_and_parses_typed_attrs() {
        let src = r#"
            #[attrs(plugin = "demo", target = record)]
            record DemoAttrs {
                greeting?: string,
                enabled?: bool,
            }

            #[demo(greeting = "hello", enabled = true)]
            record Target {}
        "#;
        let ir = build_ir(src);
        let target_def = ir
            .defs
            .iter()
            .find(|d| d.name.as_str() == "Target")
            .expect("Target def not found");
        let demo = target_def
            .typed_attrs
            .get("demo")
            .expect("typed_attrs[demo] should be populated");
        assert_eq!(demo["greeting"], serde_json::json!("hello"));
        assert_eq!(demo["enabled"], serde_json::json!(true));
    }

    #[test]
    fn unknown_plugins_are_silently_skipped() {
        let src = r#"
            #[unknown(foo = "bar")]
            record Target {}
        "#;
        let ir = build_ir(src);
        let target_def = ir
            .defs
            .iter()
            .find(|d| d.name.as_str() == "Target")
            .expect("Target def not found");
        assert!(
            target_def.typed_attrs.is_empty(),
            "unknown plugins must not appear in typed_attrs"
        );
        assert!(
            !target_def.attrs.is_empty(),
            "raw attrs must still be retained"
        );
    }

    #[test]
    fn merges_repeated_plugin_attrs() {
        let src = r#"
            #[attrs(plugin = "demo", target = record)]
            record DemoAttrs {
                greeting?: string,
                enabled?: bool,
            }

            #[demo(greeting = "hi")]
            #[demo(enabled = true)]
            record Target {}
        "#;
        let ir = build_ir(src);
        let target_def = ir
            .defs
            .iter()
            .find(|d| d.name.as_str() == "Target")
            .expect("Target def not found");
        let demo = target_def.typed_attrs.get("demo").unwrap();
        assert_eq!(demo["greeting"], serde_json::json!("hi"));
        assert_eq!(demo["enabled"], serde_json::json!(true));
    }
}
