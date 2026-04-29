/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for the Python codegen target.\n\nPlugins use these schemas to drive code generation; the compiler parses\nsource `#[py(...)]` attributes against them and stores the result in the\nIR's `typed_attrs[\"py\"]` field. Downstream codegens deserialize that\nvalue directly into the generated Rust types via `serde_json::from_value`.\n"]
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
    #[doc = "Python-specific attributes that may be applied to an `opaque` definition.\n\nSource form: `#[py(typ = \"module.Type\")]`. The `typ` value is the\nfully-qualified Python type path that the codegen emits as a type alias.\n"]
    #[derive(Clone, Debug)]
    pub struct OpaqueTypeAttrs {
        #[doc = "The Python type path (e.g. `\"datetime.datetime\"`).\n"]
        pub typ: ::std::option::Option<::std::string::String>,
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
                __sidex_serde::ser::RecordSerializer::new(__serializer, "OpaqueTypeAttrs", 1usize)?;
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
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(OpaqueTypeAttrs { typ: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["typ"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"typ\"]";
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
                                "typ" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                    ::core::result::Result::Ok(OpaqueTypeAttrs { typ: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["typ"];
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
}
