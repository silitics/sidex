pub mod generated;

use builder::{RecordTypeSchemaBuilder, VariantTypeSchemaBuilder};
pub use generated as types;
pub use generated::schema::*;
use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};
use sidex_attrs_json::{
    JsonFieldAttrs, JsonRecordTypeAttrs, JsonVariantAttrs, JsonVariantTypeAttrs,
};
use sidex_gen::{
    attrs::TryFromAttrs,
    diagnostics,
    ir::{self, TypeVarType, STD_BUNDLE_IDX},
    Generator,
};

pub mod builder;

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

pub struct JsonSchemaCtx<'cx> {
    pub unit: &'cx ir::Unit,

    pub id_prefix: Option<&'cx str>,

    pub def_prefix: String,
    pub defs: IndexMap<String, Option<SchemaObject>>,
}

fn make_schema_ref<S: AsRef<str>>(string: S) -> SchemaObject {
    SchemaObject::new().with_reference(Some(SchemaRef(string.as_ref().to_owned())))
}

fn make_schema_var<S: AsRef<str>>(name: S) -> SchemaObject {
    SchemaObject::new().with_extensions(Some(indexmap! {
        "x-sidex-var".to_owned() => Any::String(name.as_ref().to_owned()),
    }))
}

impl<'cx> JsonSchemaCtx<'cx> {
    pub fn new(unit: &'cx ir::Unit) -> Self {
        Self {
            unit,
            id_prefix: None,
            def_prefix: "#/$defs/".to_owned(),
            defs: IndexMap::new(),
        }
    }

    pub fn into_defs(self) -> IndexMap<String, SchemaObject> {
        self.defs
            .into_iter()
            .map(|(name, schema)| (name, schema.unwrap()))
            .collect()
    }

    pub fn set_def_prefix<S: AsRef<str>>(&mut self, def_prefix: S) -> &mut Self {
        self.def_prefix = def_prefix.as_ref().to_owned();
        self
    }

    fn schema_id(&self, name: &str) -> String {
        if let Some(prefix) = &self.id_prefix {
            format!("{prefix}/{name}")
        } else {
            format!("/{name}")
        }
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
        for subst in &typ.subst {
            name.push_str("--");
            match &subst.kind {
                ir::TypeKind::TypeVar(_) => { /* ignore */ }
                ir::TypeKind::Instance(instance) => {
                    name.push_str(&self.def_name(instance));
                }
            }
        }
        name
    }

    fn def_ref(&self, def_name: &str) -> SchemaObject {
        make_schema_ref(format!("{}{}", self.def_prefix, def_name))
    }

    pub fn resolve_raw(&mut self, typ: &ir::Type) -> diagnostics::Result<SchemaObject> {
        let resolved = self.resolve(typ)?;
        if resolved.reference.is_some() {
            let instance_typ = match &typ.kind {
                ir::TypeKind::TypeVar(_) => todo!(),
                ir::TypeKind::Instance(instance) => instance,
            };
            let def_name = self.def_name(instance_typ);
            Ok(self.defs.get(&def_name).unwrap().as_ref().unwrap().clone())
        } else {
            Ok(resolved)
        }
    }

    pub fn resolve(&mut self, typ: &ir::Type) -> diagnostics::Result<SchemaObject> {
        use ir::{DefKind::*, TypeKind::*};
        Ok(match &typ.kind {
            TypeVar(_) => make_schema_var("TODO-REPLACE-WITH-VAR"),
            Instance(instance) => {
                let def_name = self.def_name(instance);
                if self.defs.contains_key(&def_name) {
                    // There exists a definition for the type, so let's reference it.
                    return Ok(self.def_ref(&def_name));
                }
                // There exists no definition, let's create a schema for the type.
                let typ_def = self.unit.type_def(typ).unwrap();
                // Insert `None` such that recursive usage yields a reference.
                if instance.bundle != STD_BUNDLE_IDX {
                    self.defs.insert(def_name.clone(), None);
                }
                let mut json_schema = match &typ_def.kind {
                    TypeAlias(_) => todo!(),
                    OpaqueType(opaque_def) => {
                        self.opaque_type_schema(instance, typ_def, opaque_def)?
                    }
                    RecordType(record_def) => self.record_type_schema(typ_def, record_def)?,
                    VariantType(variant_type) => self.variant_type_schema(typ_def, variant_type)?,
                    WrapperType(wrapper_type) => self.resolve(&wrapper_type.wrapped)?,
                    DerivedType(_) => todo!(),
                    Interface(_) => todo!(),
                };

                if instance.bundle == STD_BUNDLE_IDX {
                    json_schema
                } else {
                    json_schema.set_metadata(Some(Box::new(
                        Metadata::new()
                            .with_title(Some(typ_def.name.as_str().to_owned()))
                            .with_description(
                                typ_def
                                    .docs
                                    .as_ref()
                                    .map(|docs| docs.as_str().trim().to_owned()),
                            ),
                    )));
                    self.defs.insert(def_name.clone(), Some(json_schema));
                    self.def_ref(&def_name)
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
                json_schema.set_array_keywords(Some(Box::new(
                    ArrayKeywords::new()
                        .with_items(Some(Box::new(self.resolve(&element_type)?.into()))),
                )));
            }
            "::core::builtins::Map" => {
                let key_type = &instance.subst[0];
                let value_type = &instance.subst[1];
                let value_schema = self.resolve(&value_type)?;
                // TODO: Check that key is string.
                json_schema.set_allowed_types(Some(Type::Object.into()));
                json_schema.set_object_keywords(Some(Box::new(
                    ObjectKeywords::new()
                        .with_additional_properties(Some(Box::new(value_schema.into()))),
                )));
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
    ) -> diagnostics::Result<SchemaObject> {
        let json_type_attrs = JsonRecordTypeAttrs::try_from_attrs(&def.attrs)?;
        let mut builder = RecordTypeSchemaBuilder::new();
        for field in &record_type.fields {
            let json_field_attrs = JsonFieldAttrs::try_from_attrs(&field.attrs)?;
            let field_name = json_type_attrs.field_name(field, &json_field_attrs);
            let field_schema = self.resolve(&field.typ)?;
            if json_field_attrs.inline {
                builder.add_inline_field(self.resolve_raw(&field.typ)?);
            } else if field.is_optional {
                builder.add_optional_field(field_name, field_schema);
            } else {
                builder.add_field(field_name, field_schema);
            }
        }
        Ok(builder.build())
    }

    fn variant_type_schema(
        &mut self,
        def: &ir::Def,
        variant_type: &ir::VariantTypeDef,
    ) -> diagnostics::Result<SchemaObject> {
        let json_type_attrs = JsonVariantTypeAttrs::try_from_attrs(&def.attrs)?;
        let tag_field = json_type_attrs.tag_field_name();
        let mut builder = VariantTypeSchemaBuilder::new(tag_field, json_type_attrs.tagged);
        for variant in &variant_type.variants {
            let json_variant_attrs = JsonVariantAttrs::try_from_attrs(&variant.attrs)?;
            let variant_name = json_type_attrs.variant_name(variant, &json_variant_attrs);
            if let Some(typ) = &variant.typ {
                let content_field = json_type_attrs.content_field_name(&json_variant_attrs);
                let schema = self.resolve(typ)?;
                let raw_schema = self.resolve_raw(typ)?;
                let may_inline = self.unit.record_type(typ).is_some();
                builder.add_data_variant(
                    &content_field,
                    &variant_name,
                    schema,
                    raw_schema,
                    may_inline,
                )
            } else {
                builder.add_unit_variant(&variant_name);
            }
        }
        Ok(builder.build())
    }
}

pub struct JsonSchemaGenerator;

impl Generator for JsonSchemaGenerator {
    fn generate(&self, job: sidex_gen::Job) -> diagnostics::Result<()> {
        // let config = Config::deserialize(job.config.clone().into_deserializer())?;
        let mut ctx = JsonSchemaCtx::new(&job.unit);
        ctx.set_def_prefix("/");
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
        for (name, schema) in ctx.into_defs() {
            let schema_file = job.output.join(format!("{name}.schema.json"));
            std::fs::write(schema_file, serde_json::to_string_pretty(&schema)?)?;
        }

        Ok(())
    }
}
