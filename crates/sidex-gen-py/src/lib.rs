//! Sidex code generation for Python with pydantic compatibility.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sidex_attrs_json::{
    JsonFieldAttrs, JsonOpaqueTypeAttrs, JsonRecordTypeAttrs, JsonVariantAttrs,
    JsonVariantTypeAttrs, atoms::JsonTaggedAttr, types::JsonType,
};
use sidex_attrs_py::PyOpaqueTypeAttrs;
use sidex_codegen::{Code, quote};
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
                        // The alias `_schema_<name>` is always a valid identifier;
                        // no sanitization needed on the alias side.
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
                    format!(
                        "{ext}.{}.{}",
                        sanitize_py_name(schema.name.as_str()),
                        instance_def.name.as_str()
                    )
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

/// Implements [`Generator`] for Python.
pub struct PyGenerator;

impl PyGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PyGenerator {
    fn default() -> Self {
        Self::new()
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
            let source = generate_schema(&schema_ctx)?.to_string();
            let module_name = sanitize_py_name(schema.name.as_str());
            std::fs::write(job.output.join(format!("{module_name}.py")), source)?;
        }

        std::fs::write(
            job.output.join("__init__.py"),
            generate_init(&bundle_ctx).to_string(),
        )?;
        // PEP 561 marker for typed packages.
        std::fs::write(job.output.join("py.typed"), "")?;

        Ok(())
    }
}

fn generate_init(ctx: &BundleCtx) -> Code {
    let mut schemas: Vec<_> = ctx.bundle.schemas.iter().collect();
    schemas.sort_by_key(|s| &s.name);

    let imports: Vec<Code> = schemas
        .iter()
        .map(|s| {
            let module = sanitize_py_name(s.name.as_str());
            quote!("from . import @module  # noqa: F401")
        })
        .collect();

    // The `_schemas` dict's keys use the original schema name (which drives the
    // `_schema_<name>` alias — always a valid identifier on its own), while
    // values reference the imported module by its sanitized identifier.
    let dict_entries: Vec<Code> = schemas
        .iter()
        .map(|s| {
            let key = format!("\"{}\"", s.name);
            let value = sanitize_py_name(s.name.as_str());
            quote!("@key: @value")
        })
        .collect();

    quote!(
        r#"
        # DO NOT EDIT! This file is autogenerated.

        import pydantic

        @(@imports)*

        _schemas = {@(@dict_entries), +}
        for _m in _schemas.values():
            for _name, _other in _schemas.items():
                _alias = f"_schema_{_name}"
                if not hasattr(_m, _alias):
                    setattr(_m, _alias, _other)
        for _m in _schemas.values():
            for _attr in dir(_m):
                _obj = getattr(_m, _attr)
                if isinstance(_obj, type) and issubclass(_obj, pydantic.BaseModel) and not _obj.__pydantic_complete__:
                    _obj.model_rebuild()
        "#
    )
}

fn generate_schema(ctx: &SchemaCtx) -> Result<Code> {
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
    let typing_imports = typing_imports.join(", ");

    // Build the conditional preamble blocks (TYPE_CHECKING, externals, opaque
    // imports, TypeVar declarations). Each block is collected as one element
    // in `preamble_blocks`; absent sections contribute nothing. Rendering uses
    // a single block iteration in the schema template, with `\n` separator so
    // that consecutive non-empty blocks are spaced by a blank line.
    let mut preamble_blocks: Vec<Code> = Vec::new();

    let needed_schemas = referenced_schemas(ctx.schema, ctx.bundle_ctx.bundle.idx);
    let mut others: Vec<_> = ctx
        .bundle_ctx
        .bundle
        .schemas
        .iter()
        .filter(|s| needed_schemas.contains(&s.idx))
        .collect();
    others.sort_by_key(|s| &s.name);
    // Each preamble block carries a leading blank line so that consecutive
    // non-empty blocks are visually separated; the block iteration in the
    // schema template then handles the empty case (no preamble blocks at all)
    // by collapsing its line entirely.
    if !others.is_empty() {
        let lines: Vec<Code> = others
            .iter()
            .map(|s| {
                let module = sanitize_py_name(s.name.as_str());
                let alias = format!("_schema_{}", s.name);
                quote!("from . import @module as @alias  # noqa: F401")
            })
            .collect();
        preamble_blocks.push(quote!(
            r#"

            if TYPE_CHECKING:
                @(@lines)*"#
        ));
    }

    let mut externals: Vec<_> = ctx.bundle_ctx.cfg.external.iter().collect();
    externals.sort_by_key(|(k, _)| (*k).clone());
    if !externals.is_empty() {
        let lines: Vec<Code> = externals
            .iter()
            .map(|(_, path)| quote!("import @path  # noqa: F401"))
            .collect();
        preamble_blocks.push(quote!(
            r#"

            @(@lines)*"#
        ));
    }

    let opaque_imports = collect_opaque_imports(ctx.schema)?;
    if !opaque_imports.is_empty() {
        let lines: Vec<Code> = opaque_imports
            .iter()
            .map(|m| quote!("import @m  # noqa: F401"))
            .collect();
        preamble_blocks.push(quote!(
            r#"

            @(@lines)*"#
        ));
    }

    if !type_vars.is_empty() {
        let lines: Vec<Code> = type_vars
            .iter()
            .map(|v| quote!("@v = TypeVar(\"@v\")"))
            .collect();
        preamble_blocks.push(quote!(
            r#"

            @(@lines)*"#
        ));
    }

    // Emit non-variant defs first (record types must be available as base
    // classes for internally-tagged variant models).
    let mut defs: Vec<Code> = Vec::new();
    for def in &ctx.schema.defs {
        if !matches!(def.kind, ir::DefKind::VariantType(_)) {
            defs.push(generate_def(ctx, def)?);
        }
    }
    for def in &ctx.schema.defs {
        if matches!(def.kind, ir::DefKind::VariantType(_)) {
            defs.push(generate_def(ctx, def)?);
        }
    }
    let defs: Vec<Code> = defs.into_iter().filter(|d| !is_empty_code(d)).collect();

    Ok(quote!(
        r#"
        # DO NOT EDIT! This file is autogenerated.

        from __future__ import annotations

        from typing import @typing_imports  # noqa: F401

        import pydantic  # noqa: F401
        import pydantic_core  # noqa: F401
        @(@preamble_blocks)*
        @(@defs)*
        "#
    ))
}

fn generate_def(ctx: &SchemaCtx, def: &ir::Def) -> Result<Code> {
    // Each top-level def's body template starts with two blank lines (PEP 8
    // conventions for top-level defs). The vertical join in the schema
    // template adds one newline between defs; combined with the body's two
    // leading newlines this gives three newlines (= two blank lines) before
    // each def.
    Ok(match &def.kind {
        ir::DefKind::TypeAlias(alias) => {
            let name = def.name.as_str();
            let params = generic_params(def);
            let aliased = ctx.resolve_type(def, &alias.aliased);
            let docs_lines = doc_comment_lines(def);
            quote!(
                r#"


                @(@docs_lines)*
                type @name@params = @aliased
                "#
            )
        }
        ir::DefKind::OpaqueType(_) => match resolve_opaque_type(def)? {
            Some(resolved) => generate_opaque(def, &resolved),
            None => Code::new(),
        },
        ir::DefKind::RecordType(rec) => generate_record(ctx, def, rec)?,
        ir::DefKind::VariantType(var) => generate_variant(ctx, def, var)?,
        ir::DefKind::WrapperType(wrap) => {
            let name = def.name.as_str();
            let params = generic_params(def);
            let wrapped = ctx.resolve_type(def, &wrap.wrapped);
            let docs_lines = doc_comment_lines(def);
            quote!(
                r#"


                @(@docs_lines)*
                type @name@params = @wrapped
                "#
            )
        }
        _ => Code::new(),
    })
}

fn generate_opaque(def: &ir::Def, resolved: &OpaqueResolvedType) -> Code {
    let name = def.name.as_str();
    match resolved {
        OpaqueResolvedType::Wrapper(base) => {
            let docstring_block = optional_block(docstring(def));
            quote!(
                r#"


                class @name(@base):
                    @(@docstring_block)*
                    @@classmethod
                    def __get_pydantic_core_schema__(cls, source_type: type, handler: pydantic.GetCoreSchemaHandler) -> pydantic_core.CoreSchema:
                        return handler(@base)
                "#
            )
        }
        OpaqueResolvedType::Alias(aliased) => {
            let docs_lines = doc_comment_lines(def);
            quote!(
                r#"


                @(@docs_lines)*
                type @name = @aliased
                "#
            )
        }
    }
}

fn generate_record(ctx: &SchemaCtx, def: &ir::Def, rec: &ir::RecordTypeDef) -> Result<Code> {
    let name = def.name.as_str();
    let generics = generic_bases(def);
    let ty_json = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;

    let docstring_block = optional_block(docstring(def));
    let fields: Vec<Code> = rec
        .fields
        .iter()
        .map(|f| generate_record_field(ctx, def, f, &ty_json))
        .collect::<Result<Vec<_>>>()?;

    // Empty body needs a `pass` only when there's no docstring AND no fields,
    // which is impossible here (we always emit `model_config`). The
    // `model_config` line plus optional docstring/fields handles every case.
    Ok(quote!(
        r#"


        class @name(pydantic.BaseModel@generics):
            @(@docstring_block)*
            model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)
            @(@fields)*
        "#
    ))
}

fn generate_record_field(
    ctx: &SchemaCtx,
    def: &ir::Def,
    field: &ir::Field,
    ty_json: &JsonRecordTypeAttrs,
) -> Result<Code> {
    let json_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
    let py_name = sanitize_py_name(&to_snake_case(field.name.as_str()));
    let json_name = ty_json.field_name(field, &json_attrs);
    let field_type = ctx.resolve_type(def, &field.typ);
    let needs_alias = py_name != json_name;

    let description = field.docs.as_ref().and_then(|docs| {
        let text = docs.text.trim();
        if text.is_empty() {
            None
        } else {
            Some(escape_string_literal(text))
        }
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
        Ok(quote!("@py_name: @type_str"))
    } else {
        let args = field_args.join(", ");
        Ok(quote!("@py_name: @type_str = pydantic.Field(@args)"))
    }
}

fn generate_variant(
    ctx: &SchemaCtx,
    def: &ir::Def,
    var_def: &ir::VariantTypeDef,
) -> Result<Code> {
    let ty_json = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
    let name = def.name.as_str();
    let params = generic_params(def);
    let tag_json = ty_json.tag_field_name();
    let tag_py = sanitize_py_name(&to_snake_case(&tag_json));
    let tag_needs_alias = tag_py != tag_json;

    let mut variant_classes: Vec<Code> = Vec::new();
    let mut union_members: Vec<String> = Vec::new();

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
                    let gen_bases = generic_bases_for(&used_vars);
                    let subscript = subscript_for(&used_vars);
                    variant_classes.push(quote!(
                        r#"


                        class @class_name(pydantic.RootModel[Literal["@json_name"]]@gen_bases):
                            pass
                        "#
                    ));
                    union_members.push(format!("{class_name}{subscript}"));
                }
            }
            JsonTaggedAttr::Externally => {
                let gen_bases = generic_bases_for(&used_vars);
                let subscript = subscript_for(&used_vars);

                if let Some(typ) = &variant.typ {
                    let inner = ctx.resolve_type(def, typ);
                    variant_classes.push(quote!(
                        r#"


                        class @class_name(pydantic.BaseModel@gen_bases):
                            model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)

                            value: @inner = pydantic.Field(validation_alias="@json_name", serialization_alias="@json_name")
                        "#
                    ));
                } else {
                    variant_classes.push(quote!(
                        r#"


                        class @class_name(pydantic.RootModel[Literal["@json_name"]]@gen_bases):
                            pass
                        "#
                    ));
                }

                union_members.push(format!("{class_name}{subscript}"));
            }
            JsonTaggedAttr::Internally => {
                variant_classes.push(generate_internally_tagged_variant(
                    ctx,
                    def,
                    variant,
                    &class_name,
                    &json_name,
                    &json_attrs,
                    &ty_json,
                    &used_vars,
                    &tag_py,
                    &tag_json,
                    tag_needs_alias,
                )?);
                let subscript = subscript_for(&used_vars);
                union_members.push(format!("{class_name}{subscript}"));
            }
            JsonTaggedAttr::Adjacently => {
                variant_classes.push(generate_adjacently_tagged_variant(
                    ctx,
                    def,
                    variant,
                    &class_name,
                    &json_name,
                    &json_attrs,
                    &ty_json,
                    &used_vars,
                    &tag_py,
                    &tag_json,
                    tag_needs_alias,
                )?);
                let subscript = subscript_for(&used_vars);
                union_members.push(format!("{class_name}{subscript}"));
            }
        }
    }

    let union_expr = union_members.join(" | ");
    let use_discriminator = union_members.len() > 1
        && matches!(
            ty_json.tagged,
            JsonTaggedAttr::Internally | JsonTaggedAttr::Adjacently
        );

    let docs_lines = doc_comment_lines(def);
    let alias_decl = if use_discriminator {
        quote!("type @name@params = Annotated[@union_expr, pydantic.Discriminator(\"@tag_py\")]")
    } else {
        quote!("type @name@params = @union_expr")
    };

    // Each variant class needs two blank lines before it. The variant template
    // bodies already include their own leading "\n\n"; the alias_decl follows
    // the last variant class with the same spacing.
    Ok(quote!(
        r#"
        @(@variant_classes)*


        @(@docs_lines)*
        @alias_decl
        "#
    ))
}

#[allow(clippy::too_many_arguments)]
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
) -> Result<Code> {
    let tag_field = tag_field_line(tag_py, tag_json, json_name, tag_needs_alias);
    if let Some(typ) = &variant.typ {
        let resolved = ctx.bundle_ctx.unit.resolve_aliases(typ);
        let is_flat_record =
            json_attrs.content.is_none() && ctx.bundle_ctx.unit.record_type(&resolved).is_some();

        if is_flat_record {
            let base = ctx.resolve_type(def, &resolved);
            Ok(quote!(
                r#"


                class @class_name(@base):
                    model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)

                    @tag_field
                "#
            ))
        } else {
            let content_json = ty_json.content_field_name(json_attrs);
            let content_py = sanitize_py_name(&to_snake_case(&content_json));
            let content_alias = content_py != content_json;
            let inner = ctx.resolve_type(def, typ);
            let gen_bases = generic_bases_for(used_vars);
            let content_field =
                content_field_line(&content_py, &content_json, &inner, content_alias);
            Ok(quote!(
                r#"


                class @class_name(pydantic.BaseModel@gen_bases):
                    model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)

                    @tag_field
                    @content_field
                "#
            ))
        }
    } else {
        let gen_bases = generic_bases_for(used_vars);
        Ok(quote!(
            r#"


            class @class_name(pydantic.BaseModel@gen_bases):
                model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)

                @tag_field
            "#
        ))
    }
}

#[allow(clippy::too_many_arguments)]
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
) -> Result<Code> {
    let gen_bases = generic_bases_for(used_vars);
    let tag_field = tag_field_line(tag_py, tag_json, json_name, tag_needs_alias);

    let content_json = ty_json.content_field_name(json_attrs);
    let content_py = sanitize_py_name(&to_snake_case(&content_json));
    let content_alias = content_py != content_json;

    let content_field_block = if let Some(typ) = &variant.typ {
        let inner = ctx.resolve_type(def, typ);
        vec![content_field_line(&content_py, &content_json, &inner, content_alias)]
    } else {
        Vec::new()
    };

    Ok(quote!(
        r#"


        class @class_name(pydantic.BaseModel@gen_bases):
            model_config = pydantic.ConfigDict(populate_by_name=True, serialize_by_alias=True)

            @tag_field
            @(@content_field_block)*
        "#
    ))
}

fn tag_field_line(py_name: &str, json_name: &str, value: &str, needs_alias: bool) -> Code {
    if needs_alias {
        quote!(
            "@py_name: Literal[\"@value\"] = pydantic.Field(\"@value\", validation_alias=\"@json_name\", serialization_alias=\"@json_name\")"
        )
    } else {
        quote!("@py_name: Literal[\"@value\"] = \"@value\"")
    }
}

fn content_field_line(py_name: &str, json_name: &str, type_expr: &str, needs_alias: bool) -> Code {
    if needs_alias {
        quote!(
            "@py_name: @type_expr = pydantic.Field(validation_alias=\"@json_name\", serialization_alias=\"@json_name\")"
        )
    } else {
        quote!("@py_name: @type_expr")
    }
}

fn doc_comment_lines(def: &ir::Def) -> Vec<Code> {
    let Some(docs) = &def.docs else {
        return Vec::new();
    };
    let text = docs.text.trim();
    if text.is_empty() {
        return Vec::new();
    }
    text.lines().map(|l| quote!("# @l")).collect()
}

fn docstring(def: &ir::Def) -> Code {
    let Some(docs) = &def.docs else {
        return Code::new();
    };
    let text = docs.text.trim();
    if text.is_empty() {
        return Code::new();
    }
    let escaped = escape_docstring(text);
    Code::from(format!("\"\"\"{escaped}\"\"\""))
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

fn collect_referenced_schemas(
    typ: &ir::Type,
    bundle_idx: ir::BundleIdx,
    current_schema: ir::SchemaIdx,
    out: &mut Vec<ir::SchemaIdx>,
) {
    match &typ.kind {
        ir::TypeKind::TypeVar(_) => {}
        ir::TypeKind::Instance(inst) => {
            if inst.bundle == bundle_idx
                && inst.schema != current_schema
                && !out.contains(&inst.schema)
            {
                out.push(inst.schema);
            }
            for sub in &inst.subst {
                collect_referenced_schemas(sub, bundle_idx, current_schema, out);
            }
        }
    }
}

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
        | "class" | "continue" | "def" | "del" | "elif" | "else" | "except" | "finally" | "for"
        | "from" | "global" | "if" | "import" | "in" | "is" | "lambda" | "nonlocal" | "not"
        | "or" | "pass" | "raise" | "return" | "try" | "while" | "with" | "yield" => {
            format!("{name}_")
        }
        _ => name.to_owned(),
    }
}

fn escape_docstring(s: &str) -> String {
    s.replace('\\', "\\\\").replace("\"\"\"", "\\\"\\\"\\\"")
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
    /// Type alias expression.
    Alias(String),
}

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

/// Returns `true` if `code` would render as an empty string. Used to suppress
/// definitions that have no Python output (e.g. `OpaqueType` defs without
/// resolved types).
fn is_empty_code(code: &Code) -> bool {
    code.to_string().is_empty()
}

/// Wraps `code` in a `Vec` containing it iff it has any rendered content. Used
/// in templates as `@(@block)*` so that an absent value collapses its line.
fn optional_block(code: Code) -> Vec<Code> {
    if is_empty_code(&code) {
        Vec::new()
    } else {
        vec![code]
    }
}
