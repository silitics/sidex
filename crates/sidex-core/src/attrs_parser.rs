//! Parses raw source attributes against registered plugin attribute schemas
//! into typed JSON values that are stored on each IR node's `typed_attrs`.
//!
//! Scope of the current implementation:
//! - Records of optional / required primitive fields (bool, string, number, path).
//! - Multiple `#[plugin(...)]` instances on a single node merge into one record.
//! - Bare-path `#[plugin]` is accepted only when every field of the schema is optional,
//!   and produces an empty record.
//! - Unknown plugins are silently passed through (raw `attrs` remains the only
//!   representation). Unknown fields against a known schema emit a warning diagnostic and
//!   are dropped from the typed value.
//!
//! Out of scope (future passes):
//! - Sequence-typed fields.
//! - `#[attr(name = "...")]` / `#[attr(path)]` per-field knobs.

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
    // The DefRef of `core::attrs::TypeRef` (if loaded). Used to detect
    // fields whose type is the compiler-special `TypeRef` opaque so the
    // parser can resolve their source path into a typed reference.
    let type_ref_def = find_type_ref_def(ir);

    // Schema-level attrs.
    for schema_idx in 0..ir.schemas.len() {
        let attrs = ir.schemas[schema_idx].attrs.clone();
        let enclosing = ir::SchemaIdx::from(schema_idx);
        let typed = parse_node_attrs(
            &attrs,
            AttrTarget::Schema,
            registry,
            ir,
            enclosing,
            type_ref_def,
        );
        ir.schemas[schema_idx].typed_attrs = typed;
    }
    // Def / Field / Variant attrs.
    for def_idx in 0..ir.defs.len() {
        let def_attrs = ir.defs[def_idx].attrs.clone();
        let enclosing = ir.defs[def_idx].schema;
        let def_target = match &ir.defs[def_idx].kind {
            ir::DefKind::TypeAlias(_) => AttrTarget::Alias,
            ir::DefKind::OpaqueType(_) => AttrTarget::Opaque,
            ir::DefKind::RecordType(_) => AttrTarget::Record,
            ir::DefKind::VariantType(_) => AttrTarget::Variant,
            ir::DefKind::WrapperType(_) => AttrTarget::Wrapper,
        };
        // Both the kind-specific target and the generic Def target apply.
        let mut def_typed = parse_node_attrs(
            &def_attrs,
            def_target,
            registry,
            ir,
            enclosing,
            type_ref_def,
        );
        let generic = parse_node_attrs(
            &def_attrs,
            AttrTarget::Def,
            registry,
            ir,
            enclosing,
            type_ref_def,
        );
        for (plugin, value) in generic {
            def_typed.entry(plugin).or_insert(value);
        }
        ir.defs[def_idx].typed_attrs = def_typed;

        // Field / variant-case attrs.
        if let ir::DefKind::RecordType(record) = ir.defs[def_idx].kind.clone() {
            let mut new_fields = record.fields.clone();
            for (field_idx, field) in record.fields.iter().enumerate() {
                let typed = parse_node_attrs(
                    &field.attrs,
                    AttrTarget::Field,
                    registry,
                    ir,
                    enclosing,
                    type_ref_def,
                );
                new_fields[field_idx].typed_attrs = typed;
            }
            ir.defs[def_idx].kind =
                ir::DefKind::RecordType(ir::RecordTypeDef::new().with_fields(new_fields));
        }
        if let ir::DefKind::VariantType(variant) = ir.defs[def_idx].kind.clone() {
            let mut new_variants = variant.variants.clone();
            for (var_idx, var) in variant.variants.iter().enumerate() {
                let typed = parse_node_attrs(
                    &var.attrs,
                    AttrTarget::VariantCase,
                    registry,
                    ir,
                    enclosing,
                    type_ref_def,
                );
                new_variants[var_idx].typed_attrs = typed;
            }
            ir.defs[def_idx].kind =
                ir::DefKind::VariantType(ir::VariantTypeDef::new().with_variants(new_variants));
        }
    }
}

/// Locate the `core::attrs::TypeRef` def in the loaded bundles, or return
/// `None` if the attrs schema is not loaded (which we treat as "no TypeRef
/// resolution available").
fn find_type_ref_def(ir: &ir::Ir) -> Option<ir::DefRef> {
    for (bundle_idx, bundle) in ir.bundles.iter().enumerate() {
        if bundle.metadata.name != "core" {
            continue;
        }
        let bundle_idx = ir::BundleIdx::from(bundle_idx);
        for (schema_idx, schema) in ir.schemas_of(bundle_idx) {
            if schema.name != "attrs" {
                continue;
            }
            for (def_idx, def) in ir.defs_of(schema_idx) {
                if def.name.as_str() == "TypeRef" {
                    return Some(ir::DefRef::new(bundle_idx, schema_idx, def_idx));
                }
            }
        }
    }
    None
}

/// Look up a single-segment path in the IR, searching the enclosing schema
/// first, then `core::builtins`. Returns `None` if no match is found.
fn resolve_type_ref_path(
    path: &str,
    enclosing_schema: ir::SchemaIdx,
    ir: &ir::Ir,
) -> Option<ir::DefRef> {
    // Same-schema defs.
    for (def_idx, def) in ir.defs_of(enclosing_schema) {
        if def.name.as_str() == path {
            let bundle = ir.schemas[enclosing_schema.idx()].bundle;
            return Some(ir::DefRef::new(bundle, enclosing_schema, def_idx));
        }
    }
    // core::builtins fallback (covers references like `string`, `i32`, etc.).
    for (bundle_idx, bundle) in ir.bundles.iter().enumerate() {
        if bundle.metadata.name != "core" {
            continue;
        }
        let bundle_idx = ir::BundleIdx::from(bundle_idx);
        for (schema_idx, schema) in ir.schemas_of(bundle_idx) {
            if schema.name != "builtins" {
                continue;
            }
            for (def_idx, def) in ir.defs_of(schema_idx) {
                if def.name.as_str() == path {
                    return Some(ir::DefRef::new(bundle_idx, schema_idx, def_idx));
                }
            }
        }
    }
    None
}

/// Encode a `DefRef` as a JSON object with the same shape that
/// `serde_json::to_value` would produce — three integer indices.
fn def_ref_to_json(def_ref: ir::DefRef) -> Value {
    serde_json::json!({
        "bundle": def_ref.bundle.idx(),
        "schema": def_ref.schema.idx(),
        "def": def_ref.def.idx(),
    })
}

/// Returns `true` if `field`'s type resolves to `core::attrs::TypeRef`.
fn field_is_type_ref(field: &ir::Field, ir: &ir::Ir, type_ref_def: Option<ir::DefRef>) -> bool {
    let Some(type_ref) = type_ref_def else {
        return false;
    };
    matches!(
        ir.type_def_ref(&field.typ),
        Some(def) if def == type_ref
    )
}

/// Parse the typed attributes for one node (a slice of source attrs at a
/// given target position) into a per-plugin map.
fn parse_node_attrs(
    attrs: &[ir::Attr],
    target: AttrTarget,
    registry: &PluginRegistry,
    ir: &ir::Ir,
    enclosing_schema: ir::SchemaIdx,
    type_ref_def: Option<ir::DefRef>,
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
        match parse_against_schema(
            &plugin_attrs,
            schema_def,
            ir,
            enclosing_schema,
            type_ref_def,
        ) {
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
    enclosing_schema: ir::SchemaIdx,
    type_ref_def: Option<ir::DefRef>,
) -> Result<Value, Diagnostic> {
    match &def.kind {
        ir::DefKind::RecordType(record) => {
            parse_record(attrs, def, record, ir, enclosing_schema, type_ref_def)
        }
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
    enclosing_schema: ir::SchemaIdx,
    type_ref_def: Option<ir::DefRef>,
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
                return Err(Diagnostic::error("Attribute argument must have a name.")
                    .with_span(arg.span.clone()));
            };
            let Some(field) = record.fields.iter().find(|f| field_source_name(f) == name) else {
                Diagnostic::warning(format!(
                    "Unknown attribute field `{}` for schema `{}`.",
                    name,
                    def.name.as_str()
                ))
                .with_span(arg.span.clone())
                .emit();
                continue;
            };
            let value = parse_field_value(arg, field, ir, enclosing_schema, type_ref_def)?;
            object.insert(field.name.as_str().to_owned(), value);
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
/// field it's targeting. Handles:
///   * Primitive AttrValue leaves (Bool / Number / String / Path).
///   * Bool fields with bare-path source args (`#[plugin(my_flag)]` → true).
///   * Nested record-typed fields: bare path → all-default record; list `name(args)` →
///     recurse into the record's fields.
///   * Variant-typed fields with tag-only cases: source `name = path` resolves to the
///     matching variant case; output is the canonical tag as a JSON string (the shape
///     serde gives `serialize_tag`).
///   * `core::attrs::TypeRef` fields: path resolves to a `DefRef`.
fn parse_field_value(
    arg: &ir::Attr,
    field: &ir::Field,
    ir: &ir::Ir,
    enclosing_schema: ir::SchemaIdx,
    type_ref_def: Option<ir::DefRef>,
) -> Result<Value, Diagnostic> {
    if field_is_type_ref(field, ir, type_ref_def) {
        return parse_type_ref_field(arg, field, ir, enclosing_schema);
    }
    if let Some((nested_def, nested_record)) = field_record_schema(field, ir) {
        return parse_nested_record_field(
            arg,
            nested_def,
            nested_record,
            ir,
            enclosing_schema,
            type_ref_def,
        );
    }
    if let Some((variant_def, variant)) = field_variant_schema(field, ir) {
        return parse_variant_field(arg, variant_def, variant);
    }
    match &arg.kind {
        ir::AttrKind::Assign(assign) => Ok(attr_value_to_json(&assign.value)),
        ir::AttrKind::Path(_) => Ok(Value::Bool(true)),
        ir::AttrKind::List(_) => {
            Err(Diagnostic::error(format!(
                "Field `{}` is a primitive but the source attribute is a list.",
                field.name.as_str(),
            ))
            .with_span(arg.span.clone()))
        }
    }
}

/// Parse a variant-typed field. Currently supports tag-only cases only:
/// the source must be `name = <path>` where `<path>` matches one of the
/// variant's case names (case-insensitive); the JSON output is the
/// canonical case name as a string.
fn parse_variant_field(
    arg: &ir::Attr,
    def: &ir::Def,
    variant: &ir::VariantTypeDef,
) -> Result<Value, Diagnostic> {
    let ir::AttrKind::Assign(assign) = &arg.kind else {
        return Err(Diagnostic::error(format!(
            "Field `{}` expects a variant tag (e.g., `{} = SomeCase`).",
            field_name_for_diag(arg),
            field_name_for_diag(arg),
        ))
        .with_span(arg.span.clone()));
    };
    let ir::AttrValue::Path(tag) = &assign.value else {
        return Err(Diagnostic::error(format!(
            "Field `{}` expects a variant tag, not a literal value.",
            field_name_for_diag(arg),
        ))
        .with_span(arg.span.clone()));
    };
    let case = variant
        .variants
        .iter()
        .find(|c| c.name.as_str().eq_ignore_ascii_case(tag));
    let Some(case) = case else {
        let known: Vec<&str> = variant.variants.iter().map(|c| c.name.as_str()).collect();
        return Err(Diagnostic::error(format!(
            "Unknown variant case `{}` for `{}`. Expected one of: {}.",
            tag,
            def.name.as_str(),
            known.join(", ")
        ))
        .with_span(arg.span.clone()));
    };
    if case.typ.is_some() {
        return Err(Diagnostic::error(format!(
            "Variant case `{}::{}` carries a payload, which is not yet supported in attribute schemas.",
            def.name.as_str(),
            case.name.as_str()
        ))
        .with_span(arg.span.clone()));
    }
    Ok(Value::String(case.name.as_str().to_owned()))
}

fn parse_type_ref_field(
    arg: &ir::Attr,
    field: &ir::Field,
    ir: &ir::Ir,
    enclosing_schema: ir::SchemaIdx,
) -> Result<Value, Diagnostic> {
    let ir::AttrKind::Assign(assign) = &arg.kind else {
        return Err(Diagnostic::error(format!(
            "Field `{}` expects a type reference (e.g., `{} = MyType`).",
            field.name.as_str(),
            field.name.as_str(),
        ))
        .with_span(arg.span.clone()));
    };
    let ir::AttrValue::Path(path) = &assign.value else {
        return Err(Diagnostic::error(format!(
            "Field `{}` expects a type reference (e.g., `{} = MyType`).",
            field.name.as_str(),
            field.name.as_str(),
        ))
        .with_span(arg.span.clone()));
    };
    let Some(def_ref) = resolve_type_ref_path(path, enclosing_schema, ir) else {
        return Err(
            Diagnostic::error(format!("Cannot resolve type reference `{}`.", path))
                .with_span(arg.span.clone()),
        );
    };
    Ok(def_ref_to_json(def_ref))
}

fn parse_nested_record_field<'a>(
    arg: &ir::Attr,
    def: &'a ir::Def,
    record: &'a ir::RecordTypeDef,
    ir: &ir::Ir,
    enclosing_schema: ir::SchemaIdx,
    type_ref_def: Option<ir::DefRef>,
) -> Result<Value, Diagnostic> {
    match &arg.kind {
        // Bare-path: parse against an empty arg list; valid only if every
        // field in the nested record is optional.
        ir::AttrKind::Path(_) => {
            parse_record(&[arg], def, record, ir, enclosing_schema, type_ref_def)
        }
        // List form: recurse into the record's fields.
        ir::AttrKind::List(_) => {
            parse_record(&[arg], def, record, ir, enclosing_schema, type_ref_def)
        }
        ir::AttrKind::Assign(_) => {
            Err(Diagnostic::error(format!(
                "Field `{}` is a record — use `{}(...)` form, not `{} = ...`.",
                field_name_for_diag(arg),
                field_name_for_diag(arg),
                field_name_for_diag(arg),
            ))
            .with_span(arg.span.clone()))
        }
    }
}

fn field_name_for_diag(arg: &ir::Attr) -> &str {
    arg_name(arg).unwrap_or("?")
}

/// Returns the resolved record def for `field`, or `None` if `field`'s type
/// is not a record (or aliases through to one).
fn field_record_schema<'a>(
    field: &ir::Field,
    ir: &'a ir::Ir,
) -> Option<(&'a ir::Def, &'a ir::RecordTypeDef)> {
    let def_ref = ir.type_def_ref(&field.typ)?;
    let def = &ir[def_ref];
    if let ir::DefKind::RecordType(record) = &def.kind {
        Some((def, record))
    } else {
        None
    }
}

/// Returns the resolved variant def for `field`, or `None` if `field`'s
/// type is not a variant.
fn field_variant_schema<'a>(
    field: &ir::Field,
    ir: &'a ir::Ir,
) -> Option<(&'a ir::Def, &'a ir::VariantTypeDef)> {
    let def_ref = ir.type_def_ref(&field.typ)?;
    let def = &ir[def_ref];
    if let ir::DefKind::VariantType(variant) = &def.kind {
        Some((def, variant))
    } else {
        None
    }
}

/// Source-side name for a schema field. Defaults to the field's Sidex
/// identifier; a `#[attr(name = "<source>")]` annotation on the field
/// overrides it. This lets schemas use a Sidex-friendly identifier (e.g.
/// `typ`) while accepting source attributes that use a different name
/// (e.g. `#[json(type = "...")]`).
fn field_source_name(field: &ir::Field) -> &str {
    for attr in &field.attrs {
        let ir::AttrKind::List(list) = &attr.kind else {
            continue;
        };
        if list.path != "attr" {
            continue;
        }
        for arg in &list.args {
            let ir::AttrKind::Assign(assign) = &arg.kind else {
                continue;
            };
            if assign.path != "name" {
                continue;
            }
            if let ir::AttrValue::String(s) = &assign.value {
                return s;
            }
        }
    }
    field.name.as_str()
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
            import ::core::attrs::*

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
    fn type_ref_field_resolves_to_def_ref() {
        let src = r#"
            import ::core::attrs::*

            #[attrs(plugin = "request", target = record)]
            record RequestAttrs {
                response?: TypeRef,
            }

            record ResponseType {}

            #[request(response = ResponseType)]
            record Endpoint {}
        "#;
        let ir = build_ir(src);
        let endpoint = ir
            .defs
            .iter()
            .find(|d| d.name.as_str() == "Endpoint")
            .expect("Endpoint def not found");
        let request = endpoint
            .typed_attrs
            .get("request")
            .expect("typed_attrs[request] should be populated");
        let response = &request["response"];
        // The DefRef should resolve to the ResponseType def in the same schema.
        let response_def = ir
            .defs
            .iter()
            .position(|d| d.name.as_str() == "ResponseType")
            .expect("ResponseType def not found");
        assert_eq!(response["def"], serde_json::json!(response_def));
    }

    /// End-to-end: a downstream consumer of the IR who has a typed Rust
    /// struct (here hand-coded; in practice generated from the same Sidex
    /// schema with serde derives) can deserialize a node's typed_attrs
    /// directly via serde_json::from_value. No re-parsing of raw `attrs`
    /// needed.
    #[test]
    fn round_trips_through_serde_into_typed_struct() {
        #[derive(serde::Deserialize, Debug, PartialEq, Eq, Default)]
        struct DemoAttrs {
            #[serde(default)]
            greeting: Option<String>,
            #[serde(default)]
            enabled: Option<bool>,
        }

        let src = r#"
            import ::core::attrs::*

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
        let value = target_def
            .typed_attrs
            .get("demo")
            .expect("typed_attrs[demo] populated");
        let typed: DemoAttrs =
            serde_json::from_value(value.clone()).expect("deserializes into typed struct");
        assert_eq!(
            typed,
            DemoAttrs {
                greeting: Some("hello".to_owned()),
                enabled: Some(true),
            }
        );
    }

    /// The `pub(crate)` / `pub(super)` Rust-attribute pattern, expressed as
    /// a nested-record field. `#[rust(pub)]` populates the field with a
    /// default record; `#[rust(pub(crate))]` recursively parses `crate` as
    /// a bare-path bool flag inside the record.
    #[test]
    fn nested_record_field_models_pub_crate() {
        let src = r#"
            import ::core::attrs::*

            record PubVis {
                isCrate?: bool,
                isSuper?: bool,
            }

            #[attrs(plugin = "rust", target = field)]
            record FieldAttrs {
                pub?: PubVis,
            }

            record Target {
                #[rust(pub)]
                bare_pub: string,

                #[rust(pub(isCrate))]
                pub_crate: string,

                #[rust(pub(isSuper))]
                pub_super: string,
            }
        "#;
        let ir = build_ir(src);
        let target_def = ir
            .defs
            .iter()
            .find(|d| d.name.as_str() == "Target")
            .expect("Target def not found");
        let ir::DefKind::RecordType(record) = &target_def.kind else {
            panic!("Target is not a record");
        };
        let pub_of = |field_name: &str| -> serde_json::Value {
            let field = record
                .fields
                .iter()
                .find(|f| f.name.as_str() == field_name)
                .expect("field not found");
            field
                .typed_attrs
                .get("rust")
                .expect("rust typed_attrs populated")
                .get("pub")
                .expect("pub field present")
                .clone()
        };
        // bare `#[rust(pub)]` → empty record.
        assert_eq!(pub_of("bare_pub"), serde_json::json!({}));
        // `#[rust(pub(isCrate))]` → record with `isCrate: true`.
        assert_eq!(pub_of("pub_crate"), serde_json::json!({"isCrate": true}));
        // `#[rust(pub(isSuper))]` → record with `isSuper: true`.
        assert_eq!(pub_of("pub_super"), serde_json::json!({"isSuper": true}));
    }

    /// Variant-typed fields (with tag-only cases) accept a path as the
    /// source value and emit the canonical case name as a JSON string —
    /// the shape serde produces for unit variants without a `#[json(...)]`
    /// override.
    #[test]
    fn variant_field_accepts_tag_only_cases() {
        let src = r#"
            import ::core::attrs::*

            variant Mode {
                Internally,
                Externally,
                Adjacently,
                Implicitly,
            }

            #[attrs(plugin = "json", target = variant)]
            record VariantTypeAttrs {
                tagged?: Mode,
            }

            // Source uses lowercase tag; matching is case-insensitive,
            // canonical PascalCase comes through in the typed value.
            #[json(tagged = adjacently)]
            variant Either {
                Left,
                Right,
            }
        "#;
        let ir = build_ir(src);
        let either = ir
            .defs
            .iter()
            .find(|d| d.name.as_str() == "Either")
            .expect("Either def not found");
        let json_attrs = either
            .typed_attrs
            .get("json")
            .expect("typed_attrs[json] populated");
        assert_eq!(json_attrs["tagged"], serde_json::json!("Adjacently"));
    }

    #[test]
    fn merges_repeated_plugin_attrs() {
        let src = r#"
            import ::core::attrs::*

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
