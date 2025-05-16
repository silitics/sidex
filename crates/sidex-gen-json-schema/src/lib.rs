pub mod generated;

use builder::{RecordTypeSchemaBuilder, VariantTypeSchemaBuilder};
pub use generated as types;
pub use generated::schema::*;
use indexmap::{IndexMap, indexmap};
use serde::{Deserialize, Serialize};
use sidex_attrs_json::{
    JsonFieldAttrs, JsonRecordTypeAttrs, JsonVariantAttrs, JsonVariantTypeAttrs,
};
use sidex_gen::{
    Generator,
    attrs::TryFromAttrs,
    diagnostics,
    ir::{self, STD_BUNDLE_IDX, TypeVarType},
    rename::RenameFunction,
};

pub mod builder;

/// Configuration options for the JSON representation.
pub struct JsonConfig {
    /// The default renaming function to use for field names.
    ///
    /// Defaults to `camelCase`.
    pub default_field_rename: RenameFunction,
    /// The default renaming function to use for variant names.
    ///
    /// Defaults to `none`.
    pub default_variant_rename: RenameFunction,
    /// Indicates how to deal with optional fields.
    pub default_optional_field_repr: OptionalFieldRepr,
    /// Indicates whether to represent `NaN` and the infinities as strings.
    ///
    /// Note that neither `NaN` nor the infinites can be represented as a number in JSON.
    pub enable_floats_as_strings: bool,
    /// Indicates whether to represent integers with more than 32 bits as strings.
    ///
    /// Note that JavaScript's native JSON functionality is limited to 32 bit integers.
    pub enable_integers_as_strings: bool,
}

/// TODO: We probably want to be able to control this with a attributes.
pub enum OptionalFieldRepr {
    /// If there is no value, then the field is missing.
    Undefined,
    /// The field is always there but may be null.
    Null,
    /// The field may be missing or its value may be `null`.
    Both,
}

impl Default for JsonConfig {
    fn default() -> Self {
        Self {
            default_field_rename: RenameFunction::CamelCase,
            default_variant_rename: RenameFunction::None,
            default_optional_field_repr: OptionalFieldRepr::Undefined,
            enable_floats_as_strings: false,
            enable_integers_as_strings: false,
        }
    }
}

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

impl From<serde_json::Value> for Any {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => todo!(),
            serde_json::Value::Bool(v) => Self::Boolean(v),
            serde_json::Value::Number(_) => {
                todo!()
            }
            serde_json::Value::String(v) => Self::String(v),
            serde_json::Value::Array(v) => Self::Array(v.into_iter().map(Into::into).collect()),
            serde_json::Value::Object(v) => {
                Self::Object(v.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

impl SchemaObject {
    pub fn set_extension<V: Serialize>(&mut self, name: &str, value: V) {
        let value = Any::from(serde_json::to_value(value).unwrap());
        self.extensions
            .get_or_insert_with(Default::default)
            .insert(name.to_owned(), value);
    }
}

impl From<SchemaObject> for Schema {
    fn from(value: SchemaObject) -> Self {
        Schema::Object(value)
    }
}

impl From<bool> for Schema {
    fn from(value: bool) -> Self {
        Schema::Bool(value)
    }
}

impl<T> From<T> for MaybeArray<T> {
    fn from(value: T) -> Self {
        MaybeArray::Single(value)
    }
}

impl<T> From<Vec<T>> for MaybeArray<T> {
    fn from(value: Vec<T>) -> Self {
        MaybeArray::Array(value)
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::Integer(value.into())
    }
}

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
    pub unit: &'cx ir::Unit,

    pub id_prefix: Option<&'cx str>,

    pub def_prefix: String,
    pub defs: IndexMap<String, TypeSchema>,
    pub config: JsonSchemaConfig,
}

fn make_schema_ref<S: AsRef<str>>(string: S) -> SchemaObject {
    SchemaObject::new().with_reference(Some(SchemaRef(string.as_ref().to_owned())))
}

fn make_schema_var<S: AsRef<str>>(name: S) -> TypeSchema {
    let schema = SchemaObject::new().with_extensions(Some(indexmap! {
        "x-sidex-var".to_owned() => Any::String(name.as_ref().to_owned()),
    }));
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
    pub fn new(unit: &'cx ir::Unit, config: JsonSchemaConfig) -> Self {
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
        let bundle = &self.unit[typ.bundle];
        let schema = &bundle[typ.schema];
        let def = &schema[typ.def];
        format!(
            "::{}::{}::{}",
            bundle.metadata.name,
            schema.name,
            def.name.as_str()
        )
    }

    fn def_name(&self, typ: &ir::InstanceType) -> String {
        let bundle = &self.unit[typ.bundle];
        let schema = &bundle[typ.schema];
        let def = &schema[typ.def];
        let mut name = format!(
            "{}.{}.{}",
            bundle.metadata.name,
            schema.name,
            def.name.as_str()
        );
        if !typ.subst.is_empty() {
            // name.push_str(".");
            for (_idx, (subst, _var)) in typ.subst.iter().zip(&def.vars).enumerate() {
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
        use ir::{DefKind::*, TypeKind::*};
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
                // Insert `None` such that recursive usage yields a reference.
                if instance.bundle != STD_BUNDLE_IDX {
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
                let (mut def_schema, inline_schema) = match &typ_def.kind {
                    TypeAlias(_) => todo!(),
                    OpaqueType(opaque_def) => {
                        let schema = self.opaque_type_schema(instance, typ_def, opaque_def)?;
                        (schema.clone(), schema)
                    }
                    RecordType(record_def) => self.record_type_schema(typ_def, record_def)?,
                    VariantType(variant_type) => self.variant_type_schema(typ_def, variant_type)?,
                    WrapperType(wrapper_type) => {
                        let resolved = self.resolve(&wrapper_type.wrapped)?;
                        (resolved.use_schema.clone(), resolved.use_schema)
                    }
                    DerivedType(_) => todo!(),
                    Interface(_) => todo!(),
                };

                if instance.bundle == STD_BUNDLE_IDX {
                    TypeSchema {
                        name: def_name,
                        use_schema: inline_schema.clone(),
                        def_schema: None,
                        inline_schema: Some(inline_schema),
                    }
                } else {
                    // inline_schema.set_extension("x-idl-type-ref", &def_name);
                    // def_schema.set_extension("x-idl-type-def", &def_name);
                    def_schema.set_metadata(Some(Box::new(
                        Metadata::new()
                            // .with_title(Some(typ_def.name.as_str().to_owned()))
                            .with_description(
                                typ_def
                                    .docs
                                    .as_ref()
                                    .map(|docs| docs.as_str().trim().to_owned()),
                            ),
                    )));
                    if self.config.emit_ids {
                        def_schema.set_id(Some(format!("{def_name}")));
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
        _: &ir::Def,
        _: &ir::OpaqueTypeDef,
    ) -> diagnostics::Result<SchemaObject> {
        let mut json_schema = SchemaObject::new();
        let name = self.qualified_path(instance);
        match name.as_str() {
            "::core::builtins::Sequence" => {
                let element_type = &instance.subst[0];
                json_schema.set_allowed_types(Some(Type::Array.into()));
                json_schema.set_array_keywords(Some(Box::new(ArrayKeywords::new().with_items(
                    Some(Box::new(self.resolve(&element_type)?.use_schema.into())),
                ))));
            }
            "::core::builtins::Map" => {
                let _key_type = &instance.subst[0];
                let value_type = &instance.subst[1];
                let type_schema = self.resolve(&value_type)?;
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
            "::core::builtins::string" => {
                json_schema.set_allowed_types(Some(Type::String.into()));
            }
            "::core::builtins::idx" => {
                json_schema.set_allowed_types(Some(Type::Integer.into()));
            }
            "::core::builtins::i32" => {
                json_schema
                    .set_allowed_types(Some(Type::Integer.into()))
                    .set_format(Some("int32".to_owned()));
            }
            "::core::builtins::i64" => {
                json_schema
                    .set_allowed_types(Some(Type::Integer.into()))
                    .set_format(Some("int64".to_owned()));
            }
            "::core::builtins::u32" => {
                json_schema
                    .set_allowed_types(Some(Type::Integer.into()))
                    .set_format(Some("uint32".to_owned()));
            }
            "::core::builtins::u64" => {
                json_schema
                    .set_allowed_types(Some(Type::Integer.into()))
                    .set_format(Some("uint64".to_owned()));
            }
            "::core::builtins::f64" => {
                json_schema.set_allowed_types(Some(Type::Number.into()));
            }
            "::core::builtins::bool" => {
                json_schema.set_allowed_types(Some(Type::Boolean.into()));
            }
            _ => {}
        }
        Ok(json_schema)
    }

    fn record_type_schema(
        &mut self,
        def: &ir::Def,
        record_type: &ir::RecordTypeDef,
    ) -> diagnostics::Result<(SchemaObject, SchemaObject)> {
        let json_type_attrs = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;
        let mut builder = RecordTypeSchemaBuilder::new();
        // builder.deny_other_fields();
        let mut idl_fields = Vec::new();
        for field in &record_type.fields {
            let json_field_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
            let field_name = json_type_attrs.field_name(field, &json_field_attrs);
            let type_schema = self.resolve(&field.typ)?;
            let mut idl_field = IdlField::new(field_name.clone(), type_schema.name.clone());
            if json_field_attrs.inline {
                builder.add_inline_field(type_schema);
                idl_field.set_inlined(Some(true));
            } else if field.is_optional {
                builder.add_optional_field(field_name, type_schema);
                idl_field.set_optional(Some(true));
            } else {
                builder.add_field(field_name, type_schema);
            }
            idl_fields.push(idl_field);
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
        let json_type_attrs = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
        let tag_field = json_type_attrs.tag_field_name();
        let mut builder = VariantTypeSchemaBuilder::new(tag_field, json_type_attrs.tagged);
        let mut idl_variants = Vec::new();
        for variant in &variant_type.variants {
            let json_variant_attrs = JsonVariantAttrs::try_from_attrs(&variant.attrs)?;
            let variant_name = json_type_attrs.variant_name(variant, &json_variant_attrs);
            let mut idl_variant = IdlVariant::new(variant.name.as_str().to_owned());
            if let Some(typ) = &variant.typ {
                let content_field = json_type_attrs.content_field_name(&json_variant_attrs);
                let type_schema = self.resolve(typ)?;
                let may_inline = self.unit.record_type(typ).is_some();
                idl_variant.set_type_ref(Some(type_schema.name.clone()));
                builder.add_data_variant(&content_field, &variant_name, type_schema, may_inline);
            } else {
                builder.add_unit_variant(&variant_name);
            }
            idl_variants.push(idl_variant);
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
        let mut ctx = JsonSchemaCtx::new(&job.unit, Default::default());
        // ctx.set_def_prefix("");
        for bundle in &job.unit.bundles {
            for schema in &bundle.schemas {
                for (idx, def) in schema.defs.iter().enumerate() {
                    if matches!(def.kind, ir::DefKind::Interface(_)) {
                        // Do not generate JSON schemas for interface types.
                        continue;
                    }
                    let typ = ir::Type::new(ir::TypeKind::Instance(
                        ir::InstanceType::new(bundle.idx, schema.idx, idx.into()).with_subst(
                            def.vars
                                .iter()
                                .enumerate()
                                .map(|(idx, _)| {
                                    ir::Type::new(ir::TypeKind::TypeVar(TypeVarType::new(
                                        idx.into(),
                                    )))
                                })
                                .collect(),
                        ),
                    ));
                    ctx.resolve(&typ)?;
                }
            }
        }
        let defs = ctx.into_defs();
        for (name, schema) in &defs {
            let mut defs = defs
                .iter()
                .filter(|(def_name, _)| *def_name != name)
                .map(|(name, schema)| {
                    (
                        name.clone(),
                        schema.clone().with_id(Some(name.clone())).into(),
                    )
                })
                .collect::<IndexMap<_, _>>();
            defs.sort_keys();
            let root_schema = RootSchema::new(schema.clone())
                .with_meta_schema(Some(
                    "https://json-schema.org/draft/2020-12/schema".to_owned(),
                ))
                .with_defs(Some(defs));
            let schema_file = job.output.join(format!("{name}.schema.json"));
            std::fs::write(schema_file, serde_json::to_string_pretty(&root_schema)?)?;
        }

        let mut config = JsonSchemaConfig::default();
        config.emit_ids = false;

        let mut ctx = JsonSchemaCtx::new(&job.unit, config);
        ctx.set_def_prefix("#/components/schemas/");

        for bundle in &job.unit.bundles {
            for schema in &bundle.schemas {
                for (idx, def) in schema.defs.iter().enumerate() {
                    if matches!(def.kind, ir::DefKind::Interface(_)) {
                        // Do not generate JSON schemas for interface types.
                        continue;
                    }
                    let typ = ir::Type::new(ir::TypeKind::Instance(
                        ir::InstanceType::new(bundle.idx, schema.idx, idx.into()).with_subst(
                            def.vars
                                .iter()
                                .enumerate()
                                .map(|(idx, _)| {
                                    ir::Type::new(ir::TypeKind::TypeVar(TypeVarType::new(
                                        idx.into(),
                                    )))
                                })
                                .collect(),
                        ),
                    ));
                    ctx.resolve(&typ)?;
                }
            }
        }

        let defs = ctx.into_defs();

        let schema_file = job.output.join(format!("schema-defs.json"));
        std::fs::write(schema_file, serde_json::to_string_pretty(&defs)?)?;

        Ok(())
    }
}
