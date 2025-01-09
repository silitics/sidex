//! Helpers for building schemas from type definitions.

use indexmap::{indexmap, IndexMap};
use sidex_attrs_json::atoms::JsonTaggedAttr;

use crate::{
    Any, Metadata, ObjectKeywords, Schema, SchemaObject, SubschemaKeywords, Type, TypeSchema,
};

#[derive(Debug, Clone)]
pub struct RecordTypeSchemaBuilder {
    json_schema: SchemaObject,
    inline_schemas: Vec<SchemaObject>,
}

impl RecordTypeSchemaBuilder {
    pub fn new() -> Self {
        Self {
            json_schema: SchemaObject::new()
                .with_allowed_types(Some(Type::Object.into()))
                .with_object_keywords(Some(Box::new(
                    ObjectKeywords::new()
                        .with_properties(Some(Default::default()))
                        .with_required(Some(Default::default())),
                ))),
            inline_schemas: Vec::new(),
        }
    }

    pub fn add_field(&mut self, name: String, type_schema: TypeSchema) {
        self.properties_mut()
            .insert(name.clone(), type_schema.use_schema.into());
        self.required_mut().push(name)
    }

    pub fn add_optional_field(&mut self, name: String, type_schema: TypeSchema) {
        self.properties_mut()
            .insert(name, type_schema.use_schema.into());
    }

    pub fn add_inline_field(&mut self, schema: TypeSchema) {
        self.inline_schemas
            .push(schema.inline_schema.unwrap().into());
    }

    pub fn deny_other_fields(&mut self) {
        self.object_keywords_mut().unevaluated_properties = Some(Box::new(Schema::Bool(false)));
    }

    pub fn build_ref(&self) -> SchemaObject {
        self.clone().build()
    }

    pub fn build(mut self) -> SchemaObject {
        if !self.inline_schemas.is_empty() {
            self.json_schema.set_subschema_keywords(Some(Box::new(
                SubschemaKeywords::new().with_all_of(Some(
                    self.inline_schemas.into_iter().map(Into::into).collect(),
                )),
            )));
        }
        self.json_schema
    }

    fn object_keywords_mut(&mut self) -> &mut ObjectKeywords {
        self.json_schema
            .object_keywords
            .as_mut()
            .expect("invariant violation: object keywords should be initialized")
    }

    fn properties_mut(&mut self) -> &mut IndexMap<String, Schema> {
        self.object_keywords_mut()
            .properties
            .as_mut()
            .expect("invariant violation: `properties` keyword should be initialized")
    }

    fn required_mut(&mut self) -> &mut Vec<String> {
        self.object_keywords_mut()
            .required
            .as_mut()
            .expect("invariant violation: `required` keyword should be initialized")
    }
}

pub struct VariantTypeSchemaBuilder {
    tag_field: String,
    tagged: JsonTaggedAttr,
    names: Vec<String>,
    all_unit: bool,
    variants: Vec<Schema>,
}

impl VariantTypeSchemaBuilder {
    pub fn new(tag_field: String, tagged: JsonTaggedAttr) -> Self {
        Self {
            tag_field,
            tagged,
            names: Vec::new(),
            all_unit: true,
            variants: Vec::new(),
        }
    }

    pub fn add_unit_variant(&mut self, variant_name: &str) {
        use JsonTaggedAttr::*;
        self.names.push(variant_name.to_owned());
        self.variants.push(match self.tagged {
            Adjacently => object_tag(&self.tag_field, variant_name).into(),
            Externally => pure_tag(variant_name).into(),
            Internally => object_tag(&self.tag_field, variant_name).into(),
            Implicitly => pure_tag(variant_name).into(),
        })
    }

    pub fn add_data_variant(
        &mut self,
        content_field: &str,
        variant_name: &str,
        type_schema: TypeSchema,
        may_inline: bool,
    ) {
        use JsonTaggedAttr::*;
        self.names.push(variant_name.to_owned());
        self.all_unit = false;
        self.variants.push(match self.tagged {
            Adjacently => {
                adjacently_tagged(
                    &self.tag_field,
                    content_field,
                    variant_name,
                    type_schema.use_schema,
                )
                .into()
            }
            Externally => externally_tagged(variant_name, type_schema.use_schema).into(),
            Internally => {
                if may_inline {
                    internally_tagged(
                        &self.tag_field,
                        variant_name,
                        type_schema.inline_schema.unwrap(),
                    )
                    .into()
                } else {
                    adjacently_tagged(
                        &self.tag_field,
                        content_field,
                        variant_name,
                        type_schema.use_schema,
                    )
                    .into()
                }
            }
            Implicitly => type_schema.inline_schema.unwrap().into(),
        })
    }

    pub fn build(self) -> SchemaObject {
        use JsonTaggedAttr::*;
        if self.all_unit {
            SchemaObject::new().with_allowed_values(Some(
                self.names
                    .into_iter()
                    .map(|name| {
                        match self.tagged {
                            Externally | Implicitly => Any::String(name),
                            Internally | Adjacently => {
                                Any::Object(indexmap! {
                                    self.tag_field.clone() => Any::String(name),
                                })
                            }
                        }
                    })
                    .collect(),
            ))
        } else {
            SchemaObject::new().with_subschema_keywords(Some(Box::new(
                if matches!(self.tagged, JsonTaggedAttr::Implicitly) {
                    // As there is no explicit tag, the variants may overlap, hence, we use `anyOf`.
                    SubschemaKeywords::new().with_any_of(Some(self.variants))
                } else {
                    // The explicit tag uniquely identifies the variant, hence, we use `oneOf`.
                    SubschemaKeywords::new().with_one_of(Some(self.variants))
                },
            )))
        }
    }
}

pub fn inline_object(target: &mut SchemaObject, other: &SchemaObject) {
    assert_eq!(
        target.allowed_types,
        Some(Type::Object.into()),
        "invalid `target` schema {target:#?}"
    );
    assert_eq!(
        other.allowed_types,
        Some(Type::Object.into()),
        "invalid `other` schema {other:#?}"
    );
    let Some(target_object_keywords) = &mut target.object_keywords else {
        panic!("`target` must have object keywords");
    };
    let Some(other_object_keywords) = &other.object_keywords else {
        panic!("`other` must have object keywords");
    };
    if let Some(other_properties) = &other_object_keywords.properties {
        let target_properties = target_object_keywords
            .properties
            .get_or_insert_with(Default::default);
        for (name, schema) in other_properties {
            if target_properties.contains_key(name) {
                panic!("property `{name}` exists in `target` and `other`");
            }
            target_properties.insert(name.clone(), schema.clone());
        }
    }
    if let Some(other_additional_properties) = &other_object_keywords.additional_properties {
        if let Some(target_additional_properties) = &target_object_keywords.additional_properties {
            match target_additional_properties.as_ref() {
                Schema::Bool(false) => { /* ignore */ }
                _ => panic!("incompatible `additionalProperties` of `target` and `other`"),
            }
        }
        target_object_keywords.set_additional_properties(Some(other_additional_properties.clone()));
    }
    if let Some(other_requires) = &other_object_keywords.required {
        target_object_keywords
            .required
            .get_or_insert_with(Default::default)
            .extend(other_requires.iter().cloned())
    }
}

fn make_schema_string<S: AsRef<str>>(string: S) -> SchemaObject {
    SchemaObject::new()
        .with_allowed_value(Some(Any::String(string.as_ref().to_owned())))
        .into()
}

pub fn pure_tag(variant_name: &str) -> SchemaObject {
    make_schema_string(variant_name)
}

pub fn object_tag(tag_field: &str, variant_name: &str) -> SchemaObject {
    SchemaObject::new()
        .with_allowed_types(Some(Type::Object.into()))
        .with_object_keywords(Some(Box::new(
            ObjectKeywords::new()
                .with_properties(Some(indexmap! {
                    tag_field.to_owned() => make_schema_string(variant_name).into(),
                }))
                .with_required(Some(vec![tag_field.to_owned()])),
        )))
}

pub fn externally_tagged(variant_name: &str, content_schema: SchemaObject) -> SchemaObject {
    SchemaObject::new()
        .with_allowed_types(Some(Type::Object.into()))
        .with_object_keywords(Some(Box::new(ObjectKeywords::new().with_properties(Some(
            indexmap! {
                variant_name.to_owned() => content_schema.into(),
            },
        )))))
}

pub fn adjacently_tagged(
    tag_field: &str,
    content_field: &str,
    variant_name: &str,
    content_schema: SchemaObject,
) -> SchemaObject {
    SchemaObject::new()
        .with_allowed_types(Some(Type::Object.into()))
        .with_object_keywords(Some(Box::new(ObjectKeywords::new().with_properties(Some(
            indexmap! {
                tag_field.to_owned() => make_schema_string(variant_name).into(),
                content_field.to_owned() => content_schema.into(),
            },
        )))))
}

pub fn internally_tagged(
    tag_field: &str,
    variant_name: &str,
    content_schema: SchemaObject,
) -> SchemaObject {
    let mut variant_schema = object_tag(tag_field, variant_name);
    inline_object(&mut variant_schema, &content_schema);
    variant_schema
}

/// The encoding recommended for OAS enums.
///
/// https://github.com/OAI/OpenAPI-Specification/issues/348#issuecomment-336194030
pub fn oas_enum_variant(name: &str, description: &str, value: Any) -> SchemaObject {
    SchemaObject::new()
        .with_metadata(Some(Box::new(
            Metadata::new()
                .with_title(Some(name.to_owned()))
                .with_description(Some(description.to_owned())),
        )))
        .with_allowed_value(Some(value))
}
