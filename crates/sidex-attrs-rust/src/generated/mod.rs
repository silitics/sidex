/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod attrs {
    #![doc = "Typed attributes for the Rust codegen target.\n\nSource `#[rust(...)]` attributes are validated against these schemas at\nIR build time and stored on the IR's `typed_attrs[\"rust\"]` map. Each\nrecord uses `#[json(rename_all = \"snake_case\")]` so wire keys match the\nparser's emit shape.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Restriction on a `pub` visibility — `pub(crate)` or `pub(super)`.\n"]
    #[derive(Clone, Debug)]
    pub struct PubVis {
        #[doc = "`#[rust(pub(crate))]` — restrict to the current crate.\n"]
        pub is_crate: ::std::option::Option<bool>,
        #[doc = "`#[rust(pub(super))]` — restrict to the parent module.\n"]
        pub is_super: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for PubVis {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rust::attrs::PubVis",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("is_crate", ::sidex::shape::shape_of::<bool>)
                    .with_docs("`#[rust(pub(crate))]` — restrict to the current crate.\n")
                    .with_attrs(&[::sidex::shape::Attr::List {
                        path: "attr",
                        args: &[::sidex::shape::Attr::Assign {
                            path: "name",
                            value: ::sidex::shape::AttrValue::String("crate"),
                        }],
                    }])
                    .optional(),
                ::sidex::shape::FieldShape::new("is_super", ::sidex::shape::shape_of::<bool>)
                    .with_docs("`#[rust(pub(super))]` — restrict to the parent module.\n")
                    .with_attrs(&[::sidex::shape::Attr::List {
                        path: "attr",
                        args: &[::sidex::shape::Attr::Assign {
                            path: "name",
                            value: ::sidex::shape::AttrValue::String("super"),
                        }],
                    }])
                    .optional(),
            ])),
        )
        .with_docs("Restriction on a `pub` visibility — `pub(crate)` or `pub(super)`.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("snake_case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for PubVis {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for PubVis {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.is_crate
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.is_super
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for PubVis {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<bool>::None;
            let mut __field_1 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                is_crate: __field_0,
                is_super: __field_1,
            })
        }
    }
    #[doc = "Rust attributes that may apply to any definition.\n"]
    #[derive(Clone, Debug)]
    pub struct TypeAttrs {
        #[doc = "`#[rust(type = \"::path::To::Type\")]` — emit a type alias to the\nreferenced Rust type instead of generating one.\n"]
        pub typ: ::std::option::Option<::std::string::String>,
        #[doc = "`#[rust(derive(Clone, Debug, ...))]` — extra derives applied to the\ngenerated type. Multiple occurrences accumulate.\n"]
        pub derive: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "`#[rust(attr(non_exhaustive))]` — extra Rust attributes applied to\nthe generated type. Multiple occurrences accumulate; nested lists\n(e.g. `attr(serde(transparent))`) are stringified verbatim.\n"]
        pub attr: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TypeAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rust::attrs::TypeAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("typ" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("`#[rust(type = \"::path::To::Type\")]` — emit a type alias to the\nreferenced Rust type instead of generating one.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("type") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("derive" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("`#[rust(derive(Clone, Debug, ...))]` — extra derives applied to the\ngenerated type. Multiple occurrences accumulate.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("attr" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("`#[rust(attr(non_exhaustive))]` — extra Rust attributes applied to\nthe generated type. Multiple occurrences accumulate; nested lists\n(e.g. `attr(serde(transparent))`) are stringified verbatim.\n") . optional ()]))) . with_docs ("Rust attributes that may apply to any definition.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("rust") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("def") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for TypeAttrs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for TypeAttrs {
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
                    self.derive
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.attr
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for TypeAttrs {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_2 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                typ: __field_0,
                derive: __field_1,
                attr: __field_2,
            })
        }
    }
    #[doc = "Rust attributes that may apply to a record `field`.\n"]
    #[derive(Clone, Debug)]
    pub struct FieldAttrs {
        #[doc = "`#[rust(name = \"explicit\")]` — override the Rust identifier.\n"]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = "`#[rust(pub)]` (default) plus `pub(crate)` / `pub(super)` variants.\n"]
        pub is_pub: ::std::option::Option<PubVis>,
        #[doc = "`#[rust(private)]` — explicitly private (the default for fields, so\nthis is mostly used to override a record-level visibility).\n"]
        pub is_private: ::std::option::Option<bool>,
        #[doc = "`#[rust(box)]` — wrap the field type in `::std::boxed::Box`.\n"]
        pub is_box: ::std::option::Option<bool>,
        #[doc = "`#[rust(arc)]` — wrap the field type in `::std::sync::Arc`.\n"]
        pub is_arc: ::std::option::Option<bool>,
        #[doc = "`#[rust(rc)]` — wrap the field type in `::std::rc::Rc`.\n"]
        pub is_rc: ::std::option::Option<bool>,
        #[doc = "`#[rust(wrap = \"::path::To::Type\")]` — wrap the field type in a\ncustom Rust type.\n"]
        pub wrap: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for FieldAttrs {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rust::attrs::FieldAttrs" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("name" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("`#[rust(name = \"explicit\")]` — override the Rust identifier.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("is_pub" , :: sidex :: shape :: shape_of :: < PubVis > ,) . with_docs ("`#[rust(pub)]` (default) plus `pub(crate)` / `pub(super)` variants.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("pub") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("is_private" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("`#[rust(private)]` — explicitly private (the default for fields, so\nthis is mostly used to override a record-level visibility).\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("private") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("is_box" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("`#[rust(box)]` — wrap the field type in `::std::boxed::Box`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("box") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("is_arc" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("`#[rust(arc)]` — wrap the field type in `::std::sync::Arc`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("arc") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("is_rc" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("`#[rust(rc)]` — wrap the field type in `::std::rc::Rc`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Assign { path : "name" , value : :: sidex :: shape :: AttrValue :: String ("rc") }] }]) . optional () , :: sidex :: shape :: FieldShape :: new ("wrap" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("`#[rust(wrap = \"::path::To::Type\")]` — wrap the field type in a\ncustom Rust type.\n") . optional ()]))) . with_docs ("Rust attributes that may apply to a record `field`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "attrs" , args : & [:: sidex :: shape :: Attr :: Assign { path : "plugin" , value : :: sidex :: shape :: AttrValue :: String ("rust") } , :: sidex :: shape :: Attr :: Assign { path : "target" , value : :: sidex :: shape :: AttrValue :: Path ("field") }] } , :: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("snake_case") }] }]) ;
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
                    self.is_pub
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => {
                    self.is_private
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.is_box
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                4usize => {
                    self.is_arc
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                5usize => {
                    self.is_rc
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                6usize => {
                    self.wrap
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
            let mut __field_1 = ::std::option::Option::<PubVis>::None;
            let mut __field_2 = ::std::option::Option::<bool>::None;
            let mut __field_3 = ::std::option::Option::<bool>::None;
            let mut __field_4 = ::std::option::Option::<bool>::None;
            let mut __field_5 = ::std::option::Option::<bool>::None;
            let mut __field_6 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <PubVis as ::sidex::Decode>::decode(decoder)?,
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
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0,
                is_pub: __field_1,
                is_private: __field_2,
                is_box: __field_3,
                is_arc: __field_4,
                is_rc: __field_5,
                wrap: __field_6,
            })
        }
    }
}
