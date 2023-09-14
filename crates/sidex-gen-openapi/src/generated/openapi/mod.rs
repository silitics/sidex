/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod openapi {
    #![doc = "Types of the OpenAPI specification.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "A string that MAY use [CommonMark syntax](https://spec.commonmark.org/).\n"]
    #[derive(Clone, Debug)]
    pub struct Markdown(pub(crate) ::std::string::String);
    impl ::std::convert::From<Markdown> for ::std::string::String {
        fn from(wrapped: Markdown) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Markdown {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Markdown {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Markdown(__serde::Deserialize::deserialize(__deserializer)?))
        }
    }
    #[doc = "A string that MUST be an URL.\n"]
    #[derive(Clone, Debug)]
    pub struct Url(pub(crate) ::std::string::String);
    impl ::std::convert::From<Url> for ::std::string::String {
        fn from(wrapped: Url) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Url {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Url {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Url(__serde::Deserialize::deserialize(__deserializer)?))
        }
    }
    #[doc = "A string that MUST be an email address.\n"]
    #[derive(Clone, Debug)]
    pub struct Email(pub(crate) ::std::string::String);
    impl ::std::convert::From<Email> for ::std::string::String {
        fn from(wrapped: Email) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Email {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Email {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Email(__serde::Deserialize::deserialize(__deserializer)?))
        }
    }
    #[doc = "Either a value of type `T` or a reference.\n"]
    #[derive(Clone, Debug)]
    pub enum MaybeRef<T> {
        #[doc = "A value of type `T`.\n"]
        Value(T),
        #[doc = "A reference.\n"]
        Reference(Reference),
    }
    #[automatically_derived]
    impl<T: __serde::Serialize> __serde::Serialize for MaybeRef<T> {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "MaybeRef");
            match self {
                Self::Value(__value) => {
                    __serializer.serialize_implicitly_tagged("Value", 0u32, __value)
                }
                Self::Reference(__value) => {
                    __serializer.serialize_implicitly_tagged("Reference", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de, T: __serde::Deserialize<'de>> __serde::Deserialize<'de> for MaybeRef<T> {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["Value", "Reference"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"Value\", \"Reference\"]";
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
                        "Value" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Reference" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"Value" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Reference" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            const __VARIANTS: &'static [&'static str] = &["Value", "Reference"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<T, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(MaybeRef::Value(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<Reference, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(MaybeRef::Reference(__value)),
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
                    type Value = MaybeRef<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum MaybeRef")
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
                                ::core::result::Result::Ok(MaybeRef::Value(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    Reference,
                                >(__variant)?;
                                ::core::result::Result::Ok(MaybeRef::Reference(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "MaybeRef",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "An [OpenAPI Object](https://spec.openapis.org/oas/v3.1.0#openapi-object) (4.8.1).\n"]
    #[derive(Clone, Debug)]
    pub struct OpenApi {
        #[doc = ""]
        pub openapi: ::std::string::String,
        #[doc = ""]
        pub info: Info,
        #[doc = ""]
        pub json_schema_dialect: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub servers: ::std::option::Option<::std::vec::Vec<Server>>,
        #[doc = ""]
        pub paths: ::std::option::Option<Paths>,
        #[doc = ""]
        pub webhooks:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>>,
        #[doc = ""]
        pub components: ::std::option::Option<Components>,
        #[doc = ""]
        pub security: ::std::option::Option<::std::vec::Vec<SecurityRequirement>>,
        #[doc = ""]
        pub tags: ::std::option::Option<::std::vec::Vec<Tag>>,
        #[doc = ""]
        pub external_docs: ::std::option::Option<ExternalDocumentation>,
    }
    impl OpenApi {
        #[doc = "Creates a new [`OpenApi`]."]
        pub fn new(openapi: ::std::string::String, info: Info) -> Self {
            Self {
                openapi,
                info,
                json_schema_dialect: ::std::default::Default::default(),
                servers: ::std::default::Default::default(),
                paths: ::std::default::Default::default(),
                webhooks: ::std::default::Default::default(),
                components: ::std::default::Default::default(),
                security: ::std::default::Default::default(),
                tags: ::std::default::Default::default(),
                external_docs: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `openapi`."]
        pub fn set_openapi(&mut self, openapi: ::std::string::String) -> &mut Self {
            self.openapi = openapi;
            self
        }
        #[doc = "Sets the value of `openapi`."]
        pub fn with_openapi(mut self, openapi: ::std::string::String) -> Self {
            self.openapi = openapi;
            self
        }
        #[doc = "Sets the value of `info`."]
        pub fn set_info(&mut self, info: Info) -> &mut Self {
            self.info = info;
            self
        }
        #[doc = "Sets the value of `info`."]
        pub fn with_info(mut self, info: Info) -> Self {
            self.info = info;
            self
        }
        #[doc = "Sets the value of `json_schema_dialect`."]
        pub fn set_json_schema_dialect(
            &mut self,
            json_schema_dialect: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.json_schema_dialect = json_schema_dialect;
            self
        }
        #[doc = "Sets the value of `json_schema_dialect`."]
        pub fn with_json_schema_dialect(
            mut self,
            json_schema_dialect: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.json_schema_dialect = json_schema_dialect;
            self
        }
        #[doc = "Sets the value of `servers`."]
        pub fn set_servers(
            &mut self,
            servers: ::std::option::Option<::std::vec::Vec<Server>>,
        ) -> &mut Self {
            self.servers = servers;
            self
        }
        #[doc = "Sets the value of `servers`."]
        pub fn with_servers(
            mut self,
            servers: ::std::option::Option<::std::vec::Vec<Server>>,
        ) -> Self {
            self.servers = servers;
            self
        }
        #[doc = "Sets the value of `paths`."]
        pub fn set_paths(&mut self, paths: ::std::option::Option<Paths>) -> &mut Self {
            self.paths = paths;
            self
        }
        #[doc = "Sets the value of `paths`."]
        pub fn with_paths(mut self, paths: ::std::option::Option<Paths>) -> Self {
            self.paths = paths;
            self
        }
        #[doc = "Sets the value of `webhooks`."]
        pub fn set_webhooks(
            &mut self,
            webhooks: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
            >,
        ) -> &mut Self {
            self.webhooks = webhooks;
            self
        }
        #[doc = "Sets the value of `webhooks`."]
        pub fn with_webhooks(
            mut self,
            webhooks: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
            >,
        ) -> Self {
            self.webhooks = webhooks;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn set_components(
            &mut self,
            components: ::std::option::Option<Components>,
        ) -> &mut Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn with_components(mut self, components: ::std::option::Option<Components>) -> Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `security`."]
        pub fn set_security(
            &mut self,
            security: ::std::option::Option<::std::vec::Vec<SecurityRequirement>>,
        ) -> &mut Self {
            self.security = security;
            self
        }
        #[doc = "Sets the value of `security`."]
        pub fn with_security(
            mut self,
            security: ::std::option::Option<::std::vec::Vec<SecurityRequirement>>,
        ) -> Self {
            self.security = security;
            self
        }
        #[doc = "Sets the value of `tags`."]
        pub fn set_tags(&mut self, tags: ::std::option::Option<::std::vec::Vec<Tag>>) -> &mut Self {
            self.tags = tags;
            self
        }
        #[doc = "Sets the value of `tags`."]
        pub fn with_tags(mut self, tags: ::std::option::Option<::std::vec::Vec<Tag>>) -> Self {
            self.tags = tags;
            self
        }
        #[doc = "Sets the value of `external_docs`."]
        pub fn set_external_docs(
            &mut self,
            external_docs: ::std::option::Option<ExternalDocumentation>,
        ) -> &mut Self {
            self.external_docs = external_docs;
            self
        }
        #[doc = "Sets the value of `external_docs`."]
        pub fn with_external_docs(
            mut self,
            external_docs: ::std::option::Option<ExternalDocumentation>,
        ) -> Self {
            self.external_docs = external_docs;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for OpenApi {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "OpenApi", 10usize)?;
            __record.serialize_field("openapi", &self.openapi)?;
            __record.serialize_field("info", &self.info)?;
            __record.serialize_optional_field(
                "jsonSchemaDialect",
                ::core::option::Option::as_ref(&self.json_schema_dialect),
            )?;
            __record.serialize_optional_field(
                "servers",
                ::core::option::Option::as_ref(&self.servers),
            )?;
            __record
                .serialize_optional_field("paths", ::core::option::Option::as_ref(&self.paths))?;
            __record.serialize_optional_field(
                "webhooks",
                ::core::option::Option::as_ref(&self.webhooks),
            )?;
            __record.serialize_optional_field(
                "components",
                ::core::option::Option::as_ref(&self.components),
            )?;
            __record.serialize_optional_field(
                "security",
                ::core::option::Option::as_ref(&self.security),
            )?;
            __record
                .serialize_optional_field("tags", ::core::option::Option::as_ref(&self.tags))?;
            __record.serialize_optional_field(
                "externalDocs",
                ::core::option::Option::as_ref(&self.external_docs),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for OpenApi {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "openapi",
                "info",
                "jsonSchemaDialect",
                "servers",
                "paths",
                "webhooks",
                "components",
                "security",
                "tags",
                "externalDocs",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"openapi\", \"info\", \"jsonSchemaDialect\", \"servers\", \"paths\", \"webhooks\", \"components\", \"security\", \"tags\", \"externalDocs\"]" ;
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "openapi" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "info" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "jsonSchemaDialect" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier2)
                        }
                        "servers" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "paths" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "webhooks" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "components" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "security" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "tags" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        "externalDocs" => ::core::result::Result::Ok(__Identifier::__Identifier9),
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
                        b"openapi" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"info" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"jsonSchemaDialect" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier2)
                        }
                        b"servers" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"paths" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"webhooks" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"components" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"security" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"tags" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        b"externalDocs" => ::core::result::Result::Ok(__Identifier::__Identifier9),
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
                type Value = OpenApi;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record OpenApi")
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
                                __serde::de::Error::invalid_length(
                                    0usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<Info>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    1usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    2usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Server>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    3usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Paths>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    4usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    5usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Components>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    6usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<SecurityRequirement>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    7usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Tag>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    8usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field9 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<ExternalDocumentation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    9usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    ::core::result::Result::Ok(OpenApi {
                        openapi: __field0,
                        info: __field1,
                        json_schema_dialect: __field2,
                        servers: __field3,
                        paths: __field4,
                        webhooks: __field5,
                        components: __field6,
                        security: __field7,
                        tags: __field8,
                        external_docs: __field9,
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
                    let mut __field1: ::core::option::Option<Info> = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Server>>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<Paths>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::core::option::Option<Components>> =
                        ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<SecurityRequirement>>,
                    > = ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Tag>>,
                    > = ::core::option::Option::None;
                    let mut __field9: ::core::option::Option<
                        ::core::option::Option<ExternalDocumentation>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "openapi",
                                        ),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("info"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Info>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "jsonSchemaDialect",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "servers",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Server>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "paths",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Paths>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "webhooks",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<PathItem>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "components",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Components>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "security",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<SecurityRequirement>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("tags"),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Tag>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier9 => {
                                if ::core::option::Option::is_some(&__field9) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "externalDocs",
                                        ),
                                    );
                                }
                                __field9 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<ExternalDocumentation>,
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
                                <__A::Error as __serde::de::Error>::missing_field("openapi"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("info"),
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
                    ::core::result::Result::Ok(OpenApi {
                        openapi: __field0,
                        info: __field1,
                        json_schema_dialect: __field2,
                        servers: __field3,
                        paths: __field4,
                        webhooks: __field5,
                        components: __field6,
                        security: __field7,
                        tags: __field8,
                        external_docs: __field9,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "openapi",
                "info",
                "jsonSchemaDialect",
                "servers",
                "paths",
                "webhooks",
                "components",
                "security",
                "tags",
                "externalDocs",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "OpenApi",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An [Info Object](https://spec.openapis.org/oas/v3.1.0#info-object) (4.8.2).\n"]
    #[derive(Clone, Debug)]
    pub struct Info {
        #[doc = ""]
        pub title: ::std::string::String,
        #[doc = ""]
        pub version: ::std::string::String,
        #[doc = ""]
        pub summary: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub terms_of_service: ::std::option::Option<Url>,
        #[doc = ""]
        pub contact: ::std::option::Option<Contact>,
        #[doc = ""]
        pub license: ::std::option::Option<License>,
    }
    impl Info {
        #[doc = "Creates a new [`Info`]."]
        pub fn new(title: ::std::string::String, version: ::std::string::String) -> Self {
            Self {
                title,
                version,
                summary: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                terms_of_service: ::std::default::Default::default(),
                contact: ::std::default::Default::default(),
                license: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `title`."]
        pub fn set_title(&mut self, title: ::std::string::String) -> &mut Self {
            self.title = title;
            self
        }
        #[doc = "Sets the value of `title`."]
        pub fn with_title(mut self, title: ::std::string::String) -> Self {
            self.title = title;
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn set_version(&mut self, version: ::std::string::String) -> &mut Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn with_version(mut self, version: ::std::string::String) -> Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn set_summary(
            &mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn with_summary(
            mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `terms_of_service`."]
        pub fn set_terms_of_service(
            &mut self,
            terms_of_service: ::std::option::Option<Url>,
        ) -> &mut Self {
            self.terms_of_service = terms_of_service;
            self
        }
        #[doc = "Sets the value of `terms_of_service`."]
        pub fn with_terms_of_service(
            mut self,
            terms_of_service: ::std::option::Option<Url>,
        ) -> Self {
            self.terms_of_service = terms_of_service;
            self
        }
        #[doc = "Sets the value of `contact`."]
        pub fn set_contact(&mut self, contact: ::std::option::Option<Contact>) -> &mut Self {
            self.contact = contact;
            self
        }
        #[doc = "Sets the value of `contact`."]
        pub fn with_contact(mut self, contact: ::std::option::Option<Contact>) -> Self {
            self.contact = contact;
            self
        }
        #[doc = "Sets the value of `license`."]
        pub fn set_license(&mut self, license: ::std::option::Option<License>) -> &mut Self {
            self.license = license;
            self
        }
        #[doc = "Sets the value of `license`."]
        pub fn with_license(mut self, license: ::std::option::Option<License>) -> Self {
            self.license = license;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Info {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Info", 7usize)?;
            __record.serialize_field("title", &self.title)?;
            __record.serialize_field("version", &self.version)?;
            __record.serialize_optional_field(
                "summary",
                ::core::option::Option::as_ref(&self.summary),
            )?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "termsOfService",
                ::core::option::Option::as_ref(&self.terms_of_service),
            )?;
            __record.serialize_optional_field(
                "contact",
                ::core::option::Option::as_ref(&self.contact),
            )?;
            __record.serialize_optional_field(
                "license",
                ::core::option::Option::as_ref(&self.license),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Info {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "title",
                "version",
                "summary",
                "description",
                "termsOfService",
                "contact",
                "license",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"title\", \"version\", \"summary\", \"description\", \"termsOfService\", \"contact\", \"license\"]" ;
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
                        "version" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "summary" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "termsOfService" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "contact" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "license" => ::core::result::Result::Ok(__Identifier::__Identifier6),
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
                        b"version" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"summary" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"termsOfService" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier4)
                        }
                        b"contact" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"license" => ::core::result::Result::Ok(__Identifier::__Identifier6),
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
                type Value = Info;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Info")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 7 fields"),
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
                                __serde::de::Error::invalid_length(1usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::string::String>,
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
                        ::core::option::Option<Markdown>,
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
                        ::core::option::Option<Url>,
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
                        ::core::option::Option<Contact>,
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
                        ::core::option::Option<License>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 7 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Info {
                        title: __field0,
                        version: __field1,
                        summary: __field2,
                        description: __field3,
                        terms_of_service: __field4,
                        contact: __field5,
                        license: __field6,
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
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<Url>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<Contact>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::core::option::Option<License>> =
                        ::core::option::Option::None;
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
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "version",
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
                                            "summary",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "termsOfService",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Url>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "contact",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Contact>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "license",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<License>,
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
                                <__A::Error as __serde::de::Error>::missing_field("title"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("version"),
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
                    ::core::result::Result::Ok(Info {
                        title: __field0,
                        version: __field1,
                        summary: __field2,
                        description: __field3,
                        terms_of_service: __field4,
                        contact: __field5,
                        license: __field6,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "title",
                "version",
                "summary",
                "description",
                "termsOfService",
                "contact",
                "license",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Info",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Contact Object](https://spec.openapis.org/oas/v3.1.0#contact-object)  (4.8.3).\n"]
    #[derive(Clone, Debug)]
    pub struct Contact {
        #[doc = ""]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub url: ::std::option::Option<Url>,
        #[doc = ""]
        pub email: ::std::option::Option<Email>,
    }
    impl Contact {
        #[doc = "Creates a new [`Contact`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                url: ::std::default::Default::default(),
                email: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::option::Option<::std::string::String>) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(&mut self, url: ::std::option::Option<Url>) -> &mut Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: ::std::option::Option<Url>) -> Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `email`."]
        pub fn set_email(&mut self, email: ::std::option::Option<Email>) -> &mut Self {
            self.email = email;
            self
        }
        #[doc = "Sets the value of `email`."]
        pub fn with_email(mut self, email: ::std::option::Option<Email>) -> Self {
            self.email = email;
            self
        }
    }
    impl ::std::default::Default for Contact {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Contact {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Contact", 3usize)?;
            __record
                .serialize_optional_field("name", ::core::option::Option::as_ref(&self.name))?;
            __record.serialize_optional_field("url", ::core::option::Option::as_ref(&self.url))?;
            __record
                .serialize_optional_field("email", ::core::option::Option::as_ref(&self.email))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Contact {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["name", "url", "email"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"url\", \"email\"]";
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
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "url" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "email" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"url" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"email" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                type Value = Contact;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Contact")
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
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Url>,
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
                        ::core::option::Option<Email>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Contact {
                        name: __field0,
                        url: __field1,
                        email: __field2,
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
                    let mut __field1: ::core::option::Option<::core::option::Option<Url>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Email>> =
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
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("url"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Url>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "email",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Email>,
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
                    ::core::result::Result::Ok(Contact {
                        name: __field0,
                        url: __field1,
                        email: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "url", "email"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Contact",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [License Object](https://spec.openapis.org/oas/v3.1.0#license-object) (4.8.4).\n"]
    #[derive(Clone, Debug)]
    pub struct License {
        #[doc = ""]
        pub name: ::std::string::String,
        #[doc = ""]
        pub identifier: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub url: ::std::option::Option<Url>,
    }
    impl License {
        #[doc = "Creates a new [`License`]."]
        pub fn new(name: ::std::string::String) -> Self {
            Self {
                name,
                identifier: ::std::default::Default::default(),
                url: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `identifier`."]
        pub fn set_identifier(
            &mut self,
            identifier: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.identifier = identifier;
            self
        }
        #[doc = "Sets the value of `identifier`."]
        pub fn with_identifier(
            mut self,
            identifier: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.identifier = identifier;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(&mut self, url: ::std::option::Option<Url>) -> &mut Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: ::std::option::Option<Url>) -> Self {
            self.url = url;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for License {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "License", 3usize)?;
            __record.serialize_field("name", &self.name)?;
            __record.serialize_optional_field(
                "identifier",
                ::core::option::Option::as_ref(&self.identifier),
            )?;
            __record.serialize_optional_field("url", ::core::option::Option::as_ref(&self.url))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for License {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["name", "identifier", "url"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"identifier\", \"url\"]";
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
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "identifier" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "url" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"identifier" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"url" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                type Value = License;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record License")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
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
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Url>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(License {
                        name: __field0,
                        identifier: __field1,
                        url: __field2,
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
                    let mut __field2: ::core::option::Option<::core::option::Option<Url>> =
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
                                            "identifier",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("url"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Url>,
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
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(License {
                        name: __field0,
                        identifier: __field1,
                        url: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "identifier", "url"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "License",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Server Object](https://spec.openapis.org/oas/v3.1.0#server-object) (4.8.5).\n"]
    #[derive(Clone, Debug)]
    pub struct Server {
        #[doc = ""]
        pub url: Url,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub variables:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, ServerVariable>>,
    }
    impl Server {
        #[doc = "Creates a new [`Server`]."]
        pub fn new(url: Url) -> Self {
            Self {
                url,
                description: ::std::default::Default::default(),
                variables: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(&mut self, url: Url) -> &mut Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: Url) -> Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `variables`."]
        pub fn set_variables(
            &mut self,
            variables: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ServerVariable>,
            >,
        ) -> &mut Self {
            self.variables = variables;
            self
        }
        #[doc = "Sets the value of `variables`."]
        pub fn with_variables(
            mut self,
            variables: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ServerVariable>,
            >,
        ) -> Self {
            self.variables = variables;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Server {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Server", 3usize)?;
            __record.serialize_field("url", &self.url)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "variables",
                ::core::option::Option::as_ref(&self.variables),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Server {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["url", "description", "variables"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"url\", \"description\", \"variables\"]";
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
                        "url" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "variables" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"url" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"variables" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                type Value = Server;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Server")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<Url>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
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
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, ServerVariable>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Server {
                        url: __field0,
                        description: __field1,
                        variables: __field2,
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
                    let mut __field0: ::core::option::Option<Url> = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, ServerVariable>,
                        >,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("url"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Url>(&mut __map)?,
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
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "variables",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                ServerVariable,
                                            >,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("url"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Server {
                        url: __field0,
                        description: __field1,
                        variables: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["url", "description", "variables"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Server",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Server Variable Object](https://spec.openapis.org/oas/v3.1.0#server-variable-object) (4.8.6).\n"]
    #[derive(Clone, Debug)]
    pub struct ServerVariable {
        #[doc = ""]
        pub options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = ""]
        pub default: ::std::string::String,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
    }
    impl ServerVariable {
        #[doc = "Creates a new [`ServerVariable`]."]
        pub fn new(default: ::std::string::String) -> Self {
            Self {
                default,
                options: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `options`."]
        pub fn set_options(
            &mut self,
            options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.options = options;
            self
        }
        #[doc = "Sets the value of `options`."]
        pub fn with_options(
            mut self,
            options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.options = options;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn set_default(&mut self, default: ::std::string::String) -> &mut Self {
            self.default = default;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn with_default(mut self, default: ::std::string::String) -> Self {
            self.default = default;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ServerVariable {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ServerVariable", 3usize)?;
            __record
                .serialize_optional_field("enum", ::core::option::Option::as_ref(&self.options))?;
            __record.serialize_field("default", &self.default)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ServerVariable {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["enum", "default", "description"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"enum\", \"default\", \"description\"]";
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
                        "enum" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "default" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"enum" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"default" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                type Value = ServerVariable;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ServerVariable")
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
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
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
                        ::std::string::String,
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
                        ::core::option::Option<Markdown>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ServerVariable {
                        options: __field0,
                        default: __field1,
                        description: __field2,
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
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("enum"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "default",
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
                                            "description",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
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
                                <__A::Error as __serde::de::Error>::missing_field("default"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(ServerVariable {
                        options: __field0,
                        default: __field1,
                        description: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["enum", "default", "description"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ServerVariable",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Components Object](https://spec.openapis.org/oas/v3.1.0#components-object) (4.8.7).\n"]
    #[derive(Clone, Debug)]
    pub struct Components {
        #[doc = ""]
        pub schemas: ::std::option::Option<
            indexmap::IndexMap<
                ::std::string::String,
                ::sidex_gen_json_schema::types::schema::SchemaObject,
            >,
        >,
        #[doc = ""]
        pub responses:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Response>>>,
        #[doc = ""]
        pub parameters:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Parameter>>>,
        #[doc = ""]
        pub examples:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Example>>>,
        #[doc = ""]
        pub request_bodies:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<RequestBody>>>,
        #[doc = ""]
        pub headers:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Header>>>,
        #[doc = ""]
        pub security_schemes: ::std::option::Option<
            indexmap::IndexMap<::std::string::String, MaybeRef<SecurityScheme>>,
        >,
        #[doc = ""]
        pub links: ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Link>>>,
        #[doc = ""]
        pub callbacks:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>>,
        #[doc = ""]
        pub path_items:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>>,
    }
    impl Components {
        #[doc = "Creates a new [`Components`]."]
        pub fn new() -> Self {
            Self {
                schemas: ::std::default::Default::default(),
                responses: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
                examples: ::std::default::Default::default(),
                request_bodies: ::std::default::Default::default(),
                headers: ::std::default::Default::default(),
                security_schemes: ::std::default::Default::default(),
                links: ::std::default::Default::default(),
                callbacks: ::std::default::Default::default(),
                path_items: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `schemas`."]
        pub fn set_schemas(
            &mut self,
            schemas: ::std::option::Option<
                indexmap::IndexMap<
                    ::std::string::String,
                    ::sidex_gen_json_schema::types::schema::SchemaObject,
                >,
            >,
        ) -> &mut Self {
            self.schemas = schemas;
            self
        }
        #[doc = "Sets the value of `schemas`."]
        pub fn with_schemas(
            mut self,
            schemas: ::std::option::Option<
                indexmap::IndexMap<
                    ::std::string::String,
                    ::sidex_gen_json_schema::types::schema::SchemaObject,
                >,
            >,
        ) -> Self {
            self.schemas = schemas;
            self
        }
        #[doc = "Sets the value of `responses`."]
        pub fn set_responses(
            &mut self,
            responses: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Response>>,
            >,
        ) -> &mut Self {
            self.responses = responses;
            self
        }
        #[doc = "Sets the value of `responses`."]
        pub fn with_responses(
            mut self,
            responses: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Response>>,
            >,
        ) -> Self {
            self.responses = responses;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Parameter>>,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Parameter>>,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn set_examples(
            &mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> &mut Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn with_examples(
            mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `request_bodies`."]
        pub fn set_request_bodies(
            &mut self,
            request_bodies: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<RequestBody>>,
            >,
        ) -> &mut Self {
            self.request_bodies = request_bodies;
            self
        }
        #[doc = "Sets the value of `request_bodies`."]
        pub fn with_request_bodies(
            mut self,
            request_bodies: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<RequestBody>>,
            >,
        ) -> Self {
            self.request_bodies = request_bodies;
            self
        }
        #[doc = "Sets the value of `headers`."]
        pub fn set_headers(
            &mut self,
            headers: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
            >,
        ) -> &mut Self {
            self.headers = headers;
            self
        }
        #[doc = "Sets the value of `headers`."]
        pub fn with_headers(
            mut self,
            headers: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
            >,
        ) -> Self {
            self.headers = headers;
            self
        }
        #[doc = "Sets the value of `security_schemes`."]
        pub fn set_security_schemes(
            &mut self,
            security_schemes: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<SecurityScheme>>,
            >,
        ) -> &mut Self {
            self.security_schemes = security_schemes;
            self
        }
        #[doc = "Sets the value of `security_schemes`."]
        pub fn with_security_schemes(
            mut self,
            security_schemes: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<SecurityScheme>>,
            >,
        ) -> Self {
            self.security_schemes = security_schemes;
            self
        }
        #[doc = "Sets the value of `links`."]
        pub fn set_links(
            &mut self,
            links: ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Link>>>,
        ) -> &mut Self {
            self.links = links;
            self
        }
        #[doc = "Sets the value of `links`."]
        pub fn with_links(
            mut self,
            links: ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Link>>>,
        ) -> Self {
            self.links = links;
            self
        }
        #[doc = "Sets the value of `callbacks`."]
        pub fn set_callbacks(
            &mut self,
            callbacks: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
            >,
        ) -> &mut Self {
            self.callbacks = callbacks;
            self
        }
        #[doc = "Sets the value of `callbacks`."]
        pub fn with_callbacks(
            mut self,
            callbacks: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
            >,
        ) -> Self {
            self.callbacks = callbacks;
            self
        }
        #[doc = "Sets the value of `path_items`."]
        pub fn set_path_items(
            &mut self,
            path_items: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
            >,
        ) -> &mut Self {
            self.path_items = path_items;
            self
        }
        #[doc = "Sets the value of `path_items`."]
        pub fn with_path_items(
            mut self,
            path_items: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
            >,
        ) -> Self {
            self.path_items = path_items;
            self
        }
    }
    impl ::std::default::Default for Components {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Components {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Components", 10usize)?;
            __record.serialize_optional_field(
                "schemas",
                ::core::option::Option::as_ref(&self.schemas),
            )?;
            __record.serialize_optional_field(
                "responses",
                ::core::option::Option::as_ref(&self.responses),
            )?;
            __record.serialize_optional_field(
                "parameters",
                ::core::option::Option::as_ref(&self.parameters),
            )?;
            __record.serialize_optional_field(
                "examples",
                ::core::option::Option::as_ref(&self.examples),
            )?;
            __record.serialize_optional_field(
                "requestBodies",
                ::core::option::Option::as_ref(&self.request_bodies),
            )?;
            __record.serialize_optional_field(
                "headers",
                ::core::option::Option::as_ref(&self.headers),
            )?;
            __record.serialize_optional_field(
                "securitySchemes",
                ::core::option::Option::as_ref(&self.security_schemes),
            )?;
            __record
                .serialize_optional_field("links", ::core::option::Option::as_ref(&self.links))?;
            __record.serialize_optional_field(
                "callbacks",
                ::core::option::Option::as_ref(&self.callbacks),
            )?;
            __record.serialize_optional_field(
                "pathItems",
                ::core::option::Option::as_ref(&self.path_items),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Components {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "schemas",
                "responses",
                "parameters",
                "examples",
                "requestBodies",
                "headers",
                "securitySchemes",
                "links",
                "callbacks",
                "pathItems",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"schemas\", \"responses\", \"parameters\", \"examples\", \"requestBodies\", \"headers\", \"securitySchemes\", \"links\", \"callbacks\", \"pathItems\"]" ;
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "schemas" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "responses" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "parameters" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "examples" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "requestBodies" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "headers" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "securitySchemes" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier6)
                        }
                        "links" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "callbacks" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        "pathItems" => ::core::result::Result::Ok(__Identifier::__Identifier9),
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
                        b"schemas" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"responses" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"parameters" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"examples" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"requestBodies" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"headers" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"securitySchemes" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier6)
                        }
                        b"links" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"callbacks" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        b"pathItems" => ::core::result::Result::Ok(__Identifier::__Identifier9),
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
                type Value = Components;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Components")
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
                        ::core::option::Option<
                            indexmap::IndexMap<
                                ::std::string::String,
                                ::sidex_gen_json_schema::types::schema::SchemaObject,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    0usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Response>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    1usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Parameter>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    2usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    3usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<RequestBody>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    4usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    5usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<SecurityScheme>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    6usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Link>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    7usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    8usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    let __field9 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    9usize,
                                    &"record with 10 fields",
                                ),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Components {
                        schemas: __field0,
                        responses: __field1,
                        parameters: __field2,
                        examples: __field3,
                        request_bodies: __field4,
                        headers: __field5,
                        security_schemes: __field6,
                        links: __field7,
                        callbacks: __field8,
                        path_items: __field9,
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
                        ::core::option::Option<
                            indexmap::IndexMap<
                                ::std::string::String,
                                ::sidex_gen_json_schema::types::schema::SchemaObject,
                            >,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Response>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Parameter>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<RequestBody>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<SecurityScheme>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Link>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field9: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>,
                        >,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schemas",
                                        ),
                                    );
                                }
                                __field0 = :: core :: option :: Option :: Some (__serde :: de :: MapAccess :: next_value :: < :: core :: option :: Option < indexmap :: IndexMap < :: std :: string :: String < > , :: sidex_gen_json_schema :: types :: schema :: SchemaObject < > , > > > (& mut __map) ?) ;
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "responses",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Response>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "parameters",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Parameter>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "examples",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Example>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "requestBodies",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<RequestBody>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "headers",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Header>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "securitySchemes",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<SecurityScheme>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "links",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Link>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "callbacks",
                                        ),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Callback>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier9 => {
                                if ::core::option::Option::is_some(&__field9) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "pathItems",
                                        ),
                                    );
                                }
                                __field9 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<PathItem>,
                                            >,
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
                    ::core::result::Result::Ok(Components {
                        schemas: __field0,
                        responses: __field1,
                        parameters: __field2,
                        examples: __field3,
                        request_bodies: __field4,
                        headers: __field5,
                        security_schemes: __field6,
                        links: __field7,
                        callbacks: __field8,
                        path_items: __field9,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "schemas",
                "responses",
                "parameters",
                "examples",
                "requestBodies",
                "headers",
                "securitySchemes",
                "links",
                "callbacks",
                "pathItems",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Components",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Paths Object](https://spec.openapis.org/oas/v3.1.0#paths-object) (4.8.8)\n"]
    #[derive(Clone, Debug)]
    pub struct Paths(pub(crate) indexmap::IndexMap<::std::string::String, PathItem>);
    impl ::std::convert::From<Paths> for indexmap::IndexMap<::std::string::String, PathItem> {
        fn from(wrapped: Paths) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Paths {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Paths {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Paths(__serde::Deserialize::deserialize(__deserializer)?))
        }
    }
    #[doc = "A [Path Item Object](https://spec.openapis.org/oas/v3.1.0#path-item-object) (4.8.9)\n"]
    #[derive(Clone, Debug)]
    pub struct PathItem {
        #[doc = ""]
        pub reference: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub summary: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub get: ::std::option::Option<Operation>,
        #[doc = ""]
        pub put: ::std::option::Option<Operation>,
        #[doc = ""]
        pub post: ::std::option::Option<Operation>,
        #[doc = ""]
        pub delete: ::std::option::Option<Operation>,
        #[doc = ""]
        pub options: ::std::option::Option<Operation>,
        #[doc = ""]
        pub head: ::std::option::Option<Operation>,
        #[doc = ""]
        pub patch: ::std::option::Option<Operation>,
        #[doc = ""]
        pub trace: ::std::option::Option<Operation>,
        #[doc = ""]
        pub servers: ::std::option::Option<::std::vec::Vec<Server>>,
        #[doc = ""]
        pub parameters: ::std::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
    }
    impl PathItem {
        #[doc = "Creates a new [`PathItem`]."]
        pub fn new() -> Self {
            Self {
                reference: ::std::default::Default::default(),
                summary: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                get: ::std::default::Default::default(),
                put: ::std::default::Default::default(),
                post: ::std::default::Default::default(),
                delete: ::std::default::Default::default(),
                options: ::std::default::Default::default(),
                head: ::std::default::Default::default(),
                patch: ::std::default::Default::default(),
                trace: ::std::default::Default::default(),
                servers: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `reference`."]
        pub fn set_reference(
            &mut self,
            reference: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.reference = reference;
            self
        }
        #[doc = "Sets the value of `reference`."]
        pub fn with_reference(
            mut self,
            reference: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.reference = reference;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn set_summary(
            &mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn with_summary(
            mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `get`."]
        pub fn set_get(&mut self, get: ::std::option::Option<Operation>) -> &mut Self {
            self.get = get;
            self
        }
        #[doc = "Sets the value of `get`."]
        pub fn with_get(mut self, get: ::std::option::Option<Operation>) -> Self {
            self.get = get;
            self
        }
        #[doc = "Sets the value of `put`."]
        pub fn set_put(&mut self, put: ::std::option::Option<Operation>) -> &mut Self {
            self.put = put;
            self
        }
        #[doc = "Sets the value of `put`."]
        pub fn with_put(mut self, put: ::std::option::Option<Operation>) -> Self {
            self.put = put;
            self
        }
        #[doc = "Sets the value of `post`."]
        pub fn set_post(&mut self, post: ::std::option::Option<Operation>) -> &mut Self {
            self.post = post;
            self
        }
        #[doc = "Sets the value of `post`."]
        pub fn with_post(mut self, post: ::std::option::Option<Operation>) -> Self {
            self.post = post;
            self
        }
        #[doc = "Sets the value of `delete`."]
        pub fn set_delete(&mut self, delete: ::std::option::Option<Operation>) -> &mut Self {
            self.delete = delete;
            self
        }
        #[doc = "Sets the value of `delete`."]
        pub fn with_delete(mut self, delete: ::std::option::Option<Operation>) -> Self {
            self.delete = delete;
            self
        }
        #[doc = "Sets the value of `options`."]
        pub fn set_options(&mut self, options: ::std::option::Option<Operation>) -> &mut Self {
            self.options = options;
            self
        }
        #[doc = "Sets the value of `options`."]
        pub fn with_options(mut self, options: ::std::option::Option<Operation>) -> Self {
            self.options = options;
            self
        }
        #[doc = "Sets the value of `head`."]
        pub fn set_head(&mut self, head: ::std::option::Option<Operation>) -> &mut Self {
            self.head = head;
            self
        }
        #[doc = "Sets the value of `head`."]
        pub fn with_head(mut self, head: ::std::option::Option<Operation>) -> Self {
            self.head = head;
            self
        }
        #[doc = "Sets the value of `patch`."]
        pub fn set_patch(&mut self, patch: ::std::option::Option<Operation>) -> &mut Self {
            self.patch = patch;
            self
        }
        #[doc = "Sets the value of `patch`."]
        pub fn with_patch(mut self, patch: ::std::option::Option<Operation>) -> Self {
            self.patch = patch;
            self
        }
        #[doc = "Sets the value of `trace`."]
        pub fn set_trace(&mut self, trace: ::std::option::Option<Operation>) -> &mut Self {
            self.trace = trace;
            self
        }
        #[doc = "Sets the value of `trace`."]
        pub fn with_trace(mut self, trace: ::std::option::Option<Operation>) -> Self {
            self.trace = trace;
            self
        }
        #[doc = "Sets the value of `servers`."]
        pub fn set_servers(
            &mut self,
            servers: ::std::option::Option<::std::vec::Vec<Server>>,
        ) -> &mut Self {
            self.servers = servers;
            self
        }
        #[doc = "Sets the value of `servers`."]
        pub fn with_servers(
            mut self,
            servers: ::std::option::Option<::std::vec::Vec<Server>>,
        ) -> Self {
            self.servers = servers;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
        ) -> Self {
            self.parameters = parameters;
            self
        }
    }
    impl ::std::default::Default for PathItem {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for PathItem {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "PathItem", 13usize)?;
            __record.serialize_optional_field(
                "$ref",
                ::core::option::Option::as_ref(&self.reference),
            )?;
            __record.serialize_optional_field(
                "summary",
                ::core::option::Option::as_ref(&self.summary),
            )?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field("get", ::core::option::Option::as_ref(&self.get))?;
            __record.serialize_optional_field("put", ::core::option::Option::as_ref(&self.put))?;
            __record
                .serialize_optional_field("post", ::core::option::Option::as_ref(&self.post))?;
            __record
                .serialize_optional_field("delete", ::core::option::Option::as_ref(&self.delete))?;
            __record.serialize_optional_field(
                "options",
                ::core::option::Option::as_ref(&self.options),
            )?;
            __record
                .serialize_optional_field("head", ::core::option::Option::as_ref(&self.head))?;
            __record
                .serialize_optional_field("patch", ::core::option::Option::as_ref(&self.patch))?;
            __record
                .serialize_optional_field("trace", ::core::option::Option::as_ref(&self.trace))?;
            __record.serialize_optional_field(
                "servers",
                ::core::option::Option::as_ref(&self.servers),
            )?;
            __record.serialize_optional_field(
                "parameters",
                ::core::option::Option::as_ref(&self.parameters),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PathItem {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "$ref",
                "summary",
                "description",
                "get",
                "put",
                "post",
                "delete",
                "options",
                "head",
                "patch",
                "trace",
                "servers",
                "parameters",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"$ref\", \"summary\", \"description\", \"get\", \"put\", \"post\", \"delete\", \"options\", \"head\", \"patch\", \"trace\", \"servers\", \"parameters\"]" ;
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "$ref" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "summary" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "get" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "put" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "post" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "delete" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "options" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "head" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        "patch" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        "trace" => ::core::result::Result::Ok(__Identifier::__Identifier10),
                        "servers" => ::core::result::Result::Ok(__Identifier::__Identifier11),
                        "parameters" => ::core::result::Result::Ok(__Identifier::__Identifier12),
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
                        b"$ref" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"summary" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"get" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"put" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"post" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"delete" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"options" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"head" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        b"patch" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        b"trace" => ::core::result::Result::Ok(__Identifier::__Identifier10),
                        b"servers" => ::core::result::Result::Ok(__Identifier::__Identifier11),
                        b"parameters" => ::core::result::Result::Ok(__Identifier::__Identifier12),
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
                type Value = PathItem;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record PathItem")
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
                                    &"record with 13 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    1usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    2usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    3usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    4usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    5usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    6usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    7usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    8usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field9 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    9usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field10 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Operation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    10usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field11 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Server>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    11usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field12 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    12usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    ::core::result::Result::Ok(PathItem {
                        reference: __field0,
                        summary: __field1,
                        description: __field2,
                        get: __field3,
                        put: __field4,
                        post: __field5,
                        delete: __field6,
                        options: __field7,
                        head: __field8,
                        patch: __field9,
                        trace: __field10,
                        servers: __field11,
                        parameters: __field12,
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
                    let mut __field2: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field9: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field10: ::core::option::Option<::core::option::Option<Operation>> =
                        ::core::option::Option::None;
                    let mut __field11: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Server>>,
                    > = ::core::option::Option::None;
                    let mut __field12: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("$ref"),
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
                                            "summary",
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
                                            "description",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("get"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("put"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("post"),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "delete",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "options",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("head"),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier9 => {
                                if ::core::option::Option::is_some(&__field9) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "patch",
                                        ),
                                    );
                                }
                                __field9 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier10 => {
                                if ::core::option::Option::is_some(&__field10) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "trace",
                                        ),
                                    );
                                }
                                __field10 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Operation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier11 => {
                                if ::core::option::Option::is_some(&__field11) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "servers",
                                        ),
                                    );
                                }
                                __field11 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Server>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier12 => {
                                if ::core::option::Option::is_some(&__field12) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "parameters",
                                        ),
                                    );
                                }
                                __field12 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<MaybeRef<Parameter>>,
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
                    ::core::result::Result::Ok(PathItem {
                        reference: __field0,
                        summary: __field1,
                        description: __field2,
                        get: __field3,
                        put: __field4,
                        post: __field5,
                        delete: __field6,
                        options: __field7,
                        head: __field8,
                        patch: __field9,
                        trace: __field10,
                        servers: __field11,
                        parameters: __field12,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "$ref",
                "summary",
                "description",
                "get",
                "put",
                "post",
                "delete",
                "options",
                "head",
                "patch",
                "trace",
                "servers",
                "parameters",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "PathItem",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An [Operation Object](https://spec.openapis.org/oas/v3.1.0#operation-object) (4.8.10)\n"]
    #[derive(Clone, Debug)]
    pub struct Operation {
        #[doc = ""]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = ""]
        pub summary: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub external_docs: ::std::option::Option<ExternalDocumentation>,
        #[doc = ""]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub parameters: ::std::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
        #[doc = ""]
        pub request_body: ::std::option::Option<MaybeRef<RequestBody>>,
        #[doc = ""]
        pub responses: ::std::option::Option<Responses>,
        #[doc = ""]
        pub callbacks:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>>,
        #[doc = ""]
        pub deprecated: ::std::option::Option<bool>,
        #[doc = ""]
        pub security: ::std::option::Option<::std::vec::Vec<SecurityRequirement>>,
        #[doc = ""]
        pub servers: ::std::option::Option<::std::vec::Vec<Server>>,
    }
    impl Operation {
        #[doc = "Creates a new [`Operation`]."]
        pub fn new() -> Self {
            Self {
                tags: ::std::default::Default::default(),
                summary: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                external_docs: ::std::default::Default::default(),
                operation_id: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
                request_body: ::std::default::Default::default(),
                responses: ::std::default::Default::default(),
                callbacks: ::std::default::Default::default(),
                deprecated: ::std::default::Default::default(),
                security: ::std::default::Default::default(),
                servers: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `tags`."]
        pub fn set_tags(
            &mut self,
            tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.tags = tags;
            self
        }
        #[doc = "Sets the value of `tags`."]
        pub fn with_tags(
            mut self,
            tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.tags = tags;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn set_summary(
            &mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn with_summary(
            mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `external_docs`."]
        pub fn set_external_docs(
            &mut self,
            external_docs: ::std::option::Option<ExternalDocumentation>,
        ) -> &mut Self {
            self.external_docs = external_docs;
            self
        }
        #[doc = "Sets the value of `external_docs`."]
        pub fn with_external_docs(
            mut self,
            external_docs: ::std::option::Option<ExternalDocumentation>,
        ) -> Self {
            self.external_docs = external_docs;
            self
        }
        #[doc = "Sets the value of `operation_id`."]
        pub fn set_operation_id(
            &mut self,
            operation_id: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.operation_id = operation_id;
            self
        }
        #[doc = "Sets the value of `operation_id`."]
        pub fn with_operation_id(
            mut self,
            operation_id: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.operation_id = operation_id;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
        ) -> Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `request_body`."]
        pub fn set_request_body(
            &mut self,
            request_body: ::std::option::Option<MaybeRef<RequestBody>>,
        ) -> &mut Self {
            self.request_body = request_body;
            self
        }
        #[doc = "Sets the value of `request_body`."]
        pub fn with_request_body(
            mut self,
            request_body: ::std::option::Option<MaybeRef<RequestBody>>,
        ) -> Self {
            self.request_body = request_body;
            self
        }
        #[doc = "Sets the value of `responses`."]
        pub fn set_responses(&mut self, responses: ::std::option::Option<Responses>) -> &mut Self {
            self.responses = responses;
            self
        }
        #[doc = "Sets the value of `responses`."]
        pub fn with_responses(mut self, responses: ::std::option::Option<Responses>) -> Self {
            self.responses = responses;
            self
        }
        #[doc = "Sets the value of `callbacks`."]
        pub fn set_callbacks(
            &mut self,
            callbacks: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
            >,
        ) -> &mut Self {
            self.callbacks = callbacks;
            self
        }
        #[doc = "Sets the value of `callbacks`."]
        pub fn with_callbacks(
            mut self,
            callbacks: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
            >,
        ) -> Self {
            self.callbacks = callbacks;
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
        #[doc = "Sets the value of `security`."]
        pub fn set_security(
            &mut self,
            security: ::std::option::Option<::std::vec::Vec<SecurityRequirement>>,
        ) -> &mut Self {
            self.security = security;
            self
        }
        #[doc = "Sets the value of `security`."]
        pub fn with_security(
            mut self,
            security: ::std::option::Option<::std::vec::Vec<SecurityRequirement>>,
        ) -> Self {
            self.security = security;
            self
        }
        #[doc = "Sets the value of `servers`."]
        pub fn set_servers(
            &mut self,
            servers: ::std::option::Option<::std::vec::Vec<Server>>,
        ) -> &mut Self {
            self.servers = servers;
            self
        }
        #[doc = "Sets the value of `servers`."]
        pub fn with_servers(
            mut self,
            servers: ::std::option::Option<::std::vec::Vec<Server>>,
        ) -> Self {
            self.servers = servers;
            self
        }
    }
    impl ::std::default::Default for Operation {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Operation {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Operation", 12usize)?;
            __record
                .serialize_optional_field("tags", ::core::option::Option::as_ref(&self.tags))?;
            __record.serialize_optional_field(
                "summary",
                ::core::option::Option::as_ref(&self.summary),
            )?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "externalDocs",
                ::core::option::Option::as_ref(&self.external_docs),
            )?;
            __record.serialize_optional_field(
                "operationId",
                ::core::option::Option::as_ref(&self.operation_id),
            )?;
            __record.serialize_optional_field(
                "parameters",
                ::core::option::Option::as_ref(&self.parameters),
            )?;
            __record.serialize_optional_field(
                "requestBody",
                ::core::option::Option::as_ref(&self.request_body),
            )?;
            __record.serialize_optional_field(
                "responses",
                ::core::option::Option::as_ref(&self.responses),
            )?;
            __record.serialize_optional_field(
                "callbacks",
                ::core::option::Option::as_ref(&self.callbacks),
            )?;
            __record.serialize_optional_field(
                "deprecated",
                ::core::option::Option::as_ref(&self.deprecated),
            )?;
            __record.serialize_optional_field(
                "security",
                ::core::option::Option::as_ref(&self.security),
            )?;
            __record.serialize_optional_field(
                "servers",
                ::core::option::Option::as_ref(&self.servers),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Operation {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "tags",
                "summary",
                "description",
                "externalDocs",
                "operationId",
                "parameters",
                "requestBody",
                "responses",
                "callbacks",
                "deprecated",
                "security",
                "servers",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"tags\", \"summary\", \"description\", \"externalDocs\", \"operationId\", \"parameters\", \"requestBody\", \"responses\", \"callbacks\", \"deprecated\", \"security\", \"servers\"]" ;
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "tags" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "summary" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "externalDocs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "operationId" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "parameters" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "requestBody" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "responses" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "callbacks" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        "deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        "security" => ::core::result::Result::Ok(__Identifier::__Identifier10),
                        "servers" => ::core::result::Result::Ok(__Identifier::__Identifier11),
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
                        b"tags" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"summary" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"externalDocs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"operationId" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"parameters" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"requestBody" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"responses" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"callbacks" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        b"deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        b"security" => ::core::result::Result::Ok(__Identifier::__Identifier10),
                        b"servers" => ::core::result::Result::Ok(__Identifier::__Identifier11),
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
                type Value = Operation;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Operation")
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
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    0usize,
                                    &"record with 12 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    1usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    2usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<ExternalDocumentation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    3usize,
                                    &"record with 12 fields",
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
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    5usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<MaybeRef<RequestBody>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    6usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Responses>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    7usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    8usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field9 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    9usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field10 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<SecurityRequirement>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    10usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    let __field11 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<::std::vec::Vec<Server>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    11usize,
                                    &"record with 12 fields",
                                ),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Operation {
                        tags: __field0,
                        summary: __field1,
                        description: __field2,
                        external_docs: __field3,
                        operation_id: __field4,
                        parameters: __field5,
                        request_body: __field6,
                        responses: __field7,
                        callbacks: __field8,
                        deprecated: __field9,
                        security: __field10,
                        servers: __field11,
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
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<ExternalDocumentation>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<MaybeRef<Parameter>>>,
                    > = ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<
                        ::core::option::Option<MaybeRef<RequestBody>>,
                    > = ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<::core::option::Option<Responses>> =
                        ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Callback>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field9: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field10: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<SecurityRequirement>>,
                    > = ::core::option::Option::None;
                    let mut __field11: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<Server>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("tags"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "summary",
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
                                            "description",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "externalDocs",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<ExternalDocumentation>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "operationId",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "parameters",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<MaybeRef<Parameter>>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "requestBody",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<MaybeRef<RequestBody>>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "responses",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Responses>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "callbacks",
                                        ),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Callback>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier9 => {
                                if ::core::option::Option::is_some(&__field9) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "deprecated",
                                        ),
                                    );
                                }
                                __field9 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier10 => {
                                if ::core::option::Option::is_some(&__field10) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "security",
                                        ),
                                    );
                                }
                                __field10 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<SecurityRequirement>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier11 => {
                                if ::core::option::Option::is_some(&__field11) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "servers",
                                        ),
                                    );
                                }
                                __field11 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::vec::Vec<Server>>,
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
                    ::core::result::Result::Ok(Operation {
                        tags: __field0,
                        summary: __field1,
                        description: __field2,
                        external_docs: __field3,
                        operation_id: __field4,
                        parameters: __field5,
                        request_body: __field6,
                        responses: __field7,
                        callbacks: __field8,
                        deprecated: __field9,
                        security: __field10,
                        servers: __field11,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "tags",
                "summary",
                "description",
                "externalDocs",
                "operationId",
                "parameters",
                "requestBody",
                "responses",
                "callbacks",
                "deprecated",
                "security",
                "servers",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Operation",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An [External Documentation Object](https://spec.openapis.org/oas/v3.1.0#external-documentation-object) (4.8.11).\n"]
    #[derive(Clone, Debug)]
    pub struct ExternalDocumentation {
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub url: Url,
    }
    impl ExternalDocumentation {
        #[doc = "Creates a new [`ExternalDocumentation`]."]
        pub fn new(url: Url) -> Self {
            Self {
                url,
                description: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(&mut self, url: Url) -> &mut Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: Url) -> Self {
            self.url = url;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ExternalDocumentation {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "ExternalDocumentation",
                2usize,
            )?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_field("url", &self.url)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ExternalDocumentation {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["description", "url"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"description\", \"url\"]";
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
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "url" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"url" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                type Value = ExternalDocumentation;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ExternalDocumentation")
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
                        ::core::option::Option<Markdown>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<Url>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ExternalDocumentation {
                        description: __field0,
                        url: __field1,
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
                    let mut __field0: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<Url> = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("url"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Url>(&mut __map)?,
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
                                <__A::Error as __serde::de::Error>::missing_field("url"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ExternalDocumentation {
                        description: __field0,
                        url: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["description", "url"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ExternalDocumentation",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum Any {
        #[doc = ""]
        Integer(i64),
        #[doc = ""]
        Float(f64),
        #[doc = ""]
        String(::std::string::String),
        #[doc = ""]
        Bool(bool),
        #[doc = ""]
        Array(::std::vec::Vec<Any>),
        #[doc = ""]
        Object(indexmap::IndexMap<::std::string::String, Any>),
    }
    #[automatically_derived]
    impl __serde::Serialize for Any {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "Any");
            match self {
                Self::Integer(__value) => {
                    __serializer.serialize_implicitly_tagged("Integer", 0u32, __value)
                }
                Self::Float(__value) => {
                    __serializer.serialize_implicitly_tagged("Float", 1u32, __value)
                }
                Self::String(__value) => {
                    __serializer.serialize_implicitly_tagged("String", 2u32, __value)
                }
                Self::Bool(__value) => {
                    __serializer.serialize_implicitly_tagged("Bool", 3u32, __value)
                }
                Self::Array(__value) => {
                    __serializer.serialize_implicitly_tagged("Array", 4u32, __value)
                }
                Self::Object(__value) => {
                    __serializer.serialize_implicitly_tagged("Object", 5u32, __value)
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
                &["Integer", "Float", "String", "Bool", "Array", "Object"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"Integer\", \"Float\", \"String\", \"Bool\", \"Array\", \"Object\"]" ;
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
                        "Integer" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Float" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "String" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "Bool" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "Array" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "Object" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                        b"String" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"Bool" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"Array" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"Object" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                &["Integer", "Float", "String", "Bool", "Array", "Object"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<i64, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Any::Integer(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<f64, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Any::Float(__value)),
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
                match __sidex_serde::de::content::deserialize_content_ref::<bool, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(Any::Bool(__value)),
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
                match __sidex_serde::de::content::deserialize_content_ref::<
                    indexmap::IndexMap<::std::string::String, Any>,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(Any::Object(__value)),
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
                                    __serde::de::VariantAccess::newtype_variant::<i64>(__variant)?;
                                ::core::result::Result::Ok(Any::Integer(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<f64>(__variant)?;
                                ::core::result::Result::Ok(Any::Float(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(Any::String(__value))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<bool>(__variant)?;
                                ::core::result::Result::Ok(Any::Bool(__value))
                            }
                            (__Identifier::__Identifier4, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::vec::Vec<Any>,
                                >(__variant)?;
                                ::core::result::Result::Ok(Any::Array(__value))
                            }
                            (__Identifier::__Identifier5, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    indexmap::IndexMap<::std::string::String, Any>,
                                >(__variant)?;
                                ::core::result::Result::Ok(Any::Object(__value))
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
    #[doc = "A [Parameter Object](https://spec.openapis.org/oas/v3.1.0#parameter-object) (4.8.12).\n"]
    #[derive(Clone, Debug)]
    pub struct Parameter {
        #[doc = ""]
        pub name: ::std::string::String,
        #[doc = ""]
        pub location: ParameterLocation,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub required: ::std::option::Option<bool>,
        #[doc = ""]
        pub deprecated: ::std::option::Option<bool>,
        #[doc = ""]
        pub allow_empty_value: ::std::option::Option<bool>,
        #[doc = ""]
        pub style: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub explode: ::std::option::Option<bool>,
        #[doc = ""]
        pub allow_reserved: ::std::option::Option<bool>,
        #[doc = ""]
        pub schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        #[doc = ""]
        pub example: ::std::option::Option<Any>,
        #[doc = ""]
        pub examples:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Example>>>,
        #[doc = ""]
        pub content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
    }
    impl Parameter {
        #[doc = "Creates a new [`Parameter`]."]
        pub fn new(name: ::std::string::String, location: ParameterLocation) -> Self {
            Self {
                name,
                location,
                description: ::std::default::Default::default(),
                required: ::std::default::Default::default(),
                deprecated: ::std::default::Default::default(),
                allow_empty_value: ::std::default::Default::default(),
                style: ::std::default::Default::default(),
                explode: ::std::default::Default::default(),
                allow_reserved: ::std::default::Default::default(),
                schema: ::std::default::Default::default(),
                example: ::std::default::Default::default(),
                examples: ::std::default::Default::default(),
                content: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `location`."]
        pub fn set_location(&mut self, location: ParameterLocation) -> &mut Self {
            self.location = location;
            self
        }
        #[doc = "Sets the value of `location`."]
        pub fn with_location(mut self, location: ParameterLocation) -> Self {
            self.location = location;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn set_required(&mut self, required: ::std::option::Option<bool>) -> &mut Self {
            self.required = required;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn with_required(mut self, required: ::std::option::Option<bool>) -> Self {
            self.required = required;
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
        #[doc = "Sets the value of `allow_empty_value`."]
        pub fn set_allow_empty_value(
            &mut self,
            allow_empty_value: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.allow_empty_value = allow_empty_value;
            self
        }
        #[doc = "Sets the value of `allow_empty_value`."]
        pub fn with_allow_empty_value(
            mut self,
            allow_empty_value: ::std::option::Option<bool>,
        ) -> Self {
            self.allow_empty_value = allow_empty_value;
            self
        }
        #[doc = "Sets the value of `style`."]
        pub fn set_style(
            &mut self,
            style: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.style = style;
            self
        }
        #[doc = "Sets the value of `style`."]
        pub fn with_style(mut self, style: ::std::option::Option<::std::string::String>) -> Self {
            self.style = style;
            self
        }
        #[doc = "Sets the value of `explode`."]
        pub fn set_explode(&mut self, explode: ::std::option::Option<bool>) -> &mut Self {
            self.explode = explode;
            self
        }
        #[doc = "Sets the value of `explode`."]
        pub fn with_explode(mut self, explode: ::std::option::Option<bool>) -> Self {
            self.explode = explode;
            self
        }
        #[doc = "Sets the value of `allow_reserved`."]
        pub fn set_allow_reserved(
            &mut self,
            allow_reserved: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.allow_reserved = allow_reserved;
            self
        }
        #[doc = "Sets the value of `allow_reserved`."]
        pub fn with_allow_reserved(mut self, allow_reserved: ::std::option::Option<bool>) -> Self {
            self.allow_reserved = allow_reserved;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn set_schema(
            &mut self,
            schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        ) -> &mut Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn with_schema(
            mut self,
            schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        ) -> Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `example`."]
        pub fn set_example(&mut self, example: ::std::option::Option<Any>) -> &mut Self {
            self.example = example;
            self
        }
        #[doc = "Sets the value of `example`."]
        pub fn with_example(mut self, example: ::std::option::Option<Any>) -> Self {
            self.example = example;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn set_examples(
            &mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> &mut Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn with_examples(
            mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn set_content(
            &mut self,
            content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
        ) -> &mut Self {
            self.content = content;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn with_content(
            mut self,
            content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
        ) -> Self {
            self.content = content;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Parameter {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Parameter", 13usize)?;
            __record.serialize_field("name", &self.name)?;
            __record.serialize_field("in", &self.location)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "required",
                ::core::option::Option::as_ref(&self.required),
            )?;
            __record.serialize_optional_field(
                "deprecated",
                ::core::option::Option::as_ref(&self.deprecated),
            )?;
            __record.serialize_optional_field(
                "allowEmptyValue",
                ::core::option::Option::as_ref(&self.allow_empty_value),
            )?;
            __record
                .serialize_optional_field("style", ::core::option::Option::as_ref(&self.style))?;
            __record.serialize_optional_field(
                "explode",
                ::core::option::Option::as_ref(&self.explode),
            )?;
            __record.serialize_optional_field(
                "allowReserved",
                ::core::option::Option::as_ref(&self.allow_reserved),
            )?;
            __record
                .serialize_optional_field("schema", ::core::option::Option::as_ref(&self.schema))?;
            __record.serialize_optional_field(
                "example",
                ::core::option::Option::as_ref(&self.example),
            )?;
            __record.serialize_optional_field(
                "examples",
                ::core::option::Option::as_ref(&self.examples),
            )?;
            __record.serialize_optional_field(
                "content",
                ::core::option::Option::as_ref(&self.content),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Parameter {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "name",
                "in",
                "description",
                "required",
                "deprecated",
                "allowEmptyValue",
                "style",
                "explode",
                "allowReserved",
                "schema",
                "example",
                "examples",
                "content",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"name\", \"in\", \"description\", \"required\", \"deprecated\", \"allowEmptyValue\", \"style\", \"explode\", \"allowReserved\", \"schema\", \"example\", \"examples\", \"content\"]" ;
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "in" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "required" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "allowEmptyValue" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier5)
                        }
                        "style" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "explode" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "allowReserved" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        "schema" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        "example" => ::core::result::Result::Ok(__Identifier::__Identifier10),
                        "examples" => ::core::result::Result::Ok(__Identifier::__Identifier11),
                        "content" => ::core::result::Result::Ok(__Identifier::__Identifier12),
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
                        b"in" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"required" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"allowEmptyValue" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier5)
                        }
                        b"style" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"explode" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"allowReserved" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        b"schema" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        b"example" => ::core::result::Result::Ok(__Identifier::__Identifier10),
                        b"examples" => ::core::result::Result::Ok(__Identifier::__Identifier11),
                        b"content" => ::core::result::Result::Ok(__Identifier::__Identifier12),
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
                type Value = Parameter;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Parameter")
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
                                __serde::de::Error::invalid_length(
                                    0usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<ParameterLocation>(
                        &mut __seq,
                    )? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    1usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    2usize,
                                    &"record with 13 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    3usize,
                                    &"record with 13 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    4usize,
                                    &"record with 13 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    5usize,
                                    &"record with 13 fields",
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
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    7usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    8usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field9 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    9usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field10 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Any>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    10usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field11 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    11usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    let __field12 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MediaType>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    12usize,
                                    &"record with 13 fields",
                                ),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Parameter {
                        name: __field0,
                        location: __field1,
                        description: __field2,
                        required: __field3,
                        deprecated: __field4,
                        allow_empty_value: __field5,
                        style: __field6,
                        explode: __field7,
                        allow_reserved: __field8,
                        schema: __field9,
                        example: __field10,
                        examples: __field11,
                        content: __field12,
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
                    let mut __field1: ::core::option::Option<ParameterLocation> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field9: ::core::option::Option<
                        ::core::option::Option<
                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field10: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field11: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field12: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MediaType>,
                        >,
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("in"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<ParameterLocation>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "required",
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
                                            "deprecated",
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
                                            "allowEmptyValue",
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
                                            "style",
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
                                            "explode",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "allowReserved",
                                        ),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier9 => {
                                if ::core::option::Option::is_some(&__field9) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schema",
                                        ),
                                    );
                                }
                                __field9 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier10 => {
                                if ::core::option::Option::is_some(&__field10) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "example",
                                        ),
                                    );
                                }
                                __field10 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Any>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier11 => {
                                if ::core::option::Option::is_some(&__field11) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "examples",
                                        ),
                                    );
                                }
                                __field11 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Example>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier12 => {
                                if ::core::option::Option::is_some(&__field12) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "content",
                                        ),
                                    );
                                }
                                __field12 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, MediaType>,
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
                                <__A::Error as __serde::de::Error>::missing_field("in"),
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
                    ::core::result::Result::Ok(Parameter {
                        name: __field0,
                        location: __field1,
                        description: __field2,
                        required: __field3,
                        deprecated: __field4,
                        allow_empty_value: __field5,
                        style: __field6,
                        explode: __field7,
                        allow_reserved: __field8,
                        schema: __field9,
                        example: __field10,
                        examples: __field11,
                        content: __field12,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "name",
                "in",
                "description",
                "required",
                "deprecated",
                "allowEmptyValue",
                "style",
                "explode",
                "allowReserved",
                "schema",
                "example",
                "examples",
                "content",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Parameter",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Location of a [Parameter Object](https://spec.openapis.org/oas/v3.1.0#parameter-object) (4.8.12).\n"]
    #[derive(Clone, Debug)]
    pub enum ParameterLocation {
        #[doc = ""]
        Query,
        #[doc = ""]
        Header,
        #[doc = ""]
        Path,
        #[doc = ""]
        Cookie,
    }
    #[automatically_derived]
    impl __serde::Serialize for ParameterLocation {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "ParameterLocation");
            match self {
                Self::Query => __serializer.serialize_tag("query", 0u32),
                Self::Header => __serializer.serialize_tag("header", 1u32),
                Self::Path => __serializer.serialize_tag("path", 2u32),
                Self::Cookie => __serializer.serialize_tag("cookie", 3u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ParameterLocation {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["query", "header", "path", "cookie"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"query\", \"header\", \"path\", \"cookie\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
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
                        "query" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "header" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "path" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "cookie" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"query" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"header" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"path" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"cookie" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
            const __VARIANTS: &'static [&'static str] = &["query", "header", "path", "cookie"];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ParameterLocation;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum ParameterLocation")
                }
                #[inline]
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    let __identifier = __IdentifierVisitor.visit_str(__value)?;
                    #[allow(unreachable_patterns)]
                    match __identifier {
                        __Identifier::__Identifier0 => {
                            ::core::result::Result::Ok(ParameterLocation::Query)
                        }
                        __Identifier::__Identifier1 => {
                            ::core::result::Result::Ok(ParameterLocation::Header)
                        }
                        __Identifier::__Identifier2 => {
                            ::core::result::Result::Ok(ParameterLocation::Path)
                        }
                        __Identifier::__Identifier3 => {
                            ::core::result::Result::Ok(ParameterLocation::Cookie)
                        }
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
                            ::core::result::Result::Ok(ParameterLocation::Query)
                        }
                        (__Identifier::__Identifier1, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(ParameterLocation::Header)
                        }
                        (__Identifier::__Identifier2, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(ParameterLocation::Path)
                        }
                        (__Identifier::__Identifier3, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(ParameterLocation::Cookie)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "ParameterLocation",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Request Body Object](https://spec.openapis.org/oas/v3.1.0#request-body-object) (4.8.13).\n"]
    #[derive(Clone, Debug)]
    pub struct RequestBody {
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub content: indexmap::IndexMap<::std::string::String, MediaType>,
        #[doc = ""]
        pub required: ::std::option::Option<bool>,
    }
    impl RequestBody {
        #[doc = "Creates a new [`RequestBody`]."]
        pub fn new(content: indexmap::IndexMap<::std::string::String, MediaType>) -> Self {
            Self {
                content,
                description: ::std::default::Default::default(),
                required: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn set_content(
            &mut self,
            content: indexmap::IndexMap<::std::string::String, MediaType>,
        ) -> &mut Self {
            self.content = content;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn with_content(
            mut self,
            content: indexmap::IndexMap<::std::string::String, MediaType>,
        ) -> Self {
            self.content = content;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn set_required(&mut self, required: ::std::option::Option<bool>) -> &mut Self {
            self.required = required;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn with_required(mut self, required: ::std::option::Option<bool>) -> Self {
            self.required = required;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for RequestBody {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "RequestBody", 3usize)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_field("content", &self.content)?;
            __record.serialize_optional_field(
                "required",
                ::core::option::Option::as_ref(&self.required),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RequestBody {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["description", "content", "required"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"description\", \"content\", \"required\"]";
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
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "content" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "required" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"content" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"required" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                type Value = RequestBody;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record RequestBody")
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
                        ::core::option::Option<Markdown>,
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
                        indexmap::IndexMap<::std::string::String, MediaType>,
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
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(RequestBody {
                        description: __field0,
                        content: __field1,
                        required: __field2,
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
                    let mut __field0: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        indexmap::IndexMap<::std::string::String, MediaType>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "content",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        indexmap::IndexMap<::std::string::String, MediaType>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "required",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
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
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("content"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(RequestBody {
                        description: __field0,
                        content: __field1,
                        required: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["description", "content", "required"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "RequestBody",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Media Type Object](https://spec.openapis.org/oas/v3.1.0#media-type-object) (4.8.14).\n"]
    #[derive(Clone, Debug)]
    pub struct MediaType {
        #[doc = ""]
        pub schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        #[doc = ""]
        pub example: ::std::option::Option<Any>,
        #[doc = ""]
        pub examples:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Example>>>,
        #[doc = ""]
        pub encoding: ::std::option::Option<indexmap::IndexMap<::std::string::String, Encoding>>,
    }
    impl MediaType {
        #[doc = "Creates a new [`MediaType`]."]
        pub fn new() -> Self {
            Self {
                schema: ::std::default::Default::default(),
                example: ::std::default::Default::default(),
                examples: ::std::default::Default::default(),
                encoding: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `schema`."]
        pub fn set_schema(
            &mut self,
            schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        ) -> &mut Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn with_schema(
            mut self,
            schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        ) -> Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `example`."]
        pub fn set_example(&mut self, example: ::std::option::Option<Any>) -> &mut Self {
            self.example = example;
            self
        }
        #[doc = "Sets the value of `example`."]
        pub fn with_example(mut self, example: ::std::option::Option<Any>) -> Self {
            self.example = example;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn set_examples(
            &mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> &mut Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn with_examples(
            mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `encoding`."]
        pub fn set_encoding(
            &mut self,
            encoding: ::std::option::Option<indexmap::IndexMap<::std::string::String, Encoding>>,
        ) -> &mut Self {
            self.encoding = encoding;
            self
        }
        #[doc = "Sets the value of `encoding`."]
        pub fn with_encoding(
            mut self,
            encoding: ::std::option::Option<indexmap::IndexMap<::std::string::String, Encoding>>,
        ) -> Self {
            self.encoding = encoding;
            self
        }
    }
    impl ::std::default::Default for MediaType {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for MediaType {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "MediaType", 4usize)?;
            __record
                .serialize_optional_field("schema", ::core::option::Option::as_ref(&self.schema))?;
            __record.serialize_optional_field(
                "example",
                ::core::option::Option::as_ref(&self.example),
            )?;
            __record.serialize_optional_field(
                "examples",
                ::core::option::Option::as_ref(&self.examples),
            )?;
            __record.serialize_optional_field(
                "encoding",
                ::core::option::Option::as_ref(&self.encoding),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for MediaType {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["schema", "example", "examples", "encoding"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"schema\", \"example\", \"examples\", \"encoding\"]";
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
                        "schema" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "example" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "examples" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "encoding" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"schema" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"example" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"examples" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"encoding" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                type Value = MediaType;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record MediaType")
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
                        ::core::option::Option<
                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                        >,
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
                        ::core::option::Option<Any>,
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
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
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
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Encoding>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(MediaType {
                        schema: __field0,
                        example: __field1,
                        examples: __field2,
                        encoding: __field3,
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
                        ::core::option::Option<
                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Encoding>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schema",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "example",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Any>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "examples",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Example>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "encoding",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, Encoding>,
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
                    ::core::result::Result::Ok(MediaType {
                        schema: __field0,
                        example: __field1,
                        examples: __field2,
                        encoding: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["schema", "example", "examples", "encoding"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "MediaType",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An [Encoding Object](https://spec.openapis.org/oas/v3.1.0#encoding-object) (4.8.15).\n"]
    #[derive(Clone, Debug)]
    pub struct Encoding {
        #[doc = ""]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub headers:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Header>>>,
        #[doc = ""]
        pub style: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub explode: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub allow_reserved: ::std::option::Option<bool>,
    }
    impl Encoding {
        #[doc = "Creates a new [`Encoding`]."]
        pub fn new() -> Self {
            Self {
                content_type: ::std::default::Default::default(),
                headers: ::std::default::Default::default(),
                style: ::std::default::Default::default(),
                explode: ::std::default::Default::default(),
                allow_reserved: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `content_type`."]
        pub fn set_content_type(
            &mut self,
            content_type: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.content_type = content_type;
            self
        }
        #[doc = "Sets the value of `content_type`."]
        pub fn with_content_type(
            mut self,
            content_type: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.content_type = content_type;
            self
        }
        #[doc = "Sets the value of `headers`."]
        pub fn set_headers(
            &mut self,
            headers: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
            >,
        ) -> &mut Self {
            self.headers = headers;
            self
        }
        #[doc = "Sets the value of `headers`."]
        pub fn with_headers(
            mut self,
            headers: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
            >,
        ) -> Self {
            self.headers = headers;
            self
        }
        #[doc = "Sets the value of `style`."]
        pub fn set_style(
            &mut self,
            style: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.style = style;
            self
        }
        #[doc = "Sets the value of `style`."]
        pub fn with_style(mut self, style: ::std::option::Option<::std::string::String>) -> Self {
            self.style = style;
            self
        }
        #[doc = "Sets the value of `explode`."]
        pub fn set_explode(
            &mut self,
            explode: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.explode = explode;
            self
        }
        #[doc = "Sets the value of `explode`."]
        pub fn with_explode(
            mut self,
            explode: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.explode = explode;
            self
        }
        #[doc = "Sets the value of `allow_reserved`."]
        pub fn set_allow_reserved(
            &mut self,
            allow_reserved: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.allow_reserved = allow_reserved;
            self
        }
        #[doc = "Sets the value of `allow_reserved`."]
        pub fn with_allow_reserved(mut self, allow_reserved: ::std::option::Option<bool>) -> Self {
            self.allow_reserved = allow_reserved;
            self
        }
    }
    impl ::std::default::Default for Encoding {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Encoding {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Encoding", 5usize)?;
            __record.serialize_optional_field(
                "contentType",
                ::core::option::Option::as_ref(&self.content_type),
            )?;
            __record.serialize_optional_field(
                "headers",
                ::core::option::Option::as_ref(&self.headers),
            )?;
            __record
                .serialize_optional_field("style", ::core::option::Option::as_ref(&self.style))?;
            __record.serialize_optional_field(
                "explode",
                ::core::option::Option::as_ref(&self.explode),
            )?;
            __record.serialize_optional_field(
                "allowReserved",
                ::core::option::Option::as_ref(&self.allow_reserved),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Encoding {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "contentType",
                "headers",
                "style",
                "explode",
                "allowReserved",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"contentType\", \"headers\", \"style\", \"explode\", \"allowReserved\"]" ;
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
                        "contentType" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "headers" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "style" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "explode" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "allowReserved" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                        b"contentType" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"headers" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"style" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"explode" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"allowReserved" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                type Value = Encoding;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Encoding")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
                        >,
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
                        ::core::option::Option<::std::string::String>,
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
                        ::core::option::Option<::std::string::String>,
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
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Encoding {
                        content_type: __field0,
                        headers: __field1,
                        style: __field2,
                        explode: __field3,
                        allow_reserved: __field4,
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
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "contentType",
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
                                            "headers",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Header>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "style",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "explode",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "allowReserved",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
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
                    ::core::result::Result::Ok(Encoding {
                        content_type: __field0,
                        headers: __field1,
                        style: __field2,
                        explode: __field3,
                        allow_reserved: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "contentType",
                "headers",
                "style",
                "explode",
                "allowReserved",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Encoding",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Responses Object](https://spec.openapis.org/oas/v3.1.0#responses-object) (4.8.16).\n"]
    #[derive(Clone, Debug)]
    pub struct Responses(pub(crate) indexmap::IndexMap<::std::string::String, MaybeRef<Response>>);
    impl ::std::convert::From<Responses>
        for indexmap::IndexMap<::std::string::String, MaybeRef<Response>>
    {
        fn from(wrapped: Responses) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Responses {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Responses {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Responses(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = "A [Response Object](https://spec.openapis.org/oas/v3.1.0#response-object) (4.8.17).\n"]
    #[derive(Clone, Debug)]
    pub struct Response {
        #[doc = ""]
        pub description: Markdown,
        #[doc = ""]
        pub headers:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Header>>>,
        #[doc = ""]
        pub content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
        #[doc = ""]
        pub links: ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Link>>>,
    }
    impl Response {
        #[doc = "Creates a new [`Response`]."]
        pub fn new(description: Markdown) -> Self {
            Self {
                description,
                headers: ::std::default::Default::default(),
                content: ::std::default::Default::default(),
                links: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(&mut self, description: Markdown) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: Markdown) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `headers`."]
        pub fn set_headers(
            &mut self,
            headers: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
            >,
        ) -> &mut Self {
            self.headers = headers;
            self
        }
        #[doc = "Sets the value of `headers`."]
        pub fn with_headers(
            mut self,
            headers: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
            >,
        ) -> Self {
            self.headers = headers;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn set_content(
            &mut self,
            content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
        ) -> &mut Self {
            self.content = content;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn with_content(
            mut self,
            content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
        ) -> Self {
            self.content = content;
            self
        }
        #[doc = "Sets the value of `links`."]
        pub fn set_links(
            &mut self,
            links: ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Link>>>,
        ) -> &mut Self {
            self.links = links;
            self
        }
        #[doc = "Sets the value of `links`."]
        pub fn with_links(
            mut self,
            links: ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Link>>>,
        ) -> Self {
            self.links = links;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Response {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Response", 4usize)?;
            __record.serialize_field("description", &self.description)?;
            __record.serialize_optional_field(
                "headers",
                ::core::option::Option::as_ref(&self.headers),
            )?;
            __record.serialize_optional_field(
                "content",
                ::core::option::Option::as_ref(&self.content),
            )?;
            __record
                .serialize_optional_field("links", ::core::option::Option::as_ref(&self.links))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Response {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["description", "headers", "content", "links"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"description\", \"headers\", \"content\", \"links\"]";
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
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "headers" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "content" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "links" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"headers" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"content" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"links" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                type Value = Response;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Response")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<Markdown>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 4 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
                        >,
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
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MediaType>,
                        >,
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
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Link>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Response {
                        description: __field0,
                        headers: __field1,
                        content: __field2,
                        links: __field3,
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
                    let mut __field0: ::core::option::Option<Markdown> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Header>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MediaType>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Link>>,
                        >,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Markdown>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "headers",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Header>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "content",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, MediaType>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "links",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Link>,
                                            >,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("description"),
                            );
                        }
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
                    ::core::result::Result::Ok(Response {
                        description: __field0,
                        headers: __field1,
                        content: __field2,
                        links: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["description", "headers", "content", "links"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Response",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Callback Object](https://spec.openapis.org/oas/v3.1.0#callback-object) (4.8.18).\n"]
    #[derive(Clone, Debug)]
    pub struct Callback(pub(crate) indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>);
    impl ::std::convert::From<Callback>
        for indexmap::IndexMap<::std::string::String, MaybeRef<PathItem>>
    {
        fn from(wrapped: Callback) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Callback {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Callback {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Callback(__serde::Deserialize::deserialize(__deserializer)?))
        }
    }
    #[doc = "A [Example Object](https://spec.openapis.org/oas/v3.1.0#example-object) (4.8.19).\n"]
    #[derive(Clone, Debug)]
    pub struct Example {
        #[doc = ""]
        pub summary: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub value: ::std::option::Option<Any>,
        #[doc = ""]
        pub external_value: ::std::option::Option<::std::string::String>,
    }
    impl Example {
        #[doc = "Creates a new [`Example`]."]
        pub fn new() -> Self {
            Self {
                summary: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                value: ::std::default::Default::default(),
                external_value: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `summary`."]
        pub fn set_summary(
            &mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn with_summary(
            mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn set_value(&mut self, value: ::std::option::Option<Any>) -> &mut Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn with_value(mut self, value: ::std::option::Option<Any>) -> Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `external_value`."]
        pub fn set_external_value(
            &mut self,
            external_value: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.external_value = external_value;
            self
        }
        #[doc = "Sets the value of `external_value`."]
        pub fn with_external_value(
            mut self,
            external_value: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.external_value = external_value;
            self
        }
    }
    impl ::std::default::Default for Example {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Example {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Example", 4usize)?;
            __record.serialize_optional_field(
                "summary",
                ::core::option::Option::as_ref(&self.summary),
            )?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record
                .serialize_optional_field("value", ::core::option::Option::as_ref(&self.value))?;
            __record.serialize_optional_field(
                "externalValue",
                ::core::option::Option::as_ref(&self.external_value),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Example {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["summary", "description", "value", "externalValue"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"summary\", \"description\", \"value\", \"externalValue\"]";
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
                        "summary" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "value" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "externalValue" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"summary" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"value" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"externalValue" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                type Value = Example;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Example")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
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
                        ::core::option::Option<Any>,
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
                        ::core::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Example {
                        summary: __field0,
                        description: __field1,
                        value: __field2,
                        external_value: __field3,
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
                    let mut __field1: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "summary",
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
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "value",
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
                                            "externalValue",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
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
                    ::core::result::Result::Ok(Example {
                        summary: __field0,
                        description: __field1,
                        value: __field2,
                        external_value: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["summary", "description", "value", "externalValue"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Example",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Link Object](https://spec.openapis.org/oas/v3.1.0#link-object) (4.8.20).\n"]
    #[derive(Clone, Debug)]
    pub struct Link {
        #[doc = ""]
        pub operation_ref: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub parameters: ::std::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
        #[doc = ""]
        pub request_body: ::std::option::Option<Any>,
        #[doc = ""]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub server: ::std::option::Option<Server>,
    }
    impl Link {
        #[doc = "Creates a new [`Link`]."]
        pub fn new() -> Self {
            Self {
                operation_ref: ::std::default::Default::default(),
                operation_id: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
                request_body: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                server: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `operation_ref`."]
        pub fn set_operation_ref(
            &mut self,
            operation_ref: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.operation_ref = operation_ref;
            self
        }
        #[doc = "Sets the value of `operation_ref`."]
        pub fn with_operation_ref(
            mut self,
            operation_ref: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.operation_ref = operation_ref;
            self
        }
        #[doc = "Sets the value of `operation_id`."]
        pub fn set_operation_id(
            &mut self,
            operation_id: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.operation_id = operation_id;
            self
        }
        #[doc = "Sets the value of `operation_id`."]
        pub fn with_operation_id(
            mut self,
            operation_id: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.operation_id = operation_id;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
        ) -> Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `request_body`."]
        pub fn set_request_body(&mut self, request_body: ::std::option::Option<Any>) -> &mut Self {
            self.request_body = request_body;
            self
        }
        #[doc = "Sets the value of `request_body`."]
        pub fn with_request_body(mut self, request_body: ::std::option::Option<Any>) -> Self {
            self.request_body = request_body;
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
        #[doc = "Sets the value of `server`."]
        pub fn set_server(&mut self, server: ::std::option::Option<Server>) -> &mut Self {
            self.server = server;
            self
        }
        #[doc = "Sets the value of `server`."]
        pub fn with_server(mut self, server: ::std::option::Option<Server>) -> Self {
            self.server = server;
            self
        }
    }
    impl ::std::default::Default for Link {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Link {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Link", 6usize)?;
            __record.serialize_optional_field(
                "operationRef",
                ::core::option::Option::as_ref(&self.operation_ref),
            )?;
            __record.serialize_optional_field(
                "operationId",
                ::core::option::Option::as_ref(&self.operation_id),
            )?;
            __record.serialize_optional_field(
                "parameters",
                ::core::option::Option::as_ref(&self.parameters),
            )?;
            __record.serialize_optional_field(
                "requestBody",
                ::core::option::Option::as_ref(&self.request_body),
            )?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record
                .serialize_optional_field("server", ::core::option::Option::as_ref(&self.server))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Link {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "operationRef",
                "operationId",
                "parameters",
                "requestBody",
                "description",
                "server",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"operationRef\", \"operationId\", \"parameters\", \"requestBody\", \"description\", \"server\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "operationRef" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "operationId" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "parameters" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "requestBody" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "server" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                        b"operationRef" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"operationId" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"parameters" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"requestBody" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"server" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                type Value = Link;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Link")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 6 fields"),
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
                                __serde::de::Error::invalid_length(1usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 6 fields"),
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
                                __serde::de::Error::invalid_length(3usize, &"record with 6 fields"),
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
                                __serde::de::Error::invalid_length(4usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Server>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 6 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Link {
                        operation_ref: __field0,
                        operation_id: __field1,
                        parameters: __field2,
                        request_body: __field3,
                        description: __field4,
                        server: __field5,
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
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<indexmap::IndexMap<::std::string::String, Any>>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<Server>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "operationRef",
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
                                            "operationId",
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
                                            "parameters",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, Any>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "requestBody",
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
                                            "description",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "server",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Server>,
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
                    ::core::result::Result::Ok(Link {
                        operation_ref: __field0,
                        operation_id: __field1,
                        parameters: __field2,
                        request_body: __field3,
                        description: __field4,
                        server: __field5,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "operationRef",
                "operationId",
                "parameters",
                "requestBody",
                "description",
                "server",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Link",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Header Object](https://spec.openapis.org/oas/v3.1.0#header-object) (4.8.21).\n"]
    #[derive(Clone, Debug)]
    pub struct Header {
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub required: ::std::option::Option<bool>,
        #[doc = ""]
        pub deprecated: ::std::option::Option<bool>,
        #[doc = ""]
        pub allow_empty_value: ::std::option::Option<bool>,
        #[doc = ""]
        pub style: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub explode: ::std::option::Option<bool>,
        #[doc = ""]
        pub allow_reserved: ::std::option::Option<bool>,
        #[doc = ""]
        pub schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        #[doc = ""]
        pub example: ::std::option::Option<Any>,
        #[doc = ""]
        pub examples:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, MaybeRef<Example>>>,
        #[doc = ""]
        pub content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
    }
    impl Header {
        #[doc = "Creates a new [`Header`]."]
        pub fn new() -> Self {
            Self {
                description: ::std::default::Default::default(),
                required: ::std::default::Default::default(),
                deprecated: ::std::default::Default::default(),
                allow_empty_value: ::std::default::Default::default(),
                style: ::std::default::Default::default(),
                explode: ::std::default::Default::default(),
                allow_reserved: ::std::default::Default::default(),
                schema: ::std::default::Default::default(),
                example: ::std::default::Default::default(),
                examples: ::std::default::Default::default(),
                content: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn set_required(&mut self, required: ::std::option::Option<bool>) -> &mut Self {
            self.required = required;
            self
        }
        #[doc = "Sets the value of `required`."]
        pub fn with_required(mut self, required: ::std::option::Option<bool>) -> Self {
            self.required = required;
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
        #[doc = "Sets the value of `allow_empty_value`."]
        pub fn set_allow_empty_value(
            &mut self,
            allow_empty_value: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.allow_empty_value = allow_empty_value;
            self
        }
        #[doc = "Sets the value of `allow_empty_value`."]
        pub fn with_allow_empty_value(
            mut self,
            allow_empty_value: ::std::option::Option<bool>,
        ) -> Self {
            self.allow_empty_value = allow_empty_value;
            self
        }
        #[doc = "Sets the value of `style`."]
        pub fn set_style(
            &mut self,
            style: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.style = style;
            self
        }
        #[doc = "Sets the value of `style`."]
        pub fn with_style(mut self, style: ::std::option::Option<::std::string::String>) -> Self {
            self.style = style;
            self
        }
        #[doc = "Sets the value of `explode`."]
        pub fn set_explode(&mut self, explode: ::std::option::Option<bool>) -> &mut Self {
            self.explode = explode;
            self
        }
        #[doc = "Sets the value of `explode`."]
        pub fn with_explode(mut self, explode: ::std::option::Option<bool>) -> Self {
            self.explode = explode;
            self
        }
        #[doc = "Sets the value of `allow_reserved`."]
        pub fn set_allow_reserved(
            &mut self,
            allow_reserved: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.allow_reserved = allow_reserved;
            self
        }
        #[doc = "Sets the value of `allow_reserved`."]
        pub fn with_allow_reserved(mut self, allow_reserved: ::std::option::Option<bool>) -> Self {
            self.allow_reserved = allow_reserved;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn set_schema(
            &mut self,
            schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        ) -> &mut Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn with_schema(
            mut self,
            schema: ::std::option::Option<::sidex_gen_json_schema::types::schema::SchemaObject>,
        ) -> Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `example`."]
        pub fn set_example(&mut self, example: ::std::option::Option<Any>) -> &mut Self {
            self.example = example;
            self
        }
        #[doc = "Sets the value of `example`."]
        pub fn with_example(mut self, example: ::std::option::Option<Any>) -> Self {
            self.example = example;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn set_examples(
            &mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> &mut Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `examples`."]
        pub fn with_examples(
            mut self,
            examples: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
            >,
        ) -> Self {
            self.examples = examples;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn set_content(
            &mut self,
            content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
        ) -> &mut Self {
            self.content = content;
            self
        }
        #[doc = "Sets the value of `content`."]
        pub fn with_content(
            mut self,
            content: ::std::option::Option<indexmap::IndexMap<::std::string::String, MediaType>>,
        ) -> Self {
            self.content = content;
            self
        }
    }
    impl ::std::default::Default for Header {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Header {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Header", 11usize)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "required",
                ::core::option::Option::as_ref(&self.required),
            )?;
            __record.serialize_optional_field(
                "deprecated",
                ::core::option::Option::as_ref(&self.deprecated),
            )?;
            __record.serialize_optional_field(
                "allowEmptyValue",
                ::core::option::Option::as_ref(&self.allow_empty_value),
            )?;
            __record
                .serialize_optional_field("style", ::core::option::Option::as_ref(&self.style))?;
            __record.serialize_optional_field(
                "explode",
                ::core::option::Option::as_ref(&self.explode),
            )?;
            __record.serialize_optional_field(
                "allowReserved",
                ::core::option::Option::as_ref(&self.allow_reserved),
            )?;
            __record
                .serialize_optional_field("schema", ::core::option::Option::as_ref(&self.schema))?;
            __record.serialize_optional_field(
                "example",
                ::core::option::Option::as_ref(&self.example),
            )?;
            __record.serialize_optional_field(
                "examples",
                ::core::option::Option::as_ref(&self.examples),
            )?;
            __record.serialize_optional_field(
                "content",
                ::core::option::Option::as_ref(&self.content),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Header {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "description",
                "required",
                "deprecated",
                "allowEmptyValue",
                "style",
                "explode",
                "allowReserved",
                "schema",
                "example",
                "examples",
                "content",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"description\", \"required\", \"deprecated\", \"allowEmptyValue\", \"style\", \"explode\", \"allowReserved\", \"schema\", \"example\", \"examples\", \"content\"]" ;
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "required" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "allowEmptyValue" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier3)
                        }
                        "style" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "explode" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "allowReserved" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        "schema" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        "example" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        "examples" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        "content" => ::core::result::Result::Ok(__Identifier::__Identifier10),
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
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"required" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"deprecated" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"allowEmptyValue" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier3)
                        }
                        b"style" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"explode" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"allowReserved" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                        b"schema" => ::core::result::Result::Ok(__Identifier::__Identifier7),
                        b"example" => ::core::result::Result::Ok(__Identifier::__Identifier8),
                        b"examples" => ::core::result::Result::Ok(__Identifier::__Identifier9),
                        b"content" => ::core::result::Result::Ok(__Identifier::__Identifier10),
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
                type Value = Header;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Header")
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
                        ::core::option::Option<Markdown>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    0usize,
                                    &"record with 11 fields",
                                ),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    1usize,
                                    &"record with 11 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    2usize,
                                    &"record with 11 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    3usize,
                                    &"record with 11 fields",
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
                                    &"record with 11 fields",
                                ),
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
                                __serde::de::Error::invalid_length(
                                    5usize,
                                    &"record with 11 fields",
                                ),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    6usize,
                                    &"record with 11 fields",
                                ),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    7usize,
                                    &"record with 11 fields",
                                ),
                            );
                        }
                    };
                    let __field8 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Any>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    8usize,
                                    &"record with 11 fields",
                                ),
                            );
                        }
                    };
                    let __field9 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    9usize,
                                    &"record with 11 fields",
                                ),
                            );
                        }
                    };
                    let __field10 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MediaType>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(
                                    10usize,
                                    &"record with 11 fields",
                                ),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Header {
                        description: __field0,
                        required: __field1,
                        deprecated: __field2,
                        allow_empty_value: __field3,
                        style: __field4,
                        explode: __field5,
                        allow_reserved: __field6,
                        schema: __field7,
                        example: __field8,
                        examples: __field9,
                        content: __field10,
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
                    let mut __field0: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::core::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<
                        ::core::option::Option<
                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field8: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field9: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MaybeRef<Example>>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field10: ::core::option::Option<
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, MediaType>,
                        >,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "required",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "deprecated",
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
                                            "allowEmptyValue",
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
                                            "style",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "explode",
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
                                            "allowReserved",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schema",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::sidex_gen_json_schema::types::schema::SchemaObject,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier8 => {
                                if ::core::option::Option::is_some(&__field8) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "example",
                                        ),
                                    );
                                }
                                __field8 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Any>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier9 => {
                                if ::core::option::Option::is_some(&__field9) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "examples",
                                        ),
                                    );
                                }
                                __field9 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                MaybeRef<Example>,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier10 => {
                                if ::core::option::Option::is_some(&__field10) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "content",
                                        ),
                                    );
                                }
                                __field10 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<::std::string::String, MediaType>,
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
                    ::core::result::Result::Ok(Header {
                        description: __field0,
                        required: __field1,
                        deprecated: __field2,
                        allow_empty_value: __field3,
                        style: __field4,
                        explode: __field5,
                        allow_reserved: __field6,
                        schema: __field7,
                        example: __field8,
                        examples: __field9,
                        content: __field10,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "description",
                "required",
                "deprecated",
                "allowEmptyValue",
                "style",
                "explode",
                "allowReserved",
                "schema",
                "example",
                "examples",
                "content",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Header",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Tag Object](https://spec.openapis.org/oas/v3.1.0#tag-object) (4.8.22).\n"]
    #[derive(Clone, Debug)]
    pub struct Tag {
        #[doc = ""]
        pub name: ::std::string::String,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub external_docs: ::std::option::Option<ExternalDocumentation>,
    }
    impl Tag {
        #[doc = "Creates a new [`Tag`]."]
        pub fn new(name: ::std::string::String) -> Self {
            Self {
                name,
                description: ::std::default::Default::default(),
                external_docs: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `external_docs`."]
        pub fn set_external_docs(
            &mut self,
            external_docs: ::std::option::Option<ExternalDocumentation>,
        ) -> &mut Self {
            self.external_docs = external_docs;
            self
        }
        #[doc = "Sets the value of `external_docs`."]
        pub fn with_external_docs(
            mut self,
            external_docs: ::std::option::Option<ExternalDocumentation>,
        ) -> Self {
            self.external_docs = external_docs;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Tag {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Tag", 3usize)?;
            __record.serialize_field("name", &self.name)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "externalDocs",
                ::core::option::Option::as_ref(&self.external_docs),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Tag {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["name", "description", "externalDocs"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"description\", \"externalDocs\"]";
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
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "externalDocs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"externalDocs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                type Value = Tag;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Tag")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
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
                        ::core::option::Option<ExternalDocumentation>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Tag {
                        name: __field0,
                        description: __field1,
                        external_docs: __field2,
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
                    let mut __field1: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<ExternalDocumentation>,
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
                                            "description",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "externalDocs",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<ExternalDocumentation>,
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
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Tag {
                        name: __field0,
                        description: __field1,
                        external_docs: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "description", "externalDocs"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Tag",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Reference Object](https://spec.openapis.org/oas/v3.1.0#reference-object) (4.8.23).\n"]
    #[derive(Clone, Debug)]
    pub struct Reference {
        #[doc = ""]
        pub reference: ::std::string::String,
        #[doc = ""]
        pub summary: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
    }
    impl Reference {
        #[doc = "Creates a new [`Reference`]."]
        pub fn new(reference: ::std::string::String) -> Self {
            Self {
                reference,
                summary: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `reference`."]
        pub fn set_reference(&mut self, reference: ::std::string::String) -> &mut Self {
            self.reference = reference;
            self
        }
        #[doc = "Sets the value of `reference`."]
        pub fn with_reference(mut self, reference: ::std::string::String) -> Self {
            self.reference = reference;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn set_summary(
            &mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `summary`."]
        pub fn with_summary(
            mut self,
            summary: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.summary = summary;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Reference {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Reference", 3usize)?;
            __record.serialize_field("$ref", &self.reference)?;
            __record.serialize_optional_field(
                "summary",
                ::core::option::Option::as_ref(&self.summary),
            )?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Reference {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["$ref", "summary", "description"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"$ref\", \"summary\", \"description\"]";
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
                        "$ref" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "summary" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"$ref" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"summary" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                type Value = Reference;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Reference")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
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
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Reference {
                        reference: __field0,
                        summary: __field1,
                        description: __field2,
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
                    let mut __field2: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("$ref"),
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
                                            "summary",
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
                                            "description",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
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
                                <__A::Error as __serde::de::Error>::missing_field("$ref"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Reference {
                        reference: __field0,
                        summary: __field1,
                        description: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["$ref", "summary", "description"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Reference",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Discriminator Object](https://spec.openapis.org/oas/v3.1.0#discriminator-object) (4.8.25).\n"]
    #[derive(Clone, Debug)]
    pub struct Discriminator {
        #[doc = ""]
        pub property_name: ::std::string::String,
        #[doc = ""]
        pub mapping:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, ::std::string::String>>,
    }
    impl Discriminator {
        #[doc = "Creates a new [`Discriminator`]."]
        pub fn new(property_name: ::std::string::String) -> Self {
            Self {
                property_name,
                mapping: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `property_name`."]
        pub fn set_property_name(&mut self, property_name: ::std::string::String) -> &mut Self {
            self.property_name = property_name;
            self
        }
        #[doc = "Sets the value of `property_name`."]
        pub fn with_property_name(mut self, property_name: ::std::string::String) -> Self {
            self.property_name = property_name;
            self
        }
        #[doc = "Sets the value of `mapping`."]
        pub fn set_mapping(
            &mut self,
            mapping: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ::std::string::String>,
            >,
        ) -> &mut Self {
            self.mapping = mapping;
            self
        }
        #[doc = "Sets the value of `mapping`."]
        pub fn with_mapping(
            mut self,
            mapping: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ::std::string::String>,
            >,
        ) -> Self {
            self.mapping = mapping;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Discriminator {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Discriminator", 2usize)?;
            __record.serialize_field("propertyName", &self.property_name)?;
            __record.serialize_optional_field(
                "mapping",
                ::core::option::Option::as_ref(&self.mapping),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Discriminator {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["propertyName", "mapping"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"propertyName\", \"mapping\"]";
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
                        "propertyName" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "mapping" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"propertyName" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"mapping" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                type Value = Discriminator;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Discriminator")
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
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, ::std::string::String>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Discriminator {
                        property_name: __field0,
                        mapping: __field1,
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
                        ::core::option::Option<
                            indexmap::IndexMap<::std::string::String, ::std::string::String>,
                        >,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "propertyName",
                                        ),
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
                                            "mapping",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                ::std::string::String,
                                            >,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("propertyName"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Discriminator {
                        property_name: __field0,
                        mapping: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["propertyName", "mapping"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Discriminator",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An [XML Object](https://spec.openapis.org/oas/v3.1.0#xml-object) (4.8.26).\n"]
    #[derive(Clone, Debug)]
    pub struct XmlObject(pub(crate) Any);
    impl ::std::convert::From<XmlObject> for Any {
        fn from(wrapped: XmlObject) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for XmlObject {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for XmlObject {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(XmlObject(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = "A [Security Scheme Object](https://spec.openapis.org/oas/v3.1.0#security-scheme-object) (4.8.27).\n"]
    #[derive(Clone, Debug)]
    pub struct SecurityScheme {
        #[doc = ""]
        pub ty: ::std::string::String,
        #[doc = ""]
        pub description: ::std::option::Option<Markdown>,
        #[doc = ""]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub scheme: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub bearer_format: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub flows: ::std::option::Option<Any>,
        #[doc = ""]
        pub open_id_connect_url: ::std::option::Option<Url>,
    }
    impl SecurityScheme {
        #[doc = "Creates a new [`SecurityScheme`]."]
        pub fn new(ty: ::std::string::String) -> Self {
            Self {
                ty,
                description: ::std::default::Default::default(),
                name: ::std::default::Default::default(),
                scheme: ::std::default::Default::default(),
                bearer_format: ::std::default::Default::default(),
                flows: ::std::default::Default::default(),
                open_id_connect_url: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `ty`."]
        pub fn set_ty(&mut self, ty: ::std::string::String) -> &mut Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn with_ty(mut self, ty: ::std::string::String) -> Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<Markdown>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(mut self, description: ::std::option::Option<Markdown>) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::option::Option<::std::string::String>) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `scheme`."]
        pub fn set_scheme(
            &mut self,
            scheme: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.scheme = scheme;
            self
        }
        #[doc = "Sets the value of `scheme`."]
        pub fn with_scheme(mut self, scheme: ::std::option::Option<::std::string::String>) -> Self {
            self.scheme = scheme;
            self
        }
        #[doc = "Sets the value of `bearer_format`."]
        pub fn set_bearer_format(
            &mut self,
            bearer_format: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.bearer_format = bearer_format;
            self
        }
        #[doc = "Sets the value of `bearer_format`."]
        pub fn with_bearer_format(
            mut self,
            bearer_format: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.bearer_format = bearer_format;
            self
        }
        #[doc = "Sets the value of `flows`."]
        pub fn set_flows(&mut self, flows: ::std::option::Option<Any>) -> &mut Self {
            self.flows = flows;
            self
        }
        #[doc = "Sets the value of `flows`."]
        pub fn with_flows(mut self, flows: ::std::option::Option<Any>) -> Self {
            self.flows = flows;
            self
        }
        #[doc = "Sets the value of `open_id_connect_url`."]
        pub fn set_open_id_connect_url(
            &mut self,
            open_id_connect_url: ::std::option::Option<Url>,
        ) -> &mut Self {
            self.open_id_connect_url = open_id_connect_url;
            self
        }
        #[doc = "Sets the value of `open_id_connect_url`."]
        pub fn with_open_id_connect_url(
            mut self,
            open_id_connect_url: ::std::option::Option<Url>,
        ) -> Self {
            self.open_id_connect_url = open_id_connect_url;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SecurityScheme {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SecurityScheme", 7usize)?;
            __record.serialize_field("type", &self.ty)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record
                .serialize_optional_field("name", ::core::option::Option::as_ref(&self.name))?;
            __record
                .serialize_optional_field("scheme", ::core::option::Option::as_ref(&self.scheme))?;
            __record.serialize_optional_field(
                "bearerFormat",
                ::core::option::Option::as_ref(&self.bearer_format),
            )?;
            __record
                .serialize_optional_field("flows", ::core::option::Option::as_ref(&self.flows))?;
            __record.serialize_optional_field(
                "openIdConnectUrl",
                ::core::option::Option::as_ref(&self.open_id_connect_url),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SecurityScheme {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "type",
                "description",
                "name",
                "scheme",
                "bearerFormat",
                "flows",
                "openIdConnectUrl",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"type\", \"description\", \"name\", \"scheme\", \"bearerFormat\", \"flows\", \"openIdConnectUrl\"]" ;
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
                        "type" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "scheme" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "bearerFormat" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "flows" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        "openIdConnectUrl" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier6)
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
                        b"type" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"name" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"scheme" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"bearerFormat" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"flows" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                        b"openIdConnectUrl" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier6)
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
                type Value = SecurityScheme;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SecurityScheme")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Markdown>,
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
                        ::core::option::Option<::std::string::String>,
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
                        ::core::option::Option<::std::string::String>,
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
                        ::core::option::Option<::std::string::String>,
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
                        ::core::option::Option<Any>,
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
                        ::core::option::Option<Url>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 7 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SecurityScheme {
                        ty: __field0,
                        description: __field1,
                        name: __field2,
                        scheme: __field3,
                        bearer_format: __field4,
                        flows: __field5,
                        open_id_connect_url: __field6,
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
                    let mut __field1: ::core::option::Option<::core::option::Option<Markdown>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<Any>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::core::option::Option<Url>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("type"),
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
                                            "description",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Markdown>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "scheme",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "bearerFormat",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "flows",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Any>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "openIdConnectUrl",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Url>,
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
                                <__A::Error as __serde::de::Error>::missing_field("type"),
                            );
                        }
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
                    ::core::result::Result::Ok(SecurityScheme {
                        ty: __field0,
                        description: __field1,
                        name: __field2,
                        scheme: __field3,
                        bearer_format: __field4,
                        flows: __field5,
                        open_id_connect_url: __field6,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "type",
                "description",
                "name",
                "scheme",
                "bearerFormat",
                "flows",
                "openIdConnectUrl",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SecurityScheme",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A [Security Requirement Object](https://spec.openapis.org/oas/v3.1.0#security-requirement-object) (4.8.30).\n"]
    #[derive(Clone, Debug)]
    pub struct SecurityRequirement(
        pub(crate) indexmap::IndexMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
    );
    impl ::std::convert::From<SecurityRequirement>
        for indexmap::IndexMap<::std::string::String, ::std::vec::Vec<::std::string::String>>
    {
        fn from(wrapped: SecurityRequirement) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SecurityRequirement {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SecurityRequirement {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(SecurityRequirement(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
}
