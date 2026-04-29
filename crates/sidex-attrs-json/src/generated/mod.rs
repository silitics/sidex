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
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "JSON-specific attributes that may be applied to an `opaque` definition.\n\nSource form: `#[json(type = \"string|null\", schema = \"/path\")]`.\n"]
    #[derive(Clone, Debug)]
    pub struct OpaqueTypeAttrs {
        #[doc = "JSON union type expression (e.g. `\"string|null\"`).\n"]
        pub typ: ::std::option::Option<::std::string::String>,
        #[doc = "JSON Schema path emitted as `$ref` for the opaque type.\n"]
        pub schema: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for OpaqueTypeAttrs {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for OpaqueTypeAttrs {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "OpaqueTypeAttrs", 2usize)?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.typ),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record
                    .serialize_optional_field("typ", ::core::option::Option::as_ref(&__wrapped))?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.schema),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "schema",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for OpaqueTypeAttrs {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = OpaqueTypeAttrs;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record OpaqueTypeAttrs")
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
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(OpaqueTypeAttrs {
                        typ: __field0,
                        schema: __field1,
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
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["typ", "schema"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"typ\", \"schema\"]";
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
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "typ" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "schema" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"typ" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"schema" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("typ"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                    ::core::result::Result::Ok(OpaqueTypeAttrs {
                        typ: __field0,
                        schema: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["typ", "schema"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "OpaqueTypeAttrs",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "JSON-specific attributes that may be applied to a `record` definition.\n\nSource form: `#[json(rename_all = \"PascalCase\")]`.\n"]
    #[derive(Clone, Debug)]
    pub struct RecordTypeAttrs {
        #[doc = "Default rename function applied to every field name.\n"]
        pub rename_all: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for RecordTypeAttrs {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for RecordTypeAttrs {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "RecordTypeAttrs", 1usize)?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.rename_all),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "rename_all",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RecordTypeAttrs {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = RecordTypeAttrs;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record RecordTypeAttrs")
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
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(RecordTypeAttrs {
                        rename_all: __field0,
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
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["rename_all"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"rename_all\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
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
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "rename_all" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
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
                                b"rename_all" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "rename_all",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                    ::core::result::Result::Ok(RecordTypeAttrs {
                        rename_all: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["rename_all"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "RecordTypeAttrs",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "JSON-specific attributes that may be applied to a record `field`.\n\nSource form: `#[json(name = \"explicit\", rename = \"camelCase\", inline)]`.\n"]
    #[derive(Clone, Debug)]
    pub struct FieldAttrs {
        #[doc = "Override the JSON key with an explicit string.\n"]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = "Apply a rename function to the field name.\n"]
        pub rename: ::std::option::Option<::std::string::String>,
        #[doc = "Whether to inline the field's contents into the enclosing object.\n"]
        pub inline: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for FieldAttrs {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for FieldAttrs {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "FieldAttrs", 3usize)?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.name),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record
                    .serialize_optional_field("name", ::core::option::Option::as_ref(&__wrapped))?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.rename),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "rename",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.inline),
                    |__v| __sidex_serde::SerializeAsWrap::<bool, __sidex_serde::AsSelf>::new(__v),
                );
                __record.serialize_optional_field(
                    "inline",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for FieldAttrs {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = FieldAttrs;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record FieldAttrs")
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
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<bool>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(FieldAttrs {
                        name: __field0,
                        rename: __field1,
                        inline: __field2,
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
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["name", "rename", "inline"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"name\", \"rename\", \"inline\"]";
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
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
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
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "rename" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "inline" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                                b"rename" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"inline" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<bool>> =
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
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "rename",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "inline",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<bool>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                    ::core::result::Result::Ok(FieldAttrs {
                        name: __field0,
                        rename: __field1,
                        inline: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "rename", "inline"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "FieldAttrs",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
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
    impl __sidex_serde::SidexType for VariantTypeAttrs {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for VariantTypeAttrs {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "VariantTypeAttrs",
                4usize,
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.rename_all),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "rename_all",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.tagged),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "tagged",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.tag),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record
                    .serialize_optional_field("tag", ::core::option::Option::as_ref(&__wrapped))?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.content),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "content",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for VariantTypeAttrs {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = VariantTypeAttrs;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record VariantTypeAttrs")
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
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(VariantTypeAttrs {
                        rename_all: __field0,
                        tagged: __field1,
                        tag: __field2,
                        content: __field3,
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
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] =
                        &["rename_all", "tagged", "tag", "content"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"rename_all\", \"tagged\", \"tag\", \"content\"]";
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
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
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
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "rename_all" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "tagged" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "tag" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "content" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
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
                                b"rename_all" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"tagged" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"tag" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"content" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "rename_all",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "tagged",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("tag"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "content",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                    ::core::result::Result::Ok(VariantTypeAttrs {
                        rename_all: __field0,
                        tagged: __field1,
                        tag: __field2,
                        content: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["rename_all", "tagged", "tag", "content"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "VariantTypeAttrs",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
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
    impl __sidex_serde::SidexType for VariantAttrs {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for VariantAttrs {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "VariantAttrs", 3usize)?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.name),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record
                    .serialize_optional_field("name", ::core::option::Option::as_ref(&__wrapped))?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.rename),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "rename",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.content),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "content",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for VariantAttrs {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = VariantAttrs;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record VariantAttrs")
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
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<::std::string::String>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(VariantAttrs {
                        name: __field0,
                        rename: __field1,
                        content: __field2,
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
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["name", "rename", "content"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"name\", \"rename\", \"content\"]";
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
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
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
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "rename" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "content" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
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
                                b"name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"rename" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"content" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
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
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "rename",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<::std::string::String>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                    ::core::result::Result::Ok(VariantAttrs {
                        name: __field0,
                        rename: __field1,
                        content: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "rename", "content"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "VariantAttrs",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
