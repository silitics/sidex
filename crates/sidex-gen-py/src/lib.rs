//! Sidex code generation for Python with pydantic compatibility.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sidex_attrs_json::{
    JsonFieldAttrs, JsonOpaqueTypeAttrs, JsonRecordTypeAttrs, JsonVariantAttrs,
    JsonVariantTypeAttrs, atoms::JsonTaggedAttr, types::JsonType,
};
use sidex_attrs_py::PyOpaqueTypeAttrs;
use sidex_gen::{
    Generator, Job,
    attrs::TryFromAttrs,
    diagnostics::{self, Result},
    ir,
    rename::to_snake_case,
};

/// Configuration for the Python backend.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub types: TypesConfig,
    #[serde(default)]
    pub external: HashMap<String, String>,
}

/// Type mapping configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypesConfig {
    #[serde(default)]
    pub table: HashMap<String, String>,
}

impl Default for TypesConfig {
    fn default() -> Self {
        let mut cfg = Self {
            table: Default::default(),
        };
        cfg.populate_builtins();
        cfg
    }
}

impl TypesConfig {
    fn populate_builtins(&mut self) {
        for (path, py_type) in [
            ("::core::builtins::string", "str"),
            ("::core::builtins::bytes", "bytes"),
            ("::core::builtins::i8", "int"),
            ("::core::builtins::i16", "int"),
            ("::core::builtins::i32", "int"),
            ("::core::builtins::i64", "int"),
            ("::core::builtins::u8", "int"),
            ("::core::builtins::u16", "int"),
            ("::core::builtins::u32", "int"),
            ("::core::builtins::u64", "int"),
            ("::core::builtins::idx", "int"),
            ("::core::builtins::f32", "float"),
            ("::core::builtins::f64", "float"),
            ("::core::builtins::bool", "bool"),
            ("::core::builtins::unit", "None"),
            ("::core::builtins::Sequence", "list"),
            ("::core::builtins::Map", "dict"),
        ] {
            self.table
                .entry(path.to_owned())
                .or_insert_with(|| py_type.to_owned());
        }
    }
}

struct BundleCtx<'cx> {
    cfg: &'cx Config,
    unit: &'cx ir::Unit,
    bundle: &'cx ir::Bundle,
}

struct SchemaCtx<'cx> {
    bundle_ctx: &'cx BundleCtx<'cx>,
    schema: &'cx ir::Schema,
}

impl<'cx> SchemaCtx<'cx> {
    /// Resolves a Sidex type to a Python type expression string.
    fn resolve_type(&self, def: &ir::Def, typ: &ir::Type) -> String {
        match &typ.kind {
            ir::TypeKind::TypeVar(var) => def[var.idx].name.as_str().to_owned(),
            ir::TypeKind::Instance(instance) => {
                let bundle = &self.bundle_ctx.unit[instance.bundle];
                let schema = &bundle[instance.schema];
                let instance_def = &schema[instance.def];

                let qualified = format!(
                    "::{}::{}::{}",
                    bundle.metadata.name,
                    schema.name,
                    instance_def.name.as_str()
                );

                let base = if let Some(mapped) = self.bundle_ctx.cfg.types.table.get(&qualified) {
                    mapped.clone()
                } else if instance.bundle == self.bundle_ctx.bundle.idx {
                    if instance.schema == self.schema.idx {
                        instance_def.name.as_str().to_owned()
                    } else {
                        format!("_schema_{}.{}", schema.name, instance_def.name.as_str())
                    }
                } else {
                    let ext = self
                        .bundle_ctx
                        .cfg
                        .external
                        .get(&bundle.metadata.name)
                        .map(String::as_str)
                        .unwrap_or(&bundle.metadata.name);
                    format!("{ext}.{}.{}", schema.name, instance_def.name.as_str())
                };

                if instance.subst.is_empty() {
                    base
                } else {
                    let args: Vec<_> = instance
                        .subst
                        .iter()
                        .map(|t| self.resolve_type(def, t))
                        .collect();
                    format!("{base}[{}]", args.join(", "))
                }
            }
        }
    }
}

struct PyWriter {
    indent: usize,
    buf: String,
}

impl PyWriter {
    fn new() -> Self {
        Self {
            indent: 0,
            buf: String::new(),
        }
    }

    fn line(&mut self, text: &str) {
        for _ in 0..self.indent {
            self.buf.push_str("    ");
        }
        self.buf.push_str(text);
        self.buf.push('\n');
    }

    fn blank(&mut self) {
        self.buf.push('\n');
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn dedent(&mut self) {
        self.indent -= 1;
    }

    fn finish(self) -> String {
        self.buf
    }
}

fn generic_params(def: &ir::Def) -> String {
    if def.vars.is_empty() {
        String::new()
    } else {
        let vars: Vec<_> = def.vars.iter().map(|v| v.name.as_str()).collect();
        format!("[{}]", vars.join(", "))
    }
}

fn generic_bases(def: &ir::Def) -> String {
    if def.vars.is_empty() {
        String::new()
    } else {
        let vars: Vec<_> = def.vars.iter().map(|v| v.name.as_str()).collect();
        format!(", Generic[{}]", vars.join(", "))
    }
}

fn collect_type_var_indices(typ: &ir::Type, out: &mut Vec<ir::TypeVarIdx>) {
    match &typ.kind {
        ir::TypeKind::TypeVar(var) => {
            if !out.contains(&var.idx) {
                out.push(var.idx);
            }
        }
        ir::TypeKind::Instance(inst) => {
            for sub in &inst.subst {
                collect_type_var_indices(sub, out);
            }
        }
    }
}

fn variant_used_vars(def: &ir::Def, typ: Option<&ir::Type>) -> Vec<String> {
    let Some(typ) = typ else {
        return Vec::new();
    };
    let mut indices = Vec::new();
    collect_type_var_indices(typ, &mut indices);
    indices.sort_by_key(|idx| idx.idx());
    indices
        .iter()
        .map(|idx| def[*idx].name.as_str().to_owned())
        .collect()
}

/// Collects the schema indices referenced by a type (same bundle only).
fn collect_referenced_schemas(
    typ: &ir::Type,
    bundle_idx: ir::BundleIdx,
    current_schema: ir::SchemaIdx,
    out: &mut Vec<ir::SchemaIdx>,
) {
    match &typ.kind {
        ir::TypeKind::TypeVar(_) => {}
        ir::TypeKind::Instance(inst) => {
            if inst.bundle == bundle_idx && inst.schema != current_schema && !out.contains(&inst.schema) {
                out.push(inst.schema);
            }
            for sub in &inst.subst {
                collect_referenced_schemas(sub, bundle_idx, current_schema, out);
            }
        }
    }
}

/// Collects all schema indices referenced by definitions in a schema.
fn referenced_schemas(schema: &ir::Schema, bundle_idx: ir::BundleIdx) -> Vec<ir::SchemaIdx> {
    let mut refs = Vec::new();
    for def in &schema.defs {
        match &def.kind {
            ir::DefKind::TypeAlias(a) => {
                collect_referenced_schemas(&a.aliased, bundle_idx, schema.idx, &mut refs);
            }
            ir::DefKind::RecordType(r) => {
                for field in &r.fields {
                    collect_referenced_schemas(&field.typ, bundle_idx, schema.idx, &mut refs);
                }
            }
            ir::DefKind::VariantType(v) => {
                for variant in &v.variants {
                    if let Some(typ) = &variant.typ {
                        collect_referenced_schemas(typ, bundle_idx, schema.idx, &mut refs);
                    }
                }
            }
            ir::DefKind::WrapperType(w) => {
                collect_referenced_schemas(&w.wrapped, bundle_idx, schema.idx, &mut refs);
            }
            _ => {}
        }
    }
    refs
}

fn generic_bases_for(var_names: &[String]) -> String {
    if var_names.is_empty() {
        String::new()
    } else {
        format!(", Generic[{}]", var_names.join(", "))
    }
}

fn subscript_for(var_names: &[String]) -> String {
    if var_names.is_empty() {
        String::new()
    } else {
        format!("[{}]", var_names.join(", "))
    }
}

/// Appends `_` if the name is a Python hard keyword.
fn sanitize_py_name(name: &str) -> String {
    match name {
        "False" | "None" | "True" | "and" | "as" | "assert" | "async" | "await" | "break"
        | "class" | "continue" | "def" | "del" | "elif" | "else" | "except" | "finally"
        | "for" | "from" | "global" | "if" | "import" | "in" | "is" | "lambda" | "nonlocal"
        | "not" | "or" | "pass" | "raise" | "return" | "try" | "while" | "with" | "yield" => {
            format!("{name}_")
        }
        _ => name.to_owned(),
    }
}

fn escape_docstring(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace("\"\"\"", "\\\"\\\"\\\"")
}

fn escape_string_literal(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Resolved Python type for an opaque type definition.
enum OpaqueResolvedType {
    /// Subclassable base type (e.g., `str`, `float`, `uuid.UUID`).
    Wrapper(String),
    /// Type alias expression (e.g., `pydantic.JsonValue`, `dict[str, pydantic.JsonValue]`).
    Alias(String),
}

/// Maps a single JSON type to a Python type string and whether it is subclassable.
fn json_type_to_py(ty: &JsonType) -> OpaqueResolvedType {
    match ty {
        JsonType::String => OpaqueResolvedType::Wrapper("str".into()),
        JsonType::Number => OpaqueResolvedType::Wrapper("float".into()),
        JsonType::Boolean => OpaqueResolvedType::Wrapper("int".into()),
        JsonType::Object => OpaqueResolvedType::Alias("dict[str, pydantic.JsonValue]".into()),
        JsonType::Array => OpaqueResolvedType::Alias("list[pydantic.JsonValue]".into()),
        JsonType::Null => OpaqueResolvedType::Alias("None".into()),
        JsonType::Any => OpaqueResolvedType::Alias("pydantic.JsonValue".into()),
    }
}

/// Resolves the Python type for an opaque type definition.
fn resolve_opaque_type(def: &ir::Def) -> Result<Option<OpaqueResolvedType>> {
    let py_attrs = PyOpaqueTypeAttrs::try_from_attrs(&def.attrs)?;
    if let Some(typ) = py_attrs.typ {
        return Ok(Some(OpaqueResolvedType::Wrapper(typ.path)));
    }

    let json_attrs = JsonOpaqueTypeAttrs::try_from_attrs(&def.attrs)?;
    if let Some(typ_attr) = json_attrs.typ {
        let types: Vec<_> = typ_attr.typ.types.iter().collect();
        if types.len() == 1 {
            return Ok(Some(json_type_to_py(types[0])));
        }
        // Union of JSON types — cannot subclass, emit a type alias.
        let parts: Vec<_> = types
            .iter()
            .map(|ty| match json_type_to_py(ty) {
                OpaqueResolvedType::Wrapper(s) | OpaqueResolvedType::Alias(s) => s,
            })
            .collect();
        return Ok(Some(OpaqueResolvedType::Alias(parts.join(" | "))));
    }

    Ok(None)
}

/// Collects module imports required by opaque types with explicit `#[py(type = ...)]`.
fn collect_opaque_imports(schema: &ir::Schema) -> Result<Vec<String>> {
    let mut modules = Vec::new();
    for def in &schema.defs {
        if let ir::DefKind::OpaqueType(_) = &def.kind {
            let py_attrs = PyOpaqueTypeAttrs::try_from_attrs(&def.attrs)?;
            if let Some(typ) = py_attrs.typ {
                if let Some(dot) = typ.path.rfind('.') {
                    let module = &typ.path[..dot];
                    if !modules.contains(&module.to_owned()) {
                        modules.push(module.to_owned());
                    }
                }
            }
        }
    }
    modules.sort();
    Ok(modules)
}

/// Implements [`Generator`] for Python.
pub struct PyGenerator;

impl PyGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Generator for PyGenerator {
    fn generate(&self, job: Job) -> diagnostics::Result<()> {
        let cfg = serde_json::from_value::<Option<Config>>(job.config.clone())
            .unwrap()
            .unwrap_or_default();

        let bundle = &job.unit[job.bundle];
        let bundle_ctx = BundleCtx {
            cfg: &cfg,
            unit: job.unit,
            bundle,
        };

        for schema in &bundle.schemas {
            let schema_ctx = SchemaCtx {
                bundle_ctx: &bundle_ctx,
                schema,
            };
            let source = generate_schema(&schema_ctx)?;
            std::fs::write(job.output.join(format!("{}.py", schema.name)), source)?;
        }

        std::fs::write(job.output.join("__init__.py"), generate_init(&bundle_ctx))?;
        // PEP 561 marker for typed packages.
        std::fs::write(job.output.join("py.typed"), "")?;

        Ok(())
    }
}

fn generate_init(ctx: &BundleCtx) -> String {
    let mut w = PyWriter::new();
    w.line("# DO NOT EDIT! This file is autogenerated.");
    w.blank();
    w.line("import pydantic");
    w.blank();

    let mut schemas: Vec<_> = ctx.bundle.schemas.iter().collect();
    schemas.sort_by_key(|s| &s.name);

    for schema in &schemas {
        w.line(&format!("from . import {}  # noqa: F401", schema.name));
    }

    // Inject cross-schema references into each module's namespace so that
    // pydantic can resolve forward references at model_rebuild time.
    w.blank();
    let schema_list = schemas
        .iter()
        .map(|s| s.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    w.line(&format!("_schemas = {{{}}}", schemas
        .iter()
        .map(|s| format!("\"{0}\": {0}", s.name))
        .collect::<Vec<_>>()
        .join(", ")));
    w.line(&format!("for _m in [{}]:", schema_list));
    w.indent();
    w.line("for _name, _other in _schemas.items():");
    w.indent();
    w.line("_alias = f\"_schema_{_name}\"");
    w.line("if not hasattr(_m, _alias):");
    w.indent();
    w.line("setattr(_m, _alias, _other)");
    w.dedent();
    w.dedent();
    w.line("for _attr in dir(_m):");
    w.indent();
    w.line("_obj = getattr(_m, _attr)");
    w.line("if isinstance(_obj, type) and issubclass(_obj, pydantic.BaseModel) and not _obj.__pydantic_complete__:");
    w.indent();
    w.line("_obj.model_rebuild()");
    w.dedent();
    w.dedent();
    w.dedent();

    w.finish()
}

fn generate_schema(ctx: &SchemaCtx) -> Result<String> {
    let mut w = PyWriter::new();

    w.line("# DO NOT EDIT! This file is autogenerated.");
    w.blank();
    w.line("from __future__ import annotations");
    w.blank();

    let mut type_vars = Vec::new();
    for def in &ctx.schema.defs {
        for var in &def.vars {
            let name = var.name.as_str().to_owned();
            if !type_vars.contains(&name) {
                type_vars.push(name);
            }
        }
    }

    let mut typing_imports = vec!["Annotated", "Literal", "TYPE_CHECKING", "Union"];
    if !type_vars.is_empty() {
        typing_imports.push("Generic");
        typing_imports.push("TypeVar");
    }
    typing_imports.sort();
    w.line(&format!(
        "from typing import {}  # noqa: F401",
        typing_imports.join(", ")
    ));
    w.blank();
    w.line("import pydantic  # noqa: F401");
    w.line("import pydantic_core  # noqa: F401");

    let needed_schemas = referenced_schemas(ctx.schema, ctx.bundle_ctx.bundle.idx);
    let mut others: Vec<_> = ctx
        .bundle_ctx
        .bundle
        .schemas
        .iter()
        .filter(|s| needed_schemas.contains(&s.idx))
        .collect();
    others.sort_by_key(|s| &s.name);
    if !others.is_empty() {
        w.blank();
        w.line("if TYPE_CHECKING:");
        w.indent();
        for schema in &others {
            w.line(&format!(
                "from . import {} as _schema_{}  # noqa: F401",
                schema.name, schema.name
            ));
        }
        w.dedent();
    }

    let mut externals: Vec<_> = ctx.bundle_ctx.cfg.external.iter().collect();
    externals.sort_by_key(|(k, _)| (*k).clone());
    if !externals.is_empty() {
        w.blank();
        for (_, path) in &externals {
            w.line(&format!("import {path}  # noqa: F401"));
        }
    }

    let opaque_imports = collect_opaque_imports(ctx.schema)?;
    if !opaque_imports.is_empty() {
        w.blank();
        for module in &opaque_imports {
            w.line(&format!("import {module}  # noqa: F401"));
        }
    }

    if !type_vars.is_empty() {
        w.blank();
        for var in &type_vars {
            w.line(&format!("{var} = TypeVar(\"{var}\")"));
        }
    }

    // Emit non-variant definitions first so that record types are available
    // as base classes for internally-tagged variant models.
    for def in &ctx.schema.defs {
        if !matches!(def.kind, ir::DefKind::VariantType(_)) {
            generate_def(ctx, def, &mut w)?;
        }
    }
    for def in &ctx.schema.defs {
        if matches!(def.kind, ir::DefKind::VariantType(_)) {
            generate_def(ctx, def, &mut w)?;
        }
    }

    w.blank();
    Ok(w.finish())
}

fn generate_def(ctx: &SchemaCtx, def: &ir::Def, w: &mut PyWriter) -> Result<()> {
    match &def.kind {
        ir::DefKind::TypeAlias(alias) => {
            w.blank();
            w.blank();
            let name = def.name.as_str();
            let params = generic_params(def);
            let aliased = ctx.resolve_type(def, &alias.aliased);
            write_doc_comments(def, w);
            w.line(&format!("type {name}{params} = {aliased}"));
        }
        ir::DefKind::OpaqueType(_) => {
            if let Some(resolved) = resolve_opaque_type(def)? {
                w.blank();
                w.blank();
                generate_opaque(def, &resolved, w);
            }
        }
        ir::DefKind::RecordType(rec) => {
            w.blank();
            w.blank();
            generate_record(ctx, def, rec, w)?;
        }
        ir::DefKind::VariantType(var) => {
            w.blank();
            w.blank();
            generate_variant(ctx, def, var, w)?;
        }
        ir::DefKind::WrapperType(wrap) => {
            w.blank();
            w.blank();
            let name = def.name.as_str();
            let params = generic_params(def);
            let wrapped = ctx.resolve_type(def, &wrap.wrapped);
            write_doc_comments(def, w);
            w.line(&format!("type {name}{params} = {wrapped}"));
        }
        _ => {}
    }
    Ok(())
}

fn write_doc_comments(def: &ir::Def, w: &mut PyWriter) {
    if let Some(docs) = &def.docs {
        let text = docs.text.trim();
        if !text.is_empty() {
            for line in text.lines() {
                w.line(&format!("# {line}"));
            }
        }
    }
}

fn write_docstring(def: &ir::Def, w: &mut PyWriter) -> bool {
    if let Some(docs) = &def.docs {
        let text = docs.text.trim();
        if !text.is_empty() {
            w.line(&format!("\"\"\"{}\"\"\"", escape_docstring(text)));
            return true;
        }
    }
    false
}

fn generate_opaque(def: &ir::Def, resolved: &OpaqueResolvedType, w: &mut PyWriter) {
    let name = def.name.as_str();
    match resolved {
        OpaqueResolvedType::Wrapper(base) => {
            w.line(&format!("class {name}({base}):"));
            w.indent();
            write_docstring(def, w);
            w.blank();
            w.line("@classmethod");
            w.line("def __get_pydantic_core_schema__(cls, source_type: type, handler: pydantic.GetCoreSchemaHandler) -> pydantic_core.CoreSchema:");
            w.indent();
            w.line(&format!("return handler({base})"));
            w.dedent();
            w.dedent();
        }
        OpaqueResolvedType::Alias(aliased) => {
            write_doc_comments(def, w);
            w.line(&format!("type {name} = {aliased}"));
        }
    }
}

fn generate_record(
    ctx: &SchemaCtx,
    def: &ir::Def,
    rec: &ir::RecordTypeDef,
    w: &mut PyWriter,
) -> Result<()> {
    let name = def.name.as_str();
    let generics = generic_bases(def);
    let ty_json = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;

    w.line(&format!("class {name}(pydantic.BaseModel{generics}):"));
    w.indent();

    let mut has_preamble = write_docstring(def, w);

    w.line("model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)");
    has_preamble = true;

    if rec.fields.is_empty() {
        if !has_preamble {
            w.line("pass");
        }
    } else {
        if has_preamble {
            w.blank();
        }
        for field in &rec.fields {
            generate_record_field(ctx, def, field, &ty_json, w)?;
        }
    }

    w.dedent();
    Ok(())
}

fn generate_record_field(
    ctx: &SchemaCtx,
    def: &ir::Def,
    field: &ir::Field,
    ty_json: &JsonRecordTypeAttrs,
    w: &mut PyWriter,
) -> Result<()> {
    let json_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
    let py_name = sanitize_py_name(&to_snake_case(field.name.as_str()));
    let json_name = ty_json.field_name(field, &json_attrs);
    let field_type = ctx.resolve_type(def, &field.typ);
    let needs_alias = py_name != json_name;

    let description = field.docs.as_ref().and_then(|docs| {
        let text = docs.text.trim();
        if text.is_empty() { None } else { Some(escape_string_literal(text)) }
    });

    let mut field_args = Vec::new();
    if field.is_optional {
        field_args.push("default=None".to_owned());
    }
    if let Some(desc) = &description {
        field_args.push(format!("description=\"{desc}\""));
    }
    if needs_alias {
        field_args.push(format!("validation_alias=\"{json_name}\""));
        field_args.push(format!("serialization_alias=\"{json_name}\""));
    }

    let type_str = if field.is_optional {
        format!("{field_type} | None")
    } else {
        field_type
    };

    if field_args.is_empty() {
        w.line(&format!("{py_name}: {type_str}"));
    } else {
        w.line(&format!(
            "{py_name}: {type_str} = pydantic.Field({})",
            field_args.join(", ")
        ));
    }
    Ok(())
}

fn generate_variant(
    ctx: &SchemaCtx,
    def: &ir::Def,
    var_def: &ir::VariantTypeDef,
    w: &mut PyWriter,
) -> Result<()> {
    let ty_json = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
    generate_variant_inner(ctx, def, var_def, &ty_json, w)
}

fn generate_variant_inner(
    ctx: &SchemaCtx,
    def: &ir::Def,
    var_def: &ir::VariantTypeDef,
    ty_json: &JsonVariantTypeAttrs,
    w: &mut PyWriter,
) -> Result<()> {
    let name = def.name.as_str();
    let params = generic_params(def);
    let tag_json = ty_json.tag_field_name();
    let tag_py = sanitize_py_name(&to_snake_case(&tag_json));
    let tag_needs_alias = tag_py != tag_json;

    let mut union_members = Vec::new();

    for variant in &var_def.variants {
        let json_attrs = JsonVariantAttrs::try_from_attrs(&variant.attrs)?;
        let json_name = ty_json.variant_name(variant, &json_attrs);
        let variant_ident = variant.name.as_str();
        let class_name = format!("{name}_{variant_ident}");
        let used_vars = variant_used_vars(def, variant.typ.as_ref());

        match ty_json.tagged {
            JsonTaggedAttr::Implicitly => {
                if let Some(typ) = &variant.typ {
                    union_members.push(ctx.resolve_type(def, typ));
                } else {
                    // Unit variant in an implicitly-tagged enum: the JSON
                    // value is just the variant name string.  We wrap it in
                    // a model so that adding data later is non-breaking.
                    let gen_bases = generic_bases_for(&used_vars);
                    let subscript = subscript_for(&used_vars);
                    w.line(&format!("class {class_name}(pydantic.RootModel[Literal[\"{json_name}\"]]{gen_bases}):"));
                    w.indent();
                    w.line("pass");
                    w.dedent();
                    w.blank();
                    w.blank();
                    union_members.push(format!("{class_name}{subscript}"));
                }
            }
            JsonTaggedAttr::Externally => {
                let gen_bases = generic_bases_for(&used_vars);
                let subscript = subscript_for(&used_vars);

                if let Some(typ) = &variant.typ {
                    let inner = ctx.resolve_type(def, typ);
                    w.line(&format!("class {class_name}(pydantic.BaseModel{gen_bases}):"));
                    w.indent();
                    w.line("model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)");
                    w.blank();
                    w.line(&format!(
                        "value: {inner} = pydantic.Field(validation_alias=\"{json_name}\", serialization_alias=\"{json_name}\")"
                    ));
                    w.dedent();
                } else {
                    // Unit variant — JSON is the bare string "VariantName".
                    w.line(&format!(
                        "class {class_name}(pydantic.RootModel[Literal[\"{json_name}\"]]{gen_bases}):"
                    ));
                    w.indent();
                    w.line("pass");
                    w.dedent();
                }
                w.blank();
                w.blank();

                union_members.push(format!("{class_name}{subscript}"));
            }
            JsonTaggedAttr::Internally => {
                generate_internally_tagged_variant(
                    ctx,
                    def,
                    variant,
                    &class_name,
                    &json_name,
                    &json_attrs,
                    ty_json,
                    &used_vars,
                    &tag_py,
                    &tag_json,
                    tag_needs_alias,
                    w,
                )?;
                let subscript = subscript_for(&used_vars);
                w.blank();
                w.blank();
                union_members.push(format!("{class_name}{subscript}"));
            }
            JsonTaggedAttr::Adjacently => {
                generate_adjacently_tagged_variant(
                    ctx,
                    def,
                    variant,
                    &class_name,
                    &json_name,
                    &json_attrs,
                    ty_json,
                    &used_vars,
                    &tag_py,
                    &tag_json,
                    tag_needs_alias,
                    w,
                )?;
                let subscript = subscript_for(&used_vars);
                w.blank();
                w.blank();
                union_members.push(format!("{class_name}{subscript}"));
            }
        }
    }

    write_doc_comments(def, w);

    let union_expr = union_members.join(" | ");
    let use_discriminator = union_members.len() > 1
        && matches!(
            ty_json.tagged,
            JsonTaggedAttr::Internally | JsonTaggedAttr::Adjacently
        );
    if use_discriminator {
        w.line(&format!(
            "type {name}{params} = Annotated[{union_expr}, pydantic.Discriminator(\"{tag_py}\")]"
        ));
    } else {
        w.line(&format!("type {name}{params} = {union_expr}"));
    }

    Ok(())
}

fn generate_internally_tagged_variant(
    ctx: &SchemaCtx,
    def: &ir::Def,
    variant: &ir::Variant,
    class_name: &str,
    json_name: &str,
    json_attrs: &JsonVariantAttrs,
    ty_json: &JsonVariantTypeAttrs,
    used_vars: &[String],
    tag_py: &str,
    tag_json: &str,
    tag_needs_alias: bool,
    w: &mut PyWriter,
) -> Result<()> {
    if let Some(typ) = &variant.typ {
        let resolved = ctx.bundle_ctx.unit.resolve_aliases(typ);
        let is_flat_record =
            json_attrs.content.is_none() && ctx.bundle_ctx.unit.record_type(&resolved).is_some();

        if is_flat_record {
            // Inherit from the record model and add the tag field.
            let base = ctx.resolve_type(def, &resolved);
            w.line(&format!("class {class_name}({base}):"));
            w.indent();
            w.line("model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)");
            w.blank();
            write_tag_field(w, tag_py, tag_json, json_name, tag_needs_alias);
            w.dedent();
        } else {
            let content_json = ty_json.content_field_name(json_attrs);
            let content_py = sanitize_py_name(&to_snake_case(&content_json));
            let content_alias = content_py != content_json;
            let inner = ctx.resolve_type(def, typ);
            let gen_bases = generic_bases_for(used_vars);

            w.line(&format!("class {class_name}(pydantic.BaseModel{gen_bases}):"));
            w.indent();
            w.line("model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)");
            w.blank();
            write_tag_field(w, tag_py, tag_json, json_name, tag_needs_alias);
            write_content_field(w, &content_py, &content_json, &inner, content_alias);
            w.dedent();
        }
    } else {
        let gen_bases = generic_bases_for(used_vars);
        w.line(&format!("class {class_name}(pydantic.BaseModel{gen_bases}):"));
        w.indent();
        w.line("model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)");
        w.blank();
        write_tag_field(w, tag_py, tag_json, json_name, tag_needs_alias);
        w.dedent();
    }
    Ok(())
}

fn generate_adjacently_tagged_variant(
    ctx: &SchemaCtx,
    def: &ir::Def,
    variant: &ir::Variant,
    class_name: &str,
    json_name: &str,
    json_attrs: &JsonVariantAttrs,
    ty_json: &JsonVariantTypeAttrs,
    used_vars: &[String],
    tag_py: &str,
    tag_json: &str,
    tag_needs_alias: bool,
    w: &mut PyWriter,
) -> Result<()> {
    let gen_bases = generic_bases_for(used_vars);
    w.line(&format!("class {class_name}(pydantic.BaseModel{gen_bases}):"));
    w.indent();

    let content_json = ty_json.content_field_name(json_attrs);
    let content_py = sanitize_py_name(&to_snake_case(&content_json));
    let content_alias = content_py != content_json;

    w.line("model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)");
    w.blank();

    write_tag_field(w, tag_py, tag_json, json_name, tag_needs_alias);

    if let Some(typ) = &variant.typ {
        let inner = ctx.resolve_type(def, typ);
        write_content_field(w, &content_py, &content_json, &inner, content_alias);
    }

    w.dedent();
    Ok(())
}

fn write_tag_field(
    w: &mut PyWriter,
    py_name: &str,
    json_name: &str,
    value: &str,
    needs_alias: bool,
) {
    if needs_alias {
        w.line(&format!(
            "{py_name}: Literal[\"{value}\"] = pydantic.Field(\"{value}\", validation_alias=\"{json_name}\", serialization_alias=\"{json_name}\")"
        ));
    } else {
        w.line(&format!("{py_name}: Literal[\"{value}\"] = \"{value}\""));
    }
}

fn write_content_field(
    w: &mut PyWriter,
    py_name: &str,
    json_name: &str,
    type_expr: &str,
    needs_alias: bool,
) {
    if needs_alias {
        w.line(&format!(
            "{py_name}: {type_expr} = pydantic.Field(validation_alias=\"{json_name}\", serialization_alias=\"{json_name}\")"
        ));
    } else {
        w.line(&format!("{py_name}: {type_expr}"));
    }
}
