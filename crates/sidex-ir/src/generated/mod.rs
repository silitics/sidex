/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod ir {
    #![doc = ""]
    use :: serde as __serde;
    use :: sidex_serde as __sidex_serde;
    #[doc = "Uniquely identifies a source in a unit.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct SourceIdx(pub(crate) usize);
    impl ::std::convert::From<SourceIdx> for usize {
        fn from(wrapped: SourceIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SourceIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SourceIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(SourceIdx(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = "Uniquely identifies a bundle in a unit.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct BundleIdx(pub(crate) usize);
    impl ::std::convert::From<BundleIdx> for usize {
        fn from(wrapped: BundleIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BundleIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BundleIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(BundleIdx(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = "Uniquely identifies a schema in bundle.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct SchemaIdx(pub(crate) usize);
    impl ::std::convert::From<SchemaIdx> for usize {
        fn from(wrapped: SchemaIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SchemaIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SchemaIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(SchemaIdx(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = "Uniquely identifies a definition in a schema.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct DefIdx(pub(crate) usize);
    impl ::std::convert::From<DefIdx> for usize {
        fn from(wrapped: DefIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for DefIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DefIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(DefIdx(__serde::Deserialize::deserialize(__deserializer)?))
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
    impl __serde::Serialize for TypeVarIdx {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeVarIdx {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(TypeVarIdx(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
        }
    }
    #[doc = "A *unit* is a collection of bundles.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Unit {
        #[doc = "The bundles of the unit.\n"]
        pub bundles: ::std::vec::Vec<Bundle>,
        #[doc = "The sources of the unit.\n"]
        pub sources: SourceStorage,
    }
    impl Unit {
        #[doc = "Creates a new [`Unit`]."]
        pub fn new() -> Self {
            Self {
                bundles: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `sources`."]
        pub fn set_sources(&mut self, sources: SourceStorage) -> &mut Self {
            self.sources = sources;
            self
        }
        #[doc = "Sets the value of `sources`."]
        pub fn with_sources(mut self, sources: SourceStorage) -> Self {
            self.sources = sources;
            self
        }
    }
    impl ::std::default::Default for Unit {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Unit {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct = __serde::Serializer::serialize_struct(__serializer, "Unit", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "bundles",
                &self.bundles,
            )?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "sources",
                &self.sources,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Unit {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["bundles", "sources"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"bundles\", \"sources\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
                        "bundles" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "sources" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"bundles" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"sources" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Unit;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Unit")
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
                        ::std::vec::Vec<Bundle>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 =
                        match __serde::de::SeqAccess::next_element::<SourceStorage>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        1usize,
                                        &"record with 2 fields",
                                    ),
                                );
                            }
                        };
                    ::core::result::Result::Ok(Unit {
                        bundles: __field0,
                        sources: __field1,
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
                    let mut __field0: ::core::option::Option<::std::vec::Vec<Bundle>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<SourceStorage> =
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Bundle>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "sources",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<SourceStorage>(
                                        &mut __map,
                                    )?,
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
                                <__A::Error as __serde::de::Error>::missing_field("sources"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Unit {
                        bundles: __field0,
                        sources: __field1,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["bundles", "sources"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Unit",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A storage for sources.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct SourceStorage {
        #[doc = "The sources.\n"]
        pub sources: ::std::vec::Vec<Source>,
    }
    impl SourceStorage {
        #[doc = "Creates a new [`SourceStorage`]."]
        pub fn new() -> Self {
            Self {
                sources: ::std::default::Default::default(),
            }
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
    }
    impl ::std::default::Default for SourceStorage {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SourceStorage {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "SourceStorage", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "sources",
                &self.sources,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SourceStorage {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["sources"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"sources\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "sources" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        b"sources" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SourceStorage;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SourceStorage")
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
                        ::std::vec::Vec<Source>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SourceStorage { sources: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<::std::vec::Vec<Source>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "sources",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Source>>(
                                        &mut __map,
                                    )?,
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
                                <__A::Error as __serde::de::Error>::missing_field("sources"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SourceStorage { sources: __field0 })
                }
            }
            const __FIELDS: &'static [&'static str] = &["sources"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SourceStorage",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A *source* is simply a chunk of text.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Source {
        #[doc = "The source index of the source.\n"]
        pub idx: SourceIdx,
        #[doc = "The origin of the source, e.g., a filesystem path.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "The optional text of the source.\n\nThis field is optional to allow for programmatically generated schemas.\n"]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl Source {
        #[doc = "Creates a new [`Source`]."]
        pub fn new(idx: SourceIdx) -> Self {
            Self {
                idx,
                origin: ::std::default::Default::default(),
                text: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `idx`."]
        pub fn set_idx(&mut self, idx: SourceIdx) -> &mut Self {
            self.idx = idx;
            self
        }
        #[doc = "Sets the value of `idx`."]
        pub fn with_idx(mut self, idx: SourceIdx) -> Self {
            self.idx = idx;
            self
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
    #[automatically_derived]
    impl __serde::Serialize for Source {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Source", 3usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "idx", &self.idx)?;
            match &self.origin {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "origin",
                        __value,
                    )?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "origin")?;
                }
            }
            match &self.text {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "text", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "text")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Source {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["idx", "origin", "text"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"idx\", \"origin\", \"text\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Unknown,
            }
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
                        "idx" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "origin" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "text" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"origin" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"text" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<SourceIdx>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 3 fields",
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
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
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
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Source {
                        idx: __field0,
                        origin: __field1,
                        text: __field2,
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
                    let mut __field0: ::core::option::Option<SourceIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
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
                                    __serde::de::MapAccess::next_value::<SourceIdx>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "origin",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("text"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
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
                                <__A::Error as __serde::de::Error>::missing_field("idx"),
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
                    ::core::result::Result::Ok(Source {
                        idx: __field0,
                        origin: __field1,
                        text: __field2,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["idx", "origin", "text"];
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
        #[doc = "The index of the bundle.\n"]
        pub idx: BundleIdx,
        #[doc = "The metadata of the bundle.\n"]
        pub metadata: Metadata,
        #[doc = "The dependencies of the bundle.\n"]
        pub dependencies: ::std::vec::Vec<Dependency>,
        #[doc = "The schemas of the bundle.\n"]
        pub schemas: ::std::vec::Vec<Schema>,
    }
    impl Bundle {
        #[doc = "Creates a new [`Bundle`]."]
        pub fn new(idx: BundleIdx, metadata: Metadata) -> Self {
            Self {
                idx,
                metadata,
                dependencies: ::std::default::Default::default(),
                schemas: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `idx`."]
        pub fn set_idx(&mut self, idx: BundleIdx) -> &mut Self {
            self.idx = idx;
            self
        }
        #[doc = "Sets the value of `idx`."]
        pub fn with_idx(mut self, idx: BundleIdx) -> Self {
            self.idx = idx;
            self
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
        #[doc = "Sets the value of `dependencies`."]
        pub fn set_dependencies(&mut self, dependencies: ::std::vec::Vec<Dependency>) -> &mut Self {
            self.dependencies = dependencies;
            self
        }
        #[doc = "Sets the value of `dependencies`."]
        pub fn with_dependencies(mut self, dependencies: ::std::vec::Vec<Dependency>) -> Self {
            self.dependencies = dependencies;
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
    }
    #[automatically_derived]
    impl __serde::Serialize for Bundle {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Bundle", 4usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "idx", &self.idx)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "metadata",
                &self.metadata,
            )?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "dependencies",
                &self.dependencies,
            )?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "schemas",
                &self.schemas,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Bundle {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] =
                &["idx", "metadata", "dependencies", "schemas"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"idx\", \"metadata\", \"dependencies\", \"schemas\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Unknown,
            }
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
                        "idx" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "metadata" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "dependencies" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "schemas" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"metadata" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"dependencies" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"schemas" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<BundleIdx>(&mut __seq)? {
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
                    let __field1 =
                        match __serde::de::SeqAccess::next_element::<Metadata>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        1usize,
                                        &"record with 4 fields",
                                    ),
                                );
                            }
                        };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::std::vec::Vec<Dependency>,
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
                        ::std::vec::Vec<Schema>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Bundle {
                        idx: __field0,
                        metadata: __field1,
                        dependencies: __field2,
                        schemas: __field3,
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
                    let mut __field0: ::core::option::Option<BundleIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<Metadata> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Dependency>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<Schema>> =
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
                                    __serde::de::MapAccess::next_value::<BundleIdx>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "metadata",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Metadata>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "dependencies",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::vec::Vec<Dependency>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "schemas",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Schema>>(
                                        &mut __map,
                                    )?,
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
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("metadata"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("dependencies"),
                            );
                        }
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("schemas"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Bundle {
                        idx: __field0,
                        metadata: __field1,
                        dependencies: __field2,
                        schemas: __field3,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] =
                &["idx", "metadata", "dependencies", "schemas"];
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
    #[doc = "A dependency of a bundle.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Dependency {
        #[doc = "The name of the dependency.\n"]
        pub name: ::std::string::String,
        #[doc = "The bundle.\n"]
        pub bundle: BundleIdx,
    }
    impl Dependency {
        #[doc = "Creates a new [`Dependency`]."]
        pub fn new(name: ::std::string::String, bundle: BundleIdx) -> Self {
            Self { name, bundle }
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
    }
    #[automatically_derived]
    impl __serde::Serialize for Dependency {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Dependency", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "bundle", &self.bundle)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Dependency {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["name", "bundle"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"name\", \"bundle\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
                        "bundle" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"bundle" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Dependency;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Dependency")
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
                    let __field1 =
                        match __serde::de::SeqAccess::next_element::<BundleIdx>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        1usize,
                                        &"record with 2 fields",
                                    ),
                                );
                            }
                        };
                    ::core::result::Result::Ok(Dependency {
                        name: __field0,
                        bundle: __field1,
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
                    let mut __field1: ::core::option::Option<BundleIdx> =
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
                                            "bundle",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<BundleIdx>(&mut __map)?,
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
                                <__A::Error as __serde::de::Error>::missing_field("bundle"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Dependency {
                        name: __field0,
                        bundle: __field1,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["name", "bundle"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Dependency",
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
    impl __serde::Serialize for Metadata {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Metadata", 4usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "version",
                &self.version,
            )?;
            match &self.description {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "description",
                        __value,
                    )?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "description")?;
                }
            }
            match &self.authors {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "authors",
                        __value,
                    )?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "authors")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Metadata {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] =
                &["name", "version", "description", "authors"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"version\", \"description\", \"authors\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Unknown,
            }
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
                        "version" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "authors" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"version" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"description" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"authors" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        ::core::option::Option<::std::string::String>,
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
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::core::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::core::option::Option<::std::vec::Vec<::std::string::String>>,
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
                                            "description",
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
                                            "authors",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
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
    pub struct Identifier {
        #[doc = "The identifier.\n"]
        pub identifier: ::std::string::String,
        #[doc = "The optional span of the identifier.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Identifier {
        #[doc = "Creates a new [`Identifier`]."]
        pub fn new(identifier: ::std::string::String) -> Self {
            Self {
                identifier,
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `identifier`."]
        pub fn set_identifier(&mut self, identifier: ::std::string::String) -> &mut Self {
            self.identifier = identifier;
            self
        }
        #[doc = "Sets the value of `identifier`."]
        pub fn with_identifier(mut self, identifier: ::std::string::String) -> Self {
            self.identifier = identifier;
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
    impl __serde::Serialize for Identifier {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Identifier", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "identifier",
                &self.identifier,
            )?;
            match &self.span {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "span", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "span")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Identifier {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["identifier", "span"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"identifier\", \"span\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
                        "identifier" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        b"identifier" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"span" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Identifier")
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
                        ::core::option::Option<Span>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Identifier {
                        identifier: __field0,
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Span>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "identifier",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("span"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Span>,
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
                                <__A::Error as __serde::de::Error>::missing_field("identifier"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Identifier {
                        identifier: __field0,
                        span: __field1,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["identifier", "span"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Identifier",
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
    impl __serde::Serialize for Docs {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct = __serde::Serializer::serialize_struct(__serializer, "Docs", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "text", &self.text)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Docs {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["text"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"text\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
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
                        ::std::string::String,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
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
    #[doc = "A schema is a collection of definitions.\n\nNote that imports have already been processed and resolved.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Schema {
        #[doc = "The index of the schema.\n"]
        pub idx: SchemaIdx,
        #[doc = "The name of the schema.\n"]
        pub name: ::std::string::String,
        #[doc = "The documentation of the schema.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The attributes of the schema.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The definitions of the schema.\n"]
        pub defs: ::std::vec::Vec<Def>,
        #[doc = "The source of the schema.\n"]
        pub source: ::std::option::Option<SourceIdx>,
    }
    impl Schema {
        #[doc = "Creates a new [`Schema`]."]
        pub fn new(idx: SchemaIdx, name: ::std::string::String) -> Self {
            Self {
                idx,
                name,
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                defs: ::std::default::Default::default(),
                source: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `idx`."]
        pub fn set_idx(&mut self, idx: SchemaIdx) -> &mut Self {
            self.idx = idx;
            self
        }
        #[doc = "Sets the value of `idx`."]
        pub fn with_idx(mut self, idx: SchemaIdx) -> Self {
            self.idx = idx;
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
    }
    #[automatically_derived]
    impl __serde::Serialize for Schema {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Schema", 6usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "idx", &self.idx)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            match &self.docs {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "docs", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "docs")?;
                }
            }
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "attrs", &self.attrs)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "defs", &self.defs)?;
            match &self.source {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "source",
                        __value,
                    )?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "source")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Schema {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] =
                &["idx", "name", "docs", "attrs", "defs", "source"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"idx\", \"name\", \"docs\", \"attrs\", \"defs\", \"source\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
                __Unknown,
            }
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
                        "idx" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "docs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "defs" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "source" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                        b"name" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"docs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"attrs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"defs" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"source" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<SchemaIdx>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 6 fields",
                                    ),
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
                                __serde::de::Error::invalid_length(1usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Docs>,
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
                        ::std::vec::Vec<Attr>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<::std::vec::Vec<Def>>(
                        &mut __seq,
                    )? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 6 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<SourceIdx>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 6 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Schema {
                        idx: __field0,
                        name: __field1,
                        docs: __field2,
                        attrs: __field3,
                        defs: __field4,
                        source: __field5,
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
                    let mut __field0: ::core::option::Option<SchemaIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::core::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::std::vec::Vec<Def>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<::core::option::Option<SourceIdx>> =
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
                                    __serde::de::MapAccess::next_value::<SchemaIdx>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("name"),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("docs"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Docs>,
                                    >(&mut __map)?,
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Attr>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("defs"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Def>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "source",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<SourceIdx>,
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
                                <__A::Error as __serde::de::Error>::missing_field("idx"),
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
                                <__A::Error as __serde::de::Error>::missing_field("defs"),
                            );
                        }
                    };
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Schema {
                        idx: __field0,
                        name: __field1,
                        docs: __field2,
                        attrs: __field3,
                        defs: __field4,
                        source: __field5,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] =
                &["idx", "name", "docs", "attrs", "defs", "source"];
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
        #[doc = "The name of the definition.\n"]
        pub name: Identifier,
        #[doc = "The documentation of the definition.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The type variables of the definition.\n"]
        pub vars: ::std::vec::Vec<TypeVar>,
        #[doc = "The attributes of the definition.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The kind of the definition.\n"]
        pub kind: DefKind,
    }
    impl Def {
        #[doc = "Creates a new [`Def`]."]
        pub fn new(name: Identifier, kind: DefKind) -> Self {
            Self {
                name,
                kind,
                docs: ::std::default::Default::default(),
                vars: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Identifier) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Identifier) -> Self {
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
    }
    #[automatically_derived]
    impl __serde::Serialize for Def {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct = __serde::Serializer::serialize_struct(__serializer, "Def", 5usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            match &self.docs {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "docs", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "docs")?;
                }
            }
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "vars", &self.vars)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "attrs", &self.attrs)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "kind", &self.kind)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Def {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] =
                &["name", "docs", "vars", "attrs", "kind"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"docs\", \"vars\", \"attrs\", \"kind\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Unknown,
            }
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
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "vars" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "kind" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                        b"vars" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"attrs" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"kind" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<Identifier>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 5 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Docs>,
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
                        ::std::vec::Vec<TypeVar>,
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
                        ::std::vec::Vec<Attr>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field4 =
                        match __serde::de::SeqAccess::next_element::<DefKind>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        4usize,
                                        &"record with 5 fields",
                                    ),
                                );
                            }
                        };
                    ::core::result::Result::Ok(Def {
                        name: __field0,
                        docs: __field1,
                        vars: __field2,
                        attrs: __field3,
                        kind: __field4,
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
                    let mut __field0: ::core::option::Option<Identifier> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<TypeVar>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<DefKind> =
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
                                    __serde::de::MapAccess::next_value::<Identifier>(&mut __map)?,
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
                                        ::core::option::Option<Docs>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("vars"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<TypeVar>>(
                                        &mut __map,
                                    )?,
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Attr>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("kind"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<DefKind>(&mut __map)?,
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
                                <__A::Error as __serde::de::Error>::missing_field("vars"),
                            );
                        }
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
                                <__A::Error as __serde::de::Error>::missing_field("kind"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Def {
                        name: __field0,
                        docs: __field1,
                        vars: __field2,
                        attrs: __field3,
                        kind: __field4,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["name", "docs", "vars", "attrs", "kind"];
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
        pub name: Identifier,
    }
    impl TypeVar {
        #[doc = "Creates a new [`TypeVar`]."]
        pub fn new(name: Identifier) -> Self {
            Self { name }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Identifier) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Identifier) -> Self {
            self.name = name;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for TypeVar {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "TypeVar", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeVar {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["name"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"name\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<Identifier>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 1 fields",
                                    ),
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
                    let mut __field0: ::core::option::Option<Identifier> =
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
                                    __serde::de::MapAccess::next_value::<Identifier>(&mut __map)?,
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
        #[doc = "Definition of a service.\n"]
        Service(ServiceDef),
    }
    #[automatically_derived]
    impl __serde::Serialize for DefKind {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            if __serde::Serializer::is_human_readable(&__serializer) {
                match self {
                    Self::TypeAlias(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "DefKind", 2usize)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "TypeAlias",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "aliased",
                            &__value.aliased,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::OpaqueType(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "DefKind", 1usize)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "OpaqueType",
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::RecordType(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "DefKind", 2usize)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "RecordType",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "fields",
                            &__value.fields,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::VariantType(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "DefKind", 2usize)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "VariantType",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "variants",
                            &__value.variants,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::WrapperType(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "DefKind", 2usize)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "WrapperType",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "wrapped",
                            &__value.wrapped,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Service(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "DefKind", 2usize)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Service",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "methods",
                            &__value.methods,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                }
            } else {
                match self {
                    Self::TypeAlias(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "DefKind",
                            0u32,
                            "TypeAlias",
                            __value,
                        )
                    }
                    Self::OpaqueType(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "DefKind",
                            1u32,
                            "OpaqueType",
                            __value,
                        )
                    }
                    Self::RecordType(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "DefKind",
                            2u32,
                            "RecordType",
                            __value,
                        )
                    }
                    Self::VariantType(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "DefKind",
                            3u32,
                            "VariantType",
                            __value,
                        )
                    }
                    Self::WrapperType(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "DefKind",
                            4u32,
                            "WrapperType",
                            __value,
                        )
                    }
                    Self::Service(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "DefKind",
                            5u32,
                            "Service",
                            __value,
                        )
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DefKind {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &[
                "TypeAlias",
                "OpaqueType",
                "RecordType",
                "VariantType",
                "WrapperType",
                "Service",
            ];
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"TypeAlias\", \"OpaqueType\", \"RecordType\", \"VariantType\", \"WrapperType\", \"Service\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
            }
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
                        "TypeAlias" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "OpaqueType" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "RecordType" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "VariantType" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "WrapperType" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "Service" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                        b"Service" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
            const __VARIANTS: &'static [&'static str] = &[
                "TypeAlias",
                "OpaqueType",
                "RecordType",
                "VariantType",
                "WrapperType",
                "Service",
            ];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(DefKind::TypeAlias(
                            __tagged.deserialize_internally_tagged::<TypeAliasDef, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(DefKind::OpaqueType(
                            __tagged
                                .deserialize_internally_tagged::<OpaqueTypeDef, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(DefKind::RecordType(
                            __tagged
                                .deserialize_internally_tagged::<RecordTypeDef, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier3 => {
                        ::core::result::Result::Ok(DefKind::VariantType(
                            __tagged
                                .deserialize_internally_tagged::<VariantTypeDef, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier4 => {
                        ::core::result::Result::Ok(DefKind::WrapperType(
                            __tagged
                                .deserialize_internally_tagged::<WrapperTypeDef, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier5 => {
                        ::core::result::Result::Ok(DefKind::Service(
                            __tagged.deserialize_internally_tagged::<ServiceDef, __D::Error>()?,
                        ))
                    }
                }
            } else {
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = DefKind;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum ExternallyTagged")
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
                                    TypeAliasDef,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::TypeAlias(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    OpaqueTypeDef,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::OpaqueType(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    RecordTypeDef,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::RecordType(__value))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    VariantTypeDef,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::VariantType(__value))
                            }
                            (__Identifier::__Identifier4, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    WrapperTypeDef,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::WrapperType(__value))
                            }
                            (__Identifier::__Identifier5, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ServiceDef,
                                >(__variant)?;
                                ::core::result::Result::Ok(DefKind::Service(__value))
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
    impl __serde::Serialize for TypeAliasDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "TypeAliasDef", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "aliased",
                &self.aliased,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeAliasDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["aliased"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"aliased\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "aliased" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        b"aliased" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                    let __field0 = match __serde::de::SeqAccess::next_element::<Type>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
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
                                    __serde::de::MapAccess::next_value::<Type>(&mut __map)?,
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
    impl __serde::Serialize for OpaqueTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "OpaqueTypeDef", 0usize)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for OpaqueTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &[];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in []";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
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
    impl __serde::Serialize for RecordTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "RecordTypeDef", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "fields", &self.fields)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RecordTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["fields"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"fields\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
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
                        b"fields" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        ::std::vec::Vec<Field>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Field>>(
                                        &mut __map,
                                    )?,
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
        pub name: Identifier,
        #[doc = "The documentation of the field.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The attributes of the field.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The type of the field.\n"]
        pub typ: Type,
        #[doc = "Indicates whether the field is optional.\n"]
        pub is_optional: bool,
    }
    impl Field {
        #[doc = "Creates a new [`Field`]."]
        pub fn new(name: Identifier, typ: Type) -> Self {
            Self {
                name,
                typ,
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                is_optional: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Identifier) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Identifier) -> Self {
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
    }
    #[automatically_derived]
    impl __serde::Serialize for Field {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Field", 5usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            match &self.docs {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "docs", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "docs")?;
                }
            }
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "attrs", &self.attrs)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "typ", &self.typ)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "isOptional",
                &self.is_optional,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Field {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] =
                &["name", "docs", "attrs", "typ", "isOptional"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"docs\", \"attrs\", \"typ\", \"isOptional\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Unknown,
            }
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
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "typ" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "isOptional" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                        b"typ" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"isOptional" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<Identifier>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 5 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Docs>,
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
                        ::std::vec::Vec<Attr>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<Type>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<bool>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Field {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typ: __field3,
                        is_optional: __field4,
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
                    let mut __field0: ::core::option::Option<Identifier> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<Type> = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<bool> = ::core::option::Option::None;
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
                                    __serde::de::MapAccess::next_value::<Identifier>(&mut __map)?,
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
                                        ::core::option::Option<Docs>,
                                    >(&mut __map)?,
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Attr>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("typ"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Type>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "isOptional",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<bool>(&mut __map)?,
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
                                <__A::Error as __serde::de::Error>::missing_field("typ"),
                            );
                        }
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("isOptional"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Field {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typ: __field3,
                        is_optional: __field4,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] =
                &["name", "docs", "attrs", "typ", "isOptional"];
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
    impl __serde::Serialize for VariantTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "VariantTypeDef", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "variants",
                &self.variants,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for VariantTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["variants"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"variants\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "variants" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        b"variants" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        ::std::vec::Vec<Variant>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Variant>>(
                                        &mut __map,
                                    )?,
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
        pub name: Identifier,
        #[doc = "The documentation of the variant.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The attributes of the variant.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The optional type of the variant.\n"]
        pub typ: ::std::option::Option<Type>,
    }
    impl Variant {
        #[doc = "Creates a new [`Variant`]."]
        pub fn new(name: Identifier) -> Self {
            Self {
                name,
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                typ: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Identifier) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Identifier) -> Self {
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
    }
    #[automatically_derived]
    impl __serde::Serialize for Variant {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Variant", 4usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            match &self.docs {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "docs", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "docs")?;
                }
            }
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "attrs", &self.attrs)?;
            match &self.typ {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "typ", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "typ")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Variant {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["name", "docs", "attrs", "typ"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"docs\", \"attrs\", \"typ\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Unknown,
            }
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
                        "docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "typ" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"typ" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<Identifier>(&mut __seq)? {
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
                        ::core::option::Option<Docs>,
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
                        ::std::vec::Vec<Attr>,
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
                        ::core::option::Option<Type>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Variant {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typ: __field3,
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
                    let mut __field0: ::core::option::Option<Identifier> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::core::option::Option<Type>> =
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
                                    __serde::de::MapAccess::next_value::<Identifier>(&mut __map)?,
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
                                        ::core::option::Option<Docs>,
                                    >(&mut __map)?,
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Attr>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("typ"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Type>,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("attrs"),
                            );
                        }
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Variant {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        typ: __field3,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["name", "docs", "attrs", "typ"];
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
    impl __serde::Serialize for WrapperTypeDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "WrapperTypeDef", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "wrapped",
                &self.wrapped,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for WrapperTypeDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["wrapped"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"wrapped\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "wrapped" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        b"wrapped" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                    let __field0 = match __serde::de::SeqAccess::next_element::<Type>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
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
                                    __serde::de::MapAccess::next_value::<Type>(&mut __map)?,
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
    #[doc = "A definition of a service.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ServiceDef {
        #[doc = "The methods provided by the service.\n"]
        pub methods: ::std::vec::Vec<Method>,
    }
    impl ServiceDef {
        #[doc = "Creates a new [`ServiceDef`]."]
        pub fn new() -> Self {
            Self {
                methods: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `methods`."]
        pub fn set_methods(&mut self, methods: ::std::vec::Vec<Method>) -> &mut Self {
            self.methods = methods;
            self
        }
        #[doc = "Sets the value of `methods`."]
        pub fn with_methods(mut self, methods: ::std::vec::Vec<Method>) -> Self {
            self.methods = methods;
            self
        }
    }
    impl ::std::default::Default for ServiceDef {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ServiceDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "ServiceDef", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "methods",
                &self.methods,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ServiceDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["methods"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"methods\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "methods" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        b"methods" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ServiceDef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ServiceDef")
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
                        ::std::vec::Vec<Method>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ServiceDef { methods: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    let mut __field0: ::core::option::Option<::std::vec::Vec<Method>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "methods",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Method>>(
                                        &mut __map,
                                    )?,
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
                                <__A::Error as __serde::de::Error>::missing_field("methods"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ServiceDef { methods: __field0 })
                }
            }
            const __FIELDS: &'static [&'static str] = &["methods"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ServiceDef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A method of a service definition.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Method {
        #[doc = "The name of the method.\n"]
        pub name: Identifier,
        #[doc = "The documentation of the method.\n"]
        pub docs: ::std::option::Option<Docs>,
        #[doc = "The attributes of the method.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The parameters of the method.\n"]
        pub parameters: ::std::vec::Vec<MethodParam>,
        #[doc = "The optional return type of the method.\n"]
        pub returns: ::std::option::Option<Type>,
    }
    impl Method {
        #[doc = "Creates a new [`Method`]."]
        pub fn new(name: Identifier) -> Self {
            Self {
                name,
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
                returns: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Identifier) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Identifier) -> Self {
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
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(&mut self, parameters: ::std::vec::Vec<MethodParam>) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(mut self, parameters: ::std::vec::Vec<MethodParam>) -> Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `returns`."]
        pub fn set_returns(&mut self, returns: ::std::option::Option<Type>) -> &mut Self {
            self.returns = returns;
            self
        }
        #[doc = "Sets the value of `returns`."]
        pub fn with_returns(mut self, returns: ::std::option::Option<Type>) -> Self {
            self.returns = returns;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Method {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Method", 5usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            match &self.docs {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "docs", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "docs")?;
                }
            }
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "attrs", &self.attrs)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "parameters",
                &self.parameters,
            )?;
            match &self.returns {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "returns",
                        __value,
                    )?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "returns")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Method {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] =
                &["name", "docs", "attrs", "parameters", "returns"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"docs\", \"attrs\", \"parameters\", \"returns\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Unknown,
            }
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
                        "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "docs" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "attrs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "parameters" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "returns" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                        b"parameters" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"returns" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Method;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Method")
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
                        match __serde::de::SeqAccess::next_element::<Identifier>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 5 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Docs>,
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
                        ::std::vec::Vec<Attr>,
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
                        ::std::vec::Vec<MethodParam>,
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
                        ::core::option::Option<Type>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Method {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        parameters: __field3,
                        returns: __field4,
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
                    let mut __field0: ::core::option::Option<Identifier> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Docs>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Attr>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<MethodParam>> =
                        ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::core::option::Option<Type>> =
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
                                    __serde::de::MapAccess::next_value::<Identifier>(&mut __map)?,
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
                                        ::core::option::Option<Docs>,
                                    >(&mut __map)?,
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
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Attr>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "parameters",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::vec::Vec<MethodParam>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "returns",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::core::option::Option<Type>,
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
                                <__A::Error as __serde::de::Error>::missing_field("parameters"),
                            );
                        }
                    };
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Method {
                        name: __field0,
                        docs: __field1,
                        attrs: __field2,
                        parameters: __field3,
                        returns: __field4,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] =
                &["name", "docs", "attrs", "parameters", "returns"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Method",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "A parameter of a method.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct MethodParam {
        #[doc = "The name of the parameter.\n"]
        pub name: Identifier,
        #[doc = "The type of the parameter.\n"]
        pub typ: Type,
        #[doc = "Indicates whether the parameter is optional.\n"]
        pub is_optional: bool,
    }
    impl MethodParam {
        #[doc = "Creates a new [`MethodParam`]."]
        pub fn new(name: Identifier, typ: Type) -> Self {
            Self {
                name,
                typ,
                is_optional: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: Identifier) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: Identifier) -> Self {
            self.name = name;
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
    }
    #[automatically_derived]
    impl __serde::Serialize for MethodParam {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "MethodParam", 3usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "name", &self.name)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "typ", &self.typ)?;
            __serde::ser::SerializeStruct::serialize_field(
                &mut __struct,
                "isOptional",
                &self.is_optional,
            )?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for MethodParam {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["name", "typ", "isOptional"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"name\", \"typ\", \"isOptional\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Unknown,
            }
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
                        "typ" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "isOptional" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"typ" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"isOptional" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = MethodParam;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record MethodParam")
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
                        match __serde::de::SeqAccess::next_element::<Identifier>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 3 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<Type>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<bool>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(MethodParam {
                        name: __field0,
                        typ: __field1,
                        is_optional: __field2,
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
                    let mut __field0: ::core::option::Option<Identifier> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<Type> = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<bool> = ::core::option::Option::None;
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
                                    __serde::de::MapAccess::next_value::<Identifier>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("typ"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Type>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "isOptional",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<bool>(&mut __map)?,
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
                                <__A::Error as __serde::de::Error>::missing_field("typ"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("isOptional"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(MethodParam {
                        name: __field0,
                        typ: __field1,
                        is_optional: __field2,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["name", "typ", "isOptional"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "MethodParam",
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
    impl __serde::Serialize for Type {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct = __serde::Serializer::serialize_struct(__serializer, "Type", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "kind", &self.kind)?;
            match &self.span {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "span", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "span")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Type {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["kind", "span"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"kind\", \"span\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<TypeKind>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 2 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Span>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                    let mut __field0: ::core::option::Option<TypeKind> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Span>> =
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
                                    __serde::de::MapAccess::next_value::<TypeKind>(&mut __map)?,
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
                                        ::core::option::Option<Span>,
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
    #[doc = "An abstract type kind.\n"]
    #[derive(Clone, Debug)]
    pub enum TypeKind {
        #[doc = "A type to be determined via substitution of the respective type variable.\n"]
        TypeVar(TypeVarType),
        #[doc = "An instantiation of a type defined in some schema.\n"]
        Instance(InstanceType),
    }
    #[automatically_derived]
    impl __serde::Serialize for TypeKind {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            if __serde::Serializer::is_human_readable(&__serializer) {
                match self {
                    Self::TypeVar(__value) => {
                        let mut __struct = __serde::Serializer::serialize_struct(
                            __serializer,
                            "TypeKind",
                            2usize,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "TypeVar",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "idx",
                            &__value.idx,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Instance(__value) => {
                        let mut __struct = __serde::Serializer::serialize_struct(
                            __serializer,
                            "TypeKind",
                            5usize,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Instance",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "bundle",
                            &__value.bundle,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "schema",
                            &__value.schema,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "def",
                            &__value.def,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "subst",
                            &__value.subst,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                }
            } else {
                match self {
                    Self::TypeVar(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TypeKind",
                            0u32,
                            "TypeVar",
                            __value,
                        )
                    }
                    Self::Instance(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TypeKind",
                            1u32,
                            "Instance",
                            __value,
                        )
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeKind {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["TypeVar", "Instance"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"TypeVar\", \"Instance\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
            }
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
            const __VARIANTS: &'static [&'static str] = &["TypeVar", "Instance"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(TypeKind::TypeVar(
                            __tagged.deserialize_internally_tagged::<TypeVarType, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(TypeKind::Instance(
                            __tagged.deserialize_internally_tagged::<InstanceType, __D::Error>()?,
                        ))
                    }
                }
            } else {
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = TypeKind;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum ExternallyTagged")
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
                                    TypeVarType,
                                >(__variant)?;
                                ::core::result::Result::Ok(TypeKind::TypeVar(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    InstanceType,
                                >(__variant)?;
                                ::core::result::Result::Ok(TypeKind::Instance(__value))
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
    impl __serde::Serialize for TypeVarType {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "TypeVarType", 1usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "idx", &self.idx)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TypeVarType {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["idx"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"idx\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Unknown,
            }
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
                        _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<TypeVarIdx>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 1 fields",
                                    ),
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
                                    __serde::de::MapAccess::next_value::<TypeVarIdx>(&mut __map)?,
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
    #[doc = "An instantiation of a type defined in some schema of some bundle.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct InstanceType {
        #[doc = "The bundle containing the schema containing the definition.\n"]
        pub bundle: BundleIdx,
        #[doc = "The schema containing the definition.\n"]
        pub schema: SchemaIdx,
        #[doc = "The actual definition.\n"]
        pub def: DefIdx,
        #[doc = "Substitutions for the type variables of the definition.\n"]
        pub subst: ::std::vec::Vec<Type>,
    }
    impl InstanceType {
        #[doc = "Creates a new [`InstanceType`]."]
        pub fn new(bundle: BundleIdx, schema: SchemaIdx, def: DefIdx) -> Self {
            Self {
                bundle,
                schema,
                def,
                subst: ::std::default::Default::default(),
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
    impl __serde::Serialize for InstanceType {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "InstanceType", 4usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "bundle", &self.bundle)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "schema", &self.schema)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "def", &self.def)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "subst", &self.subst)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for InstanceType {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["bundle", "schema", "def", "subst"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"bundle\", \"schema\", \"def\", \"subst\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Unknown,
            }
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
                        "bundle" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "schema" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "def" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "subst" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"bundle" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"schema" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"def" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"subst" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<BundleIdx>(&mut __seq)? {
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
                    let __field1 =
                        match __serde::de::SeqAccess::next_element::<SchemaIdx>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        1usize,
                                        &"record with 4 fields",
                                    ),
                                );
                            }
                        };
                    let __field2 = match __serde::de::SeqAccess::next_element::<DefIdx>(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::std::vec::Vec<Type>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(InstanceType {
                        bundle: __field0,
                        schema: __field1,
                        def: __field2,
                        subst: __field3,
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
                    let mut __field0: ::core::option::Option<BundleIdx> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<SchemaIdx> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<DefIdx> = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::vec::Vec<Type>> =
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
                                    __serde::de::MapAccess::next_value::<BundleIdx>(&mut __map)?,
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
                                    __serde::de::MapAccess::next_value::<SchemaIdx>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("def"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<DefIdx>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "subst",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Type>>(
                                        &mut __map,
                                    )?,
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
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("subst"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(InstanceType {
                        bundle: __field0,
                        schema: __field1,
                        def: __field2,
                        subst: __field3,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["bundle", "schema", "def", "subst"];
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
    impl __serde::Serialize for Span {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct = __serde::Serializer::serialize_struct(__serializer, "Span", 3usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "src", &self.src)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "start", &self.start)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "end", &self.end)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Span {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["src", "start", "end"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"src\", \"start\", \"end\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Unknown,
            }
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<SourceIdx>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 3 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<usize>(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<usize>(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                                    __serde::de::MapAccess::next_value::<SourceIdx>(&mut __map)?,
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
                                    __serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<usize>(&mut __map)?,
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
    #[doc = "A token.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Token {
        #[doc = "The token itself.\n"]
        pub kind: TokenKind,
        #[doc = "The span of the token.\n"]
        pub span: ::std::option::Option<Span>,
    }
    impl Token {
        #[doc = "Creates a new [`Token`]."]
        pub fn new(kind: TokenKind) -> Self {
            Self {
                kind,
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: TokenKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: TokenKind) -> Self {
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
    impl __serde::Serialize for Token {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "Token", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "kind", &self.kind)?;
            match &self.span {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "span", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "span")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Token {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["kind", "span"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"kind\", \"span\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Token;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Token")
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
                        match __serde::de::SeqAccess::next_element::<TokenKind>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 2 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Span>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Token {
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
                    let mut __field0: ::core::option::Option<TokenKind> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Span>> =
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
                                    __serde::de::MapAccess::next_value::<TokenKind>(&mut __map)?,
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
                                        ::core::option::Option<Span>,
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
                                <__A::Error as __serde::de::Error>::missing_field("kind"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Token {
                        kind: __field0,
                        span: __field1,
                    })
                }
            }
            const __FIELDS: &'static [&'static str] = &["kind", "span"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Token",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum TokenKind {
        #[doc = ""]
        Punctuation(::std::string::String),
        #[doc = ""]
        Delimiter(::std::string::String),
        #[doc = ""]
        Literal(Literal),
        #[doc = ""]
        Identifier(::std::string::String),
    }
    #[automatically_derived]
    impl __serde::Serialize for TokenKind {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            if __serde::Serializer::is_human_readable(&__serializer) {
                match self {
                    Self::Punctuation(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "TokenKind", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Punctuation",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "token",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Delimiter(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "TokenKind", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Delimiter",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "token",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Literal(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "TokenKind", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Literal",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "literal",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Identifier(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "TokenKind", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Identifier",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "token",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                }
            } else {
                match self {
                    Self::Punctuation(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TokenKind",
                            0u32,
                            "Punctuation",
                            __value,
                        )
                    }
                    Self::Delimiter(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TokenKind",
                            1u32,
                            "Delimiter",
                            __value,
                        )
                    }
                    Self::Literal(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TokenKind",
                            2u32,
                            "Literal",
                            __value,
                        )
                    }
                    Self::Identifier(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "TokenKind",
                            3u32,
                            "Identifier",
                            __value,
                        )
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TokenKind {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] =
                &["Punctuation", "Delimiter", "Literal", "Identifier"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"Punctuation\", \"Delimiter\", \"Literal\", \"Identifier\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
            }
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
                        "Punctuation" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Delimiter" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "Literal" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "Identifier" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"Punctuation" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Delimiter" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"Literal" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"Identifier" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
            const __VARIANTS: &'static [&'static str] =
                &["Punctuation", "Delimiter", "Literal", "Identifier"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(TokenKind::Punctuation(
                            __tagged
                                .deserialize_adjacently_tagged::<::std::string::String, __D::Error>(
                                    "token",
                                )?,
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(TokenKind::Delimiter(
                            __tagged
                                .deserialize_adjacently_tagged::<::std::string::String, __D::Error>(
                                    "token",
                                )?,
                        ))
                    }
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(TokenKind::Literal(
                            __tagged
                                .deserialize_adjacently_tagged::<Literal, __D::Error>("literal")?,
                        ))
                    }
                    __Identifier::__Identifier3 => {
                        ::core::result::Result::Ok(TokenKind::Identifier(
                            __tagged
                                .deserialize_adjacently_tagged::<::std::string::String, __D::Error>(
                                    "token",
                                )?,
                        ))
                    }
                }
            } else {
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = TokenKind;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum ExternallyTagged")
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
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(TokenKind::Punctuation(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(TokenKind::Delimiter(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<Literal>(
                                    __variant,
                                )?;
                                ::core::result::Result::Ok(TokenKind::Literal(__value))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(TokenKind::Identifier(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "TokenKind",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum Literal {
        #[doc = ""]
        String(::std::string::String),
        #[doc = ""]
        Number(::std::string::String),
        #[doc = ""]
        Bool(bool),
    }
    #[automatically_derived]
    impl __serde::Serialize for Literal {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            if __serde::Serializer::is_human_readable(&__serializer) {
                match self {
                    Self::String(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "Literal", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "String",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Number(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "Literal", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Number",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Bool(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "Literal", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Bool",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                }
            } else {
                match self {
                    Self::String(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Literal",
                            0u32,
                            "String",
                            __value,
                        )
                    }
                    Self::Number(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Literal",
                            1u32,
                            "Number",
                            __value,
                        )
                    }
                    Self::Bool(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Literal",
                            2u32,
                            "Bool",
                            __value,
                        )
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Literal {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["String", "Number", "Bool"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"String\", \"Number\", \"Bool\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
            }
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
                        "String" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Number" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "Bool" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"String" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Number" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"Bool" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
            const __VARIANTS: &'static [&'static str] = &["String", "Number", "Bool"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(Literal::String(
                            __tagged
                                .deserialize_adjacently_tagged::<::std::string::String, __D::Error>(
                                    "value",
                                )?,
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(Literal::Number(
                            __tagged
                                .deserialize_adjacently_tagged::<::std::string::String, __D::Error>(
                                    "value",
                                )?,
                        ))
                    }
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(Literal::Bool(
                            __tagged.deserialize_adjacently_tagged::<bool, __D::Error>("value")?,
                        ))
                    }
                }
            } else {
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = Literal;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum ExternallyTagged")
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
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(Literal::String(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(Literal::Number(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<bool>(__variant)?;
                                ::core::result::Result::Ok(Literal::Bool(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Literal",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "A `::` separated path of identifiers.\n"]
    #[derive(Clone, Debug)]
    pub struct Path(pub(crate) ::std::string::String);
    impl ::std::convert::From<Path> for ::std::string::String {
        fn from(wrapped: Path) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Path {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Path {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(Path(__serde::Deserialize::deserialize(__deserializer)?))
        }
    }
    #[doc = "A stream of tokens.\n"]
    #[derive(Clone, Debug)]
    pub struct TokenStream(pub(crate) ::std::vec::Vec<Token>);
    impl ::std::convert::From<TokenStream> for ::std::vec::Vec<Token> {
        fn from(wrapped: TokenStream) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for TokenStream {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            self.0.serialize(__serializer)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TokenStream {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            Ok(TokenStream(__serde::Deserialize::deserialize(
                __deserializer,
            )?))
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
    impl __serde::Serialize for Attr {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct = __serde::Serializer::serialize_struct(__serializer, "Attr", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "kind", &self.kind)?;
            match &self.span {
                ::core::option::Option::Some(__value) => {
                    __serde::ser::SerializeStruct::serialize_field(&mut __struct, "span", __value)?;
                }
                ::core::option::Option::None => {
                    __serde::ser::SerializeStruct::skip_field(&mut __struct, "span")?;
                }
            }
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Attr {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["kind", "span"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"kind\", \"span\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<AttrKind>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 2 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::core::option::Option<Span>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                    let mut __field0: ::core::option::Option<AttrKind> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::core::option::Option<Span>> =
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
                                    __serde::de::MapAccess::next_value::<AttrKind>(&mut __map)?,
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
                                        ::core::option::Option<Span>,
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
    #[doc = "The four kinds of attributes.\n"]
    #[derive(Clone, Debug)]
    pub enum AttrKind {
        #[doc = "A `::` separated path.\n"]
        Path(Path),
        #[doc = "A list attribute of the form `<PATH> ( <ARGS> )` where `<ARGS>` is a\n`,`-separated sequence of arguments.\n"]
        List(AttrList),
        #[doc = "An assign attribute of the form `<PATH> = <ATTR>` where `<ATTR>` is an attribute.\n"]
        Assign(AttrAssign),
        #[doc = "A stream of tokens (if the other alternatives do not apply).\n"]
        Tokens(TokenStream),
    }
    #[automatically_derived]
    impl __serde::Serialize for AttrKind {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            if __serde::Serializer::is_human_readable(&__serializer) {
                match self {
                    Self::Path(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "AttrKind", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Path",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "path",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::List(__value) => {
                        let mut __struct = __serde::Serializer::serialize_struct(
                            __serializer,
                            "AttrKind",
                            3usize,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "List",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "path",
                            &__value.path,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "args",
                            &__value.args,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Assign(__value) => {
                        let mut __struct = __serde::Serializer::serialize_struct(
                            __serializer,
                            "AttrKind",
                            3usize,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Assign",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "path",
                            &__value.path,
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "value",
                            &__value.value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                    Self::Tokens(__value) => {
                        let mut __struct =
                            __serde::Serializer::serialize_struct(__serializer, "AttrKind", 2)?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "tag",
                            "Tokens",
                        )?;
                        __serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "content",
                            &__value,
                        )?;
                        __serde::ser::SerializeStruct::end(__struct)
                    }
                }
            } else {
                match self {
                    Self::Path(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "AttrKind",
                            0u32,
                            "Path",
                            __value,
                        )
                    }
                    Self::List(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "AttrKind",
                            1u32,
                            "List",
                            __value,
                        )
                    }
                    Self::Assign(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "AttrKind",
                            2u32,
                            "Assign",
                            __value,
                        )
                    }
                    Self::Tokens(__value) => {
                        __serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "AttrKind",
                            3u32,
                            "Tokens",
                            __value,
                        )
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for AttrKind {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["Path", "List", "Assign", "Tokens"];
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"Path\", \"List\", \"Assign\", \"Tokens\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
            }
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
                        "Path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "List" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "Assign" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "Tokens" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"Tokens" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
            const __VARIANTS: &'static [&'static str] = &["Path", "List", "Assign", "Tokens"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "tag")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(AttrKind::Path(
                            __tagged.deserialize_adjacently_tagged::<Path, __D::Error>("path")?,
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(AttrKind::List(
                            __tagged.deserialize_internally_tagged::<AttrList, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(AttrKind::Assign(
                            __tagged.deserialize_internally_tagged::<AttrAssign, __D::Error>()?,
                        ))
                    }
                    __Identifier::__Identifier3 => {
                        ::core::result::Result::Ok(AttrKind::Tokens(
                            __tagged.deserialize_adjacently_tagged::<TokenStream, __D::Error>(
                                "content",
                            )?,
                        ))
                    }
                }
            } else {
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = AttrKind;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum ExternallyTagged")
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
                                    __serde::de::VariantAccess::newtype_variant::<Path>(__variant)?;
                                ::core::result::Result::Ok(AttrKind::Path(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    AttrList,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrKind::List(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    AttrAssign,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrKind::Assign(__value))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    TokenStream,
                                >(__variant)?;
                                ::core::result::Result::Ok(AttrKind::Tokens(__value))
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
    #[doc = "A list attribute of the form `<PATH> ( <ARGS> )` where `<ARGS>` is a `,`-separated\nsequence of arguments.\n"]
    #[derive(Clone, Debug)]
    pub struct AttrList {
        #[doc = "The path of the attribute.\n"]
        pub path: Path,
        #[doc = "The arguments of the attribute.\n"]
        pub args: ::std::vec::Vec<Attr>,
    }
    impl AttrList {
        #[doc = "Creates a new [`AttrList`]."]
        pub fn new(path: Path) -> Self {
            Self {
                path,
                args: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(&mut self, path: Path) -> &mut Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: Path) -> Self {
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
    impl __serde::Serialize for AttrList {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "AttrList", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "path", &self.path)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "args", &self.args)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for AttrList {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["path", "args"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"path\", \"args\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
                    let __field0 = match __serde::de::SeqAccess::next_element::<Path>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::vec::Vec<Attr>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                    let mut __field0: ::core::option::Option<Path> = ::core::option::Option::None;
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
                                    __serde::de::MapAccess::next_value::<Path>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("args"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Attr>>(
                                        &mut __map,
                                    )?,
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
    #[doc = "An assign attribute of the form `<PATH> = <ATTR>` where `<ATTR>` is an attribute.\n"]
    #[derive(Clone, Debug)]
    pub struct AttrAssign {
        #[doc = "The path of the attribute.\n"]
        pub path: Path,
        #[doc = "The assigned value.\n"]
        pub value: ::std::boxed::Box<Attr>,
    }
    impl AttrAssign {
        #[doc = "Creates a new [`AttrAssign`]."]
        pub fn new(path: Path, value: ::std::boxed::Box<Attr>) -> Self {
            Self { path, value }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(&mut self, path: Path) -> &mut Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: Path) -> Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn set_value(&mut self, value: ::std::boxed::Box<Attr>) -> &mut Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn with_value(mut self, value: ::std::boxed::Box<Attr>) -> Self {
            self.value = value;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for AttrAssign {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __struct =
                __serde::Serializer::serialize_struct(__serializer, "AttrAssign", 2usize)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "path", &self.path)?;
            __serde::ser::SerializeStruct::serialize_field(&mut __struct, "value", &self.value)?;
            __serde::ser::SerializeStruct::end(__struct)
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for AttrAssign {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            const __IDENTIFIERS: &'static [&'static str] = &["path", "value"];
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"path\", \"value\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Unknown,
            }
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
                    let __field0 = match __serde::de::SeqAccess::next_element::<Path>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::boxed::Box<Attr>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
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
                    let mut __field0: ::core::option::Option<Path> = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::boxed::Box<Attr>> =
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
                                    __serde::de::MapAccess::next_value::<Path>(&mut __map)?,
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
                                    __serde::de::MapAccess::next_value::<::std::boxed::Box<Attr>>(
                                        &mut __map,
                                    )?,
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
}
