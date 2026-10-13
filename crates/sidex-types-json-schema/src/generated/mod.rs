/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod schema {
    #![doc = "Types of [JSON Schema](https://json-schema.org) version `2020-12`.\n\nThe types have been inspired by [Schemars](https://crates.io/crates/schemars).\n\n**References**:\n\n- [JSON Schema Core](https://json-schema.org/draft/2020-12/json-schema-core.html)\n- [JSON Schema Validation](https://json-schema.org/draft/2020-12/json-schema-validation.html)\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "A JSON schema.\n\nSee [JSON Schema 4.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n"]
    #[derive(Clone, Debug)]
    pub enum Schema {
        #[doc = "A boolean JSON schema.\n\nSee [JSON Schema 4.3.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-boolean-json-schemas).\n"]
        Bool(bool),
        #[doc = "A schema object.\n\nSee [JSON Schema 4.3.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n"]
        Object(SchemaObject),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Schema {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::Schema" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Bool" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < bool >) ,) . with_docs ("A boolean JSON schema.\n\nSee [JSON Schema 4.3.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-boolean-json-schemas).\n") , :: sidex :: shape :: CaseShape :: new ("Object" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < SchemaObject >) ,) . with_docs ("A schema object.\n\nSee [JSON Schema 4.3.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: Implicit))) . with_docs ("A JSON schema.\n\nSee [JSON Schema 4.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("implicitly") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Schema {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Schema {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Bool(_) => 0usize,
                Self::Object(_) => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Bool(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Object(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Schema {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Bool(<bool as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Object(<SchemaObject as ::sidex::Decode>::decode(decoder)?),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Schema {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Schema {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
        pub defs: ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RootSchema {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::json_schema::schema::RootSchema",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "meta_schema",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_attrs(&[::sidex::shape::Attr::List {
                    path: "json",
                    args: &[::sidex::shape::Attr::Assign {
                        path: "name",
                        value: ::sidex::shape::AttrValue::String("$schema"),
                    }],
                }])
                .optional()
                .json_name("$schema"),
                ::sidex::shape::FieldShape::new("schema", ::sidex::shape::shape_of::<SchemaObject>)
                    .with_attrs(&[::sidex::shape::Attr::List {
                        path: "json",
                        args: &[::sidex::shape::Attr::Path("inline")],
                    }])
                    .json_inline(),
                ::sidex::shape::FieldShape::new(
                    "defs",
                    ::sidex::shape::shape_of::<::sidex::types::Map<::std::string::String, Schema>>,
                )
                .with_attrs(&[::sidex::shape::Attr::List {
                    path: "json",
                    args: &[::sidex::shape::Attr::Assign {
                        path: "name",
                        value: ::sidex::shape::AttrValue::String("$defs"),
                    }],
                }])
                .optional()
                .json_name("$defs"),
            ])),
        )
        .with_docs("A root schema.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for RootSchema {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RootSchema {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.meta_schema
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.schema)),
                2usize => {
                    self.defs
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RootSchema {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<SchemaObject>::None;
            let mut __field_2 =
                ::std::option::Option::<::sidex::types::Map<::std::string::String, Schema>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <SchemaObject as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::std::string::String,
                            Schema,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                meta_schema: __field_0,
                schema: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `schema`"))?,
                defs: __field_2,
            })
        }
    }
    impl RootSchema {
        #[doc = "Creates a new [`RootSchema`]."]
        pub fn new(schema: SchemaObject) -> Self {
            Self {
                schema: schema,
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
            defs: ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
        ) -> &mut Self {
            self.defs = defs;
            self
        }
        #[doc = "Sets the value of `defs`."]
        pub fn with_defs(
            mut self,
            defs: ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
        ) -> Self {
            self.defs = defs;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RootSchema {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RootSchema {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
        #[doc = "Encoding of string contents.\n\nSee JSON Schema Validation 8.3.\n"]
        pub content_encoding: ::std::option::Option<::std::string::String>,
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
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SchemaObject {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::SchemaObject" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("id" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The `$id` keyword.\n\nSee [JSON Schema 8.2.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-id-keyword).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("$id") }] }]) . optional () . json_name ("$id") , :: sidex :: shape :: FieldShape :: new ("type" , :: sidex :: shape :: shape_of :: < MaybeArray < Type , > > ,) . with_docs ("The `type` keyword.\n\nSee [JSON Schema Validation 6.1.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-type).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("allowed_types") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("enum" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Any , > > ,) . with_docs ("The `enum` keyword.\n\nSee [JSON Schema Validation 6.1.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-enum).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("allowed_values") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("const" , :: sidex :: shape :: shape_of :: < Any > ,) . with_docs ("The `const` keyword.\n\nSee [JSON Schema Validation 6.1.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-const).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("allowed_value") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("format" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The `format` keyword.\n\nSee [JSON Schema Validation 7](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-vocabularies-for-semantic-c).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("content_encoding" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Encoding of string contents.\n\nSee JSON Schema Validation 8.3.\n") . optional () . json_name ("contentEncoding") , :: sidex :: shape :: FieldShape :: new ("ref" , :: sidex :: shape :: shape_of :: < SchemaRef > ,) . with_docs ("The `$ref` keyword.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("$ref") }] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("reference") }] }]) . optional () . json_name ("$ref") , :: sidex :: shape :: FieldShape :: new ("comment" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The `$comment` keyword.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("$comment") }] }]) . optional () . json_name ("$comment") , :: sidex :: shape :: FieldShape :: new ("metadata" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < Metadata > > ,) . with_docs ("Basic metadata annotations.\n\nSee [JSON Schema Validation 9](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-a-vocabulary-for-basic-meta).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Path ("inline")] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_inline () , :: sidex :: shape :: FieldShape :: new ("subschema_keywords" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < SubschemaKeywords > > ,) . with_docs ("Keywords for combining and conditionally applying subschemas.\n\nSee [JSON Schema 10.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keywords-for-applying-subsc).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Path ("inline")] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("subschemaKeywords") . json_inline () , :: sidex :: shape :: FieldShape :: new ("number_keywords" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < NumberKeywords > > ,) . with_docs ("Keywords applying to numbers.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Path ("inline")] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("numberKeywords") . json_inline () , :: sidex :: shape :: FieldShape :: new ("string_keywords" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < StringKeywords > > ,) . with_docs ("Keywords applying to strings.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Path ("inline")] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("stringKeywords") . json_inline () , :: sidex :: shape :: FieldShape :: new ("array_keywords" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < ArrayKeywords > > ,) . with_docs ("Keywords applying to arrays.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Path ("inline")] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("arrayKeywords") . json_inline () , :: sidex :: shape :: FieldShape :: new ("object_keywords" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < ObjectKeywords > > ,) . with_docs ("Keywords applying to objects.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Path ("inline")] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("objectKeywords") . json_inline ()]))) . with_docs ("A schema object.\n\nSee [JSON Schema 4.3.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-json-schema-documents).\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for SchemaObject {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SchemaObject {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self.id.as_ref().map(|value| ::sidex::Encode::encode(value)),
                1usize => {
                    self.allowed_types
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.allowed_values
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.allowed_value
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                4usize => {
                    self.format
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                5usize => {
                    self.content_encoding
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                6usize => {
                    self.reference
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                7usize => {
                    self.comment
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                8usize => {
                    self.metadata
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                9usize => {
                    self.subschema_keywords
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                10usize => {
                    self.number_keywords
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                11usize => {
                    self.string_keywords
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                12usize => {
                    self.array_keywords
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                13usize => {
                    self.object_keywords
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SchemaObject {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<MaybeArray<Type>>::None;
            let mut __field_2 = ::std::option::Option::<::std::vec::Vec<Any>>::None;
            let mut __field_3 = ::std::option::Option::<Any>::None;
            let mut __field_4 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_5 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_6 = ::std::option::Option::<SchemaRef>::None;
            let mut __field_7 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_8 = ::std::option::Option::<::std::boxed::Box<Metadata>>::None;
            let mut __field_9 = ::std::option::Option::<::std::boxed::Box<SubschemaKeywords>>::None;
            let mut __field_10 = ::std::option::Option::<::std::boxed::Box<NumberKeywords>>::None;
            let mut __field_11 = ::std::option::Option::<::std::boxed::Box<StringKeywords>>::None;
            let mut __field_12 = ::std::option::Option::<::std::boxed::Box<ArrayKeywords>>::None;
            let mut __field_13 = ::std::option::Option::<::std::boxed::Box<ObjectKeywords>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <MaybeArray<Type> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::vec::Vec<Any> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 =
                            ::std::option::Option::Some(<Any as ::sidex::Decode>::decode(decoder)?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <SchemaRef as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(
                            <::std::boxed::Box<Metadata> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    9usize => {
                        __field_9 = ::std::option::Option::Some(<::std::boxed::Box<
                            SubschemaKeywords,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    10usize => {
                        __field_10 = ::std::option::Option::Some(
                            <::std::boxed::Box<NumberKeywords> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    11usize => {
                        __field_11 = ::std::option::Option::Some(
                            <::std::boxed::Box<StringKeywords> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    12usize => {
                        __field_12 = ::std::option::Option::Some(
                            <::std::boxed::Box<ArrayKeywords> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    13usize => {
                        __field_13 = ::std::option::Option::Some(
                            <::std::boxed::Box<ObjectKeywords> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0,
                allowed_types: __field_1,
                allowed_values: __field_2,
                allowed_value: __field_3,
                format: __field_4,
                content_encoding: __field_5,
                reference: __field_6,
                comment: __field_7,
                metadata: __field_8,
                subschema_keywords: __field_9,
                number_keywords: __field_10,
                string_keywords: __field_11,
                array_keywords: __field_12,
                object_keywords: __field_13,
            })
        }
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
                content_encoding: ::std::default::Default::default(),
                reference: ::std::default::Default::default(),
                comment: ::std::default::Default::default(),
                metadata: ::std::default::Default::default(),
                subschema_keywords: ::std::default::Default::default(),
                number_keywords: ::std::default::Default::default(),
                string_keywords: ::std::default::Default::default(),
                array_keywords: ::std::default::Default::default(),
                object_keywords: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `content_encoding`."]
        pub fn set_content_encoding(
            &mut self,
            content_encoding: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.content_encoding = content_encoding;
            self
        }
        #[doc = "Sets the value of `content_encoding`."]
        pub fn with_content_encoding(
            mut self,
            content_encoding: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.content_encoding = content_encoding;
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
    }
    impl ::std::default::Default for SchemaObject {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SchemaObject {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SchemaObject {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
    impl ::sidex::Shaped for Type {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::Type" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Null" , :: std :: option :: Option :: None) . json_name ("null") , :: sidex :: shape :: CaseShape :: new ("Boolean" , :: std :: option :: Option :: None) . json_name ("boolean") , :: sidex :: shape :: CaseShape :: new ("Object" , :: std :: option :: Option :: None) . json_name ("object") , :: sidex :: shape :: CaseShape :: new ("Array" , :: std :: option :: Option :: None) . json_name ("array") , :: sidex :: shape :: CaseShape :: new ("Number" , :: std :: option :: Option :: None) . json_name ("number") , :: sidex :: shape :: CaseShape :: new ("String" , :: std :: option :: Option :: None) . json_name ("string") , :: sidex :: shape :: CaseShape :: new ("Integer" , :: std :: option :: Option :: None) . json_name ("integer")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("Types for the `type` keyword.\n\nSee [JSON Schema Validation 6.1.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-type).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("lowercase") } , :: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") }] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "derive" , args : & [:: sidex :: shape :: Attr :: Path ("PartialEq") , :: sidex :: shape :: Attr :: Path ("Eq")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Type {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Type {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Null => 0usize,
                Self::Boolean => 1usize,
                Self::Object => 2usize,
                Self::Array => 3usize,
                Self::Number => 4usize,
                Self::String => 5usize,
                Self::Integer => 6usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Null => ::std::option::Option::None,
                Self::Boolean => ::std::option::Option::None,
                Self::Object => ::std::option::Option::None,
                Self::Array => ::std::option::Option::None,
                Self::Number => ::std::option::Option::None,
                Self::String => ::std::option::Option::None,
                Self::Integer => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Type {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Null,
                    1usize => Self::Boolean,
                    2usize => Self::Object,
                    3usize => Self::Array,
                    4usize => Self::Number,
                    5usize => Self::String,
                    6usize => Self::Integer,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Type {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Type {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
    impl<T: ::sidex::Shaped> ::sidex::Shaped for MaybeArray<T> {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::json_schema::schema::MaybeArray",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Single",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<T>),
                    ),
                    ::sidex::shape::CaseShape::new(
                        "Array",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<::std::vec::Vec<T>>),
                    ),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Implicit),
            ),
        )
        .with_docs("Either a single value of type `T` or an array of type `T`.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "tagged",
                    value: ::sidex::shape::AttrValue::Path("implicitly"),
                }],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                    ],
                }],
            },
        ])
        .with_args(&[::sidex::shape::shape_of::<T>]);
    }
    #[automatically_derived]
    impl<T: ::sidex::Shaped + ::sidex::Encode> ::sidex::Encode for MaybeArray<T> {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl<T: ::sidex::Shaped + ::sidex::Encode> ::sidex::encode::VariantView for MaybeArray<T> {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Single(_) => 0usize,
                Self::Array(_) => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Single(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Array(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl<T: ::sidex::Shaped + ::sidex::Decode> ::sidex::Decode for MaybeArray<T> {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Single(<T as ::sidex::Decode>::decode(decoder)?),
                    1usize => {
                        Self::Array(<::std::vec::Vec<T> as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl<T: ::sidex::Shaped + ::sidex::Encode> ::serde::Serialize for MaybeArray<T> {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de, T: ::sidex::Shaped + ::sidex::Decode> ::serde::Deserialize<'de> for MaybeArray<T> {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Any JSON value.\n"]
    pub type Any = ::sidex::JsonValue;
    #[doc = "A reference to a JSON schema.\n"]
    #[derive(Clone, Debug)]
    pub struct SchemaRef(pub(crate) ::std::string::String);
    impl ::std::convert::From<SchemaRef> for ::std::string::String {
        fn from(wrapped: SchemaRef) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SchemaRef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::json_schema::schema::SchemaRef",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<::std::string::String>),
        )
        .with_docs("A reference to a JSON schema.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for SchemaRef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for SchemaRef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<::std::string::String as ::sidex::Decode>::decode(
                decoder,
            )?))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SchemaRef {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SchemaRef {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
    #[automatically_derived]
    impl ::sidex::Shaped for Metadata {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::Metadata" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("title" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The `title` keyword.\n\nSee [JSON Schema Validation 9.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-title-and-description).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("description" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The `description` keyword.\n\nSee [JSON Schema Validation 9.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-title-and-description).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("default" , :: sidex :: shape :: shape_of :: < Any > ,) . with_docs ("The `default` keyword.\n\nSee [JSON Schema Validation 9.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-default).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("deprecated" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("The `deprecated` keyword.\n\nSee [JSON Schema Validation 9.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-deprecated).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("read_only" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("The `readOnly` keyword.\n\nSee [JSON Schema Validation 9.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-readonly-and-writeonly).\n") . optional () . json_name ("readOnly") , :: sidex :: shape :: FieldShape :: new ("write_only" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("The `readWrite` keyword.\n\nSee [JSON Schema Validation 9.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-readonly-and-writeonly).\n") . optional () . json_name ("writeOnly") , :: sidex :: shape :: FieldShape :: new ("examples" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Any , > > ,) . with_docs ("The `examples` keyword.\n\nSee [JSON Schema Validation 9.5](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-examples).\n") . optional ()]))) . with_docs ("Basic metadata annotations.\n\nSee [JSON Schema Validation 9](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-a-vocabulary-for-basic-meta).\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Metadata {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Metadata {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.title
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.description
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.default
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.deprecated
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                4usize => {
                    self.read_only
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                5usize => {
                    self.write_only
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                6usize => {
                    self.examples
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Metadata {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<Any>::None;
            let mut __field_3 = ::std::option::Option::<bool>::None;
            let mut __field_4 = ::std::option::Option::<bool>::None;
            let mut __field_5 = ::std::option::Option::<bool>::None;
            let mut __field_6 = ::std::option::Option::<::std::vec::Vec<Any>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<Any as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <::std::vec::Vec<Any> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                title: __field_0,
                description: __field_1,
                default: __field_2,
                deprecated: __field_3,
                read_only: __field_4,
                write_only: __field_5,
                examples: __field_6,
            })
        }
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
    impl ::serde::Serialize for Metadata {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Metadata {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
            ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SubschemaKeywords {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::SubschemaKeywords" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("all_of" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Schema , > > ,) . with_docs ("The `allOf` keyword.\n\nSee [JSON Schema 10.2.1.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-allof).\n") . optional () . json_name ("allOf") , :: sidex :: shape :: FieldShape :: new ("any_of" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Schema , > > ,) . with_docs ("The `andOf` keyword.\n\nSee [JSON Schema 10.2.1.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-anyof).\n") . optional () . json_name ("anyOf") , :: sidex :: shape :: FieldShape :: new ("one_of" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Schema , > > ,) . with_docs ("The `oneOf` keyword.\n\nSee [JSON Schema 10.2.1.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-oneof).\n") . optional () . json_name ("oneOf") , :: sidex :: shape :: FieldShape :: new ("not" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Schema , > > ,) . with_docs ("The `not` keyword.\n\nSee [JSON Schema 10.2.1.4](https://json-schema.org/draft/2020-12/json-schema-core.html#name-not).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("if" , :: sidex :: shape :: shape_of :: < Schema > ,) . with_docs ("The `if` keyword.\n\nSee [JSON Schema 10.2.2.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-if).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("condition") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("then" , :: sidex :: shape :: shape_of :: < Schema > ,) . with_docs ("The `then` keyword.\n\nSee [JSON Schema 10.2.2.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-then).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("consequence") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("else" , :: sidex :: shape :: shape_of :: < Schema > ,) . with_docs ("The `else` keyword.\n\nSee [JSON Schema 10.2.2.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-else).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("alternative") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("dependent_schemas" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: std :: string :: String , Schema , > > ,) . with_docs ("The `dependentSchemas` keyword.\n\nSee [JSON Schema 10.2.2.4](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dependentschemas)\n") . optional () . json_name ("dependentSchemas")]))) . with_docs ("Keywords for combining and conditionally applying subschemas.\n\nSee [JSON Schema 10.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keywords-for-applying-subsc).\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for SubschemaKeywords {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SubschemaKeywords {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.all_of
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.any_of
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.one_of
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.not
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                4usize => {
                    self.condition
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                5usize => {
                    self.consequence
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                6usize => {
                    self.alternative
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                7usize => {
                    self.dependent_schemas
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SubschemaKeywords {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::vec::Vec<Schema>>::None;
            let mut __field_1 = ::std::option::Option::<::std::vec::Vec<Schema>>::None;
            let mut __field_2 = ::std::option::Option::<::std::vec::Vec<Schema>>::None;
            let mut __field_3 = ::std::option::Option::<::std::vec::Vec<Schema>>::None;
            let mut __field_4 = ::std::option::Option::<Schema>::None;
            let mut __field_5 = ::std::option::Option::<Schema>::None;
            let mut __field_6 = ::std::option::Option::<Schema>::None;
            let mut __field_7 =
                ::std::option::Option::<::sidex::types::Map<::std::string::String, Schema>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::vec::Vec<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::vec::Vec<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::vec::Vec<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <Schema as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <Schema as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <Schema as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::std::string::String,
                            Schema,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                all_of: __field_0,
                any_of: __field_1,
                one_of: __field_2,
                not: __field_3,
                condition: __field_4,
                consequence: __field_5,
                alternative: __field_6,
                dependent_schemas: __field_7,
            })
        }
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
                ::sidex::types::Map<::std::string::String, Schema>,
            >,
        ) -> &mut Self {
            self.dependent_schemas = dependent_schemas;
            self
        }
        #[doc = "Sets the value of `dependent_schemas`."]
        pub fn with_dependent_schemas(
            mut self,
            dependent_schemas: ::std::option::Option<
                ::sidex::types::Map<::std::string::String, Schema>,
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
    impl ::serde::Serialize for SubschemaKeywords {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SubschemaKeywords {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
    #[automatically_derived]
    impl ::sidex::Shaped for NumberKeywords {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::NumberKeywords" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("multiple_of" , :: sidex :: shape :: shape_of :: < Number > ,) . with_docs ("The `multipleOf` keyword.\n\nSee [JSON Schema Validation 6.2.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-multipleof).\n") . optional () . json_name ("multipleOf") , :: sidex :: shape :: FieldShape :: new ("maximum" , :: sidex :: shape :: shape_of :: < Number > ,) . with_docs ("The `maximum` keyword.\n\nSee [JSON Schema Validation 6.2.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maximum).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("exclusive_maximum" , :: sidex :: shape :: shape_of :: < Number > ,) . with_docs ("The `exclusiveMaximum` keyword.\n\nSee [JSON Schema Validation 6.2.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-exclusivemaximum).\n") . optional () . json_name ("exclusiveMaximum") , :: sidex :: shape :: FieldShape :: new ("minimum" , :: sidex :: shape :: shape_of :: < Number > ,) . with_docs ("The `minimum` keyword.\n\nSee [JSON Schema Validation 6.2.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minimum).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("exclusive_minimum" , :: sidex :: shape :: shape_of :: < Number > ,) . with_docs ("The `exclusiveMaximum` keyword.\n\nSee [JSON Schema Validation 6.2.5](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-exclusiveminimum).\n") . optional () . json_name ("exclusiveMinimum")]))) . with_docs ("Keywords applying to numbers.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for NumberKeywords {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for NumberKeywords {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.multiple_of
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.maximum
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.exclusive_maximum
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.minimum
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                4usize => {
                    self.exclusive_minimum
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for NumberKeywords {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Number>::None;
            let mut __field_1 = ::std::option::Option::<Number>::None;
            let mut __field_2 = ::std::option::Option::<Number>::None;
            let mut __field_3 = ::std::option::Option::<Number>::None;
            let mut __field_4 = ::std::option::Option::<Number>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <Number as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <Number as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <Number as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <Number as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <Number as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                multiple_of: __field_0,
                maximum: __field_1,
                exclusive_maximum: __field_2,
                minimum: __field_3,
                exclusive_minimum: __field_4,
            })
        }
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
    impl ::serde::Serialize for NumberKeywords {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for NumberKeywords {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A JSON number.\n"]
    pub type Number = ::sidex::JsonValue;
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
    #[automatically_derived]
    impl ::sidex::Shaped for StringKeywords {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::StringKeywords" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("max_length" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `maxLength` keyword.\n\nSee [JSON Schema Validation 6.3.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxlength).\n") . optional () . json_name ("maxLength") , :: sidex :: shape :: FieldShape :: new ("min_length" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `minLength` keyword.\n\nSee [JSON Schema Validation 6.3.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minlength).\n") . optional () . json_name ("minLength") , :: sidex :: shape :: FieldShape :: new ("pattern" , :: sidex :: shape :: shape_of :: < Pattern > ,) . with_docs ("The `pattern` keyword.\n\nSee [JSON Schema Validation 6.3.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-pattern).\n") . optional ()]))) . with_docs ("Keywords applying to strings.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for StringKeywords {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for StringKeywords {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.max_length
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.min_length
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.pattern
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for StringKeywords {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<usize>::None;
            let mut __field_1 = ::std::option::Option::<usize>::None;
            let mut __field_2 = ::std::option::Option::<Pattern>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <Pattern as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                max_length: __field_0,
                min_length: __field_1,
                pattern: __field_2,
            })
        }
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
    impl ::serde::Serialize for StringKeywords {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for StringKeywords {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
    impl ::sidex::Shaped for Pattern {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::json_schema::schema::Pattern",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<::std::string::String>),
        )
        .with_docs("A regular expression according to ECMA-262.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for Pattern {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for Pattern {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<::std::string::String as ::sidex::Decode>::decode(
                decoder,
            )?))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Pattern {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Pattern {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
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
    #[automatically_derived]
    impl ::sidex::Shaped for ArrayKeywords {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::ArrayKeywords" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("prefix_items" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Schema , > > ,) . with_docs ("The `prefixItems` keyword.\n\nSee [JSON Schema 10.3.1.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-prefixitems).\n") . optional () . json_name ("prefixItems") , :: sidex :: shape :: FieldShape :: new ("items" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < Schema > > ,) . with_docs ("The `items` keyword.\n\nSee [JSON Schema 10.3.1.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-items).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("contains" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < Schema > > ,) . with_docs ("The `contains` keyword.\n\nSee [JSON Schema 10.3.1.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-contains).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("max_items" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `maxItems` keyword.\n\nSee [JSON Schema Validation 6.4.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxitems).\n") . optional () . json_name ("maxItems") , :: sidex :: shape :: FieldShape :: new ("min_items" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `minItems` keyword.\n\nSee [JSON Schema Validation 6.4.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minitems).\n") . optional () . json_name ("minItems") , :: sidex :: shape :: FieldShape :: new ("unique_items" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("The `uniqueItems` keyword.\n\nSee [JSON Schema Validation 6.4.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-uniqueitems).\n") . optional () . json_name ("uniqueItems") , :: sidex :: shape :: FieldShape :: new ("max_contains" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `maxContains` keyword.\n\nSee [JSON Schema Validation 6.4.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxcontains).\n") . optional () . json_name ("maxContains") , :: sidex :: shape :: FieldShape :: new ("min_contains" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `minContains` keyword.\n\nSee [JSON Schema Validation 6.4.5](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxcontains).\n") . optional () . json_name ("minContains") , :: sidex :: shape :: FieldShape :: new ("unevaluated_items" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < Schema > > ,) . with_docs ("The `unevaluatedItems` keyword.\n\nSee [JSON Schema Core 11.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluateditems)\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("unevaluatedItems")]))) . with_docs ("Keywords applying to arrays.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for ArrayKeywords {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ArrayKeywords {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.prefix_items
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.items
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.contains
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.max_items
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                4usize => {
                    self.min_items
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                5usize => {
                    self.unique_items
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                6usize => {
                    self.max_contains
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                7usize => {
                    self.min_contains
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                8usize => {
                    self.unevaluated_items
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ArrayKeywords {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::vec::Vec<Schema>>::None;
            let mut __field_1 = ::std::option::Option::<::std::boxed::Box<Schema>>::None;
            let mut __field_2 = ::std::option::Option::<::std::boxed::Box<Schema>>::None;
            let mut __field_3 = ::std::option::Option::<usize>::None;
            let mut __field_4 = ::std::option::Option::<usize>::None;
            let mut __field_5 = ::std::option::Option::<bool>::None;
            let mut __field_6 = ::std::option::Option::<usize>::None;
            let mut __field_7 = ::std::option::Option::<usize>::None;
            let mut __field_8 = ::std::option::Option::<::std::boxed::Box<Schema>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::boxed::Box<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::boxed::Box<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(
                            <::std::boxed::Box<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                prefix_items: __field_0,
                items: __field_1,
                contains: __field_2,
                max_items: __field_3,
                min_items: __field_4,
                unique_items: __field_5,
                max_contains: __field_6,
                min_contains: __field_7,
                unevaluated_items: __field_8,
            })
        }
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
    impl ::serde::Serialize for ArrayKeywords {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ArrayKeywords {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Keywords applying to objects.\n"]
    #[derive(Clone, Debug)]
    pub struct ObjectKeywords {
        #[doc = "The `properties` keyword.\n\nSee [JSON Schema 10.3.2.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-properties).\n"]
        pub properties: ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
        #[doc = "The `patternProperties` keyword.\n\nSee [JSON Schema 10.3.2.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-patternproperties).\n"]
        pub pattern_properties:
            ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
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
            ::sidex::types::Map<::std::string::String, ::std::vec::Vec<::std::string::String>>,
        >,
        #[doc = "The `unevaluatedProperties` keyword.\n\nSee [JSON Schema Core 11.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluatedproperties)\n"]
        pub unevaluated_properties: ::std::option::Option<::std::boxed::Box<Schema>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ObjectKeywords {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json_schema::schema::ObjectKeywords" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("properties" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: std :: string :: String , Schema , > > ,) . with_docs ("The `properties` keyword.\n\nSee [JSON Schema 10.3.2.1](https://json-schema.org/draft/2020-12/json-schema-core.html#name-properties).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("pattern_properties" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: std :: string :: String , Schema , > > ,) . with_docs ("The `patternProperties` keyword.\n\nSee [JSON Schema 10.3.2.2](https://json-schema.org/draft/2020-12/json-schema-core.html#name-patternproperties).\n") . optional () . json_name ("patternProperties") , :: sidex :: shape :: FieldShape :: new ("additional_properties" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < Schema > > ,) . with_docs ("The `additionalProperties` keyword.\n\nSee [JSON Schema 10.3.2.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-additionalproperties).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("additionalProperties") , :: sidex :: shape :: FieldShape :: new ("property_names" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < Schema > > ,) . with_docs ("The `propertyNames` keyword.\n\nSee [JSON Schema 10.3.2.4](https://json-schema.org/draft/2020-12/json-schema-core.html#name-propertynames).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("propertyNames") , :: sidex :: shape :: FieldShape :: new ("max_properties" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `maxProperties` keyword.\n\nSee [JSON Schema Validation 6.5.1](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-maxproperties).\n") . optional () . json_name ("maxProperties") , :: sidex :: shape :: FieldShape :: new ("min_properties" , :: sidex :: shape :: shape_of :: < usize > ,) . with_docs ("The `minProperties` keyword.\n\nSee [JSON Schema Validation 6.5.2](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-minproperties).\n") . optional () . json_name ("minProperties") , :: sidex :: shape :: FieldShape :: new ("required" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("The `required` keyword.\n\nSee [JSON Schema Validation 6.5.3](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-required).\n") . optional () , :: sidex :: shape :: FieldShape :: new ("dependent_required" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: std :: string :: String , :: std :: vec :: Vec < :: std :: string :: String , > , > > ,) . with_docs ("The `dependentRequired` keyword.\n\nSee [JSON Schema Validation 6.5.4](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-dependentrequired).\n") . optional () . json_name ("dependentRequired") , :: sidex :: shape :: FieldShape :: new ("unevaluated_properties" , :: sidex :: shape :: shape_of :: < :: std :: boxed :: Box < Schema > > ,) . with_docs ("The `unevaluatedProperties` keyword.\n\nSee [JSON Schema Core 11.3](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluatedproperties)\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: Path ("box")] }]) . optional () . json_name ("unevaluatedProperties")]))) . with_docs ("Keywords applying to objects.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for ObjectKeywords {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ObjectKeywords {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.properties
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.pattern_properties
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.additional_properties
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.property_names
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                4usize => {
                    self.max_properties
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                5usize => {
                    self.min_properties
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                6usize => {
                    self.required
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                7usize => {
                    self.dependent_required
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                8usize => {
                    self.unevaluated_properties
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ObjectKeywords {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 =
                ::std::option::Option::<::sidex::types::Map<::std::string::String, Schema>>::None;
            let mut __field_1 =
                ::std::option::Option::<::sidex::types::Map<::std::string::String, Schema>>::None;
            let mut __field_2 = ::std::option::Option::<::std::boxed::Box<Schema>>::None;
            let mut __field_3 = ::std::option::Option::<::std::boxed::Box<Schema>>::None;
            let mut __field_4 = ::std::option::Option::<usize>::None;
            let mut __field_5 = ::std::option::Option::<usize>::None;
            let mut __field_6 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_7 = ::std::option::Option::<
                ::sidex::types::Map<::std::string::String, ::std::vec::Vec<::std::string::String>>,
            >::None;
            let mut __field_8 = ::std::option::Option::<::std::boxed::Box<Schema>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::std::string::String,
                            Schema,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::std::string::String,
                            Schema,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::boxed::Box<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::boxed::Box<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::std::string::String,
                            ::std::vec::Vec<::std::string::String>,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(
                            <::std::boxed::Box<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                properties: __field_0,
                pattern_properties: __field_1,
                additional_properties: __field_2,
                property_names: __field_3,
                max_properties: __field_4,
                min_properties: __field_5,
                required: __field_6,
                dependent_required: __field_7,
                unevaluated_properties: __field_8,
            })
        }
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
            properties: ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
        ) -> &mut Self {
            self.properties = properties;
            self
        }
        #[doc = "Sets the value of `properties`."]
        pub fn with_properties(
            mut self,
            properties: ::std::option::Option<::sidex::types::Map<::std::string::String, Schema>>,
        ) -> Self {
            self.properties = properties;
            self
        }
        #[doc = "Sets the value of `pattern_properties`."]
        pub fn set_pattern_properties(
            &mut self,
            pattern_properties: ::std::option::Option<
                ::sidex::types::Map<::std::string::String, Schema>,
            >,
        ) -> &mut Self {
            self.pattern_properties = pattern_properties;
            self
        }
        #[doc = "Sets the value of `pattern_properties`."]
        pub fn with_pattern_properties(
            mut self,
            pattern_properties: ::std::option::Option<
                ::sidex::types::Map<::std::string::String, Schema>,
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
                ::sidex::types::Map<::std::string::String, ::std::vec::Vec<::std::string::String>>,
            >,
        ) -> &mut Self {
            self.dependent_required = dependent_required;
            self
        }
        #[doc = "Sets the value of `dependent_required`."]
        pub fn with_dependent_required(
            mut self,
            dependent_required: ::std::option::Option<
                ::sidex::types::Map<::std::string::String, ::std::vec::Vec<::std::string::String>>,
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
    impl ::serde::Serialize for ObjectKeywords {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ObjectKeywords {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
