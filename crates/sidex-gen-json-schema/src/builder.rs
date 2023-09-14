//! Schema builders.

use indexmap::{indexmap, IndexMap};
use sidex_attrs_json::atoms::JsonTaggedAttr;

use crate::{Any, ObjectKeywords, Schema, SchemaObject, SubschemaKeywords, Type};

#[derive(Debug, Clone)]
pub struct RecordTypeSchemaBuilder {
    json_schema: SchemaObject,
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
        }
    }

    pub fn add_field(&mut self, name: String, schema: SchemaObject) {
        self.properties_mut().insert(name.clone(), schema.into());
        self.required_mut().push(name)
    }

    pub fn add_optional_field(&mut self, name: String, schema: SchemaObject) {
        self.properties_mut().insert(name, schema.into());
    }

    pub fn add_inline_field(&mut self, schema: SchemaObject) {
        inline_object(&mut self.json_schema, &schema);
    }

    pub fn deny_other_fields(&mut self) {
        self.object_keywords_mut().additional_properties = Some(Box::new(Schema::Bool(false)));
    }

    pub fn build(self) -> SchemaObject {
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
    variants: Vec<Schema>,
}

impl VariantTypeSchemaBuilder {
    pub fn new(tag_field: String, tagged: JsonTaggedAttr) -> Self {
        Self {
            tag_field,
            tagged,
            variants: Vec::new(),
        }
    }

    pub fn add_unit_variant(&mut self, variant_name: &str) {
        use JsonTaggedAttr::*;
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
        schema: SchemaObject,
        raw_schema: SchemaObject,
        may_inline: bool,
    ) {
        use JsonTaggedAttr::*;
        self.variants.push(match self.tagged {
            Adjacently => {
                adjacently_tagged(&self.tag_field, content_field, variant_name, schema).into()
            }
            Externally => externally_tagged(variant_name, schema).into(),
            Internally => {
                if may_inline {
                    internally_tagged(&self.tag_field, variant_name, schema).into()
                } else {
                    adjacently_tagged(&self.tag_field, content_field, variant_name, raw_schema)
                        .into()
                }
            }
            Implicitly => schema.into(),
        })
    }

    pub fn build(self) -> SchemaObject {
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
        .with_object_keywords(Some(Box::new(ObjectKeywords::new().with_properties(Some(
            indexmap! {
                tag_field.to_owned() => make_schema_string(variant_name).into(),
            },
        )))))
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
