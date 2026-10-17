/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for API stability and lifecycle.\n\nThree plugins describe a definition's lifecycle:\n\n- `#[deprecated(...)]` — the item is deprecated; use something else.\n- `#[unstable(...)]`   — the item's contract may change between versions.\n- `#[since(...)]`      — the version in which the item was introduced.\n\nEach plugin applies to a definition (record / variant / wrapper / opaque /\nalias), a record field, a variant case, and a schema. Codegens propagate\nthe result to the target language (Rust `#[deprecated]`, TSDoc `@deprecated`\n/ `@experimental` / `@since`, PEP 702 `@typing.deprecated`, JSON-Schema\n`deprecated` keyword) and prepend a stability prelude to the rendered doc\ncomment.\n\nA fourth plugin, `#[group(...)]`, assigns a definition to a group declared\nin the bundle manifest. Codegens prepend the group descriptions to the\nrendered doc comment.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "`#[deprecated]` on a top-level definition.\n\nSource forms:\n\n```sidex\n#[deprecated]\n#[deprecated(since = \"2.3.0\", note = \"Use `NewThing`.\")]\n#[deprecated(since = \"2.3.0\", remove_in = \"4.0.0\", note = \"Use `NewThing`.\")]\n```\n\n`since` and `remove_in` are free-form version strings (SemVer by\nconvention); `note` is human-readable migration guidance.\n"]
    #[derive(Clone, Debug)]
    pub struct DefDeprecated {
        #[doc = "Version in which the item was deprecated.\n"]
        pub since: ::std::option::Option<::std::string::String>,
        #[doc = "Migration guidance — what to use instead.\n"]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = "Version in which the item is scheduled to be removed. While the\ncurrent bundle version is below `remove_in`, tooling emits a\ndeprecation warning; once the bundle version reaches `remove_in`\nand the item is still present, the warning is escalated to an error.\n"]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DefDeprecated {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::api::attrs::DefDeprecated" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("since" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version in which the item was deprecated.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("note" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Migration guidance — what to use instead.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("remove_in" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version in which the item is scheduled to be removed. While the\ncurrent bundle version is below `remove_in`, tooling emits a\ndeprecation warning; once the bundle version reaches `remove_in`\nand the item is still present, the warning is escalated to an error.\n") . optional ()]))) . with_docs ("`#[deprecated]` on a top-level definition.\n\nSource forms:\n\n```sidex\n#[deprecated]\n#[deprecated(since = \"2.3.0\", note = \"Use `NewThing`.\")]\n#[deprecated(since = \"2.3.0\", remove_in = \"4.0.0\", note = \"Use `NewThing`.\")]\n```\n\n`since` and `remove_in` are free-form version strings (SemVer by\nconvention); `note` is human-readable migration guidance.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("deprecated") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("def") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefDeprecated {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for DefDeprecated {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.since
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.remove_in
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefDeprecated {
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
                since: __field_0,
                note: __field_1,
                remove_in: __field_2,
            })
        }
    }
    #[doc = "`#[deprecated]` on a record field.\n"]
    #[derive(Clone, Debug)]
    pub struct FieldDeprecated {
        #[doc = ""]
        pub since: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for FieldDeprecated {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::FieldDeprecated",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "since",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "note",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "remove_in",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
            ])),
        )
        .with_docs("`#[deprecated]` on a record field.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "attrs",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "plugin",
                        value: ::sidex::shape::AttrValue::String("deprecated"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "target",
                        value: ::sidex::shape::AttrValue::Path("field"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("snake_case"),
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for FieldDeprecated {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for FieldDeprecated {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.since
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.remove_in
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for FieldDeprecated {
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
                since: __field_0,
                note: __field_1,
                remove_in: __field_2,
            })
        }
    }
    #[doc = "`#[deprecated]` on a variant case.\n"]
    #[derive(Clone, Debug)]
    pub struct VariantDeprecated {
        #[doc = ""]
        pub since: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for VariantDeprecated {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::VariantDeprecated",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "since",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "note",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "remove_in",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
            ])),
        )
        .with_docs("`#[deprecated]` on a variant case.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "attrs",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "plugin",
                        value: ::sidex::shape::AttrValue::String("deprecated"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "target",
                        value: ::sidex::shape::AttrValue::Path("variant_case"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("snake_case"),
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for VariantDeprecated {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for VariantDeprecated {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.since
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.remove_in
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for VariantDeprecated {
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
                since: __field_0,
                note: __field_1,
                remove_in: __field_2,
            })
        }
    }
    #[doc = "`#[deprecated]` at the top of a schema file.\n"]
    #[derive(Clone, Debug)]
    pub struct SchemaDeprecated {
        #[doc = ""]
        pub since: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub remove_in: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SchemaDeprecated {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::SchemaDeprecated",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "since",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "note",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "remove_in",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
            ])),
        )
        .with_docs("`#[deprecated]` at the top of a schema file.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "attrs",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "plugin",
                        value: ::sidex::shape::AttrValue::String("deprecated"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "target",
                        value: ::sidex::shape::AttrValue::Path("schema"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("snake_case"),
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SchemaDeprecated {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SchemaDeprecated {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.since
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.remove_in
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SchemaDeprecated {
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
                since: __field_0,
                note: __field_1,
                remove_in: __field_2,
            })
        }
    }
    #[doc = "`#[unstable]` on a top-level definition.\n\nSource forms:\n\n```sidex\n#[unstable]\n#[unstable(feature = \"tags\", note = \"Field set still in flux.\",\nissue = \"https://github.com/silitics/sidex/issues/42\")]\n```\n"]
    #[derive(Clone, Debug)]
    pub struct DefUnstable {
        #[doc = "Optional feature-gate identifier. Lets a bundle expose multiple\nunstable surfaces independently; consumers can opt in granularly.\n"]
        pub feature: ::std::option::Option<::std::string::String>,
        #[doc = "Human-readable rationale — what's still in flux and why.\n"]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = "Optional URL to a tracking issue / RFC.\n"]
        pub issue: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DefUnstable {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::api::attrs::DefUnstable" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("feature" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Optional feature-gate identifier. Lets a bundle expose multiple\nunstable surfaces independently; consumers can opt in granularly.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("note" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Human-readable rationale — what's still in flux and why.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("issue" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Optional URL to a tracking issue / RFC.\n") . optional ()]))) . with_docs ("`#[unstable]` on a top-level definition.\n\nSource forms:\n\n```sidex\n#[unstable]\n#[unstable(feature = \"tags\", note = \"Field set still in flux.\",\nissue = \"https://github.com/silitics/sidex/issues/42\")]\n```\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("unstable") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("def") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefUnstable {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for DefUnstable {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.feature
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.issue
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefUnstable {
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
                feature: __field_0,
                note: __field_1,
                issue: __field_2,
            })
        }
    }
    #[doc = "`#[unstable]` on a record field.\n"]
    #[derive(Clone, Debug)]
    pub struct FieldUnstable {
        #[doc = ""]
        pub feature: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub issue: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for FieldUnstable {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::FieldUnstable",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "feature",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "note",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "issue",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
            ])),
        )
        .with_docs("`#[unstable]` on a record field.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "attrs",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "plugin",
                        value: ::sidex::shape::AttrValue::String("unstable"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "target",
                        value: ::sidex::shape::AttrValue::Path("field"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("snake_case"),
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for FieldUnstable {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for FieldUnstable {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.feature
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.issue
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for FieldUnstable {
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
                feature: __field_0,
                note: __field_1,
                issue: __field_2,
            })
        }
    }
    #[doc = "`#[unstable]` on a variant case.\n"]
    #[derive(Clone, Debug)]
    pub struct VariantUnstable {
        #[doc = ""]
        pub feature: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub issue: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for VariantUnstable {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::VariantUnstable",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "feature",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "note",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "issue",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
            ])),
        )
        .with_docs("`#[unstable]` on a variant case.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "attrs",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "plugin",
                        value: ::sidex::shape::AttrValue::String("unstable"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "target",
                        value: ::sidex::shape::AttrValue::Path("variant_case"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("snake_case"),
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for VariantUnstable {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for VariantUnstable {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.feature
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.issue
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for VariantUnstable {
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
                feature: __field_0,
                note: __field_1,
                issue: __field_2,
            })
        }
    }
    #[doc = "`#[unstable]` at the top of a schema file.\n"]
    #[derive(Clone, Debug)]
    pub struct SchemaUnstable {
        #[doc = ""]
        pub feature: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub note: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub issue: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SchemaUnstable {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::SchemaUnstable",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "feature",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "note",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "issue",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .optional(),
            ])),
        )
        .with_docs("`#[unstable]` at the top of a schema file.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "attrs",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "plugin",
                        value: ::sidex::shape::AttrValue::String("unstable"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "target",
                        value: ::sidex::shape::AttrValue::Path("schema"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("snake_case"),
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SchemaUnstable {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SchemaUnstable {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.feature
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.note
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.issue
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SchemaUnstable {
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
                feature: __field_0,
                note: __field_1,
                issue: __field_2,
            })
        }
    }
    #[doc = "`#[since]` on a top-level definition.\n\nSource form:\n\n```sidex\n#[since(version = \"1.4.0\")]\n```\n\n`version` is per-item: `since` on a field means \"this field appeared in\nversion X\" and is independent of the enclosing def's `since`.\n"]
    #[derive(Clone, Debug)]
    pub struct DefSince {
        #[doc = "Version in which the item was introduced. Free-form; SemVer by\nconvention.\n"]
        pub version: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DefSince {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::api::attrs::DefSince" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version in which the item was introduced. Free-form; SemVer by\nconvention.\n")]))) . with_docs ("`#[since]` on a top-level definition.\n\nSource form:\n\n```sidex\n#[since(version = \"1.4.0\")]\n```\n\n`version` is per-item: `since` on a field means \"this field appeared in\nversion X\" and is independent of the enclosing def's `since`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("since") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("def") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefSince {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for DefSince {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.version)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefSince {
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
                version: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `version`"))?,
            })
        }
    }
    #[doc = "`#[since]` on a record field.\n"]
    #[derive(Clone, Debug)]
    pub struct FieldSince {
        #[doc = ""]
        pub version: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for FieldSince {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::FieldSince",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                ),
            ])),
        )
        .with_docs("`#[since]` on a record field.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "attrs",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "plugin",
                    value: ::sidex::shape::AttrValue::String("since"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "target",
                    value: ::sidex::shape::AttrValue::Path("field"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for FieldSince {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for FieldSince {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.version)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for FieldSince {
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
                version: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `version`"))?,
            })
        }
    }
    #[doc = "`#[since]` on a variant case.\n"]
    #[derive(Clone, Debug)]
    pub struct VariantSince {
        #[doc = ""]
        pub version: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for VariantSince {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::VariantSince",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                ),
            ])),
        )
        .with_docs("`#[since]` on a variant case.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "attrs",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "plugin",
                    value: ::sidex::shape::AttrValue::String("since"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "target",
                    value: ::sidex::shape::AttrValue::Path("variant_case"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for VariantSince {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for VariantSince {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.version)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for VariantSince {
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
                version: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `version`"))?,
            })
        }
    }
    #[doc = "`#[since]` at the top of a schema file.\n"]
    #[derive(Clone, Debug)]
    pub struct SchemaSince {
        #[doc = ""]
        pub version: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SchemaSince {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::api::attrs::SchemaSince",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                ),
            ])),
        )
        .with_docs("`#[since]` at the top of a schema file.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "attrs",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "plugin",
                    value: ::sidex::shape::AttrValue::String("since"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "target",
                    value: ::sidex::shape::AttrValue::Path("schema"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SchemaSince {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SchemaSince {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.version)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SchemaSince {
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
                version: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `version`"))?,
            })
        }
    }
    #[doc = "`#[group]` on a top-level definition.\n\nSource form:\n\n```sidex\n#[group(name = \"device\")]\n```\n\nRepeat the attribute to assign a definition to multiple groups.\n"]
    #[derive(Clone, Debug)]
    pub struct DefGroup {
        #[doc = "Name of a group declared in the bundle manifest.\n"]
        pub name: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DefGroup {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::api::attrs::DefGroup" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("name" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Name of a group declared in the bundle manifest.\n")]))) . with_docs ("`#[group]` on a top-level definition.\n\nSource form:\n\n```sidex\n#[group(name = \"device\")]\n```\n\nRepeat the attribute to assign a definition to multiple groups.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("group") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("def") } , :: sidex :: shape :: Attr :: Path ("repeated")] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefGroup {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for DefGroup {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefGroup {
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
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
            })
        }
    }
}
