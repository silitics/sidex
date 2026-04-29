/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod ir {
    #![doc = ""]
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
    #[doc = "An arbitrary, schema-validated Sidex value.\n\nUsed by [`typed_attrs`] to carry the parsed result of a plugin's typed\nattribute schema. Each codegen target maps `Value` to its language's\n\"any\" type (e.g., `serde_json::Value` in Rust, `unknown` in TypeScript,\n`typing.Any` in Python). Producers and consumers communicate through\nJSON shape, validated at IR build time against the plugin's Sidex schema.\n"]
    pub type Value = ::serde_json::Value;
    #[doc = "Uniquely identifies a source in an IR.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct SourceIdx(pub(crate) usize);
    impl ::std::convert::From<SourceIdx> for usize {
        fn from(wrapped: SourceIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for SourceIdx {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for SourceIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            __sidex_serde::SerializeAsWrap::<usize, __sidex_serde::AsSelf>::new(&self.0)
                .serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SourceIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            let __wrapped: __sidex_serde::DeserializeAsWrap<usize, __sidex_serde::AsSelf> =
                __serde::Deserialize::deserialize(__deserializer)?;
            Ok(SourceIdx(__wrapped.into_inner()))
        }
    }
    #[doc = "Uniquely identifies a bundle in an IR.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct BundleIdx(pub(crate) usize);
    impl ::std::convert::From<BundleIdx> for usize {
        fn from(wrapped: BundleIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for BundleIdx {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for BundleIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            __sidex_serde::SerializeAsWrap::<usize, __sidex_serde::AsSelf>::new(&self.0)
                .serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BundleIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            let __wrapped: __sidex_serde::DeserializeAsWrap<usize, __sidex_serde::AsSelf> =
                __serde::Deserialize::deserialize(__deserializer)?;
            Ok(BundleIdx(__wrapped.into_inner()))
        }
    }
    #[doc = "Uniquely identifies a schema in an IR.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct SchemaIdx(pub(crate) usize);
    impl ::std::convert::From<SchemaIdx> for usize {
        fn from(wrapped: SchemaIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for SchemaIdx {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for SchemaIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            __sidex_serde::SerializeAsWrap::<usize, __sidex_serde::AsSelf>::new(&self.0)
                .serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SchemaIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            let __wrapped: __sidex_serde::DeserializeAsWrap<usize, __sidex_serde::AsSelf> =
                __serde::Deserialize::deserialize(__deserializer)?;
            Ok(SchemaIdx(__wrapped.into_inner()))
        }
    }
    #[doc = "Uniquely identifies a definition in an IR.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct DefIdx(pub(crate) usize);
    impl ::std::convert::From<DefIdx> for usize {
        fn from(wrapped: DefIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for DefIdx {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for DefIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            __sidex_serde::SerializeAsWrap::<usize, __sidex_serde::AsSelf>::new(&self.0)
                .serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DefIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            let __wrapped: __sidex_serde::DeserializeAsWrap<usize, __sidex_serde::AsSelf> =
                __serde::Deserialize::deserialize(__deserializer)?;
            Ok(DefIdx(__wrapped.into_inner()))
        }
    }
    #[doc = "Uniquely identifies a type variable in a definition.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct TypeVarIdx(pub(crate) usize);
    impl ::std::convert::From<TypeVarIdx> for usize {
        fn from(wrapped: TypeVarIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for TypeVarIdx {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for TypeVarIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            __sidex_serde::SerializeAsWrap::<usize, __sidex_serde::AsSelf>::new(&self.0)
                .serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeVarIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            let __wrapped: __sidex_serde::DeserializeAsWrap<usize, __sidex_serde::AsSelf> =
                __serde::Deserialize::deserialize(__deserializer)?;
            Ok(TypeVarIdx(__wrapped.into_inner()))
        }
    }
    #[doc = "References a definition.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct DefRef {
        #[doc = "The bundle containing the definition.\n"]
        pub bundle: BundleIdx,
        #[doc = "The schema containing the definition.\n"]
        pub schema: SchemaIdx,
        #[doc = "The definition.\n"]
        pub def: DefIdx,
    }
    impl DefRef {
        #[doc = "Creates a new [`DefRef`]."]
        pub fn new(bundle: BundleIdx, schema: SchemaIdx, def: DefIdx) -> Self {
            Self {
                bundle,
                schema,
                def,
            }
        }
        #[doc = "Sets the value of `bundle`."]
        pub fn set_bundle(&mut self, bundle: BundleIdx) -> &mut Self {
            self.bundle = bundle;
            self
        }
        #[doc = "Sets the value of `bundle`."]
        pub fn with_bundle(mut self, bundle: BundleIdx) -> Self {
            self.bundle = bundle;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn set_schema(&mut self, schema: SchemaIdx) -> &mut Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn with_schema(mut self, schema: SchemaIdx) -> Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `def`."]
        pub fn set_def(&mut self, def: DefIdx) -> &mut Self {
            self.def = def;
            self
        }
        #[doc = "Sets the value of `def`."]
        pub fn with_def(mut self, def: DefIdx) -> Self {
            self.def = def;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for DefRef {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for DefRef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "DefRef", 3usize)?;
            __record.serialize_field(
                "bundle",
                &__sidex_serde::SerializeAsWrap::<BundleIdx, __sidex_serde::AsSelf>::new(
                    &self.bundle,
                ),
            )?;
            __record.serialize_field(
                "schema",
                &__sidex_serde::SerializeAsWrap::<SchemaIdx, __sidex_serde::AsSelf>::new(
                    &self.schema,
                ),
            )?;
            __record.serialize_field(
                "def",
                &__sidex_serde::SerializeAsWrap::<DefIdx, __sidex_serde::AsSelf>::new(&self.def),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DefRef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = DefRef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record DefRef")
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
                        __sidex_serde::DeserializeAsWrap<BundleIdx, __sidex_serde::AsSelf>,
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
                        __sidex_serde::DeserializeAsWrap<SchemaIdx, __sidex_serde::AsSelf>,
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
                        __sidex_serde::DeserializeAsWrap<DefIdx, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(DefRef {
                        bundle: __field0,
                        schema: __field1,
                        def: __field2,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["bundle", "schema", "def"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"bundle\", \"schema\", \"def\"]";
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
                                "bundle" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "schema" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "def" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                                b"bundle" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"schema" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"def" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                    let mut __field0: ::core::option::Option<BundleIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<SchemaIdx> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<DefIdx> = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "bundle",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            BundleIdx,
                                            __sidex_serde::AsSelf,
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
                                            SchemaIdx,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("def"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            DefIdx,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("bundle"),
                            );
                        }
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("def"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(DefRef {
                        bundle: __field0,
                        schema: __field1,
                        def: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["bundle", "schema", "def"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "DefRef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Sidex IR — a fully-resolved, mostly-processed representation that\ndownstream code generators consume.\n\nEvery [`DefRef`] reachable from the IR resolves to a real entry in\n[`Ir.defs`]. Codegens walk this graph without needing to think about\nnames, imports, or version constraints — those concerns are handled\nbefore the IR is built.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Ir {
        #[doc = "The bundles of the IR.\n"]
        pub bundles: ::std::vec::Vec<Bundle>,
        #[doc = "The schemas of the IR.\n"]
        pub schemas: ::std::vec::Vec<Schema>,
        #[doc = "The definitions of the IR.\n"]
        pub defs: ::std::vec::Vec<Def>,
        #[doc = "The sources of the IR.\n"]
        pub sources: ::std::vec::Vec<Source>,
        #[doc = "The bundle this IR was assembled around. Codegens typically only\nemit code for definitions belonging to schemas in this bundle.\n"]
        pub root: BundleIdx,
    }
    impl Ir {
        #[doc = "Creates a new [`Ir`]."]
        pub fn new(root: BundleIdx) -> Self {
            Self {
                root,
                bundles: ::std::default::Default::default(),
                schemas: ::std::default::Default::default(),
                defs: ::std::default::Default::default(),
                sources: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `bundles`."]
        pub fn set_bundles(&mut self, bundles: ::std::vec::Vec<Bundle>) -> &mut Self {
            self.bundles = bundles;
            self
        }
        #[doc = "Sets the value of `bundles`."]
        pub fn with_bundles(mut self, bundles: ::std::vec::Vec<Bundle>) -> Self {
            self.bundles = bundles;
            self
        }
        #[doc = "Sets the value of `schemas`."]
        pub fn set_schemas(&mut self, schemas: ::std::vec::Vec<Schema>) -> &mut Self {
            self.schemas = schemas;
            self
        }
        #[doc = "Sets the value of `schemas`."]
        pub fn with_schemas(mut self, schemas: ::std::vec::Vec<Schema>) -> Self {
            self.schemas = schemas;
            self
        }
        #[doc = "Sets the value of `defs`."]
        pub fn set_defs(&mut self, defs: ::std::vec::Vec<Def>) -> &mut Self {
            self.defs = defs;
            self
        }
        #[doc = "Sets the value of `defs`."]
        pub fn with_defs(mut self, defs: ::std::vec::Vec<Def>) -> Self {
            self.defs = defs;
            self
        }
        #[doc = "Sets the value of `sources`."]
        pub fn set_sources(&mut self, sources: ::std::vec::Vec<Source>) -> &mut Self {
            self.sources = sources;
            self
        }
        #[doc = "Sets the value of `sources`."]
        pub fn with_sources(mut self, sources: ::std::vec::Vec<Source>) -> Self {
            self.sources = sources;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn set_root(&mut self, root: BundleIdx) -> &mut Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn with_root(mut self, root: BundleIdx) -> Self {
            self.root = root;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Ir {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Ir {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Ir", 5usize)?;
            __record.serialize_field(
                "bundles",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Bundle>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.bundles),
            )?;
            __record.serialize_field(
                "schemas",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Schema>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.schemas),
            )?;
            __record.serialize_field(
                "defs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Def>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.defs),
            )?;
            __record.serialize_field(
                "sources",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Source>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.sources),
            )?;
            __record.serialize_field(
                "root",
                &__sidex_serde::SerializeAsWrap::<BundleIdx, __sidex_serde::AsSelf>::new(
                    &self.root,
                ),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Ir {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Ir;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Ir")
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
                            ::std::vec::Vec<Bundle>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<Schema>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<Def>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<Source>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<BundleIdx, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Ir {
                        bundles: __field0,
                        schemas: __field1,
                        defs: __field2,
                        sources: __field3,
                        root: __field4,
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
                        &["bundles", "schemas", "defs", "sources", "root"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"bundles\", \"schemas\", \"defs\", \"sources\", \"root\"]";
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
                                4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                                "bundles" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "schemas" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "defs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "sources" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                "root" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                                b"bundles" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"schemas" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"defs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"sources" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                b"root" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                    let mut __field0: ::core::option::Option<::std::vec::Vec<Bundle>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::vec::Vec<Schema>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Def>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<Source>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<BundleIdx> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "bundles",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Bundle>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schemas",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Schema>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("defs"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Def>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "sources",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Source>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("root"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            BundleIdx,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("bundles"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("schemas"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("defs"),
                            );
                        }
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("sources"),
                            );
                        }
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("root"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Ir {
                        bundles: __field0,
                        schemas: __field1,
                        defs: __field2,
                        sources: __field3,
                        root: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["bundles", "schemas", "defs", "sources", "root"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Ir",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A *source* is a chunk of text.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Source {
        #[doc = "The origin of the source, e.g., a filesystem path.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "The optional text of the source.\n\nThis field is optional to allow for programmatically generated schemas.\n"]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl Source {
        #[doc = "Creates a new [`Source`]."]
        pub fn new() -> Self {
            Self {
                origin: ::std::default::Default::default(),
                text: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `origin`."]
        pub fn set_origin(
            &mut self,
            origin: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.origin = origin;
            self
        }
        #[doc = "Sets the value of `origin`."]
        pub fn with_origin(mut self, origin: ::std::option::Option<::std::string::String>) -> Self {
            self.origin = origin;
            self
        }
        #[doc = "Sets the value of `text`."]
        pub fn set_text(
            &mut self,
            text: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.text = text;
            self
        }
        #[doc = "Sets the value of `text`."]
        pub fn with_text(mut self, text: ::std::option::Option<::std::string::String>) -> Self {
            self.text = text;
            self
        }
    }
    impl ::std::default::Default for Source {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Source {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Source {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Source", 2usize)?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.origin),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "origin",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.text),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record
                    .serialize_optional_field("text", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Source {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Source;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Source")
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
                    ::core::result::Result::Ok(Source {
                        origin: __field0,
                        text: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["origin", "text"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"origin\", \"text\"]";
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
                                "origin" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "text" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"origin" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"text" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "origin",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("text"),
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
                    ::core::result::Result::Ok(Source {
                        origin: __field0,
                        text: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["origin", "text"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Source",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A bundle is a flat collection of schemas evolving together.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Bundle {
        #[doc = "The metadata of the bundle.\n"]
        pub metadata: Metadata,
        #[doc = "The schemas of the bundle.\n"]
        pub schemas: ::std::vec::Vec<SchemaIdx>,
        #[doc = "Whether the bundle is internal to the compiler.\n\nInternal bundles (the standard library, plugin attribute schemas\nauto-loaded by the compiler) are loaded automatically alongside any\nuser bundle so that the typed-attrs parser can validate plugin\nattributes. Code generators should usually skip them since their\ndefs are not part of the user's API surface.\n"]
        pub is_internal: bool,
    }
    impl Bundle {
        #[doc = "Creates a new [`Bundle`]."]
        pub fn new(metadata: Metadata) -> Self {
            Self {
                metadata,
                schemas: ::std::default::Default::default(),
                is_internal: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `metadata`."]
        pub fn set_metadata(&mut self, metadata: Metadata) -> &mut Self {
            self.metadata = metadata;
            self
        }
        #[doc = "Sets the value of `metadata`."]
        pub fn with_metadata(mut self, metadata: Metadata) -> Self {
            self.metadata = metadata;
            self
        }
        #[doc = "Sets the value of `schemas`."]
        pub fn set_schemas(&mut self, schemas: ::std::vec::Vec<SchemaIdx>) -> &mut Self {
            self.schemas = schemas;
            self
        }
        #[doc = "Sets the value of `schemas`."]
        pub fn with_schemas(mut self, schemas: ::std::vec::Vec<SchemaIdx>) -> Self {
            self.schemas = schemas;
            self
        }
        #[doc = "Sets the value of `is_internal`."]
        pub fn set_is_internal(&mut self, is_internal: bool) -> &mut Self {
            self.is_internal = is_internal;
            self
        }
        #[doc = "Sets the value of `is_internal`."]
        pub fn with_is_internal(mut self, is_internal: bool) -> Self {
            self.is_internal = is_internal;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Bundle {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Bundle {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Bundle", 3usize)?;
            __record.serialize_field(
                "metadata",
                &__sidex_serde::SerializeAsWrap::<Metadata, __sidex_serde::AsSelf>::new(
                    &self.metadata,
                ),
            )?;
            __record.serialize_field(
                "schemas",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<SchemaIdx>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.schemas),
            )?;
            __record.serialize_field(
                "isInternal",
                &__sidex_serde::SerializeAsWrap::<bool, __sidex_serde::AsSelf>::new(
                    &self.is_internal,
                ),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Bundle {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Bundle;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Bundle")
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
                        __sidex_serde::DeserializeAsWrap<Metadata, __sidex_serde::AsSelf>,
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
                            ::std::vec::Vec<SchemaIdx>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                        __sidex_serde::DeserializeAsWrap<bool, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Bundle {
                        metadata: __field0,
                        schemas: __field1,
                        is_internal: __field2,
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
                        &["metadata", "schemas", "isInternal"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"metadata\", \"schemas\", \"isInternal\"]";
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
                                "metadata" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "schemas" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "isInternal" => {
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
                                b"metadata" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"schemas" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"isInternal" => {
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
                    let mut __field0: ::core::option::Option<Metadata> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::vec::Vec<SchemaIdx>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<bool> = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "metadata",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            Metadata,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schemas",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<SchemaIdx>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "isInternal",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            bool,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("metadata"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("schemas"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("isInternal"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Bundle {
                        metadata: __field0,
                        schemas: __field1,
                        is_internal: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["metadata", "schemas", "isInternal"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Bundle",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Metadata of a bundle.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Metadata {
        #[doc = "The name of the bundle.\n"]
        pub name: ::std::string::String,
        #[doc = "The version of the bundle.\n"]
        pub version: ::std::string::String,
        #[doc = "The optional description of the bundle.\n"]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = "The optional authors of the bundle.\n"]
        pub authors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Metadata {
        #[doc = "Creates a new [`Metadata`]."]
        pub fn new(name: ::std::string::String, version: ::std::string::String) -> Self {
            Self {
                name,
                version,
                description: ::std::default::Default::default(),
                authors: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `authors`."]
        pub fn set_authors(
            &mut self,
            authors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.authors = authors;
            self
        }
        #[doc = "Sets the value of `authors`."]
        pub fn with_authors(
            mut self,
            authors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.authors = authors;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Metadata {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Metadata {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Metadata", 4usize)?;
            __record . serialize_field ("name" , & __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (& self . name) ,) ? ;
            __record . serialize_field ("version" , & __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (& self . version) ,) ? ;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.description),
                    |__v| {
                        __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (__v)
                    },
                );
                __record.serialize_optional_field(
                    "description",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.authors),
                    |__v| {
                        __sidex_serde::SerializeAsWrap::<
                            ::std::vec::Vec<::std::string::String>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >::new(__v)
                    },
                );
                __record.serialize_optional_field(
                    "authors",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Metadata {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
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
                        __sidex_serde::DeserializeAsWrap<
                            ::std::string::String,
                            __sidex_serde::AsSelf,
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
                            ::std::string::String,
                            __sidex_serde::AsSelf,
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
                            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                            ::std::option::Option<::std::vec::Vec<__sidex_serde::AsSelf>>,
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
                    ::core::result::Result::Ok(Metadata {
                        name: __field0,
                        version: __field1,
                        description: __field2,
                        authors: __field3,
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
                        &["name", "version", "description", "authors"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"name\", \"version\", \"description\", \"authors\"]";
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
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "version" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                "authors" => {
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
                                b"name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"version" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"authors" => {
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
                                            ::std::string::String,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::string::String,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
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
                                            "authors",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<
                                                ::std::vec::Vec<::std::string::String>,
                                            >,
                                            ::std::option::Option<
                                                ::std::vec::Vec<__sidex_serde::AsSelf>,
                                            >,
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
                    ::core::result::Result::Ok(Metadata {
                        name: __field0,
                        version: __field1,
                        description: __field2,
                        authors: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["name", "version", "description", "authors"];
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
    #[doc = "An *identifier* with an optional span.\n"]
    #[derive(Clone, Debug)]
    pub struct Ident {
        #[doc = "The identifier.\n"]
        pub name: ::std::string::String,
        #[doc = "The optional span of the identifier.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Ident {
        #[doc = "Creates a new [`Ident`]."]
        pub fn new(name: ::std::string::String) -> Self {
            Self {
                name,
                span: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `span`."]
        pub fn set_span(&mut self, span: ::std::option::Option<Span>) -> &mut Self {
            self.span = span;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn with_span(mut self, span: ::std::option::Option<Span>) -> Self {
            self.span = span;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Ident {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Ident {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Ident", 2usize)?;
            __record . serialize_field ("name" , & __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (& self . name) ,) ? ;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.span),
                    |__v| __sidex_serde::SerializeAsWrap::<Span, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("span", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Ident {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Ident;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Ident")
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
                            ::std::string::String,
                            __sidex_serde::AsSelf,
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
                            ::std::option::Option<Span>,
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
                    ::core::result::Result::Ok(Ident {
                        name: __field0,
                        span: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["name", "span"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"name\", \"span\"]";
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
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "span" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"span" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<Span>> =
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
                                            ::std::string::String,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Span>,
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
                    ::core::result::Result::Ok(Ident {
                        name: __field0,
                        span: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "span"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Ident",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Documentation attached to an item.\n"]
    #[derive(Clone, Debug)]
    pub struct Docs {
        #[doc = "The text of the documentation.\n"]
        pub text: ::std::string::String,
    }
    impl Docs {
        #[doc = "Creates a new [`Docs`]."]
        pub fn new(text: ::std::string::String) -> Self {
            Self { text }
        }
        #[doc = "Sets the value of `text`."]
        pub fn set_text(&mut self, text: ::std::string::String) -> &mut Self {
            self.text = text;
            self
        }
        #[doc = "Sets the value of `text`."]
        pub fn with_text(mut self, text: ::std::string::String) -> Self {
            self.text = text;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Docs {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Docs {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Docs", 1usize)?;
            __record . serialize_field ("text" , & __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (& self . text) ,) ? ;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Docs {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Docs;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Docs")
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
                            ::std::string::String,
                            __sidex_serde::AsSelf,
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
                    ::core::result::Result::Ok(Docs { text: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["text"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"text\"]";
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
                                "text" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                                b"text" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("text"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::string::String,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("text"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Docs { text: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["text"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Docs",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A schema is a collection of definitions.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Schema {
        #[doc = "The bundle the schema is part of.\n"]
        pub bundle: BundleIdx,
        #[doc = "The name of the schema.\n"]
        pub name: ::std::string::String,
        #[doc = "The documentation of the schema.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The attributes of the schema.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The parsed, schema-validated attributes, keyed by plugin id.\nPopulated by the compiler for plugins whose attribute schemas are\nloaded; raw [`attrs`] remains the round-trip form for unknown plugins.\n"]
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        #[doc = "The definitions of the schema.\n"]
        pub defs: ::std::vec::Vec<DefIdx>,
        #[doc = "The source of the schema.\n"]
        pub source: ::std::option::Option<SourceIdx>,
        #[doc = "The optional span of the schema.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Schema {
        #[doc = "Creates a new [`Schema`]."]
        pub fn new(bundle: BundleIdx, name: ::std::string::String) -> Self {
            Self {
                bundle,
                name,
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                typed_attrs: ::std::default::Default::default(),
                defs: ::std::default::Default::default(),
                source: ::std::default::Default::default(),
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `bundle`."]
        pub fn set_bundle(&mut self, bundle: BundleIdx) -> &mut Self {
            self.bundle = bundle;
            self
        }
        #[doc = "Sets the value of `bundle`."]
        pub fn with_bundle(mut self, bundle: BundleIdx) -> Self {
            self.bundle = bundle;
            self
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
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::option::Option<Docs>) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::option::Option<Docs>) -> Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn set_attrs(&mut self, attrs: ::std::vec::Vec<Attr>) -> &mut Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn with_attrs(mut self, attrs: ::std::vec::Vec<Attr>) -> Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn set_typed_attrs(
            &mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `defs`."]
        pub fn set_defs(&mut self, defs: ::std::vec::Vec<DefIdx>) -> &mut Self {
            self.defs = defs;
            self
        }
        #[doc = "Sets the value of `defs`."]
        pub fn with_defs(mut self, defs: ::std::vec::Vec<DefIdx>) -> Self {
            self.defs = defs;
            self
        }
        #[doc = "Sets the value of `source`."]
        pub fn set_source(&mut self, source: ::std::option::Option<SourceIdx>) -> &mut Self {
            self.source = source;
            self
        }
        #[doc = "Sets the value of `source`."]
        pub fn with_source(mut self, source: ::std::option::Option<SourceIdx>) -> Self {
            self.source = source;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn set_span(&mut self, span: ::std::option::Option<Span>) -> &mut Self {
            self.span = span;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn with_span(mut self, span: ::std::option::Option<Span>) -> Self {
            self.span = span;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Schema {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Schema {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Schema", 8usize)?;
            __record.serialize_field(
                "bundle",
                &__sidex_serde::SerializeAsWrap::<BundleIdx, __sidex_serde::AsSelf>::new(
                    &self.bundle,
                ),
            )?;
            __record . serialize_field ("name" , & __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (& self . name) ,) ? ;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.docs),
                    |__v| __sidex_serde::SerializeAsWrap::<Docs, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("docs", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.serialize_field(
                "attrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Attr>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.attrs),
            )?;
            __record.serialize_field(
                "typedAttrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    ::std::collections::HashMap<__sidex_serde::AsSelf, __sidex_serde::AsSelf>,
                >::new(&self.typed_attrs),
            )?;
            __record.serialize_field(
                "defs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<DefIdx>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.defs),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.source),
                    |__v| {
                        __sidex_serde::SerializeAsWrap::<SourceIdx, __sidex_serde::AsSelf>::new(__v)
                    },
                );
                __record.serialize_optional_field(
                    "source",
                    ::core::option::Option::as_ref(&__wrapped),
                )?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.span),
                    |__v| __sidex_serde::SerializeAsWrap::<Span, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("span", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Schema {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
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
                    ::core::fmt::Formatter::write_str(__formatter, "record Schema")
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
                        __sidex_serde::DeserializeAsWrap<BundleIdx, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::string::String,
                            __sidex_serde::AsSelf,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Docs>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<Attr>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                            ::std::collections::HashMap<
                                __sidex_serde::AsSelf,
                                __sidex_serde::AsSelf,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<DefIdx>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<SourceIdx>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Span>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(7usize, &"record with 8 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Schema {
                        bundle: __field0,
                        name: __field1,
                        docs: __field2,
                        attrs: __field3,
                        typed_attrs: __field4,
                        defs: __field5,
                        source: __field6,
                        span: __field7,
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
                    const __IDENTIFIERS: &'static [&'static str] = &[
                        "bundle",
                        "name",
                        "docs",
                        "attrs",
                        "typedAttrs",
                        "defs",
                        "source",
                        "span",
                    ];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"bundle\", \"name\", \"docs\", \"attrs\", \"typedAttrs\", \"defs\", \"source\", \"span\"]";
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
                                4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                                6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                                7u64 => ::core::result::Result::Ok(__Identifier::__Identifier7),
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
                                "bundle" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "docs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                "typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier4)
                                }
                                "defs" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                                "source" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                                "span" => ::core::result::Result::Ok(__Identifier::__Identifier7),
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
                                b"bundle" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"name" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                b"docs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"attrs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                b"typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier4)
                                }
                                b"defs" => ::core::result::Result::Ok(__Identifier::__Identifier5),
                                b"source" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier6)
                                }
                                b"span" => ::core::result::Result::Ok(__Identifier::__Identifier7),
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
                    let mut __field0: ::core::option::Option<BundleIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    > = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::std::vec::Vec<DefIdx>> =
                        ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::std::option::Option<SourceIdx>> =
                        ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<::std::option::Option<Span>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "bundle",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            BundleIdx,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::string::String,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("docs"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Docs>,
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
                                            "attrs",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Attr>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "typedAttrs",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                ::serde_json::Value,
                                            >,
                                            ::std::collections::HashMap<
                                                __sidex_serde::AsSelf,
                                                __sidex_serde::AsSelf,
                                            >,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("defs"),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<DefIdx>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "source",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<SourceIdx>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Span>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("bundle"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("attrs"),
                            );
                        }
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("typedAttrs"),
                            );
                        }
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("defs"),
                            );
                        }
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field7 = match __field7 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Schema {
                        bundle: __field0,
                        name: __field1,
                        docs: __field2,
                        attrs: __field3,
                        typed_attrs: __field4,
                        defs: __field5,
                        source: __field6,
                        span: __field7,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "bundle",
                "name",
                "docs",
                "attrs",
                "typedAttrs",
                "defs",
                "source",
                "span",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Schema",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A definition.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Def {
        #[doc = "The schema the definition belongs to.\n"]
        pub schema: SchemaIdx,
        #[doc = "The name of the definition.\n"]
        pub name: Ident,
        #[doc = "The documentation of the definition.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The type variables of the definition.\n"]
        pub vars: ::std::vec::Vec<TypeVar>,
        #[doc = "The attributes of the definition.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The parsed, schema-validated attributes, keyed by plugin id.\n"]
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        #[doc = "The kind of the definition.\n"]
        pub kind: DefKind,
        #[doc = "The optional span of the definition.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Def {
        #[doc = "Creates a new [`Def`]."]
        pub fn new(schema: SchemaIdx, name: Ident, kind: DefKind) -> Self {
            Self {
                schema,
                name,
                kind,
                docs: ::std::default::Default::default(),
                vars: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                typed_attrs: ::std::default::Default::default(),
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `schema`."]
        pub fn set_schema(&mut self, schema: SchemaIdx) -> &mut Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `schema`."]
        pub fn with_schema(mut self, schema: SchemaIdx) -> Self {
            self.schema = schema;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Ident) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Ident) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::option::Option<Docs>) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::option::Option<Docs>) -> Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `vars`."]
        pub fn set_vars(&mut self, vars: ::std::vec::Vec<TypeVar>) -> &mut Self {
            self.vars = vars;
            self
        }
        #[doc = "Sets the value of `vars`."]
        pub fn with_vars(mut self, vars: ::std::vec::Vec<TypeVar>) -> Self {
            self.vars = vars;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn set_attrs(&mut self, attrs: ::std::vec::Vec<Attr>) -> &mut Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn with_attrs(mut self, attrs: ::std::vec::Vec<Attr>) -> Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn set_typed_attrs(
            &mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: DefKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: DefKind) -> Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn set_span(&mut self, span: ::std::option::Option<Span>) -> &mut Self {
            self.span = span;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn with_span(mut self, span: ::std::option::Option<Span>) -> Self {
            self.span = span;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Def {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Def {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Def", 8usize)?;
            __record.serialize_field(
                "schema",
                &__sidex_serde::SerializeAsWrap::<SchemaIdx, __sidex_serde::AsSelf>::new(
                    &self.schema,
                ),
            )?;
            __record.serialize_field(
                "name",
                &__sidex_serde::SerializeAsWrap::<Ident, __sidex_serde::AsSelf>::new(&self.name),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.docs),
                    |__v| __sidex_serde::SerializeAsWrap::<Docs, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("docs", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.serialize_field(
                "vars",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<TypeVar>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.vars),
            )?;
            __record.serialize_field(
                "attrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Attr>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.attrs),
            )?;
            __record.serialize_field(
                "typedAttrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    ::std::collections::HashMap<__sidex_serde::AsSelf, __sidex_serde::AsSelf>,
                >::new(&self.typed_attrs),
            )?;
            __record.serialize_field(
                "kind",
                &__sidex_serde::SerializeAsWrap::<DefKind, __sidex_serde::AsSelf>::new(&self.kind),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.span),
                    |__v| __sidex_serde::SerializeAsWrap::<Span, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("span", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Def {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Def;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Def")
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
                        __sidex_serde::DeserializeAsWrap<SchemaIdx, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<Ident, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Docs>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<TypeVar>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<Attr>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                            ::std::collections::HashMap<
                                __sidex_serde::AsSelf,
                                __sidex_serde::AsSelf,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<DefKind, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Span>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(7usize, &"record with 8 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Def {
                        schema: __field0,
                        name: __field1,
                        docs: __field2,
                        vars: __field3,
                        attrs: __field4,
                        typed_attrs: __field5,
                        kind: __field6,
                        span: __field7,
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
                    const __IDENTIFIERS: &'static [&'static str] = &[
                        "schema",
                        "name",
                        "docs",
                        "vars",
                        "attrs",
                        "typedAttrs",
                        "kind",
                        "span",
                    ];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"schema\", \"name\", \"docs\", \"vars\", \"attrs\", \"typedAttrs\", \"kind\", \"span\"]";
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
                                4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                                6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                                7u64 => ::core::result::Result::Ok(__Identifier::__Identifier7),
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
                                "schema" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "docs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "vars" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                "typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier5)
                                }
                                "kind" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                                "span" => ::core::result::Result::Ok(__Identifier::__Identifier7),
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
                                b"schema" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"name" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                b"docs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"vars" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                b"attrs" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                b"typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier5)
                                }
                                b"kind" => ::core::result::Result::Ok(__Identifier::__Identifier6),
                                b"span" => ::core::result::Result::Ok(__Identifier::__Identifier7),
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
                    let mut __field0: ::core::option::Option<SchemaIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<Ident> = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<TypeVar>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<
                        ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    > = ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<DefKind> =
                        ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<::std::option::Option<Span>> =
                        ::core::option::Option::None;
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
                                        __sidex_serde::DeserializeAsWrap<
                                            SchemaIdx,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            Ident,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("docs"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Docs>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("vars"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<TypeVar>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "attrs",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Attr>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "typedAttrs",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                ::serde_json::Value,
                                            >,
                                            ::std::collections::HashMap<
                                                __sidex_serde::AsSelf,
                                                __sidex_serde::AsSelf,
                                            >,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("kind"),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            DefKind,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Span>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("schema"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("vars"),
                            );
                        }
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("attrs"),
                            );
                        }
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("typedAttrs"),
                            );
                        }
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("kind"),
                            );
                        }
                    };
                    let __field7 = match __field7 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Def {
                        schema: __field0,
                        name: __field1,
                        docs: __field2,
                        vars: __field3,
                        attrs: __field4,
                        typed_attrs: __field5,
                        kind: __field6,
                        span: __field7,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "schema",
                "name",
                "docs",
                "vars",
                "attrs",
                "typedAttrs",
                "kind",
                "span",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Def",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A type variable of a definition.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TypeVar {
        #[doc = "The name of the type variable.\n"]
        pub name: Ident,
    }
    impl TypeVar {
        #[doc = "Creates a new [`TypeVar`]."]
        pub fn new(name: Ident) -> Self {
            Self { name }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Ident) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Ident) -> Self {
            self.name = name;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for TypeVar {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for TypeVar {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "TypeVar", 1usize)?;
            __record.serialize_field(
                "name",
                &__sidex_serde::SerializeAsWrap::<Ident, __sidex_serde::AsSelf>::new(&self.name),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeVar {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = TypeVar;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record TypeVar")
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
                        __sidex_serde::DeserializeAsWrap<Ident, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TypeVar { name: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["name"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"name\"]";
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
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                    let mut __field0: ::core::option::Option<Ident> = ::core::option::Option::None;
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
                                            Ident,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TypeVar { name: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "TypeVar",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A definition kind.\n"]
    #[derive(Clone, Debug)]
    pub enum DefKind {
        #[doc = "Definition of a type alias.\n"]
        TypeAlias(TypeAliasDef),
        #[doc = "Definition of an opaque type.\n"]
        OpaqueType(OpaqueTypeDef),
        #[doc = "Definition of a record type.\n"]
        RecordType(RecordTypeDef),
        #[doc = "Definition of a variant type.\n"]
        VariantType(VariantTypeDef),
        #[doc = "Definition of a wrapper type.\n"]
        WrapperType(WrapperTypeDef),
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for DefKind {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for DefKind {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "DefKind");
            match self {
                Self::TypeAlias(__value) => {
                    __serializer.serialize_internally_tagged(
                        "tag",
                        "TypeAlias",
                        0u32,
                        &__sidex_serde::SerializeAsWrap::<TypeAliasDef, __sidex_serde::AsSelf>::new(
                            __value,
                        ),
                    )
                }
                Self::OpaqueType(__value) => __serializer.serialize_internally_tagged(
                    "tag",
                    "OpaqueType",
                    1u32,
                    &__sidex_serde::SerializeAsWrap::<OpaqueTypeDef, __sidex_serde::AsSelf>::new(
                        __value,
                    ),
                ),
                Self::RecordType(__value) => __serializer.serialize_internally_tagged(
                    "tag",
                    "RecordType",
                    2u32,
                    &__sidex_serde::SerializeAsWrap::<RecordTypeDef, __sidex_serde::AsSelf>::new(
                        __value,
                    ),
                ),
                Self::VariantType(__value) => __serializer.serialize_internally_tagged(
                    "tag",
                    "VariantType",
                    3u32,
                    &__sidex_serde::SerializeAsWrap::<VariantTypeDef, __sidex_serde::AsSelf>::new(
                        __value,
                    ),
                ),
                Self::WrapperType(__value) => __serializer.serialize_internally_tagged(
                    "tag",
                    "WrapperType",
                    4u32,
                    &__sidex_serde::SerializeAsWrap::<WrapperTypeDef, __sidex_serde::AsSelf>::new(
                        __value,
                    ),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DefKind {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "TypeAlias",
                "OpaqueType",
                "RecordType",
                "VariantType",
                "WrapperType",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"TypeAlias\", \"OpaqueType\", \"RecordType\", \"VariantType\", \"WrapperType\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
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
                        "TypeAlias" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "OpaqueType" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "RecordType" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "VariantType" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "WrapperType" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                        b"TypeAlias" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"OpaqueType" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"RecordType" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"VariantType" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"WrapperType" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                "TypeAlias",
                "OpaqueType",
                "RecordType",
                "VariantType",
                "WrapperType",
            ];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(DefKind::TypeAlias(
                            __tagged
                                .deserialize_internally_tagged::<__sidex_serde::DeserializeAsWrap<
                                    TypeAliasDef,
                                    __sidex_serde::AsSelf,
                                >, __D::Error>()?
                                .into_inner(),
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(DefKind::OpaqueType(
                            __tagged
                                .deserialize_internally_tagged::<__sidex_serde::DeserializeAsWrap<
                                    OpaqueTypeDef,
                                    __sidex_serde::AsSelf,
                                >, __D::Error>()?
                                .into_inner(),
                        ))
                    }
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(DefKind::RecordType(
                            __tagged
                                .deserialize_internally_tagged::<__sidex_serde::DeserializeAsWrap<
                                    RecordTypeDef,
                                    __sidex_serde::AsSelf,
                                >, __D::Error>()?
                                .into_inner(),
                        ))
                    }
                    __Identifier::__Identifier3 => {
                        ::core::result::Result::Ok(DefKind::VariantType(
                            __tagged
                                .deserialize_internally_tagged::<__sidex_serde::DeserializeAsWrap<
                                    VariantTypeDef,
                                    __sidex_serde::AsSelf,
                                >, __D::Error>()?
                                .into_inner(),
                        ))
                    }
                    __Identifier::__Identifier4 => {
                        ::core::result::Result::Ok(DefKind::WrapperType(
                            __tagged
                                .deserialize_internally_tagged::<__sidex_serde::DeserializeAsWrap<
                                    WrapperTypeDef,
                                    __sidex_serde::AsSelf,
                                >, __D::Error>()?
                                .into_inner(),
                        ))
                    }
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = DefKind;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum DefKind")
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
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        TypeAliasDef,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::TypeAlias(__value.into_inner()))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        OpaqueTypeDef,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::OpaqueType(
                                    __value.into_inner(),
                                ))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        RecordTypeDef,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::RecordType(
                                    __value.into_inner(),
                                ))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        VariantTypeDef,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::VariantType(
                                    __value.into_inner(),
                                ))
                            }
                            (__Identifier::__Identifier4, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        WrapperTypeDef,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::WrapperType(
                                    __value.into_inner(),
                                ))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "DefKind",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "A definition of a type alias.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TypeAliasDef {
        #[doc = "The type that is aliased.\n"]
        pub aliased: Type,
    }
    impl TypeAliasDef {
        #[doc = "Creates a new [`TypeAliasDef`]."]
        pub fn new(aliased: Type) -> Self {
            Self { aliased }
        }
        #[doc = "Sets the value of `aliased`."]
        pub fn set_aliased(&mut self, aliased: Type) -> &mut Self {
            self.aliased = aliased;
            self
        }
        #[doc = "Sets the value of `aliased`."]
        pub fn with_aliased(mut self, aliased: Type) -> Self {
            self.aliased = aliased;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for TypeAliasDef {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for TypeAliasDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "TypeAliasDef", 1usize)?;
            __record.serialize_field(
                "aliased",
                &__sidex_serde::SerializeAsWrap::<Type, __sidex_serde::AsSelf>::new(&self.aliased),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeAliasDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = TypeAliasDef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record TypeAliasDef")
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
                        __sidex_serde::DeserializeAsWrap<Type, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TypeAliasDef { aliased: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["aliased"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"aliased\"]";
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
                                "aliased" => {
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
                                b"aliased" => {
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
                    let mut __field0: ::core::option::Option<Type> = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "aliased",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            Type,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("aliased"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TypeAliasDef { aliased: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["aliased"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "TypeAliasDef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A definition of an opaque type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct OpaqueTypeDef {}
    impl OpaqueTypeDef {
        #[doc = "Creates a new [`OpaqueTypeDef`]."]
        pub fn new() -> Self {
            Self {}
        }
    }
    impl ::std::default::Default for OpaqueTypeDef {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for OpaqueTypeDef {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for OpaqueTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "OpaqueTypeDef", 0usize)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for OpaqueTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = OpaqueTypeDef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record OpaqueTypeDef")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    ::core::result::Result::Ok(OpaqueTypeDef {})
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
                    const __IDENTIFIERS: &'static [&'static str] = &[];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in []";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
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
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    ::core::result::Result::Ok(OpaqueTypeDef {})
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "OpaqueTypeDef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A definition of a record type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct RecordTypeDef {
        #[doc = "The fields of the record type.\n"]
        pub fields: ::std::vec::Vec<Field>,
    }
    impl RecordTypeDef {
        #[doc = "Creates a new [`RecordTypeDef`]."]
        pub fn new() -> Self {
            Self {
                fields: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `fields`."]
        pub fn set_fields(&mut self, fields: ::std::vec::Vec<Field>) -> &mut Self {
            self.fields = fields;
            self
        }
        #[doc = "Sets the value of `fields`."]
        pub fn with_fields(mut self, fields: ::std::vec::Vec<Field>) -> Self {
            self.fields = fields;
            self
        }
    }
    impl ::std::default::Default for RecordTypeDef {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for RecordTypeDef {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for RecordTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "RecordTypeDef", 1usize)?;
            __record.serialize_field(
                "fields",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Field>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.fields),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RecordTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = RecordTypeDef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record RecordTypeDef")
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
                            ::std::vec::Vec<Field>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                    ::core::result::Result::Ok(RecordTypeDef { fields: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["fields"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"fields\"]";
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
                                "fields" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                                b"fields" => {
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
                    let mut __field0: ::core::option::Option<::std::vec::Vec<Field>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "fields",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Field>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(RecordTypeDef { fields: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["fields"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "RecordTypeDef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A field of a record type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Field {
        #[doc = "The name of the field.\n"]
        pub name: Ident,
        #[doc = "The documentation of the field.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The attributes of the field.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The parsed, schema-validated attributes, keyed by plugin id.\n"]
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        #[doc = "The type of the field.\n"]
        pub typ: Type,
        #[doc = "Indicates whether the field is optional.\n"]
        pub is_optional: bool,
        #[doc = "The optional span of the field.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Field {
        #[doc = "Creates a new [`Field`]."]
        pub fn new(name: Ident, typ: Type) -> Self {
            Self {
                name,
                typ,
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                typed_attrs: ::std::default::Default::default(),
                is_optional: ::std::default::Default::default(),
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Ident) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Ident) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::option::Option<Docs>) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::option::Option<Docs>) -> Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn set_attrs(&mut self, attrs: ::std::vec::Vec<Attr>) -> &mut Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn with_attrs(mut self, attrs: ::std::vec::Vec<Attr>) -> Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn set_typed_attrs(
            &mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typ`."]
        pub fn set_typ(&mut self, typ: Type) -> &mut Self {
            self.typ = typ;
            self
        }
        #[doc = "Sets the value of `typ`."]
        pub fn with_typ(mut self, typ: Type) -> Self {
            self.typ = typ;
            self
        }
        #[doc = "Sets the value of `is_optional`."]
        pub fn set_is_optional(&mut self, is_optional: bool) -> &mut Self {
            self.is_optional = is_optional;
            self
        }
        #[doc = "Sets the value of `is_optional`."]
        pub fn with_is_optional(mut self, is_optional: bool) -> Self {
            self.is_optional = is_optional;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn set_span(&mut self, span: ::std::option::Option<Span>) -> &mut Self {
            self.span = span;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn with_span(mut self, span: ::std::option::Option<Span>) -> Self {
            self.span = span;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Field {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Field {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Field", 7usize)?;
            __record.serialize_field(
                "name",
                &__sidex_serde::SerializeAsWrap::<Ident, __sidex_serde::AsSelf>::new(&self.name),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.docs),
                    |__v| __sidex_serde::SerializeAsWrap::<Docs, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("docs", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.serialize_field(
                "attrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Attr>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.attrs),
            )?;
            __record.serialize_field(
                "typedAttrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    ::std::collections::HashMap<__sidex_serde::AsSelf, __sidex_serde::AsSelf>,
                >::new(&self.typed_attrs),
            )?;
            __record.serialize_field(
                "typ",
                &__sidex_serde::SerializeAsWrap::<Type, __sidex_serde::AsSelf>::new(&self.typ),
            )?;
            __record.serialize_field(
                "isOptional",
                &__sidex_serde::SerializeAsWrap::<bool, __sidex_serde::AsSelf>::new(
                    &self.is_optional,
                ),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.span),
                    |__v| __sidex_serde::SerializeAsWrap::<Span, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("span", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Field {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Field;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Field")
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
                        __sidex_serde::DeserializeAsWrap<Ident, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Docs>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<Attr>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                            ::std::collections::HashMap<
                                __sidex_serde::AsSelf,
                                __sidex_serde::AsSelf,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<Type, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<bool, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 7 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Span>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 7 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Field {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typed_attrs: __field3,
                        typ: __field4,
                        is_optional: __field5,
                        span: __field6,
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
                    const __IDENTIFIERS: &'static [&'static str] = &[
                        "name",
                        "docs",
                        "attrs",
                        "typedAttrs",
                        "typ",
                        "isOptional",
                        "span",
                    ];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"name\", \"docs\", \"attrs\", \"typedAttrs\", \"typ\", \"isOptional\", \"span\"]";
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
                                4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                                6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
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
                                "docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                "typ" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                "isOptional" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier5)
                                }
                                "span" => ::core::result::Result::Ok(__Identifier::__Identifier6),
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
                                b"docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                b"attrs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                b"typ" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                b"isOptional" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier5)
                                }
                                b"span" => ::core::result::Result::Ok(__Identifier::__Identifier6),
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
                    let mut __field0: ::core::option::Option<Ident> = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<Type> = ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<bool> = ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<::std::option::Option<Span>> =
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
                                            Ident,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("docs"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Docs>,
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
                                            "attrs",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Attr>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "typedAttrs",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                ::serde_json::Value,
                                            >,
                                            ::std::collections::HashMap<
                                                __sidex_serde::AsSelf,
                                                __sidex_serde::AsSelf,
                                            >,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("typ"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            Type,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "isOptional",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            bool,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Span>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("attrs"),
                            );
                        }
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("typedAttrs"),
                            );
                        }
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("typ"),
                            );
                        }
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("isOptional"),
                            );
                        }
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Field {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typed_attrs: __field3,
                        typ: __field4,
                        is_optional: __field5,
                        span: __field6,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "name",
                "docs",
                "attrs",
                "typedAttrs",
                "typ",
                "isOptional",
                "span",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Field",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A definition of a variant type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct VariantTypeDef {
        #[doc = "The variants of the variant type.\n"]
        pub variants: ::std::vec::Vec<Variant>,
    }
    impl VariantTypeDef {
        #[doc = "Creates a new [`VariantTypeDef`]."]
        pub fn new() -> Self {
            Self {
                variants: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `variants`."]
        pub fn set_variants(&mut self, variants: ::std::vec::Vec<Variant>) -> &mut Self {
            self.variants = variants;
            self
        }
        #[doc = "Sets the value of `variants`."]
        pub fn with_variants(mut self, variants: ::std::vec::Vec<Variant>) -> Self {
            self.variants = variants;
            self
        }
    }
    impl ::std::default::Default for VariantTypeDef {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for VariantTypeDef {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for VariantTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "VariantTypeDef", 1usize)?;
            __record.serialize_field(
                "variants",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Variant>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.variants),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for VariantTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = VariantTypeDef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record VariantTypeDef")
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
                            ::std::vec::Vec<Variant>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                    ::core::result::Result::Ok(VariantTypeDef { variants: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["variants"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"variants\"]";
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
                                "variants" => {
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
                                b"variants" => {
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
                    let mut __field0: ::core::option::Option<::std::vec::Vec<Variant>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "variants",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Variant>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("variants"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(VariantTypeDef { variants: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["variants"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "VariantTypeDef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A variant of a variant type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Variant {
        #[doc = "The name of the variant.\n"]
        pub name: Ident,
        #[doc = "The documentation of the variant.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The attributes of the variant.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The parsed, schema-validated attributes, keyed by plugin id.\n"]
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        #[doc = "The optional payload type of the variant.\n"]
        pub typ: ::std::option::Option<Type>,
        #[doc = "The optional span of the variant.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Variant {
        #[doc = "Creates a new [`Variant`]."]
        pub fn new(name: Ident) -> Self {
            Self {
                name,
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                typed_attrs: ::std::default::Default::default(),
                typ: ::std::default::Default::default(),
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Ident) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Ident) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::option::Option<Docs>) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::option::Option<Docs>) -> Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn set_attrs(&mut self, attrs: ::std::vec::Vec<Attr>) -> &mut Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `attrs`."]
        pub fn with_attrs(mut self, attrs: ::std::vec::Vec<Attr>) -> Self {
            self.attrs = attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn set_typed_attrs(
            &mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typ`."]
        pub fn set_typ(&mut self, typ: ::std::option::Option<Type>) -> &mut Self {
            self.typ = typ;
            self
        }
        #[doc = "Sets the value of `typ`."]
        pub fn with_typ(mut self, typ: ::std::option::Option<Type>) -> Self {
            self.typ = typ;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn set_span(&mut self, span: ::std::option::Option<Span>) -> &mut Self {
            self.span = span;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn with_span(mut self, span: ::std::option::Option<Span>) -> Self {
            self.span = span;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Variant {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Variant {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Variant", 6usize)?;
            __record.serialize_field(
                "name",
                &__sidex_serde::SerializeAsWrap::<Ident, __sidex_serde::AsSelf>::new(&self.name),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.docs),
                    |__v| __sidex_serde::SerializeAsWrap::<Docs, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("docs", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.serialize_field(
                "attrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Attr>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.attrs),
            )?;
            __record.serialize_field(
                "typedAttrs",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    ::std::collections::HashMap<__sidex_serde::AsSelf, __sidex_serde::AsSelf>,
                >::new(&self.typed_attrs),
            )?;
            {
                let __wrapped =
                    ::core::option::Option::map(::core::option::Option::as_ref(&self.typ), |__v| {
                        __sidex_serde::SerializeAsWrap::<Type, __sidex_serde::AsSelf>::new(__v)
                    });
                __record
                    .serialize_optional_field("typ", ::core::option::Option::as_ref(&__wrapped))?;
            }
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.span),
                    |__v| __sidex_serde::SerializeAsWrap::<Span, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("span", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Variant {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Variant;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Variant")
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
                        __sidex_serde::DeserializeAsWrap<Ident, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Docs>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::vec::Vec<Attr>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                            ::std::collections::HashMap<
                                __sidex_serde::AsSelf,
                                __sidex_serde::AsSelf,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Type>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        __sidex_serde::DeserializeAsWrap<
                            ::std::option::Option<Span>,
                            ::std::option::Option<__sidex_serde::AsSelf>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 6 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Variant {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typed_attrs: __field3,
                        typ: __field4,
                        span: __field5,
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
                        &["name", "docs", "attrs", "typedAttrs", "typ", "span"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"name\", \"docs\", \"attrs\", \"typedAttrs\", \"typ\", \"span\"]";
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
                                4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                                "docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                "typ" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                "span" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                                b"docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                b"attrs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"typedAttrs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                b"typ" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                b"span" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                    let mut __field0: ::core::option::Option<Ident> = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::collections::HashMap<::std::string::String, ::serde_json::Value>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::std::option::Option<Type>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::std::option::Option<Span>> =
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
                                            Ident,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("docs"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Docs>,
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
                                            "attrs",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Attr>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "typedAttrs",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                ::serde_json::Value,
                                            >,
                                            ::std::collections::HashMap<
                                                __sidex_serde::AsSelf,
                                                __sidex_serde::AsSelf,
                                            >,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("typ"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Type>,
                                            ::std::option::Option<__sidex_serde::AsSelf>,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Span>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("attrs"),
                            );
                        }
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("typedAttrs"),
                            );
                        }
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Variant {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typed_attrs: __field3,
                        typ: __field4,
                        span: __field5,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["name", "docs", "attrs", "typedAttrs", "typ", "span"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Variant",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A definition of a wrapper type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct WrapperTypeDef {
        #[doc = "The type that is wrapped.\n"]
        pub wrapped: Type,
    }
    impl WrapperTypeDef {
        #[doc = "Creates a new [`WrapperTypeDef`]."]
        pub fn new(wrapped: Type) -> Self {
            Self { wrapped }
        }
        #[doc = "Sets the value of `wrapped`."]
        pub fn set_wrapped(&mut self, wrapped: Type) -> &mut Self {
            self.wrapped = wrapped;
            self
        }
        #[doc = "Sets the value of `wrapped`."]
        pub fn with_wrapped(mut self, wrapped: Type) -> Self {
            self.wrapped = wrapped;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for WrapperTypeDef {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for WrapperTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "WrapperTypeDef", 1usize)?;
            __record.serialize_field(
                "wrapped",
                &__sidex_serde::SerializeAsWrap::<Type, __sidex_serde::AsSelf>::new(&self.wrapped),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for WrapperTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = WrapperTypeDef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record WrapperTypeDef")
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
                        __sidex_serde::DeserializeAsWrap<Type, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(WrapperTypeDef { wrapped: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["wrapped"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"wrapped\"]";
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
                                "wrapped" => {
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
                                b"wrapped" => {
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
                    let mut __field0: ::core::option::Option<Type> = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "wrapped",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            Type,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("wrapped"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(WrapperTypeDef { wrapped: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["wrapped"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "WrapperTypeDef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Type {
        #[doc = "The kind of the type.\n"]
        pub kind: TypeKind,
        #[doc = "The span of the type expression.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Type {
        #[doc = "Creates a new [`Type`]."]
        pub fn new(kind: TypeKind) -> Self {
            Self {
                kind,
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: TypeKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: TypeKind) -> Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn set_span(&mut self, span: ::std::option::Option<Span>) -> &mut Self {
            self.span = span;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn with_span(mut self, span: ::std::option::Option<Span>) -> Self {
            self.span = span;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Type {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Type {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Type", 2usize)?;
            __record.serialize_field(
                "kind",
                &__sidex_serde::SerializeAsWrap::<TypeKind, __sidex_serde::AsSelf>::new(&self.kind),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.span),
                    |__v| __sidex_serde::SerializeAsWrap::<Span, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("span", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Type {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
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
                    ::core::fmt::Formatter::write_str(__formatter, "record Type")
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
                        __sidex_serde::DeserializeAsWrap<TypeKind, __sidex_serde::AsSelf>,
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
                            ::std::option::Option<Span>,
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
                    ::core::result::Result::Ok(Type {
                        kind: __field0,
                        span: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["kind", "span"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"kind\", \"span\"]";
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
                                "kind" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "span" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"kind" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"span" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                    let mut __field0: ::core::option::Option<TypeKind> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<Span>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("kind"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            TypeKind,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Span>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("kind"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Type {
                        kind: __field0,
                        span: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["kind", "span"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Type",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An abstract type kind.\n\nBuilt-in types like `[T]`, `[K: V]`, and `()` are represented as\ninstantiations of `core::builtins::Sequence`, `core::builtins::Map`,\nand `core::builtins::unit`. There is no separate variant for them.\n"]
    #[derive(Clone, Debug)]
    pub enum TypeKind {
        #[doc = "A type to be determined via substitution of the respective type variable.\n"]
        TypeVar(TypeVarType),
        #[doc = "An instantiation of a type defined in some schema.\n"]
        Instance(InstanceType),
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for TypeKind {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for TypeKind {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "TypeKind");
            match self {
                Self::TypeVar(__value) => {
                    __serializer.serialize_internally_tagged(
                        "tag",
                        "TypeVar",
                        0u32,
                        &__sidex_serde::SerializeAsWrap::<TypeVarType, __sidex_serde::AsSelf>::new(
                            __value,
                        ),
                    )
                }
                Self::Instance(__value) => {
                    __serializer.serialize_internally_tagged(
                        "tag",
                        "Instance",
                        1u32,
                        &__sidex_serde::SerializeAsWrap::<InstanceType, __sidex_serde::AsSelf>::new(
                            __value,
                        ),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeKind {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["TypeVar", "Instance"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"TypeVar\", \"Instance\"]";
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
                        "TypeVar" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Instance" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"TypeVar" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Instance" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            const __VARIANTS: &'static [&'static str] = &["TypeVar", "Instance"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(TypeKind::TypeVar(
                            __tagged
                                .deserialize_internally_tagged::<__sidex_serde::DeserializeAsWrap<
                                    TypeVarType,
                                    __sidex_serde::AsSelf,
                                >, __D::Error>()?
                                .into_inner(),
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(TypeKind::Instance(
                            __tagged
                                .deserialize_internally_tagged::<__sidex_serde::DeserializeAsWrap<
                                    InstanceType,
                                    __sidex_serde::AsSelf,
                                >, __D::Error>()?
                                .into_inner(),
                        ))
                    }
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = TypeKind;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum TypeKind")
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
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        TypeVarType,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(TypeKind::TypeVar(__value.into_inner()))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        InstanceType,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(TypeKind::Instance(__value.into_inner()))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "TypeKind",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "A type to be determined via substitution of the respective type variable.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TypeVarType {
        #[doc = "The index of the type variable in the enclosing definition.\n"]
        pub idx: TypeVarIdx,
    }
    impl TypeVarType {
        #[doc = "Creates a new [`TypeVarType`]."]
        pub fn new(idx: TypeVarIdx) -> Self {
            Self { idx }
        }
        #[doc = "Sets the value of `idx`."]
        pub fn set_idx(&mut self, idx: TypeVarIdx) -> &mut Self {
            self.idx = idx;
            self
        }
        #[doc = "Sets the value of `idx`."]
        pub fn with_idx(mut self, idx: TypeVarIdx) -> Self {
            self.idx = idx;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for TypeVarType {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for TypeVarType {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "TypeVarType", 1usize)?;
            __record.serialize_field(
                "idx",
                &__sidex_serde::SerializeAsWrap::<TypeVarIdx, __sidex_serde::AsSelf>::new(
                    &self.idx,
                ),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeVarType {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = TypeVarType;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record TypeVarType")
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
                        __sidex_serde::DeserializeAsWrap<TypeVarIdx, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TypeVarType { idx: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["idx"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"idx\"]";
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
                                "idx" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                                b"idx" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                    let mut __field0: ::core::option::Option<TypeVarIdx> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("idx"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            TypeVarIdx,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("idx"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TypeVarType { idx: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["idx"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "TypeVarType",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An instantiation of a type defined in some schema.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct InstanceType {
        #[doc = "The definition being instantiated.\n"]
        pub def: DefRef,
        #[doc = "Substitutions for the type variables of the definition.\n"]
        pub subst: ::std::vec::Vec<Type>,
    }
    impl InstanceType {
        #[doc = "Creates a new [`InstanceType`]."]
        pub fn new(def: DefRef) -> Self {
            Self {
                def,
                subst: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `def`."]
        pub fn set_def(&mut self, def: DefRef) -> &mut Self {
            self.def = def;
            self
        }
        #[doc = "Sets the value of `def`."]
        pub fn with_def(mut self, def: DefRef) -> Self {
            self.def = def;
            self
        }
        #[doc = "Sets the value of `subst`."]
        pub fn set_subst(&mut self, subst: ::std::vec::Vec<Type>) -> &mut Self {
            self.subst = subst;
            self
        }
        #[doc = "Sets the value of `subst`."]
        pub fn with_subst(mut self, subst: ::std::vec::Vec<Type>) -> Self {
            self.subst = subst;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for InstanceType {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for InstanceType {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "InstanceType", 2usize)?;
            __record.serialize_field(
                "def",
                &__sidex_serde::SerializeAsWrap::<DefRef, __sidex_serde::AsSelf>::new(&self.def),
            )?;
            __record.serialize_field(
                "subst",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Type>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.subst),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for InstanceType {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = InstanceType;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record InstanceType")
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
                        __sidex_serde::DeserializeAsWrap<DefRef, __sidex_serde::AsSelf>,
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
                            ::std::vec::Vec<Type>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                    ::core::result::Result::Ok(InstanceType {
                        def: __field0,
                        subst: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["def", "subst"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"def\", \"subst\"]";
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
                                "def" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "subst" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"def" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"subst" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                    let mut __field0: ::core::option::Option<DefRef> = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::vec::Vec<Type>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("def"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            DefRef,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "subst",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Type>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("def"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("subst"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(InstanceType {
                        def: __field0,
                        subst: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["def", "subst"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "InstanceType",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A *span* identifies a range of text in a source.\n"]
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub struct Span {
        #[doc = "The source.\n"]
        pub src: SourceIdx,
        #[doc = "The start character.\n"]
        pub start: usize,
        #[doc = "The included end character.\n"]
        pub end: usize,
    }
    impl Span {
        #[doc = "Creates a new [`Span`]."]
        pub fn new(src: SourceIdx, start: usize, end: usize) -> Self {
            Self { src, start, end }
        }
        #[doc = "Sets the value of `src`."]
        pub fn set_src(&mut self, src: SourceIdx) -> &mut Self {
            self.src = src;
            self
        }
        #[doc = "Sets the value of `src`."]
        pub fn with_src(mut self, src: SourceIdx) -> Self {
            self.src = src;
            self
        }
        #[doc = "Sets the value of `start`."]
        pub fn set_start(&mut self, start: usize) -> &mut Self {
            self.start = start;
            self
        }
        #[doc = "Sets the value of `start`."]
        pub fn with_start(mut self, start: usize) -> Self {
            self.start = start;
            self
        }
        #[doc = "Sets the value of `end`."]
        pub fn set_end(&mut self, end: usize) -> &mut Self {
            self.end = end;
            self
        }
        #[doc = "Sets the value of `end`."]
        pub fn with_end(mut self, end: usize) -> Self {
            self.end = end;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Span {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Span {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Span", 3usize)?;
            __record.serialize_field(
                "src",
                &__sidex_serde::SerializeAsWrap::<SourceIdx, __sidex_serde::AsSelf>::new(&self.src),
            )?;
            __record.serialize_field(
                "start",
                &__sidex_serde::SerializeAsWrap::<usize, __sidex_serde::AsSelf>::new(&self.start),
            )?;
            __record.serialize_field(
                "end",
                &__sidex_serde::SerializeAsWrap::<usize, __sidex_serde::AsSelf>::new(&self.end),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Span {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Span;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Span")
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
                        __sidex_serde::DeserializeAsWrap<SourceIdx, __sidex_serde::AsSelf>,
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
                        __sidex_serde::DeserializeAsWrap<usize, __sidex_serde::AsSelf>,
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
                        __sidex_serde::DeserializeAsWrap<usize, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Span {
                        src: __field0,
                        start: __field1,
                        end: __field2,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["src", "start", "end"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"src\", \"start\", \"end\"]";
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
                                "src" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "start" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "end" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                                b"src" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"start" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                b"end" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                    let mut __field0: ::core::option::Option<SourceIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<usize> = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<usize> = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("src"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            SourceIdx,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "start",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            usize,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            usize,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("src"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("start"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("end"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Span {
                        src: __field0,
                        start: __field1,
                        end: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["src", "start", "end"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Span",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An attribute.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Attr {
        #[doc = "The kind of the attribute.\n"]
        pub kind: AttrKind,
        #[doc = "The span of the attribute.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Attr {
        #[doc = "Creates a new [`Attr`]."]
        pub fn new(kind: AttrKind) -> Self {
            Self {
                kind,
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: AttrKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: AttrKind) -> Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn set_span(&mut self, span: ::std::option::Option<Span>) -> &mut Self {
            self.span = span;
            self
        }
        #[doc = "Sets the value of `span`."]
        pub fn with_span(mut self, span: ::std::option::Option<Span>) -> Self {
            self.span = span;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for Attr {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for Attr {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Attr", 2usize)?;
            __record.serialize_field(
                "kind",
                &__sidex_serde::SerializeAsWrap::<AttrKind, __sidex_serde::AsSelf>::new(&self.kind),
            )?;
            {
                let __wrapped = ::core::option::Option::map(
                    ::core::option::Option::as_ref(&self.span),
                    |__v| __sidex_serde::SerializeAsWrap::<Span, __sidex_serde::AsSelf>::new(__v),
                );
                __record
                    .serialize_optional_field("span", ::core::option::Option::as_ref(&__wrapped))?;
            }
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Attr {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Attr;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Attr")
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
                        __sidex_serde::DeserializeAsWrap<AttrKind, __sidex_serde::AsSelf>,
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
                            ::std::option::Option<Span>,
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
                    ::core::result::Result::Ok(Attr {
                        kind: __field0,
                        span: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["kind", "span"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"kind\", \"span\"]";
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
                                "kind" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "span" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"kind" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"span" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                    let mut __field0: ::core::option::Option<AttrKind> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<Span>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("kind"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            AttrKind,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::option::Option<Span>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("kind"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Attr {
                        kind: __field0,
                        span: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["kind", "span"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Attr",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "The kinds of attributes.\n"]
    #[derive(Clone, Debug)]
    pub enum AttrKind {
        #[doc = "A bare `::`-separated path, e.g. `pub`.\n"]
        Path(::std::string::String),
        #[doc = "A list attribute of the form `<PATH> ( <ARGS> )` where `<ARGS>` is a\n`,`-separated sequence of arguments.\n"]
        List(AttrList),
        #[doc = "An assign attribute of the form `<PATH> = <VALUE>`.\n"]
        Assign(AttrAssign),
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for AttrKind {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for AttrKind {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "AttrKind");
            match self {
                Self::Path(__value) => {
                    __serializer.serialize_adjacently_tagged(
                        "tag",
                        "path",
                        "Path",
                        0u32,
                        &__sidex_serde::SerializeAsWrap::<
                            ::std::string::String,
                            __sidex_serde::AsSelf,
                        >::new(__value),
                    )
                }
                Self::List(__value) => {
                    __serializer.serialize_internally_tagged(
                        "tag",
                        "List",
                        1u32,
                        &__sidex_serde::SerializeAsWrap::<AttrList, __sidex_serde::AsSelf>::new(
                            __value,
                        ),
                    )
                }
                Self::Assign(__value) => {
                    __serializer.serialize_internally_tagged(
                        "tag",
                        "Assign",
                        2u32,
                        &__sidex_serde::SerializeAsWrap::<AttrAssign, __sidex_serde::AsSelf>::new(
                            __value,
                        ),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for AttrKind {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["Path", "List", "Assign"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"Path\", \"List\", \"Assign\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
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
                        "Path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "List" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "Assign" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"Path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"List" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"Assign" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
            const __VARIANTS: &'static [&'static str] = &["Path", "List", "Assign"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged . tag { __Identifier :: __Identifier0 => { :: core :: result :: Result :: Ok (AttrKind :: Path (__tagged . deserialize_adjacently_tagged :: < __sidex_serde :: DeserializeAsWrap < :: std :: string :: String < > , __sidex_serde :: AsSelf > , __D :: Error , > ("path") ? . into_inner ())) } , __Identifier :: __Identifier1 => { :: core :: result :: Result :: Ok (AttrKind :: List (__tagged . deserialize_internally_tagged :: < __sidex_serde :: DeserializeAsWrap < AttrList < > , __sidex_serde :: AsSelf > , __D :: Error , > () ? . into_inner ())) } , __Identifier :: __Identifier2 => { :: core :: result :: Result :: Ok (AttrKind :: Assign (__tagged . deserialize_internally_tagged :: < __sidex_serde :: DeserializeAsWrap < AttrAssign < > , __sidex_serde :: AsSelf > , __D :: Error , > () ? . into_inner ())) } , }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = AttrKind;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum AttrKind")
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
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        ::std::string::String,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrKind::Path(__value.into_inner()))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        AttrList,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrKind::List(__value.into_inner()))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        AttrAssign,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrKind::Assign(__value.into_inner()))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "AttrKind",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "A list attribute of the form `<PATH> ( <ARGS> )`.\n"]
    #[derive(Clone, Debug)]
    pub struct AttrList {
        #[doc = "The path of the attribute.\n"]
        pub path: ::std::string::String,
        #[doc = "The arguments of the attribute.\n"]
        pub args: ::std::vec::Vec<Attr>,
    }
    impl AttrList {
        #[doc = "Creates a new [`AttrList`]."]
        pub fn new(path: ::std::string::String, args: ::std::vec::Vec<Attr>) -> Self {
            Self { path, args }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(&mut self, path: ::std::string::String) -> &mut Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: ::std::string::String) -> Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `args`."]
        pub fn set_args(&mut self, args: ::std::vec::Vec<Attr>) -> &mut Self {
            self.args = args;
            self
        }
        #[doc = "Sets the value of `args`."]
        pub fn with_args(mut self, args: ::std::vec::Vec<Attr>) -> Self {
            self.args = args;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for AttrList {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for AttrList {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "AttrList", 2usize)?;
            __record . serialize_field ("path" , & __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (& self . path) ,) ? ;
            __record.serialize_field(
                "args",
                &__sidex_serde::SerializeAsWrap::<
                    ::std::vec::Vec<Attr>,
                    ::std::vec::Vec<__sidex_serde::AsSelf>,
                >::new(&self.args),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for AttrList {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = AttrList;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record AttrList")
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
                            ::std::string::String,
                            __sidex_serde::AsSelf,
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
                            ::std::vec::Vec<Attr>,
                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                    ::core::result::Result::Ok(AttrList {
                        path: __field0,
                        args: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["path", "args"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"path\", \"args\"]";
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
                                "path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "args" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"args" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("path"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::string::String,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("args"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::vec::Vec<Attr>,
                                            ::std::vec::Vec<__sidex_serde::AsSelf>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("path"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("args"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(AttrList {
                        path: __field0,
                        args: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["path", "args"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "AttrList",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "An assign attribute of the form `<PATH> = <VALUE>`.\n"]
    #[derive(Clone, Debug)]
    pub struct AttrAssign {
        #[doc = "The path of the attribute.\n"]
        pub path: ::std::string::String,
        #[doc = "The assigned value.\n"]
        pub value: AttrValue,
    }
    impl AttrAssign {
        #[doc = "Creates a new [`AttrAssign`]."]
        pub fn new(path: ::std::string::String, value: AttrValue) -> Self {
            Self { path, value }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(&mut self, path: ::std::string::String) -> &mut Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: ::std::string::String) -> Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn set_value(&mut self, value: AttrValue) -> &mut Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn with_value(mut self, value: AttrValue) -> Self {
            self.value = value;
            self
        }
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for AttrAssign {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for AttrAssign {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "AttrAssign", 2usize)?;
            __record . serialize_field ("path" , & __sidex_serde :: SerializeAsWrap :: < :: std :: string :: String < > , __sidex_serde :: AsSelf > :: new (& self . path) ,) ? ;
            __record.serialize_field(
                "value",
                &__sidex_serde::SerializeAsWrap::<AttrValue, __sidex_serde::AsSelf>::new(
                    &self.value,
                ),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for AttrAssign {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = AttrAssign;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record AttrAssign")
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
                            ::std::string::String,
                            __sidex_serde::AsSelf,
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
                        __sidex_serde::DeserializeAsWrap<AttrValue, __sidex_serde::AsSelf>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value.into_inner(),
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(AttrAssign {
                        path: __field0,
                        value: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["path", "value"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"path\", \"value\"]";
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
                                "path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "value" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"value" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<AttrValue> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("path"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            ::std::string::String,
                                            __sidex_serde::AsSelf,
                                        >,
                                    >(&mut __map)?
                                    .into_inner(),
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "value",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        __sidex_serde::DeserializeAsWrap<
                                            AttrValue,
                                            __sidex_serde::AsSelf,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("path"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("value"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(AttrAssign {
                        path: __field0,
                        value: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["path", "value"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "AttrAssign",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "The value of an assign attribute.\n"]
    #[derive(Clone, Debug)]
    pub enum AttrValue {
        #[doc = "A boolean value.\n"]
        Bool(bool),
        #[doc = "A numeric value (kept as a string to avoid lossy conversion).\n"]
        Number(::std::string::String),
        #[doc = "A string value.\n"]
        String(::std::string::String),
        #[doc = "A `::`-separated path.\n"]
        Path(::std::string::String),
    }
    #[automatically_derived]
    impl __sidex_serde::SidexType for AttrValue {
        type Encoding = __sidex_serde::AsSelf;
    }
    #[automatically_derived]
    impl __serde::Serialize for AttrValue {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "AttrValue");
            match self {
                Self::Bool(__value) => {
                    __serializer.serialize_adjacently_tagged(
                        "tag",
                        "content",
                        "Bool",
                        0u32,
                        &__sidex_serde::SerializeAsWrap::<bool, __sidex_serde::AsSelf>::new(
                            __value,
                        ),
                    )
                }
                Self::Number(__value) => {
                    __serializer.serialize_adjacently_tagged(
                        "tag",
                        "content",
                        "Number",
                        1u32,
                        &__sidex_serde::SerializeAsWrap::<
                            ::std::string::String,
                            __sidex_serde::AsSelf,
                        >::new(__value),
                    )
                }
                Self::String(__value) => {
                    __serializer.serialize_adjacently_tagged(
                        "tag",
                        "content",
                        "String",
                        2u32,
                        &__sidex_serde::SerializeAsWrap::<
                            ::std::string::String,
                            __sidex_serde::AsSelf,
                        >::new(__value),
                    )
                }
                Self::Path(__value) => {
                    __serializer.serialize_adjacently_tagged(
                        "tag",
                        "content",
                        "Path",
                        3u32,
                        &__sidex_serde::SerializeAsWrap::<
                            ::std::string::String,
                            __sidex_serde::AsSelf,
                        >::new(__value),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for AttrValue {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["Bool", "Number", "String", "Path"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"Bool\", \"Number\", \"String\", \"Path\"]";
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
                        "Bool" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Number" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "String" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "Path" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"Number" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"String" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"Path" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
            const __VARIANTS: &'static [&'static str] = &["Bool", "Number", "String", "Path"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged . tag { __Identifier :: __Identifier0 => { :: core :: result :: Result :: Ok (AttrValue :: Bool (__tagged . deserialize_adjacently_tagged :: < __sidex_serde :: DeserializeAsWrap < bool < > , __sidex_serde :: AsSelf > , __D :: Error , > ("content") ? . into_inner ())) } , __Identifier :: __Identifier1 => { :: core :: result :: Result :: Ok (AttrValue :: Number (__tagged . deserialize_adjacently_tagged :: < __sidex_serde :: DeserializeAsWrap < :: std :: string :: String < > , __sidex_serde :: AsSelf > , __D :: Error , > ("content") ? . into_inner ())) } , __Identifier :: __Identifier2 => { :: core :: result :: Result :: Ok (AttrValue :: String (__tagged . deserialize_adjacently_tagged :: < __sidex_serde :: DeserializeAsWrap < :: std :: string :: String < > , __sidex_serde :: AsSelf > , __D :: Error , > ("content") ? . into_inner ())) } , __Identifier :: __Identifier3 => { :: core :: result :: Result :: Ok (AttrValue :: Path (__tagged . deserialize_adjacently_tagged :: < __sidex_serde :: DeserializeAsWrap < :: std :: string :: String < > , __sidex_serde :: AsSelf > , __D :: Error , > ("content") ? . into_inner ())) } , }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = AttrValue;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum AttrValue")
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
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<bool, __sidex_serde::AsSelf>,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrValue::Bool(__value.into_inner()))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        ::std::string::String,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrValue::Number(__value.into_inner()))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        ::std::string::String,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrValue::String(__value.into_inner()))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    __sidex_serde::DeserializeAsWrap<
                                        ::std::string::String,
                                        __sidex_serde::AsSelf,
                                    >,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrValue::Path(__value.into_inner()))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "AttrValue",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
}
