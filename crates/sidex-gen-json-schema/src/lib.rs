use std::collections::BTreeSet;
use std::collections::VecDeque;

use builder::RecordTypeSchemaBuilder;
use builder::VariantTypeSchemaBuilder;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use sidex::types::Map;
use sidex_attrs_api::render_doc_prelude;
use sidex_attrs_api::stability_of_def;
use sidex_attrs_json::JsonTaggedAttr;
use sidex_attrs_json::field_attrs;
use sidex_attrs_json::opaque_type_attrs;
use sidex_attrs_json::record_type_attrs;
use sidex_attrs_json::types::JsonType;
use sidex_attrs_json::variant_attrs;
use sidex_attrs_json::variant_type_attrs;
use sidex_gen::Generator;
use sidex_gen::diagnostics;
use sidex_gen::ir::STD_BUNDLE_IDX;
use sidex_gen::ir::TypeVarType;
use sidex_gen::ir::{self};
use sidex_types_json_schema::*;

pub mod builder;

/// Configuration options for JSON Schema generation.
pub struct JsonSchemaConfig {
    /// Emit `$id` for type schemas.
    pub emit_ids: bool,
    /// Emit bounds on integer types.
    pub emit_integer_bounds: bool,
    /// Emit bounds on floating-point types.
    pub emit_float_bounds: bool,
    /// Emit descriptions.
    pub emit_descriptions: bool,
    /// Emit OAS formats for numeric types.
    ///
    /// See [OAS Version 3.1, Section 4.4](https://spec.openapis.org/oas/v3.1.0#data-types).
    pub emit_oas_numeric_formats: bool,
    /// Emit OAS discriminators where applicable.
    ///
    /// See [OAS Version 3.1, Section 4.8.25](https://spec.openapis.org/oas/v3.1.0#discriminator-object).
    pub emit_oas_discriminators: bool,
    /// Emit Sidex IDL vocabulary for reconstructing the types from the schema.
    ///
    /// **This option and the vocabulary it emits is unstable.**
    pub unstable_emit_idl_vocabulary: bool,
}

impl Default for JsonSchemaConfig {
    fn default() -> Self {
        Self {
            emit_ids: true,
            emit_integer_bounds: true,
            emit_float_bounds: false,
            emit_descriptions: true,
            emit_oas_numeric_formats: false,
            emit_oas_discriminators: false,
            unstable_emit_idl_vocabulary: false,
        }
    }
}

// /// Different JSON Schema representations for variant types without data.
// enum EnumSchema {
//     /// Use the `enum` keyword.
//     Enum,
//     /// Use the representation recommended by OAS.
//     Oas,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    id_prefix: Option<String>,
}

impl Config {
    pub fn schema_id(&self, name: &str) -> String {
        if let Some(prefix) = &self.id_prefix {
            format!("{prefix}/{name}")
        } else {
            format!("/{name}")
        }
    }
}

/// A context for generating JSON Schema.
pub struct JsonSchemaCtx<'cx> {
    pub unit: &'cx ir::Ir,

    pub id_prefix: Option<&'cx str>,

    pub def_prefix: String,
    pub defs: IndexMap<String, TypeSchema>,
    pub config: JsonSchemaConfig,
}

/// Restricts strings to the given pattern.
fn set_pattern(schema: &mut SchemaObject, pattern: &str) {
    schema.set_string_keywords(Some(Box::new(
        StringKeywords::new().with_pattern(Some(Pattern::new(pattern.to_owned()))),
    )));
}

fn make_schema_ref<S: AsRef<str>>(string: S) -> SchemaObject {
    SchemaObject::new().with_reference(Some(SchemaRef::new(string.as_ref().to_owned())))
}

fn make_schema_var<S: AsRef<str>>(_name: S) -> TypeSchema {
    let schema = SchemaObject::new();
    // .with_extensions(Some(indexmap! {
    //     "x-sidex-var".to_owned() => Any::String(name.as_ref().to_owned()),
    // }));
    TypeSchema {
        name: "A-TYPE-VARIABLE".to_owned(),
        use_schema: schema.clone(),
        def_schema: None,
        inline_schema: None,
    }
}

/// A schema for a type.
#[derive(Debug, Clone)]
pub struct TypeSchema {
    /// The name of the type.
    pub name: String,
    /// The schema used to reference the type.
    pub use_schema: SchemaObject,
    /// The schema used to define the type.
    pub def_schema: Option<SchemaObject>,
    /// The schema used to inline the type.
    pub inline_schema: Option<SchemaObject>,
}

impl<'cx> JsonSchemaCtx<'cx> {
    pub fn new(unit: &'cx ir::Ir, config: JsonSchemaConfig) -> Self {
        Self {
            unit,
            id_prefix: None,
            def_prefix: "#/$defs/".to_owned(),
            defs: IndexMap::new(),
            config,
        }
    }

    pub fn into_defs(self) -> IndexMap<String, SchemaObject> {
        self.defs
            .into_iter()
            .filter_map(|(name, schema)| schema.def_schema.map(|schema| (name, schema)))
            .collect()
    }

    pub fn set_def_prefix<S: AsRef<str>>(&mut self, def_prefix: S) -> &mut Self {
        self.def_prefix = def_prefix.as_ref().to_owned();
        self
    }

    fn qualified_path(&self, typ: &ir::InstanceType) -> String {
        let bundle = &self.unit[typ.def.bundle];
        let schema = &self.unit[typ.def.schema];
        let def = &self.unit[typ.def];
        format!(
            "::{}::{}::{}",
            bundle.metadata.name,
            schema.name,
            def.name.as_str()
        )
    }

    fn def_name(&self, typ: &ir::InstanceType) -> String {
        let bundle = &self.unit[typ.def.bundle];
        let schema = &self.unit[typ.def.schema];
        let def = &self.unit[typ.def];
        let mut name = format!(
            "{}.{}.{}",
            bundle.metadata.name,
            schema.name,
            def.name.as_str()
        );
        if !typ.subst.is_empty() {
            // name.push_str(".");
            for (subst, _var) in typ.subst.iter().zip(&def.vars) {
                // if idx > 0 {
                //     name.push_str("&");
                // }
                // name.push_str(var.name.as_str());
                // name.push_str("=");
                name.push_str("--");
                match &subst.kind {
                    ir::TypeKind::TypeVar(_) => { /* ignore */ }
                    ir::TypeKind::Instance(instance) => {
                        name.push_str(&self.def_name(instance));
                    }
                }
            }
        }
        name
    }

    fn def_ref(&self, def_name: &str) -> SchemaObject {
        make_schema_ref(format!("{}{}", self.def_prefix, def_name))
    }

    pub fn resolve(&mut self, typ: &ir::Type) -> diagnostics::Result<TypeSchema> {
        use ir::DefKind::*;
        use ir::TypeKind::*;
        Ok(match &typ.kind {
            TypeVar(_) => make_schema_var("TODO-REPLACE-WITH-VAR"),
            Instance(instance) => {
                let def_name = self.def_name(instance);
                if let Some(schema) = self.defs.get(&def_name) {
                    return Ok(schema.clone());
                }
                // There exists a definition for the type, so let's reference it.
                let ref_schema = self.def_ref(&def_name);
                // There exists no definition, let's create a schema for the type.
                let typ_def = self.unit.type_def(typ).unwrap();
                // Unlike `typ_def`, this is the alias itself for aliases.
                let own_def = &self.unit[instance.def];
                // Insert `None` such that recursive usage yields a reference.
                if instance.def.bundle != STD_BUNDLE_IDX {
                    self.defs.insert(
                        def_name.clone(),
                        TypeSchema {
                            name: def_name.clone(),
                            use_schema: ref_schema.clone(),
                            def_schema: None,
                            inline_schema: None,
                        },
                    );
                }
                let (mut def_schema, inline_schema) = match &own_def.kind {
                    TypeAlias(_) => {
                        let aliased = self.resolve(&self.unit.resolve_aliases(typ))?;
                        (aliased.use_schema.clone(), aliased.use_schema)
                    }
                    _ => {
                        match &typ_def.kind {
                            TypeAlias(_) => unreachable!("aliases are resolved"),
                            OpaqueType(opaque_def) => {
                                let schema =
                                    self.opaque_type_schema(instance, typ_def, opaque_def)?;
                                (schema.clone(), schema)
                            }
                            RecordType(record_def) => {
                                self.record_type_schema(typ_def, record_def)?
                            }
                            VariantType(variant_type) => {
                                self.variant_type_schema(typ_def, variant_type)?
                            }
                            WrapperType(wrapper_type) => {
                                let resolved = self.resolve(&wrapper_type.wrapped)?;
                                (resolved.use_schema.clone(), resolved.use_schema)
                            }
                        }
                    }
                };

                if instance.def.bundle == STD_BUNDLE_IDX {
                    TypeSchema {
                        name: def_name,
                        use_schema: inline_schema.clone(),
                        def_schema: None,
                        inline_schema: Some(inline_schema),
                    }
                } else {
                    // inline_schema.set_extension("x-idl-type-ref", &def_name);
                    // def_schema.set_extension("x-idl-type-def", &def_name);
                    let stability = stability_of_def(own_def);
                    let prelude = render_doc_prelude(&stability);
                    let body = own_def
                        .docs
                        .as_ref()
                        .map(|docs| docs.as_str().trim())
                        .unwrap_or_default();
                    let description = if prelude.is_empty() {
                        if body.is_empty() {
                            None
                        } else {
                            Some(body.to_owned())
                        }
                    } else if body.is_empty() {
                        Some(prelude.trim_end().to_owned())
                    } else {
                        Some(format!("{prelude}{body}"))
                    };
                    def_schema.set_metadata(Some(Box::new(
                        Metadata::new()
                            // .with_title(Some(typ_def.name.as_str().to_owned()))
                            .with_description(description)
                            .with_deprecated(stability.deprecated.as_ref().map(|_| true)),
                    )));
                    if self.config.emit_ids {
                        def_schema.set_id(Some(def_name.clone()));
                    }
                    let type_schema = TypeSchema {
                        name: def_name.clone(),
                        use_schema: ref_schema,
                        def_schema: Some(def_schema),
                        inline_schema: Some(inline_schema),
                    };
                    self.defs.insert(def_name, type_schema.clone());
                    type_schema
                }
            }
        })
    }

    fn opaque_type_schema(
        &mut self,
        instance: &ir::InstanceType,
        def: &ir::Def,
        _: &ir::OpaqueTypeDef,
    ) -> diagnostics::Result<SchemaObject> {
        let mut json_schema = SchemaObject::new();
        let json_type_attrs = opaque_type_attrs(def)?;
        if let Some(typ) = json_type_attrs.typ {
            let types = typ.types;
            let includes_any = types.contains(&JsonType::Any);
            // The attribute's types are unordered, so emit them in a fixed order.
            let mut allowed = [
                (JsonType::String, Type::String),
                (JsonType::Number, Type::Number),
                (JsonType::Boolean, Type::Boolean),
                (JsonType::Object, Type::Object),
                (JsonType::Array, Type::Array),
                (JsonType::Null, Type::Null),
            ]
            .into_iter()
            .filter(|(json_type, _)| types.contains(json_type))
            .map(|(_, typ)| typ)
            .collect::<Vec<_>>();
            json_schema.set_allowed_types(Some(if allowed.len() == 1 {
                MaybeArray::Single(allowed.remove(0))
            } else {
                MaybeArray::Array(allowed)
            }));
            if includes_any {
                json_schema.set_allowed_types(None);
            }
            return Ok(json_schema);
        }
        let name = self.qualified_path(instance);
        match name.as_str() {
            "::core::builtins::Sequence" => {
                let element_type = &instance.subst[0];
                json_schema.set_allowed_types(Some(Type::Array.into()));
                json_schema.set_array_keywords(Some(Box::new(ArrayKeywords::new().with_items(
                    Some(Box::new(self.resolve(element_type)?.use_schema.into())),
                ))));
            }
            "::core::builtins::Map" => {
                let _key_type = &instance.subst[0];
                let value_type = &instance.subst[1];
                let type_schema = self.resolve(value_type)?;
                // TODO: Check that key is string.
                json_schema.set_allowed_types(Some(Type::Object.into()));
                json_schema.set_object_keywords(Some(Box::new(
                    ObjectKeywords::new()
                        .with_additional_properties(Some(Box::new(type_schema.use_schema.into()))),
                )));
            }
            "::core::builtins::unit" => {
                json_schema.set_allowed_types(Some(Type::Null.into()));
            }
            "::core::builtins::value" => {
                // Anything goes — clear the inherited any-type constraints.
                json_schema.set_allowed_types(None);
            }
            "::core::builtins::bool" => {
                json_schema.set_allowed_types(Some(Type::Boolean.into()));
            }
            "::core::builtins::string" => {
                json_schema.set_allowed_types(Some(Type::String.into()));
            }
            "::core::builtins::bytes" => {
                json_schema.set_allowed_types(Some(Type::String.into()));
            }
            "::core::builtins::i8"
            | "::core::builtins::i16"
            | "::core::builtins::u8"
            | "::core::builtins::u16"
            | "::core::builtins::idx" => {
                json_schema.set_allowed_types(Some(Type::Integer.into()));
            }
            "::core::builtins::i32" => {
                json_schema
                    .set_allowed_types(Some(Type::Integer.into()))
                    .set_format(Some("int32".to_owned()));
            }
            "::core::builtins::u32" => {
                json_schema
                    .set_allowed_types(Some(Type::Integer.into()))
                    .set_format(Some("uint32".to_owned()));
            }
            // 64-bit integers are decimal strings. Decoders also accept integers.
            "::core::builtins::i64" => {
                json_schema
                    .set_allowed_types(Some(vec![Type::String, Type::Integer].into()))
                    .set_format(Some("int64".to_owned()));
                set_pattern(&mut json_schema, "^-?[0-9]+$");
            }
            "::core::builtins::u64" => {
                json_schema
                    .set_allowed_types(Some(vec![Type::String, Type::Integer].into()))
                    .set_format(Some("uint64".to_owned()));
                set_pattern(&mut json_schema, "^[0-9]+$");
            }
            // Non-finite floats are strings.
            "::core::builtins::f32" | "::core::builtins::f64" => {
                json_schema.set_allowed_types(Some(vec![Type::Number, Type::String].into()));
                set_pattern(&mut json_schema, "^(NaN|[+-]?Infinity)$");
            }
            "::core::builtins::uuid" => {
                json_schema
                    .set_allowed_types(Some(Type::String.into()))
                    .set_format(Some("uuid".to_owned()));
            }
            "::core::builtins::timestamp" => {
                json_schema
                    .set_allowed_types(Some(Type::String.into()))
                    .set_format(Some("date-time".to_owned()));
            }
            "::core::builtins::date" => {
                json_schema
                    .set_allowed_types(Some(Type::String.into()))
                    .set_format(Some("date".to_owned()));
            }
            _ => {}
        }
        Ok(json_schema)
    }

    /// Returns the schema of values whose 64-bit integers, also those in
    /// sequences and map values, are JSON numbers, if `number` is set and the
    /// type has such integers.
    fn number_schema(&self, typ: &ir::Type, number: bool) -> Option<TypeSchema> {
        if !number {
            return None;
        }
        let schema = self.number_object(typ)?;
        Some(TypeSchema {
            name: "number".to_owned(),
            use_schema: schema.clone(),
            def_schema: None,
            inline_schema: Some(schema),
        })
    }

    fn number_object(&self, typ: &ir::Type) -> Option<SchemaObject> {
        let (name, args) = self.unit.builtin(typ)?;
        let mut schema = SchemaObject::new();
        match name {
            "i64" | "u64" => {
                let format = if name == "i64" { "int64" } else { "uint64" };
                schema
                    .set_allowed_types(Some(Type::Integer.into()))
                    .set_format(Some(format.to_owned()));
            }
            "Sequence" => {
                let items = self.number_object(&args[0])?;
                schema
                    .set_allowed_types(Some(Type::Array.into()))
                    .set_array_keywords(Some(Box::new(
                        ArrayKeywords::new().with_items(Some(Box::new(items.into()))),
                    )));
            }
            "Map" => {
                let values = self.number_object(&args[1])?;
                schema
                    .set_allowed_types(Some(Type::Object.into()))
                    .set_object_keywords(Some(Box::new(
                        ObjectKeywords::new()
                            .with_additional_properties(Some(Box::new(values.into()))),
                    )));
            }
            _ => return None,
        }
        Some(schema)
    }

    fn record_type_schema(
        &mut self,
        def: &ir::Def,
        record_type: &ir::RecordTypeDef,
    ) -> diagnostics::Result<(SchemaObject, SchemaObject)> {
        let json_type_attrs = record_type_attrs(def)?;
        let mut builder = RecordTypeSchemaBuilder::new();
        // builder.deny_other_fields();
        // let mut idl_fields = Vec::new();
        for field in &record_type.fields {
            let json_field_attrs = field_attrs(field)?;
            let field_name = json_type_attrs.field_name(field, &json_field_attrs);
            let type_schema = match self.number_schema(&field.typ, json_field_attrs.number) {
                Some(schema) => schema,
                None => self.resolve(&field.typ)?,
            };
            // let mut idl_field = IdlField::new(field_name.clone(), type_schema.name.clone());
            if json_field_attrs.inline {
                builder.add_inline_field(type_schema);
                // idl_field.set_inlined(Some(true));
            } else if field.is_optional {
                builder.add_optional_field(field_name, type_schema);
                // idl_field.set_optional(Some(true));
            } else {
                builder.add_field(field_name, type_schema);
            }
            // idl_fields.push(idl_field);
        }
        let inline_schema = builder.build_ref();
        builder.deny_other_fields();
        let def_schema = builder.build();
        // def_schema.set_extension("x-idl-fields", idl_fields);
        Ok((def_schema, inline_schema))
    }

    fn variant_type_schema(
        &mut self,
        def: &ir::Def,
        variant_type: &ir::VariantTypeDef,
    ) -> diagnostics::Result<(SchemaObject, SchemaObject)> {
        let json_type_attrs = variant_type_attrs(def)?;
        let tag_field = json_type_attrs.tag_field_name();
        let mut builder = VariantTypeSchemaBuilder::new(tag_field, json_type_attrs.tagged);
        // let mut idl_variants = Vec::new();
        for variant in &variant_type.variants {
            let json_variant_attrs = variant_attrs(variant)?;
            let variant_name = json_type_attrs.variant_name(variant, &json_variant_attrs);
            // let mut idl_variant = IdlVariant::new(variant.name.as_str().to_owned());
            if let Some(typ) = &variant.typ {
                let content_field = json_type_attrs.content_field_name(&json_variant_attrs);
                // Implicitly tagged cases represent 64-bit integers as numbers.
                let number = json_type_attrs.tagged == JsonTaggedAttr::Implicitly;
                let type_schema = match self.number_schema(typ, number) {
                    Some(schema) => schema,
                    None => self.resolve(typ)?,
                };
                let may_inline = self.unit.record_type(typ).is_some();
                // idl_variant.set_type_ref(Some(type_schema.name.clone()));
                builder.add_data_variant(&content_field, &variant_name, type_schema, may_inline);
            } else {
                builder.add_unit_variant(&variant_name);
            }
            // idl_variants.push(idl_variant);
        }
        let def_schema = builder.build();
        let inline_schema = def_schema.clone();
        // def_schema.set_extension("x-idl-variants", idl_variants);
        Ok((def_schema, inline_schema))
    }
}

pub struct JsonSchemaGenerator;

impl Generator for JsonSchemaGenerator {
    fn generate(&self, job: sidex_gen::Job) -> diagnostics::Result<()> {
        // let config = Config::deserialize(job.config.clone().into_deserializer())?;
        let mut ctx = JsonSchemaCtx::new(job.unit, Default::default());
        // ctx.set_def_prefix("");
        for (bundle_idx, bundle) in job.unit.bundles.iter().enumerate() {
            if bundle.is_internal {
                continue;
            }
            let bundle_idx = ir::BundleIdx::from(bundle_idx);
            for (schema_idx, _) in job.unit.schemas_of(bundle_idx) {
                for (def_idx, def) in job.unit.defs_of(schema_idx) {
                    let typ = ir::Type::new(ir::TypeKind::Instance(
                        ir::InstanceType::new(ir::DefRef::new(bundle_idx, schema_idx, def_idx))
                            .with_subst(
                                def.vars
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, _)| {
                                        ir::Type::new(ir::TypeKind::TypeVar(TypeVarType::new(
                                            ir::TypeVarIdx::from(idx),
                                        )))
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                    ));
                    ctx.resolve(&typ)?;
                }
            }
        }
        let defs = ctx.into_defs();
        for (name, schema) in &defs {
            let reachable = transitive_def_refs(name, schema, &defs);
            let mut filtered_defs = defs
                .iter()
                .filter(|(def_name, _)| *def_name != name && reachable.contains(*def_name))
                .map(|(name, schema)| {
                    (
                        name.clone(),
                        schema.clone().with_id(Some(name.clone())).into(),
                    )
                })
                .collect::<Vec<_>>();
            filtered_defs.sort_by(|(a, _), (b, _)| a.cmp(b));
            let filtered_defs = filtered_defs.into_iter().collect::<Map<_, _>>();
            let root_schema = RootSchema::new(schema.clone())
                .with_meta_schema(Some(
                    "https://json-schema.org/draft/2020-12/schema".to_owned(),
                ))
                .with_defs(if filtered_defs.is_empty() {
                    None
                } else {
                    Some(filtered_defs)
                });
            let schema_file = job.output.join(format!("{name}.schema.json"));
            let json = sidex::formats::json::to_string_pretty(&root_schema).map_err(|error| {
                Box::new(sidex_gen::diagnostics::Diagnostic::error(format!(
                    "Unable to encode JSON Schema: {error}"
                )))
            })?;
            std::fs::write(schema_file, json)?;
        }

        let config = JsonSchemaConfig {
            emit_ids: false,
            ..Default::default()
        };

        let mut ctx = JsonSchemaCtx::new(job.unit, config);
        ctx.set_def_prefix("#/components/schemas/");

        for (bundle_idx, bundle) in job.unit.bundles.iter().enumerate() {
            if bundle.is_internal {
                continue;
            }
            let bundle_idx = ir::BundleIdx::from(bundle_idx);
            for (schema_idx, _) in job.unit.schemas_of(bundle_idx) {
                for (def_idx, def) in job.unit.defs_of(schema_idx) {
                    let typ = ir::Type::new(ir::TypeKind::Instance(
                        ir::InstanceType::new(ir::DefRef::new(bundle_idx, schema_idx, def_idx))
                            .with_subst(
                                def.vars
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, _)| {
                                        ir::Type::new(ir::TypeKind::TypeVar(TypeVarType::new(
                                            ir::TypeVarIdx::from(idx),
                                        )))
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                    ));
                    ctx.resolve(&typ)?;
                }
            }
        }

        let mut defs = ctx.into_defs();

        defs.sort_by(|k1, _, k2, _| k1.cmp(k2));

        let schema_file = job.output.join("schema-defs.json");
        std::fs::write(schema_file, serde_json::to_string_pretty(&defs)?)?;

        Ok(())
    }
}

/// Compute the set of def names transitively reachable from `root` via
/// `#/$defs/...` references, looking up further refs in `defs`. The root's own
/// name is excluded from the result.
fn transitive_def_refs(
    root_name: &str,
    root_schema: &SchemaObject,
    defs: &IndexMap<String, SchemaObject>,
) -> BTreeSet<String> {
    const PREFIX: &str = "#/$defs/";
    let mut reachable: BTreeSet<String> = BTreeSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    let enqueue = |name: String, reachable: &mut BTreeSet<String>, queue: &mut VecDeque<String>| {
        if reachable.insert(name.clone()) {
            queue.push_back(name);
        }
    };
    for r in collect_refs(root_schema, PREFIX) {
        enqueue(r, &mut reachable, &mut queue);
    }
    while let Some(next) = queue.pop_front() {
        if next == root_name {
            continue;
        }
        if let Some(schema) = defs.get(&next) {
            for r in collect_refs(schema, PREFIX) {
                enqueue(r, &mut reachable, &mut queue);
            }
        }
    }
    reachable.remove(root_name);
    reachable
}

/// Serialize a schema and collect every `$ref` value that starts with `prefix`,
/// returning the suffix (the def name) for each match.
fn collect_refs(schema: &SchemaObject, prefix: &str) -> Vec<String> {
    let value = serde_json::to_value(schema).expect("schema must be serializable");
    let mut out = Vec::new();
    walk_refs(&value, prefix, &mut out);
    out
}

fn walk_refs(value: &serde_json::Value, prefix: &str, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, v) in map {
                if key == "$ref" {
                    if let serde_json::Value::String(s) = v
                        && let Some(name) = s.strip_prefix(prefix)
                    {
                        out.push(name.to_owned());
                    }
                } else {
                    walk_refs(v, prefix, out);
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                walk_refs(v, prefix, out);
            }
        }
        _ => {}
    }
}
