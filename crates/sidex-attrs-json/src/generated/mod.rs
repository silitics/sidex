/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for the JSON codegen target.\n\nPlugins use these schemas to drive code generation; the compiler parses\nsource `#[json(...)]` attributes against them and stores the result in\nthe IR's `typed_attrs[\"json\"]` map. Downstream codegens deserialize\nthat value directly into the generated Rust types via\n`serde_json::from_value`.\n\nEach record is annotated `#[json(rename_all = \"snake_case\")]` so that\nthe generated deserializers expect JSON keys matching the parser's\nemit shape (the parser writes Sidex field names verbatim, which are\nalready in `snake_case`).\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "JSON-specific attributes that may be applied to an `opaque` definition.\n\nSource form: `#[json(type = \"string|null\", schema = \"/path\")]`.\n"]
    #[derive(Clone, Debug)]
    pub struct OpaqueTypeAttrs {
        #[doc = "JSON union type expression (e.g. `\"string|null\"`).\n"]
        pub typ: ::std::option::Option<::std::string::String>,
        #[doc = "JSON Schema path emitted as `$ref` for the opaque type.\n"]
        pub schema: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for OpaqueTypeAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json::attrs::OpaqueTypeAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("typ" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("JSON union type expression (e.g. `\"string|null\"`).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("type") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("schema" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("JSON Schema path emitted as `$ref` for the opaque type.\n") . optional ()]))) . with_docs ("JSON-specific attributes that may be applied to an `opaque` definition.\n\nSource form: `#[json(type = \"string|null\", schema = \"/path\")]`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("json") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("opaque") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for OpaqueTypeAttrs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for OpaqueTypeAttrs {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.typ
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.schema
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for OpaqueTypeAttrs {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
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
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                typ: __field_0,
                schema: __field_1,
            })
        }
    }
    #[doc = "JSON-specific attributes that may be applied to a `record` definition.\n\nSource form: `#[json(rename_all = \"PascalCase\")]`.\n"]
    #[derive(Clone, Debug)]
    pub struct RecordTypeAttrs {
        #[doc = "Default rename function applied to every field name.\n"]
        pub rename_all: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RecordTypeAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json::attrs::RecordTypeAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("rename_all" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Default rename function applied to every field name.\n") . optional ()]))) . with_docs ("JSON-specific attributes that may be applied to a `record` definition.\n\nSource form: `#[json(rename_all = \"PascalCase\")]`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("json") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("record") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for RecordTypeAttrs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RecordTypeAttrs {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.rename_all
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RecordTypeAttrs {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                rename_all: __field_0,
            })
        }
    }
    #[doc = "JSON-specific attributes that may be applied to a record `field`.\n\nSource form: `#[json(name = \"explicit\", rename = \"camelCase\", inline, number)]`.\n"]
    #[derive(Clone, Debug)]
    pub struct FieldAttrs {
        #[doc = "Override the JSON key with an explicit string.\n"]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = "Apply a rename function to the field name.\n"]
        pub rename: ::std::option::Option<::std::string::String>,
        #[doc = "Whether to inline the field's contents into the enclosing object.\n"]
        pub inline: ::std::option::Option<bool>,
        #[doc = "Whether a 64-bit integer is a JSON number instead of a string.\n"]
        pub number: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for FieldAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json::attrs::FieldAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("name" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Override the JSON key with an explicit string.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("rename" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Apply a rename function to the field name.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("inline" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("Whether to inline the field's contents into the enclosing object.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("number" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("Whether a 64-bit integer is a JSON number instead of a string.\n") . optional ()]))) . with_docs ("JSON-specific attributes that may be applied to a record `field`.\n\nSource form: `#[json(name = \"explicit\", rename = \"camelCase\", inline, number)]`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("json") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("field") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for FieldAttrs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for FieldAttrs {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.name
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.rename
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.inline
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.number
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for FieldAttrs {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<bool>::None;
            let mut __field_3 = ::std::option::Option::<bool>::None;
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
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0,
                rename: __field_1,
                inline: __field_2,
                number: __field_3,
            })
        }
    }
    #[doc = "JSON-specific attributes that may be applied to a `variant` definition.\n\nSource form:\n`#[json(rename_all = \"snake_case\", tagged = internally, tag = \"kind\")]`.\n"]
    #[derive(Clone, Debug)]
    pub struct VariantTypeAttrs {
        #[doc = "Default rename function applied to every variant case name.\n"]
        pub rename_all: ::std::option::Option<::std::string::String>,
        #[doc = "Tagging strategy: `adjacently`, `externally`, `internally`,\nor `implicitly`.\n"]
        pub tagged: ::std::option::Option<::std::string::String>,
        #[doc = "Field name carrying the tag value (for `adjacently` /\n`internally` tagging modes). Defaults to `\"tag\"`.\n"]
        pub tag: ::std::option::Option<::std::string::String>,
        #[doc = "Field name carrying the variant's payload (for `adjacently`\ntagging mode). Defaults to `\"content\"`.\n"]
        pub content: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for VariantTypeAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json::attrs::VariantTypeAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("rename_all" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Default rename function applied to every variant case name.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("tagged" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Tagging strategy: `adjacently`, `externally`, `internally`,\nor `implicitly`.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("tag" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Field name carrying the tag value (for `adjacently` /\n`internally` tagging modes). Defaults to `\"tag\"`.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("content" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Field name carrying the variant's payload (for `adjacently`\ntagging mode). Defaults to `\"content\"`.\n") . optional ()]))) . with_docs ("JSON-specific attributes that may be applied to a `variant` definition.\n\nSource form:\n`#[json(rename_all = \"snake_case\", tagged = internally, tag = \"kind\")]`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("json") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("variant") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for VariantTypeAttrs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for VariantTypeAttrs {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.rename_all
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.tagged
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.tag
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.content
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for VariantTypeAttrs {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
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
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                rename_all: __field_0,
                tagged: __field_1,
                tag: __field_2,
                content: __field_3,
            })
        }
    }
    #[doc = "JSON-specific attributes that may be applied to a single variant case.\n\nSource form: `#[json(name = \"explicit\", rename = \"kebab-case\")]`.\n"]
    #[derive(Clone, Debug)]
    pub struct VariantAttrs {
        #[doc = "Override the JSON tag value with an explicit string.\n"]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = "Apply a rename function to the case name.\n"]
        pub rename: ::std::option::Option<::std::string::String>,
        #[doc = "Override the per-case content field name (only meaningful with\n`adjacently` tagging).\n"]
        pub content: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for VariantAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::json::attrs::VariantAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("name" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Override the JSON tag value with an explicit string.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("rename" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Apply a rename function to the case name.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("content" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Override the per-case content field name (only meaningful with\n`adjacently` tagging).\n") . optional ()]))) . with_docs ("JSON-specific attributes that may be applied to a single variant case.\n\nSource form: `#[json(name = \"explicit\", rename = \"kebab-case\")]`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("json") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("variant_case") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for VariantAttrs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for VariantAttrs {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.name
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.rename
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.content
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for VariantAttrs {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
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
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0,
                rename: __field_1,
                content: __field_2,
            })
        }
    }
}
