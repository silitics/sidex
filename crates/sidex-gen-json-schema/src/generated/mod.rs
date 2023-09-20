/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod schema {
    #![doc = "Types of [JSON Schema](https://json-schema.org) version `2020-12`.\n\nThe types have been inspired by [Schemars](https://crates.io/crates/schemars).\n\n**References**:\n\n- [JSON Schema Core](https://json-schema.org/draft/2020-12/json-schema-core.html)\n- [JSON Schema Validation](https://json-schema.org/draft/2020-12/json-schema-validation.html)\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "A JSON schema.\n\nSee [JSON Schema 4.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n"]
    #[derive(Clone, Debug)]
    pub enum Schema {
        #[doc = "A boolean JSON schema.\n\nSee [JSON Schema 4.3.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-boolean-json-schemas).\n"]
        Bool(bool),
        #[doc = "A schema object.\n\nSee [JSON Schema 4.3.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n"]
        Object(SchemaObject),
    }
    #[automatically_derived]
    impl __serde::Serialize for Schema {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "Schema");
            match self {
                Self::Bool(__value) => {
                    __serializer.serialize_implicitly_tagged("Bool", 0u32, __value)
                }
                Self::Object(__value) => {
                    __serializer.serialize_implicitly_tagged("Object", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Schema {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["Bool", "Object"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"Bool\", \"Object\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "Bool" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Object" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::unknown_variant(
                                __variant,
                                __IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"Bool" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Object" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &["Bool", "Object"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<bool, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Schema::Bool(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<SchemaObject, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Schema::Object(__value)),
                    Err(_) => {}
                };
                Err(<__D::Error as __serde::de::Error>::custom(
                    "no matching variant found",
                ))
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = Schema;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum Schema")
                    }
                    #[inline]
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> ::core::result::Result<Self::Value, __E>
                    where
                        __E: __serde::de::Error,
                    {
                        let __identifier = __IdentifierVisitor.visit_str(__value)?;
                        #[allow(unreachable_patterns)]
                        match __identifier {
                            _ => {
                                Err(__E::invalid_value(
                                    __serde::de::Unexpected::Str(__value),
                                    &self,
                                ))
                            }
                        }
                    }
                    #[inline]
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> ::core::result::Result<Self::Value, __A::Error>
                    where
                        __A: __serde::de::EnumAccess<'de>,
                    {
                        match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                            (__Identifier::__Identifier0, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<bool>(__variant)?;
                                ::core::result::Result::Ok(Schema::Bool(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    SchemaObject,
                                >(__variant)?;
                                ::core::result::Result::Ok(Schema::Object(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Schema",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "A root schema.\n"]
    #[derive(Clone, Debug)]
    pub struct RootSchema {
        #[doc = ""]
        pub meta_schema: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub schema: SchemaObject,
        #[doc = ""]
        pub defs: ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
    }
    impl RootSchema {
        #[doc = "Creates a new [`RootSchema`]."]
        pub fn new(schema: SchemaObject) -> Self {
            Self {
                schema,
                meta_schema: ::std::default::Default::default(),
                defs: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `meta_schema`."]
        pub fn set_meta_schema(
            &mut self,
            meta_schema: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.meta_schema = meta_schema;
            self
        }
        #[doc = "Sets the value of `meta_schema`."]
        pub fn with_meta_schema(
            mut self,
            meta_schema: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.meta_schema = meta_schema;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn set_schema(&mut self, schema: SchemaObject) -> &mut Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn with_schema(mut self, schema: SchemaObject) -> Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `defs`."]
        pub fn set_defs(
            &mut self,
            defs: ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
        ) -> &mut Self {
            self.defs = defs;
            self
        }
        #[doc = "Sets the value of `defs`."]
        pub fn with_defs(
            mut self,
            defs: ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
        ) -> Self {
            self.defs = defs;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for RootSchema {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "RootSchema", 3usize)?;
            __record.serialize_optional_field(
                "$schema",
                ::core::option::Option::as_ref(&self.meta_schema),
            )?;
            __record.serialize_inlined_field("schema", &self.schema)?;
            __record
                .serialize_optional_field("$defs", ::core::option::Option::as_ref(&self.defs))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RootSchema {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["$schema", "schema", "$defs"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"$schema\", \"schema\", \"$defs\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "$schema" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "schema" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "$defs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"$schema" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"schema" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"$defs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = RootSchema;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record RootSchema")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 =
                        match __serde::de::SeqAccess::next_element::<SchemaObject>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        1usize,
                                        &"record with 3 fields",
                                    ),
                                );
                            }
                        };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(RootSchema {
                        meta_schema: __field0,
                        schema: __field1,
                        defs: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<SchemaObject> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "$schema",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schema",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<SchemaObject>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "$defs",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, Schema>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("schema"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(RootSchema {
                        meta_schema: __field0,
                        schema: __field1,
                        defs: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["$schema", "schema", "$defs"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "RootSchema",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A schema object.\n\nSee [JSON Schema 4.3.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n"]
    #[derive(Clone, Debug)]
    pub struct SchemaObject {
        #[doc = "The `$id` keyword.\n\nSee [JSON Schema 8.2.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-id-keyword).\n"]
        pub id: ::std::option::Option<::std::string::String>,
        #[doc = "The `type` keyword.\n\nSee [JSON Schema Validation 6.1.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-type).\n"]
        pub allowed_types: ::std::option::Option<MaybeArray<Type>>,
        #[doc = "The `enum` keyword.\n\nSee [JSON Schema Validation 6.1.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-enum).\n"]
        pub allowed_values: ::std::option::Option<::std::vec::Vec<Any>>,
        #[doc = "The `const` keyword.\n\nSee [JSON Schema Validation 6.1.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-const).\n"]
        pub allowed_value: ::std::option::Option<Any>,
        #[doc = "The `format` keyword.\n\nSee [JSON Schema Validation 7](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-vocabularies-for-semantic-c).\n"]
        pub format: ::std::option::Option<::std::string::String>,
        #[doc = "The `$ref` keyword.\n"]
        pub reference: ::std::option::Option<SchemaRef>,
        #[doc = "The `$comment` keyword.\n"]
        pub comment: ::std::option::Option<::std::string::String>,
        #[doc = "Basic metadata annotations.\n\nSee [JSON Schema Validation 9](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-a-vocabulary-for-basic-meta).\n"]
        pub metadata: ::std::option::Option<::std::boxed::Box<Metadata>>,
        #[doc = "Keywords for combining and conditionally applying subschemas.\n\nSee [JSON Schema 10.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keywords-for-applying-subsc).\n"]
        pub subschema_keywords: ::std::option::Option<::std::boxed::Box<SubschemaKeywords>>,
        #[doc = "Keywords applying to numbers.\n"]
        pub number_keywords: ::std::option::Option<::std::boxed::Box<NumberKeywords>>,
        #[doc = "Keywords applying to strings.\n"]
        pub string_keywords: ::std::option::Option<::std::boxed::Box<StringKeywords>>,
        #[doc = "Keywords applying to arrays.\n"]
        pub array_keywords: ::std::option::Option<::std::boxed::Box<ArrayKeywords>>,
        #[doc = "Keywords applying to objects.\n"]
        pub object_keywords: ::std::option::Option<::std::boxed::Box<ObjectKeywords>>,
        #[doc = "Schema extensions.\n"]
        pub extensions: ::std::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
    }
    impl SchemaObject {
        #[doc = "Creates a new [`SchemaObject`]."]
        pub fn new() -> Self {
            Self {
                id: ::std::default::Default::default(),
                allowed_types: ::std::default::Default::default(),
                allowed_values: ::std::default::Default::default(),
                allowed_value: ::std::default::Default::default(),
                format: ::std::default::Default::default(),
                reference: ::std::default::Default::default(),
                comment: ::std::default::Default::default(),
                metadata: ::std::default::Default::default(),
                subschema_keywords: ::std::default::Default::default(),
                number_keywords: ::std::default::Default::default(),
                string_keywords: ::std::default::Default::default(),
                array_keywords: ::std::default::Default::default(),
                object_keywords: ::std::default::Default::default(),
                extensions: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(&mut self, id: ::std::option::Option<::std::string::String>) -> &mut Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: ::std::option::Option<::std::string::String>) -> Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `allowed_types`."]
        pub fn set_allowed_types(
            &mut self,
            allowed_types: ::std::option::Option<MaybeArray<Type>>,
        ) -> &mut Self {
            self.allowed_types = allowed_types;
            self
        }
        #[doc = "Sets the value of `allowed_types`."]
        pub fn with_allowed_types(
            mut self,
            allowed_types: ::std::option::Option<MaybeArray<Type>>,
        ) -> Self {
            self.allowed_types = allowed_types;
            self
        }
        #[doc = "Sets the value of `allowed_values`."]
        pub fn set_allowed_values(
            &mut self,
            allowed_values: ::std::option::Option<::std::vec::Vec<Any>>,
        ) -> &mut Self {
            self.allowed_values = allowed_values;
            self
        }
        #[doc = "Sets the value of `allowed_values`."]
        pub fn with_allowed_values(
            mut self,
            allowed_values: ::std::option::Option<::std::vec::Vec<Any>>,
        ) -> Self {
            self.allowed_values = allowed_values;
            self
        }
        #[doc = "Sets the value of `allowed_value`."]
        pub fn set_allowed_value(
            &mut self,
            allowed_value: ::std::option::Option<Any>,
        ) -> &mut Self {
            self.allowed_value = allowed_value;
            self
        }
        #[doc = "Sets the value of `allowed_value`."]
        pub fn with_allowed_value(mut self, allowed_value: ::std::option::Option<Any>) -> Self {
            self.allowed_value = allowed_value;
            self
        }
        #[doc = "Sets the value of `format`."]
        pub fn set_format(
            &mut self,
            format: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.format = format;
            self
        }
        #[doc = "Sets the value of `format`."]
        pub fn with_format(mut self, format: ::std::option::Option<::std::string::String>) -> Self {
            self.format = format;
            self
        }
        #[doc = "Sets the value of `reference`."]
        pub fn set_reference(&mut self, reference: ::std::option::Option<SchemaRef>) -> &mut Self {
            self.reference = reference;
            self
        }
        #[doc = "Sets the value of `reference`."]
        pub fn with_reference(mut self, reference: ::std::option::Option<SchemaRef>) -> Self {
            self.reference = reference;
            self
        }
        #[doc = "Sets the value of `comment`."]
        pub fn set_comment(
            &mut self,
            comment: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.comment = comment;
            self
        }
        #[doc = "Sets the value of `comment`."]
        pub fn with_comment(
            mut self,
            comment: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.comment = comment;
            self
        }
        #[doc = "Sets the value of `metadata`."]
        pub fn set_metadata(
            &mut self,
            metadata: ::std::option::Option<::std::boxed::Box<Metadata>>,
        ) -> &mut Self {
            self.metadata = metadata;
            self
        }
        #[doc = "Sets the value of `metadata`."]
        pub fn with_metadata(
            mut self,
            metadata: ::std::option::Option<::std::boxed::Box<Metadata>>,
        ) -> Self {
            self.metadata = metadata;
            self
        }
        #[doc = "Sets the value of `subschema_keywords`."]
        pub fn set_subschema_keywords(
            &mut self,
            subschema_keywords: ::std::option::Option<::std::boxed::Box<SubschemaKeywords>>,
        ) -> &mut Self {
            self.subschema_keywords = subschema_keywords;
            self
        }
        #[doc = "Sets the value of `subschema_keywords`."]
        pub fn with_subschema_keywords(
            mut self,
            subschema_keywords: ::std::option::Option<::std::boxed::Box<SubschemaKeywords>>,
        ) -> Self {
            self.subschema_keywords = subschema_keywords;
            self
        }
        #[doc = "Sets the value of `number_keywords`."]
        pub fn set_number_keywords(
            &mut self,
            number_keywords: ::std::option::Option<::std::boxed::Box<NumberKeywords>>,
        ) -> &mut Self {
            self.number_keywords = number_keywords;
            self
        }
        #[doc = "Sets the value of `number_keywords`."]
        pub fn with_number_keywords(
            mut self,
            number_keywords: ::std::option::Option<::std::boxed::Box<NumberKeywords>>,
        ) -> Self {
            self.number_keywords = number_keywords;
            self
        }
        #[doc = "Sets the value of `string_keywords`."]
        pub fn set_string_keywords(
            &mut self,
            string_keywords: ::std::option::Option<::std::boxed::Box<StringKeywords>>,
        ) -> &mut Self {
            self.string_keywords = string_keywords;
            self
        }
        #[doc = "Sets the value of `string_keywords`."]
        pub fn with_string_keywords(
            mut self,
            string_keywords: ::std::option::Option<::std::boxed::Box<StringKeywords>>,
        ) -> Self {
            self.string_keywords = string_keywords;
            self
        }
        #[doc = "Sets the value of `array_keywords`."]
        pub fn set_array_keywords(
            &mut self,
            array_keywords: ::std::option::Option<::std::boxed::Box<ArrayKeywords>>,
        ) -> &mut Self {
            self.array_keywords = array_keywords;
            self
        }
        #[doc = "Sets the value of `array_keywords`."]
        pub fn with_array_keywords(
            mut self,
            array_keywords: ::std::option::Option<::std::boxed::Box<ArrayKeywords>>,
        ) -> Self {
            self.array_keywords = array_keywords;
            self
        }
        #[doc = "Sets the value of `object_keywords`."]
        pub fn set_object_keywords(
            &mut self,
            object_keywords: ::std::option::Option<::std::boxed::Box<ObjectKeywords>>,
        ) -> &mut Self {
            self.object_keywords = object_keywords;
            self
        }
        #[doc = "Sets the value of `object_keywords`."]
        pub fn with_object_keywords(
            mut self,
            object_keywords: ::std::option::Option<::std::boxed::Box<ObjectKeywords>>,
        ) -> Self {
            self.object_keywords = object_keywords;
            self
        }
        #[doc = "Sets the value of `extensions`."]
        pub fn set_extensions(
            &mut self,
            extensions: ::std::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
        ) -> &mut Self {
            self.extensions = extensions;
            self
        }
        #[doc = "Sets the value of `extensions`."]
        pub fn with_extensions(
            mut self,
            extensions: ::std::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
        ) -> Self {
            self.extensions = extensions;
            self
        }
    }
    impl ::std::default::Default for SchemaObject {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SchemaObject {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SchemaObject", 14usize)?;
            __record.serialize_optional_field("$id", ::core::option::Option::as_ref(&self.id))?;
            __record.serialize_optional_field(
                "type",
                ::core::option::Option::as_ref(&self.allowed_types),
            )?;
            __record.serialize_optional_field(
                "enum",
                ::core::option::Option::as_ref(&self.allowed_values),
            )?;
            __record.serialize_optional_field(
                "const",
                ::core::option::Option::as_ref(&self.allowed_value),
            )?;
            __record
                .serialize_optional_field("format", ::core::option::Option::as_ref(&self.format))?;
            __record.serialize_optional_field(
                "$ref",
                ::core::option::Option::as_ref(&self.reference),
            )?;
            __record.serialize_optional_field(
                "$comment",
                ::core::option::Option::as_ref(&self.comment),
            )?;
            __record.serialize_inlined_field("metadata", &self.metadata)?;
            __record.serialize_inlined_field("subschemaKeywords", &self.subschema_keywords)?;
            __record.serialize_inlined_field("numberKeywords", &self.number_keywords)?;
            __record.serialize_inlined_field("stringKeywords", &self.string_keywords)?;
            __record.serialize_inlined_field("arrayKeywords", &self.array_keywords)?;
            __record.serialize_inlined_field("objectKeywords", &self.object_keywords)?;
            __record.serialize_inlined_field("extensions", &self.extensions)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SchemaObject {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "$id",
                "type",
                "enum",
                "const",
                "format",
                "$ref",
                "$comment",
                "metadata",
                "subschemaKeywords",
                "numberKeywords",
                "stringKeywords",
                "arrayKeywords",
                "objectKeywords",
                "extensions",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"$id\", \"type\", \"enum\", \"const\", \"format\", \"$ref\", \"$comment\", \"metadata\", \"subschemaKeywords\", \"numberKeywords\", \"stringKeywords\", \"arrayKeywords\", \"objectKeywords\", \"extensions\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
                __Identifier6,
                __Identifier7,
                __Identifier8,
                __Identifier9,
                __Identifier10,
                __Identifier11,
                __Identifier12,
                __Identifier13,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        7u64 => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        8u64 => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        9u64 => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        10u64 => ::core::result::Result::Ok(__Identifier::__Identifier10),
                        11u64 => ::core::result::Result::Ok(__Identifier::__Identifier11),
                        12u64 => ::core::result::Result::Ok(__Identifier::__Identifier12),
                        13u64 => ::core::result::Result::Ok(__Identifier::__Identifier13),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "$id" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "type" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "enum" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "const" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "format" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "$ref" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "$comment" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "metadata" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "subschemaKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier8)
                        }
                        "numberKeywords" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        "stringKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier10)
                        }
                        "arrayKeywords" => ::core::result::Result::Ok(__Identifier::__Identifier11),
                        "objectKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier12)
                        }
                        "extensions" => ::core::result::Result::Ok(__Identifier::__Identifier13),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"$id" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"type" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"enum" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"const" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"format" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"$ref" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"$comment" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"metadata" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"subschemaKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier8)
                        }
                        b"numberKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier9)
                        }
                        b"stringKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier10)
                        }
                        b"arrayKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier11)
                        }
                        b"objectKeywords" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier12)
                        }
                        b"extensions" => ::core::result::Result::Ok(__Identifier::__Identifier13),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SchemaObject;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SchemaObject")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    0usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<MaybeArray<Type>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    1usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Any>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    2usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Any>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    3usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    4usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<SchemaRef>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    5usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    6usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<Metadata>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    7usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<SubschemaKeywords>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    8usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field9 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<NumberKeywords>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    9usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field10 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<StringKeywords>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    10usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field11 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<ArrayKeywords>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    11usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field12 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<ObjectKeywords>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    12usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    let __field13 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    13usize,
                                    &"record with 14 fields",
                                ),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SchemaObject {
                        id: __field0,
                        allowed_types: __field1,
                        allowed_values: __field2,
                        allowed_value: __field3,
                        format: __field4,
                        reference: __field5,
                        comment: __field6,
                        metadata: __field7,
                        subschema_keywords: __field8,
                        number_keywords: __field9,
                        string_keywords: __field10,
                        array_keywords: __field11,
                        object_keywords: __field12,
                        extensions: __field13,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<MaybeArray<Type>>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Any>>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<SchemaRef>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<Metadata>>,
                    > = ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<SubschemaKeywords>>,
                    > = ::core::option::Option::None;
                    let mut __field9: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<NumberKeywords>>,
                    > = ::core::option::Option::None;
                    let mut __field10: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<StringKeywords>>,
                    > = ::core::option::Option::None;
                    let mut __field11: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<ArrayKeywords>>,
                    > = ::core::option::Option::None;
                    let mut __field12: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<ObjectKeywords>>,
                    > = ::core::option::Option::None;
                    let mut __field13: ::core::option::Option<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("$id"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<MaybeArray<Type>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("enum"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Any>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "const",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Any>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "format",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("$ref"),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<SchemaRef>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "$comment",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "metadata",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<Metadata>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "subschemaKeywords",
                                        ),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::boxed::Box<SubschemaKeywords>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier9 => {
                                if ::core::option::Option::is_some(&__field9) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "numberKeywords",
                                        ),
                                    );
                                }
                                __field9 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<NumberKeywords>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier10 => {
                                if ::core::option::Option::is_some(&__field10) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "stringKeywords",
                                        ),
                                    );
                                }
                                __field10 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<StringKeywords>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier11 => {
                                if ::core::option::Option::is_some(&__field11) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "arrayKeywords",
                                        ),
                                    );
                                }
                                __field11 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<ArrayKeywords>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier12 => {
                                if ::core::option::Option::is_some(&__field12) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "objectKeywords",
                                        ),
                                    );
                                }
                                __field12 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<ObjectKeywords>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier13 => {
                                if ::core::option::Option::is_some(&__field13) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "extensions",
                                        ),
                                    );
                                }
                                __field13 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, Any>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field7 = match __field7 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field8 = match __field8 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field9 = match __field9 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field10 = match __field10 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field11 = match __field11 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field12 = match __field12 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field13 = match __field13 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(SchemaObject {
                        id: __field0,
                        allowed_types: __field1,
                        allowed_values: __field2,
                        allowed_value: __field3,
                        format: __field4,
                        reference: __field5,
                        comment: __field6,
                        metadata: __field7,
                        subschema_keywords: __field8,
                        number_keywords: __field9,
                        string_keywords: __field10,
                        array_keywords: __field11,
                        object_keywords: __field12,
                        extensions: __field13,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "$id",
                "type",
                "enum",
                "const",
                "format",
                "$ref",
                "$comment",
                "metadata",
                "subschemaKeywords",
                "numberKeywords",
                "stringKeywords",
                "arrayKeywords",
                "objectKeywords",
                "extensions",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SchemaObject",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Types for the `type` keyword.\n\nSee [JSON Schema Validation 6.1.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-type).\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Type {
        #[doc = ""]
        Null,
        #[doc = ""]
        Boolean,
        #[doc = ""]
        Object,
        #[doc = ""]
        Array,
        #[doc = ""]
        Number,
        #[doc = ""]
        String,
        #[doc = ""]
        Integer,
    }
    #[automatically_derived]
    impl __serde::Serialize for Type {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "Type");
            match self {
                Self::Null => __serializer.serialize_tag("null", 0u32),
                Self::Boolean => __serializer.serialize_tag("boolean", 1u32),
                Self::Object => __serializer.serialize_tag("object", 2u32),
                Self::Array => __serializer.serialize_tag("array", 3u32),
                Self::Number => __serializer.serialize_tag("number", 4u32),
                Self::String => __serializer.serialize_tag("string", 5u32),
                Self::Integer => __serializer.serialize_tag("integer", 6u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Type {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "null", "boolean", "object", "array", "number", "string", "integer",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"null\", \"boolean\", \"object\", \"array\", \"number\", \"string\", \"integer\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
                __Identifier6,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "null" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "boolean" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "object" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "array" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "number" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "string" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "integer" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::unknown_variant(
                                __variant,
                                __IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"null" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"boolean" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"object" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"array" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"number" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"string" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"integer" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &[
                "null", "boolean", "object", "array", "number", "string", "integer",
            ];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Type;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum Type")
                }
                #[inline]
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    let __identifier = __IdentifierVisitor.visit_str(__value)?;
                    #[allow(unreachable_patterns)]
                    match __identifier {
                        __Identifier::__Identifier0 => ::core::result::Result::Ok(Type::Null),
                        __Identifier::__Identifier1 => ::core::result::Result::Ok(Type::Boolean),
                        __Identifier::__Identifier2 => ::core::result::Result::Ok(Type::Object),
                        __Identifier::__Identifier3 => ::core::result::Result::Ok(Type::Array),
                        __Identifier::__Identifier4 => ::core::result::Result::Ok(Type::Number),
                        __Identifier::__Identifier5 => ::core::result::Result::Ok(Type::String),
                        __Identifier::__Identifier6 => ::core::result::Result::Ok(Type::Integer),
                        _ => {
                            Err(__E::invalid_value(
                                __serde::de::Unexpected::Str(__value),
                                &self,
                            ))
                        }
                    }
                }
                #[inline]
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::EnumAccess<'de>,
                {
                    match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                        (__Identifier::__Identifier0, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Type::Null)
                        }
                        (__Identifier::__Identifier1, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Type::Boolean)
                        }
                        (__Identifier::__Identifier2, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Type::Object)
                        }
                        (__Identifier::__Identifier3, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Type::Array)
                        }
                        (__Identifier::__Identifier4, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Type::Number)
                        }
                        (__Identifier::__Identifier5, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Type::String)
                        }
                        (__Identifier::__Identifier6, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Type::Integer)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "Type",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Either a single value of type `T` or an array of type `T`.\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum MaybeArray<T> {
        #[doc = ""]
        Single(T),
        #[doc = ""]
        Array(::std::vec::Vec<T>),
    }
    #[automatically_derived]
    impl<T: __serde::Serialize> __serde::Serialize for MaybeArray<T> {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "MaybeArray");
            match self {
                Self::Single(__value) => {
                    __serializer.serialize_implicitly_tagged("Single", 0u32, __value)
                }
                Self::Array(__value) => {
                    __serializer.serialize_implicitly_tagged("Array", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de, T: __serde::Deserialize<'de>> __serde::Deserialize<'de> for MaybeArray<T> {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["Single", "Array"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"Single\", \"Array\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "Single" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Array" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::unknown_variant(
                                __variant,
                                __IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"Single" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Array" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &["Single", "Array"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<T, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(MaybeArray::Single(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<
                    ::std::vec::Vec<T>,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(MaybeArray::Array(__value)),
                    Err(_) => {}
                };
                Err(<__D::Error as __serde::de::Error>::custom(
                    "no matching variant found",
                ))
            } else {
                #[doc(hidden)]
                struct __Visitor<T> {
                    __phantom_vars: ::core::marker::PhantomData<fn(&(T,))>,
                }
                impl<'de, T: __serde::Deserialize<'de>> __serde::de::Visitor<'de> for __Visitor<T> {
                    type Value = MaybeArray<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum MaybeArray")
                    }
                    #[inline]
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> ::core::result::Result<Self::Value, __E>
                    where
                        __E: __serde::de::Error,
                    {
                        let __identifier = __IdentifierVisitor.visit_str(__value)?;
                        #[allow(unreachable_patterns)]
                        match __identifier {
                            _ => {
                                Err(__E::invalid_value(
                                    __serde::de::Unexpected::Str(__value),
                                    &self,
                                ))
                            }
                        }
                    }
                    #[inline]
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> ::core::result::Result<Self::Value, __A::Error>
                    where
                        __A: __serde::de::EnumAccess<'de>,
                    {
                        match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                            (__Identifier::__Identifier0, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<T>(__variant)?;
                                ::core::result::Result::Ok(MaybeArray::Single(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::vec::Vec<T>,
                                >(__variant)?;
                                ::core::result::Result::Ok(MaybeArray::Array(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "MaybeArray",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Any JSON value.\n"]
    #[derive(Clone, Debug)]
    pub enum Any {
        #[doc = ""]
        Boolean(bool),
        #[doc = ""]
        Integer(i32),
        #[doc = ""]
        Number(f64),
        #[doc = ""]
        String(::std::string::String),
        #[doc = ""]
        Object(indexmap::IndexMap<::std::string::String, Any>),
        #[doc = ""]
        Array(::std::vec::Vec<Any>),
    }
    #[automatically_derived]
    impl __serde::Serialize for Any {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "Any");
            match self {
                Self::Boolean(__value) => {
                    __serializer.serialize_implicitly_tagged("Boolean", 0u32, __value)
                }
                Self::Integer(__value) => {
                    __serializer.serialize_implicitly_tagged("Integer", 1u32, __value)
                }
                Self::Number(__value) => {
                    __serializer.serialize_implicitly_tagged("Number", 2u32, __value)
                }
                Self::String(__value) => {
                    __serializer.serialize_implicitly_tagged("String", 3u32, __value)
                }
                Self::Object(__value) => {
                    __serializer.serialize_implicitly_tagged("Object", 4u32, __value)
                }
                Self::Array(__value) => {
                    __serializer.serialize_implicitly_tagged("Array", 5u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Any {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["Boolean", "Integer", "Number", "String", "Object", "Array"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"Boolean\", \"Integer\", \"Number\", \"String\", \"Object\", \"Array\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "Boolean" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Integer" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "Number" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "String" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "Object" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "Array" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::unknown_variant(
                                __variant,
                                __IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"Boolean" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Integer" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"Number" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"String" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"Object" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"Array" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] =
                &["Boolean", "Integer", "Number", "String", "Object", "Array"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<bool, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Any::Boolean(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<i32, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Any::Integer(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<f64, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Any::Number(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<
                    ::std::string::String,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(Any::String(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<
                    indexmap::IndexMap<::std::string::String, Any>,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(Any::Object(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<
                    ::std::vec::Vec<Any>,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(Any::Array(__value)),
                    Err(_) => {}
                };
                Err(<__D::Error as __serde::de::Error>::custom(
                    "no matching variant found",
                ))
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = Any;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum Any")
                    }
                    #[inline]
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> ::core::result::Result<Self::Value, __E>
                    where
                        __E: __serde::de::Error,
                    {
                        let __identifier = __IdentifierVisitor.visit_str(__value)?;
                        #[allow(unreachable_patterns)]
                        match __identifier {
                            _ => {
                                Err(__E::invalid_value(
                                    __serde::de::Unexpected::Str(__value),
                                    &self,
                                ))
                            }
                        }
                    }
                    #[inline]
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> ::core::result::Result<Self::Value, __A::Error>
                    where
                        __A: __serde::de::EnumAccess<'de>,
                    {
                        match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                            (__Identifier::__Identifier0, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<bool>(__variant)?;
                                ::core::result::Result::Ok(Any::Boolean(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<i32>(__variant)?;
                                ::core::result::Result::Ok(Any::Integer(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<f64>(__variant)?;
                                ::core::result::Result::Ok(Any::Number(__value))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(Any::String(__value))
                            }
                            (__Identifier::__Identifier4, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    indexmap::IndexMap<::std::string::String, Any>,
                                >(__variant)?;
                                ::core::result::Result::Ok(Any::Object(__value))
                            }
                            (__Identifier::__Identifier5, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::vec::Vec<Any>,
                                >(__variant)?;
                                ::core::result::Result::Ok(Any::Array(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Any",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "A reference to a JSON schema.\n"]
    #[derive(Clone, Debug)]
    pub struct SchemaRef(pub(crate) ::std::string::String);
    impl ::std::convert::From<SchemaRef> for ::std::string::String {
        fn from(wrapped: SchemaRef) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SchemaRef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SchemaRef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(SchemaRef(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = "Basic metadata annotations.\n\nSee [JSON Schema Validation 9](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-a-vocabulary-for-basic-meta).\n"]
    #[derive(Clone, Debug)]
    pub struct Metadata {
        #[doc = "The `title` keyword.\n\nSee [JSON Schema Validation 9.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-title-and-description).\n"]
        pub title: ::std::option::Option<::std::string::String>,
        #[doc = "The `description` keyword.\n\nSee [JSON Schema Validation 9.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-title-and-description).\n"]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = "The `default` keyword.\n\nSee [JSON Schema Validation 9.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-default).\n"]
        pub default: ::std::option::Option<Any>,
        #[doc = "The `deprecated` keyword.\n\nSee [JSON Schema Validation 9.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-deprecated).\n"]
        pub deprecated: ::std::option::Option<bool>,
        #[doc = "The `readOnly` keyword.\n\nSee [JSON Schema Validation 9.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-readonly-and-writeonly).\n"]
        pub read_only: ::std::option::Option<bool>,
        #[doc = "The `readWrite` keyword.\n\nSee [JSON Schema Validation 9.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-readonly-and-writeonly).\n"]
        pub write_only: ::std::option::Option<bool>,
        #[doc = "The `examples` keyword.\n\nSee [JSON Schema Validation 9.5](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-examples).\n"]
        pub examples: ::std::option::Option<::std::vec::Vec<Any>>,
    }
    impl Metadata {
        #[doc = "Creates a new [`Metadata`]."]
        pub fn new() -> Self {
            Self {
                title: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                default: ::std::default::Default::default(),
                deprecated: ::std::default::Default::default(),
                read_only: ::std::default::Default::default(),
                write_only: ::std::default::Default::default(),
                examples: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `title`."]
        pub fn set_title(
            &mut self,
            title: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.title = title;
            self
        }
        #[doc = "Sets the value of `title`."]
        pub fn with_title(mut self, title: ::std::option::Option<::std::string::String>) -> Self {
            self.title = title;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn set_default(&mut self, default: ::std::option::Option<Any>) -> &mut Self {
            self.default = default;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn with_default(mut self, default: ::std::option::Option<Any>) -> Self {
            self.default = default;
            self
        }
        #[doc = "Sets the value of `deprecated`."]
        pub fn set_deprecated(&mut self, deprecated: ::std::option::Option<bool>) -> &mut Self {
            self.deprecated = deprecated;
            self
        }
        #[doc = "Sets the value of `deprecated`."]
        pub fn with_deprecated(mut self, deprecated: ::std::option::Option<bool>) -> Self {
            self.deprecated = deprecated;
            self
        }
        #[doc = "Sets the value of `read_only`."]
        pub fn set_read_only(&mut self, read_only: ::std::option::Option<bool>) -> &mut Self {
            self.read_only = read_only;
            self
        }
        #[doc = "Sets the value of `read_only`."]
        pub fn with_read_only(mut self, read_only: ::std::option::Option<bool>) -> Self {
            self.read_only = read_only;
            self
        }
        #[doc = "Sets the value of `write_only`."]
        pub fn set_write_only(&mut self, write_only: ::std::option::Option<bool>) -> &mut Self {
            self.write_only = write_only;
            self
        }
        #[doc = "Sets the value of `write_only`."]
        pub fn with_write_only(mut self, write_only: ::std::option::Option<bool>) -> Self {
            self.write_only = write_only;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn set_examples(
            &mut self,
            examples: ::std::option::Option<::std::vec::Vec<Any>>,
        ) -> &mut Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn with_examples(
            mut self,
            examples: ::std::option::Option<::std::vec::Vec<Any>>,
        ) -> Self {
            self.examples = examples;
            self
        }
    }
    impl ::std::default::Default for Metadata {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Metadata {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Metadata", 7usize)?;
            __record
                .serialize_optional_field("title", ::core::option::Option::as_ref(&self.title))?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "default",
                ::core::option::Option::as_ref(&self.default),
            )?;
            __record.serialize_optional_field(
                "deprecated",
                ::core::option::Option::as_ref(&self.deprecated),
            )?;
            __record.serialize_optional_field(
                "readOnly",
                ::core::option::Option::as_ref(&self.read_only),
            )?;
            __record.serialize_optional_field(
                "writeOnly",
                ::core::option::Option::as_ref(&self.write_only),
            )?;
            __record.serialize_optional_field(
                "examples",
                ::core::option::Option::as_ref(&self.examples),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Metadata {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "title",
                "description",
                "default",
                "deprecated",
                "readOnly",
                "writeOnly",
                "examples",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"title\", \"description\", \"default\", \"deprecated\", \"readOnly\", \"writeOnly\", \"examples\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
                __Identifier6,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "title" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "default" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "readOnly" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "writeOnly" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "examples" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"title" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"default" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"readOnly" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"writeOnly" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"examples" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Metadata;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Metadata")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Any>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Any>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 7 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Metadata {
                        title: __field0,
                        description: __field1,
                        default: __field2,
                        deprecated: __field3,
                        read_only: __field4,
                        write_only: __field5,
                        examples: __field6,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Any>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "title",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "default",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Any>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "deprecated",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "readOnly",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "writeOnly",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "examples",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Any>>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Metadata {
                        title: __field0,
                        description: __field1,
                        default: __field2,
                        deprecated: __field3,
                        read_only: __field4,
                        write_only: __field5,
                        examples: __field6,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "title",
                "description",
                "default",
                "deprecated",
                "readOnly",
                "writeOnly",
                "examples",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Metadata",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Keywords for combining and conditionally applying subschemas.\n\nSee [JSON Schema 10.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keywords-for-applying-subsc).\n"]
    #[derive(Clone, Debug)]
    pub struct SubschemaKeywords {
        #[doc = "The `allOf` keyword.\n\nSee [JSON Schema 10.2.1.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-allof).\n"]
        pub all_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        #[doc = "The `andOf` keyword.\n\nSee [JSON Schema 10.2.1.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-anyof).\n"]
        pub any_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        #[doc = "The `oneOf` keyword.\n\nSee [JSON Schema 10.2.1.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-oneof).\n"]
        pub one_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        #[doc = "The `not` keyword.\n\nSee [JSON Schema 10.2.1.4](https://json-schema.org/draft/2020-12/json-schema-core.html#name-not).\n"]
        pub not: ::std::option::Option<::std::vec::Vec<Schema>>,
        #[doc = "The `if` keyword.\n\nSee [JSON Schema 10.2.2.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-if).\n"]
        pub condition: ::std::option::Option<Schema>,
        #[doc = "The `then` keyword.\n\nSee [JSON Schema 10.2.2.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-then).\n"]
        pub consequence: ::std::option::Option<Schema>,
        #[doc = "The `else` keyword.\n\nSee [JSON Schema 10.2.2.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-else).\n"]
        pub alternative: ::std::option::Option<Schema>,
        #[doc = "The `dependentSchemas` keyword.\n\nSee [JSON Schema 10.2.2.4](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dependentschemas)\n"]
        pub dependent_schemas:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
    }
    impl SubschemaKeywords {
        #[doc = "Creates a new [`SubschemaKeywords`]."]
        pub fn new() -> Self {
            Self {
                all_of: ::std::default::Default::default(),
                any_of: ::std::default::Default::default(),
                one_of: ::std::default::Default::default(),
                not: ::std::default::Default::default(),
                condition: ::std::default::Default::default(),
                consequence: ::std::default::Default::default(),
                alternative: ::std::default::Default::default(),
                dependent_schemas: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `all_of`."]
        pub fn set_all_of(
            &mut self,
            all_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> &mut Self {
            self.all_of = all_of;
            self
        }
        #[doc = "Sets the value of `all_of`."]
        pub fn with_all_of(
            mut self,
            all_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> Self {
            self.all_of = all_of;
            self
        }
        #[doc = "Sets the value of `any_of`."]
        pub fn set_any_of(
            &mut self,
            any_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> &mut Self {
            self.any_of = any_of;
            self
        }
        #[doc = "Sets the value of `any_of`."]
        pub fn with_any_of(
            mut self,
            any_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> Self {
            self.any_of = any_of;
            self
        }
        #[doc = "Sets the value of `one_of`."]
        pub fn set_one_of(
            &mut self,
            one_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> &mut Self {
            self.one_of = one_of;
            self
        }
        #[doc = "Sets the value of `one_of`."]
        pub fn with_one_of(
            mut self,
            one_of: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> Self {
            self.one_of = one_of;
            self
        }
        #[doc = "Sets the value of `not`."]
        pub fn set_not(
            &mut self,
            not: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> &mut Self {
            self.not = not;
            self
        }
        #[doc = "Sets the value of `not`."]
        pub fn with_not(mut self, not: ::std::option::Option<::std::vec::Vec<Schema>>) -> Self {
            self.not = not;
            self
        }
        #[doc = "Sets the value of `condition`."]
        pub fn set_condition(&mut self, condition: ::std::option::Option<Schema>) -> &mut Self {
            self.condition = condition;
            self
        }
        #[doc = "Sets the value of `condition`."]
        pub fn with_condition(mut self, condition: ::std::option::Option<Schema>) -> Self {
            self.condition = condition;
            self
        }
        #[doc = "Sets the value of `consequence`."]
        pub fn set_consequence(&mut self, consequence: ::std::option::Option<Schema>) -> &mut Self {
            self.consequence = consequence;
            self
        }
        #[doc = "Sets the value of `consequence`."]
        pub fn with_consequence(mut self, consequence: ::std::option::Option<Schema>) -> Self {
            self.consequence = consequence;
            self
        }
        #[doc = "Sets the value of `alternative`."]
        pub fn set_alternative(&mut self, alternative: ::std::option::Option<Schema>) -> &mut Self {
            self.alternative = alternative;
            self
        }
        #[doc = "Sets the value of `alternative`."]
        pub fn with_alternative(mut self, alternative: ::std::option::Option<Schema>) -> Self {
            self.alternative = alternative;
            self
        }
        #[doc = "Sets the value of `dependent_schemas`."]
        pub fn set_dependent_schemas(
            &mut self,
            dependent_schemas: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, Schema>,
            >,
        ) -> &mut Self {
            self.dependent_schemas = dependent_schemas;
            self
        }
        #[doc = "Sets the value of `dependent_schemas`."]
        pub fn with_dependent_schemas(
            mut self,
            dependent_schemas: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, Schema>,
            >,
        ) -> Self {
            self.dependent_schemas = dependent_schemas;
            self
        }
    }
    impl ::std::default::Default for SubschemaKeywords {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SubschemaKeywords {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "SubschemaKeywords",
                8usize,
            )?;
            __record
                .serialize_optional_field("allOf", ::core::option::Option::as_ref(&self.all_of))?;
            __record
                .serialize_optional_field("anyOf", ::core::option::Option::as_ref(&self.any_of))?;
            __record
                .serialize_optional_field("oneOf", ::core::option::Option::as_ref(&self.one_of))?;
            __record.serialize_optional_field("not", ::core::option::Option::as_ref(&self.not))?;
            __record
                .serialize_optional_field("if", ::core::option::Option::as_ref(&self.condition))?;
            __record.serialize_optional_field(
                "then",
                ::core::option::Option::as_ref(&self.consequence),
            )?;
            __record.serialize_optional_field(
                "else",
                ::core::option::Option::as_ref(&self.alternative),
            )?;
            __record.serialize_optional_field(
                "dependentSchemas",
                ::core::option::Option::as_ref(&self.dependent_schemas),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SubschemaKeywords {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "allOf",
                "anyOf",
                "oneOf",
                "not",
                "if",
                "then",
                "else",
                "dependentSchemas",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"allOf\", \"anyOf\", \"oneOf\", \"not\", \"if\", \"then\", \"else\", \"dependentSchemas\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
                __Identifier6,
                __Identifier7,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        7u64 => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "allOf" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "anyOf" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "oneOf" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "not" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "if" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "then" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "else" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "dependentSchemas" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier7)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"allOf" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"anyOf" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"oneOf" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"not" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"if" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"then" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"else" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"dependentSchemas" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier7)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SubschemaKeywords;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SubschemaKeywords")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Schema>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Schema>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Schema>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(7usize, &"record with 8 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SubschemaKeywords {
                        all_of: __field0,
                        any_of: __field1,
                        one_of: __field2,
                        not: __field3,
                        condition: __field4,
                        consequence: __field5,
                        alternative: __field6,
                        dependent_schemas: __field7,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<Schema>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<Schema>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::core::option::Option<Schema>> =
                        ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "allOf",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "anyOf",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "oneOf",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("not"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("if"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Schema>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("then"),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Schema>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("else"),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Schema>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "dependentSchemas",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, Schema>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field7 = match __field7 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(SubschemaKeywords {
                        all_of: __field0,
                        any_of: __field1,
                        one_of: __field2,
                        not: __field3,
                        condition: __field4,
                        consequence: __field5,
                        alternative: __field6,
                        dependent_schemas: __field7,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "allOf",
                "anyOf",
                "oneOf",
                "not",
                "if",
                "then",
                "else",
                "dependentSchemas",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SubschemaKeywords",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Keywords applying to numbers.\n"]
    #[derive(Clone, Debug)]
    pub struct NumberKeywords {
        #[doc = "The `multipleOf` keyword.\n\nSee [JSON Schema Validation 6.2.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-multipleof).\n"]
        pub multiple_of: ::std::option::Option<Number>,
        #[doc = "The `maximum` keyword.\n\nSee [JSON Schema Validation 6.2.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maximum).\n"]
        pub maximum: ::std::option::Option<Number>,
        #[doc = "The `exclusiveMaximum` keyword.\n\nSee [JSON Schema Validation 6.2.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-exclusivemaximum).\n"]
        pub exclusive_maximum: ::std::option::Option<Number>,
        #[doc = "The `minimum` keyword.\n\nSee [JSON Schema Validation 6.2.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minimum).\n"]
        pub minimum: ::std::option::Option<Number>,
        #[doc = "The `exclusiveMaximum` keyword.\n\nSee [JSON Schema Validation 6.2.5](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-exclusiveminimum).\n"]
        pub exclusive_minimum: ::std::option::Option<Number>,
    }
    impl NumberKeywords {
        #[doc = "Creates a new [`NumberKeywords`]."]
        pub fn new() -> Self {
            Self {
                multiple_of: ::std::default::Default::default(),
                maximum: ::std::default::Default::default(),
                exclusive_maximum: ::std::default::Default::default(),
                minimum: ::std::default::Default::default(),
                exclusive_minimum: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `multiple_of`."]
        pub fn set_multiple_of(&mut self, multiple_of: ::std::option::Option<Number>) -> &mut Self {
            self.multiple_of = multiple_of;
            self
        }
        #[doc = "Sets the value of `multiple_of`."]
        pub fn with_multiple_of(mut self, multiple_of: ::std::option::Option<Number>) -> Self {
            self.multiple_of = multiple_of;
            self
        }
        #[doc = "Sets the value of `maximum`."]
        pub fn set_maximum(&mut self, maximum: ::std::option::Option<Number>) -> &mut Self {
            self.maximum = maximum;
            self
        }
        #[doc = "Sets the value of `maximum`."]
        pub fn with_maximum(mut self, maximum: ::std::option::Option<Number>) -> Self {
            self.maximum = maximum;
            self
        }
        #[doc = "Sets the value of `exclusive_maximum`."]
        pub fn set_exclusive_maximum(
            &mut self,
            exclusive_maximum: ::std::option::Option<Number>,
        ) -> &mut Self {
            self.exclusive_maximum = exclusive_maximum;
            self
        }
        #[doc = "Sets the value of `exclusive_maximum`."]
        pub fn with_exclusive_maximum(
            mut self,
            exclusive_maximum: ::std::option::Option<Number>,
        ) -> Self {
            self.exclusive_maximum = exclusive_maximum;
            self
        }
        #[doc = "Sets the value of `minimum`."]
        pub fn set_minimum(&mut self, minimum: ::std::option::Option<Number>) -> &mut Self {
            self.minimum = minimum;
            self
        }
        #[doc = "Sets the value of `minimum`."]
        pub fn with_minimum(mut self, minimum: ::std::option::Option<Number>) -> Self {
            self.minimum = minimum;
            self
        }
        #[doc = "Sets the value of `exclusive_minimum`."]
        pub fn set_exclusive_minimum(
            &mut self,
            exclusive_minimum: ::std::option::Option<Number>,
        ) -> &mut Self {
            self.exclusive_minimum = exclusive_minimum;
            self
        }
        #[doc = "Sets the value of `exclusive_minimum`."]
        pub fn with_exclusive_minimum(
            mut self,
            exclusive_minimum: ::std::option::Option<Number>,
        ) -> Self {
            self.exclusive_minimum = exclusive_minimum;
            self
        }
    }
    impl ::std::default::Default for NumberKeywords {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for NumberKeywords {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "NumberKeywords", 5usize)?;
            __record.serialize_optional_field(
                "multipleOf",
                ::core::option::Option::as_ref(&self.multiple_of),
            )?;
            __record.serialize_optional_field(
                "maximum",
                ::core::option::Option::as_ref(&self.maximum),
            )?;
            __record.serialize_optional_field(
                "exclusiveMaximum",
                ::core::option::Option::as_ref(&self.exclusive_maximum),
            )?;
            __record.serialize_optional_field(
                "minimum",
                ::core::option::Option::as_ref(&self.minimum),
            )?;
            __record.serialize_optional_field(
                "exclusiveMinimum",
                ::core::option::Option::as_ref(&self.exclusive_minimum),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for NumberKeywords {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "multipleOf",
                "maximum",
                "exclusiveMaximum",
                "minimum",
                "exclusiveMinimum",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"multipleOf\", \"maximum\", \"exclusiveMaximum\", \"minimum\", \"exclusiveMinimum\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "multipleOf" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "maximum" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "exclusiveMaximum" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier2)
                        }
                        "minimum" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "exclusiveMinimum" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier4)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"multipleOf" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"maximum" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"exclusiveMaximum" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier2)
                        }
                        b"minimum" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"exclusiveMinimum" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier4)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = NumberKeywords;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record NumberKeywords")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Number>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Number>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Number>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Number>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Number>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(NumberKeywords {
                        multiple_of: __field0,
                        maximum: __field1,
                        exclusive_maximum: __field2,
                        minimum: __field3,
                        exclusive_minimum: __field4,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<::core::option::Option<Number>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Number>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Number>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<Number>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<Number>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "multipleOf",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Number>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "maximum",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Number>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "exclusiveMaximum",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Number>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "minimum",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Number>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "exclusiveMinimum",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Number>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(NumberKeywords {
                        multiple_of: __field0,
                        maximum: __field1,
                        exclusive_maximum: __field2,
                        minimum: __field3,
                        exclusive_minimum: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "multipleOf",
                "maximum",
                "exclusiveMaximum",
                "minimum",
                "exclusiveMinimum",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "NumberKeywords",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A number that is either an integer or a floating point number.\n"]
    #[derive(Clone, Debug)]
    pub enum Number {
        #[doc = ""]
        Integer(i64),
        #[doc = ""]
        Float(f64),
    }
    #[automatically_derived]
    impl __serde::Serialize for Number {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "Number");
            match self {
                Self::Integer(__value) => {
                    __serializer.serialize_implicitly_tagged("Integer", 0u32, __value)
                }
                Self::Float(__value) => {
                    __serializer.serialize_implicitly_tagged("Float", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Number {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["Integer", "Float"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"Integer\", \"Float\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "Integer" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Float" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::unknown_variant(
                                __variant,
                                __IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"Integer" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Float" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &["Integer", "Float"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<i64, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Number::Integer(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<f64, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Number::Float(__value)),
                    Err(_) => {}
                };
                Err(<__D::Error as __serde::de::Error>::custom(
                    "no matching variant found",
                ))
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = Number;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum Number")
                    }
                    #[inline]
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> ::core::result::Result<Self::Value, __E>
                    where
                        __E: __serde::de::Error,
                    {
                        let __identifier = __IdentifierVisitor.visit_str(__value)?;
                        #[allow(unreachable_patterns)]
                        match __identifier {
                            _ => {
                                Err(__E::invalid_value(
                                    __serde::de::Unexpected::Str(__value),
                                    &self,
                                ))
                            }
                        }
                    }
                    #[inline]
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> ::core::result::Result<Self::Value, __A::Error>
                    where
                        __A: __serde::de::EnumAccess<'de>,
                    {
                        match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                            (__Identifier::__Identifier0, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<i64>(__variant)?;
                                ::core::result::Result::Ok(Number::Integer(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<f64>(__variant)?;
                                ::core::result::Result::Ok(Number::Float(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Number",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Keywords applying to strings.\n"]
    #[derive(Clone, Debug)]
    pub struct StringKeywords {
        #[doc = "The `maxLength` keyword.\n\nSee [JSON Schema Validation 6.3.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxlength).\n"]
        pub max_length: ::std::option::Option<usize>,
        #[doc = "The `minLength` keyword.\n\nSee [JSON Schema Validation 6.3.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minlength).\n"]
        pub min_length: ::std::option::Option<usize>,
        #[doc = "The `pattern` keyword.\n\nSee [JSON Schema Validation 6.3.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-pattern).\n"]
        pub pattern: ::std::option::Option<Pattern>,
    }
    impl StringKeywords {
        #[doc = "Creates a new [`StringKeywords`]."]
        pub fn new() -> Self {
            Self {
                max_length: ::std::default::Default::default(),
                min_length: ::std::default::Default::default(),
                pattern: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `max_length`."]
        pub fn set_max_length(&mut self, max_length: ::std::option::Option<usize>) -> &mut Self {
            self.max_length = max_length;
            self
        }
        #[doc = "Sets the value of `max_length`."]
        pub fn with_max_length(mut self, max_length: ::std::option::Option<usize>) -> Self {
            self.max_length = max_length;
            self
        }
        #[doc = "Sets the value of `min_length`."]
        pub fn set_min_length(&mut self, min_length: ::std::option::Option<usize>) -> &mut Self {
            self.min_length = min_length;
            self
        }
        #[doc = "Sets the value of `min_length`."]
        pub fn with_min_length(mut self, min_length: ::std::option::Option<usize>) -> Self {
            self.min_length = min_length;
            self
        }
        #[doc = "Sets the value of `pattern`."]
        pub fn set_pattern(&mut self, pattern: ::std::option::Option<Pattern>) -> &mut Self {
            self.pattern = pattern;
            self
        }
        #[doc = "Sets the value of `pattern`."]
        pub fn with_pattern(mut self, pattern: ::std::option::Option<Pattern>) -> Self {
            self.pattern = pattern;
            self
        }
    }
    impl ::std::default::Default for StringKeywords {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for StringKeywords {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "StringKeywords", 3usize)?;
            __record.serialize_optional_field(
                "maxLength",
                ::core::option::Option::as_ref(&self.max_length),
            )?;
            __record.serialize_optional_field(
                "minLength",
                ::core::option::Option::as_ref(&self.min_length),
            )?;
            __record.serialize_optional_field(
                "pattern",
                ::core::option::Option::as_ref(&self.pattern),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for StringKeywords {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["maxLength", "minLength", "pattern"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"maxLength\", \"minLength\", \"pattern\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "maxLength" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "minLength" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "pattern" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"maxLength" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"minLength" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"pattern" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = StringKeywords;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record StringKeywords")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Pattern>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(StringKeywords {
                        max_length: __field0,
                        min_length: __field1,
                        pattern: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Pattern>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "maxLength",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "minLength",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "pattern",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Pattern>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(StringKeywords {
                        max_length: __field0,
                        min_length: __field1,
                        pattern: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["maxLength", "minLength", "pattern"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "StringKeywords",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A regular expression according to ECMA-262.\n"]
    #[derive(Clone, Debug)]
    pub struct Pattern(pub(crate) ::std::string::String);
    impl ::std::convert::From<Pattern> for ::std::string::String {
        fn from(wrapped: Pattern) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Pattern {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Pattern {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Pattern(__serde::Deserialize::deserialize(__deserializer)?))
        }
    }
    #[doc = "Keywords applying to arrays.\n"]
    #[derive(Clone, Debug)]
    pub struct ArrayKeywords {
        #[doc = "The `prefixItems` keyword.\n\nSee [JSON Schema 10.3.1.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-prefixitems).\n"]
        pub prefix_items: ::std::option::Option<::std::vec::Vec<Schema>>,
        #[doc = "The `items` keyword.\n\nSee [JSON Schema 10.3.1.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-items).\n"]
        pub items: ::std::option::Option<::std::boxed::Box<Schema>>,
        #[doc = "The `contains` keyword.\n\nSee [JSON Schema 10.3.1.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-contains).\n"]
        pub contains: ::std::option::Option<::std::boxed::Box<Schema>>,
        #[doc = "The `maxItems` keyword.\n\nSee [JSON Schema Validation 6.4.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxitems).\n"]
        pub max_items: ::std::option::Option<usize>,
        #[doc = "The `minItems` keyword.\n\nSee [JSON Schema Validation 6.4.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minitems).\n"]
        pub min_items: ::std::option::Option<usize>,
        #[doc = "The `uniqueItems` keyword.\n\nSee [JSON Schema Validation 6.4.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-uniqueitems).\n"]
        pub unique_items: ::std::option::Option<bool>,
        #[doc = "The `maxContains` keyword.\n\nSee [JSON Schema Validation 6.4.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxcontains).\n"]
        pub max_contains: ::std::option::Option<usize>,
        #[doc = "The `minContains` keyword.\n\nSee [JSON Schema Validation 6.4.5](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxcontains).\n"]
        pub min_contains: ::std::option::Option<usize>,
        #[doc = "The `unevaluatedItems` keyword.\n\nSee [JSON Schema Core 11.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluateditems)\n"]
        pub unevaluated_items: ::std::option::Option<::std::boxed::Box<Schema>>,
    }
    impl ArrayKeywords {
        #[doc = "Creates a new [`ArrayKeywords`]."]
        pub fn new() -> Self {
            Self {
                prefix_items: ::std::default::Default::default(),
                items: ::std::default::Default::default(),
                contains: ::std::default::Default::default(),
                max_items: ::std::default::Default::default(),
                min_items: ::std::default::Default::default(),
                unique_items: ::std::default::Default::default(),
                max_contains: ::std::default::Default::default(),
                min_contains: ::std::default::Default::default(),
                unevaluated_items: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `prefix_items`."]
        pub fn set_prefix_items(
            &mut self,
            prefix_items: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> &mut Self {
            self.prefix_items = prefix_items;
            self
        }
        #[doc = "Sets the value of `prefix_items`."]
        pub fn with_prefix_items(
            mut self,
            prefix_items: ::std::option::Option<::std::vec::Vec<Schema>>,
        ) -> Self {
            self.prefix_items = prefix_items;
            self
        }
        #[doc = "Sets the value of `items`."]
        pub fn set_items(
            &mut self,
            items: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> &mut Self {
            self.items = items;
            self
        }
        #[doc = "Sets the value of `items`."]
        pub fn with_items(
            mut self,
            items: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> Self {
            self.items = items;
            self
        }
        #[doc = "Sets the value of `contains`."]
        pub fn set_contains(
            &mut self,
            contains: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> &mut Self {
            self.contains = contains;
            self
        }
        #[doc = "Sets the value of `contains`."]
        pub fn with_contains(
            mut self,
            contains: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> Self {
            self.contains = contains;
            self
        }
        #[doc = "Sets the value of `max_items`."]
        pub fn set_max_items(&mut self, max_items: ::std::option::Option<usize>) -> &mut Self {
            self.max_items = max_items;
            self
        }
        #[doc = "Sets the value of `max_items`."]
        pub fn with_max_items(mut self, max_items: ::std::option::Option<usize>) -> Self {
            self.max_items = max_items;
            self
        }
        #[doc = "Sets the value of `min_items`."]
        pub fn set_min_items(&mut self, min_items: ::std::option::Option<usize>) -> &mut Self {
            self.min_items = min_items;
            self
        }
        #[doc = "Sets the value of `min_items`."]
        pub fn with_min_items(mut self, min_items: ::std::option::Option<usize>) -> Self {
            self.min_items = min_items;
            self
        }
        #[doc = "Sets the value of `unique_items`."]
        pub fn set_unique_items(&mut self, unique_items: ::std::option::Option<bool>) -> &mut Self {
            self.unique_items = unique_items;
            self
        }
        #[doc = "Sets the value of `unique_items`."]
        pub fn with_unique_items(mut self, unique_items: ::std::option::Option<bool>) -> Self {
            self.unique_items = unique_items;
            self
        }
        #[doc = "Sets the value of `max_contains`."]
        pub fn set_max_contains(
            &mut self,
            max_contains: ::std::option::Option<usize>,
        ) -> &mut Self {
            self.max_contains = max_contains;
            self
        }
        #[doc = "Sets the value of `max_contains`."]
        pub fn with_max_contains(mut self, max_contains: ::std::option::Option<usize>) -> Self {
            self.max_contains = max_contains;
            self
        }
        #[doc = "Sets the value of `min_contains`."]
        pub fn set_min_contains(
            &mut self,
            min_contains: ::std::option::Option<usize>,
        ) -> &mut Self {
            self.min_contains = min_contains;
            self
        }
        #[doc = "Sets the value of `min_contains`."]
        pub fn with_min_contains(mut self, min_contains: ::std::option::Option<usize>) -> Self {
            self.min_contains = min_contains;
            self
        }
        #[doc = "Sets the value of `unevaluated_items`."]
        pub fn set_unevaluated_items(
            &mut self,
            unevaluated_items: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> &mut Self {
            self.unevaluated_items = unevaluated_items;
            self
        }
        #[doc = "Sets the value of `unevaluated_items`."]
        pub fn with_unevaluated_items(
            mut self,
            unevaluated_items: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> Self {
            self.unevaluated_items = unevaluated_items;
            self
        }
    }
    impl ::std::default::Default for ArrayKeywords {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ArrayKeywords {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ArrayKeywords", 9usize)?;
            __record.serialize_optional_field(
                "prefixItems",
                ::core::option::Option::as_ref(&self.prefix_items),
            )?;
            __record
                .serialize_optional_field("items", ::core::option::Option::as_ref(&self.items))?;
            __record.serialize_optional_field(
                "contains",
                ::core::option::Option::as_ref(&self.contains),
            )?;
            __record.serialize_optional_field(
                "maxItems",
                ::core::option::Option::as_ref(&self.max_items),
            )?;
            __record.serialize_optional_field(
                "minItems",
                ::core::option::Option::as_ref(&self.min_items),
            )?;
            __record.serialize_optional_field(
                "uniqueItems",
                ::core::option::Option::as_ref(&self.unique_items),
            )?;
            __record.serialize_optional_field(
                "maxContains",
                ::core::option::Option::as_ref(&self.max_contains),
            )?;
            __record.serialize_optional_field(
                "minContains",
                ::core::option::Option::as_ref(&self.min_contains),
            )?;
            __record.serialize_optional_field(
                "unevaluatedItems",
                ::core::option::Option::as_ref(&self.unevaluated_items),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ArrayKeywords {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "prefixItems",
                "items",
                "contains",
                "maxItems",
                "minItems",
                "uniqueItems",
                "maxContains",
                "minContains",
                "unevaluatedItems",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"prefixItems\", \"items\", \"contains\", \"maxItems\", \"minItems\", \"uniqueItems\", \"maxContains\", \"minContains\", \"unevaluatedItems\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
                __Identifier6,
                __Identifier7,
                __Identifier8,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        7u64 => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        8u64 => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "prefixItems" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "items" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "contains" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "maxItems" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "minItems" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "uniqueItems" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "maxContains" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "minContains" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "unevaluatedItems" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier8)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"prefixItems" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"items" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"contains" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"maxItems" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"minItems" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"uniqueItems" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"maxContains" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"minContains" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"unevaluatedItems" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier8)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ArrayKeywords;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ArrayKeywords")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(7usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(8usize, &"record with 9 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ArrayKeywords {
                        prefix_items: __field0,
                        items: __field1,
                        contains: __field2,
                        max_items: __field3,
                        min_items: __field4,
                        unique_items: __field5,
                        max_contains: __field6,
                        min_contains: __field7,
                        unevaluated_items: __field8,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "prefixItems",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "items",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "contains",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "maxItems",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "minItems",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "uniqueItems",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "maxContains",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "minContains",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "unevaluatedItems",
                                        ),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field7 = match __field7 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field8 = match __field8 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(ArrayKeywords {
                        prefix_items: __field0,
                        items: __field1,
                        contains: __field2,
                        max_items: __field3,
                        min_items: __field4,
                        unique_items: __field5,
                        max_contains: __field6,
                        min_contains: __field7,
                        unevaluated_items: __field8,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "prefixItems",
                "items",
                "contains",
                "maxItems",
                "minItems",
                "uniqueItems",
                "maxContains",
                "minContains",
                "unevaluatedItems",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ArrayKeywords",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Keywords applying to objects.\n"]
    #[derive(Clone, Debug)]
    pub struct ObjectKeywords {
        #[doc = "The `properties` keyword.\n\nSee [JSON Schema 10.3.2.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-properties).\n"]
        pub properties: ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
        #[doc = "The `patternProperties` keyword.\n\nSee [JSON Schema 10.3.2.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-patternproperties).\n"]
        pub pattern_properties:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
        #[doc = "The `additionalProperties` keyword.\n\nSee [JSON Schema 10.3.2.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-additionalproperties).\n"]
        pub additional_properties: ::std::option::Option<::std::boxed::Box<Schema>>,
        #[doc = "The `propertyNames` keyword.\n\nSee [JSON Schema 10.3.2.4](https://json-schema.org/draft/2020-12/json-schema-core.html#name-propertynames).\n"]
        pub property_names: ::std::option::Option<::std::boxed::Box<Schema>>,
        #[doc = "The `maxProperties` keyword.\n\nSee [JSON Schema Validation 6.5.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxproperties).\n"]
        pub max_properties: ::std::option::Option<usize>,
        #[doc = "The `minProperties` keyword.\n\nSee [JSON Schema Validation 6.5.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minproperties).\n"]
        pub min_properties: ::std::option::Option<usize>,
        #[doc = "The `required` keyword.\n\nSee [JSON Schema Validation 6.5.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-required).\n"]
        pub required: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "The `dependentRequired` keyword.\n\nSee [JSON Schema Validation 6.5.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-dependentrequired).\n"]
        pub dependent_required: ::std::option::Option<
            indexmap::IndexMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
        >,
        #[doc = "The `unevaluatedProperties` keyword.\n\nSee [JSON Schema Core 11.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluatedproperties)\n"]
        pub unevaluated_properties: ::std::option::Option<::std::boxed::Box<Schema>>,
    }
    impl ObjectKeywords {
        #[doc = "Creates a new [`ObjectKeywords`]."]
        pub fn new() -> Self {
            Self {
                properties: ::std::default::Default::default(),
                pattern_properties: ::std::default::Default::default(),
                additional_properties: ::std::default::Default::default(),
                property_names: ::std::default::Default::default(),
                max_properties: ::std::default::Default::default(),
                min_properties: ::std::default::Default::default(),
                required: ::std::default::Default::default(),
                dependent_required: ::std::default::Default::default(),
                unevaluated_properties: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `properties`."]
        pub fn set_properties(
            &mut self,
            properties: ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
        ) -> &mut Self {
            self.properties = properties;
            self
        }
        #[doc = "Sets the value of `properties`."]
        pub fn with_properties(
            mut self,
            properties: ::std::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
        ) -> Self {
            self.properties = properties;
            self
        }
        #[doc = "Sets the value of `pattern_properties`."]
        pub fn set_pattern_properties(
            &mut self,
            pattern_properties: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, Schema>,
            >,
        ) -> &mut Self {
            self.pattern_properties = pattern_properties;
            self
        }
        #[doc = "Sets the value of `pattern_properties`."]
        pub fn with_pattern_properties(
            mut self,
            pattern_properties: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, Schema>,
            >,
        ) -> Self {
            self.pattern_properties = pattern_properties;
            self
        }
        #[doc = "Sets the value of `additional_properties`."]
        pub fn set_additional_properties(
            &mut self,
            additional_properties: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> &mut Self {
            self.additional_properties = additional_properties;
            self
        }
        #[doc = "Sets the value of `additional_properties`."]
        pub fn with_additional_properties(
            mut self,
            additional_properties: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> Self {
            self.additional_properties = additional_properties;
            self
        }
        #[doc = "Sets the value of `property_names`."]
        pub fn set_property_names(
            &mut self,
            property_names: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> &mut Self {
            self.property_names = property_names;
            self
        }
        #[doc = "Sets the value of `property_names`."]
        pub fn with_property_names(
            mut self,
            property_names: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> Self {
            self.property_names = property_names;
            self
        }
        #[doc = "Sets the value of `max_properties`."]
        pub fn set_max_properties(
            &mut self,
            max_properties: ::std::option::Option<usize>,
        ) -> &mut Self {
            self.max_properties = max_properties;
            self
        }
        #[doc = "Sets the value of `max_properties`."]
        pub fn with_max_properties(mut self, max_properties: ::std::option::Option<usize>) -> Self {
            self.max_properties = max_properties;
            self
        }
        #[doc = "Sets the value of `min_properties`."]
        pub fn set_min_properties(
            &mut self,
            min_properties: ::std::option::Option<usize>,
        ) -> &mut Self {
            self.min_properties = min_properties;
            self
        }
        #[doc = "Sets the value of `min_properties`."]
        pub fn with_min_properties(mut self, min_properties: ::std::option::Option<usize>) -> Self {
            self.min_properties = min_properties;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn set_required(
            &mut self,
            required: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.required = required;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn with_required(
            mut self,
            required: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.required = required;
            self
        }
        #[doc = "Sets the value of `dependent_required`."]
        pub fn set_dependent_required(
            &mut self,
            dependent_required: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
            >,
        ) -> &mut Self {
            self.dependent_required = dependent_required;
            self
        }
        #[doc = "Sets the value of `dependent_required`."]
        pub fn with_dependent_required(
            mut self,
            dependent_required: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.dependent_required = dependent_required;
            self
        }
        #[doc = "Sets the value of `unevaluated_properties`."]
        pub fn set_unevaluated_properties(
            &mut self,
            unevaluated_properties: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> &mut Self {
            self.unevaluated_properties = unevaluated_properties;
            self
        }
        #[doc = "Sets the value of `unevaluated_properties`."]
        pub fn with_unevaluated_properties(
            mut self,
            unevaluated_properties: ::std::option::Option<::std::boxed::Box<Schema>>,
        ) -> Self {
            self.unevaluated_properties = unevaluated_properties;
            self
        }
    }
    impl ::std::default::Default for ObjectKeywords {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ObjectKeywords {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ObjectKeywords", 9usize)?;
            __record.serialize_optional_field(
                "properties",
                ::core::option::Option::as_ref(&self.properties),
            )?;
            __record.serialize_optional_field(
                "patternProperties",
                ::core::option::Option::as_ref(&self.pattern_properties),
            )?;
            __record.serialize_optional_field(
                "additionalProperties",
                ::core::option::Option::as_ref(&self.additional_properties),
            )?;
            __record.serialize_optional_field(
                "propertyNames",
                ::core::option::Option::as_ref(&self.property_names),
            )?;
            __record.serialize_optional_field(
                "maxProperties",
                ::core::option::Option::as_ref(&self.max_properties),
            )?;
            __record.serialize_optional_field(
                "minProperties",
                ::core::option::Option::as_ref(&self.min_properties),
            )?;
            __record.serialize_optional_field(
                "required",
                ::core::option::Option::as_ref(&self.required),
            )?;
            __record.serialize_optional_field(
                "dependentRequired",
                ::core::option::Option::as_ref(&self.dependent_required),
            )?;
            __record.serialize_optional_field(
                "unevaluatedProperties",
                ::core::option::Option::as_ref(&self.unevaluated_properties),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ObjectKeywords {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "properties",
                "patternProperties",
                "additionalProperties",
                "propertyNames",
                "maxProperties",
                "minProperties",
                "required",
                "dependentRequired",
                "unevaluatedProperties",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"properties\", \"patternProperties\", \"additionalProperties\", \"propertyNames\", \"maxProperties\", \"minProperties\", \"required\", \"dependentRequired\", \"unevaluatedProperties\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
                __Identifier6,
                __Identifier7,
                __Identifier8,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        7u64 => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        8u64 => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "properties" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "patternProperties" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier1)
                        }
                        "additionalProperties" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier2)
                        }
                        "propertyNames" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "maxProperties" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "minProperties" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "required" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "dependentRequired" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier7)
                        }
                        "unevaluatedProperties" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier8)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"properties" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"patternProperties" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier1)
                        }
                        b"additionalProperties" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier2)
                        }
                        b"propertyNames" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"maxProperties" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"minProperties" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"required" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"dependentRequired" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier7)
                        }
                        b"unevaluatedProperties" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier8)
                        }
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ObjectKeywords;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ObjectKeywords")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<usize>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<
                                ::std::string::String,
                                ::std::vec::Vec<::std::string::String>,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(7usize, &"record with 9 fields"),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(8usize, &"record with 9 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ObjectKeywords {
                        properties: __field0,
                        pattern_properties: __field1,
                        additional_properties: __field2,
                        property_names: __field3,
                        max_properties: __field4,
                        min_properties: __field5,
                        required: __field6,
                        dependent_required: __field7,
                        unevaluated_properties: __field8,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<usize>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                    > = ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<
                                ::std::string::String,
                                ::std::vec::Vec<::std::string::String>,
                            >,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<
                        ::core::option::Option<::std::boxed::Box<Schema>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "properties",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, Schema>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "patternProperties",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, Schema>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "additionalProperties",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "propertyNames",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "maxProperties",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "minProperties",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<usize>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "required",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "dependentRequired",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                ::std::vec::Vec<::std::string::String>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "unevaluatedProperties",
                                        ),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::boxed::Box<Schema>>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field7 = match __field7 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field8 = match __field8 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(ObjectKeywords {
                        properties: __field0,
                        pattern_properties: __field1,
                        additional_properties: __field2,
                        property_names: __field3,
                        max_properties: __field4,
                        min_properties: __field5,
                        required: __field6,
                        dependent_required: __field7,
                        unevaluated_properties: __field8,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "properties",
                "patternProperties",
                "additionalProperties",
                "propertyNames",
                "maxProperties",
                "minProperties",
                "required",
                "dependentRequired",
                "unevaluatedProperties",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ObjectKeywords",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct IdlVariantsExt(pub(crate) ::std::vec::Vec<IdlVariant>);
    impl ::std::convert::From<IdlVariantsExt> for ::std::vec::Vec<IdlVariant> {
        fn from(wrapped: IdlVariantsExt) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for IdlVariantsExt {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for IdlVariantsExt {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(IdlVariantsExt(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct IdlVariant {
        #[doc = ""]
        pub name: ::std::string::String,
        #[doc = ""]
        pub type_ref: ::std::option::Option<::std::string::String>,
    }
    impl IdlVariant {
        #[doc = "Creates a new [`IdlVariant`]."]
        pub fn new(name: ::std::string::String) -> Self {
            Self {
                name,
                type_ref: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: ::std::string::String) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::string::String) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `type_ref`."]
        pub fn set_type_ref(
            &mut self,
            type_ref: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.type_ref = type_ref;
            self
        }
        #[doc = "Sets the value of `type_ref`."]
        pub fn with_type_ref(
            mut self,
            type_ref: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.type_ref = type_ref;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for IdlVariant {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "IdlVariant", 2usize)?;
            __record.serialize_field("name", &self.name)?;
            __record.serialize_optional_field(
                "typeRef",
                ::core::option::Option::as_ref(&self.type_ref),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for IdlVariant {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["name", "typeRef"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"typeRef\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "typeRef" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"typeRef" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = IdlVariant;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record IdlVariant")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::std::string::String,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(IdlVariant {
                        name: __field0,
                        type_ref: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "typeRef",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(IdlVariant {
                        name: __field0,
                        type_ref: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "typeRef"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "IdlVariant",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct IdlFieldsExt(pub(crate) ::std::vec::Vec<IdlField>);
    impl ::std::convert::From<IdlFieldsExt> for ::std::vec::Vec<IdlField> {
        fn from(wrapped: IdlFieldsExt) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for IdlFieldsExt {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for IdlFieldsExt {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(IdlFieldsExt(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct IdlField {
        #[doc = ""]
        pub name: ::std::string::String,
        #[doc = ""]
        pub type_ref: ::std::string::String,
        #[doc = ""]
        pub optional: ::std::option::Option<bool>,
        #[doc = ""]
        pub inlined: ::std::option::Option<bool>,
    }
    impl IdlField {
        #[doc = "Creates a new [`IdlField`]."]
        pub fn new(name: ::std::string::String, type_ref: ::std::string::String) -> Self {
            Self {
                name,
                type_ref,
                optional: ::std::default::Default::default(),
                inlined: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: ::std::string::String) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::string::String) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `type_ref`."]
        pub fn set_type_ref(&mut self, type_ref: ::std::string::String) -> &mut Self {
            self.type_ref = type_ref;
            self
        }
        #[doc = "Sets the value of `type_ref`."]
        pub fn with_type_ref(mut self, type_ref: ::std::string::String) -> Self {
            self.type_ref = type_ref;
            self
        }
        #[doc = "Sets the value of `optional`."]
        pub fn set_optional(&mut self, optional: ::std::option::Option<bool>) -> &mut Self {
            self.optional = optional;
            self
        }
        #[doc = "Sets the value of `optional`."]
        pub fn with_optional(mut self, optional: ::std::option::Option<bool>) -> Self {
            self.optional = optional;
            self
        }
        #[doc = "Sets the value of `inlined`."]
        pub fn set_inlined(&mut self, inlined: ::std::option::Option<bool>) -> &mut Self {
            self.inlined = inlined;
            self
        }
        #[doc = "Sets the value of `inlined`."]
        pub fn with_inlined(mut self, inlined: ::std::option::Option<bool>) -> Self {
            self.inlined = inlined;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for IdlField {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "IdlField", 4usize)?;
            __record.serialize_field("name", &self.name)?;
            __record.serialize_field("typeRef", &self.type_ref)?;
            __record.serialize_optional_field(
                "optional",
                ::core::option::Option::as_ref(&self.optional),
            )?;
            __record.serialize_optional_field(
                "inlined",
                ::core::option::Option::as_ref(&self.inlined),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for IdlField {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["name", "typeRef", "optional", "inlined"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"typeRef\", \"optional\", \"inlined\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Unknown,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "typeRef" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "optional" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "inlined" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"typeRef" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"optional" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"inlined" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = IdlField;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record IdlField")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::std::string::String,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::string::String,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(IdlField {
                        name: __field0,
                        type_ref: __field1,
                        optional: __field2,
                        inlined: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "typeRef",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "optional",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "inlined",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("typeRef"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(IdlField {
                        name: __field0,
                        type_ref: __field1,
                        optional: __field2,
                        inlined: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "typeRef", "optional", "inlined"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "IdlField",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
