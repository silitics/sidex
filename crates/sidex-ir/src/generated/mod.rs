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
    #[doc = "An arbitrary, schema-validated Sidex value.\n\nUsed by [`typed_attrs`] to carry the parsed result of a plugin's typed\nattribute schema in the JSON representation of the schema's types.\nProducers and consumers communicate through this representation, validated\nat IR build time against the plugin's Sidex schema.\n"]
    pub type Value = ::sidex::types::JsonValue;
    #[doc = "Uniquely identifies a source in an IR.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct SourceIdx(pub(crate) usize);
    impl ::std::convert::From<SourceIdx> for usize {
        fn from(wrapped: SourceIdx) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SourceIdx {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::SourceIdx",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<usize>),
        )
        .with_docs("Uniquely identifies a source in an IR.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Copy"),
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SourceIdx {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for SourceIdx {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<usize as ::sidex::Decode>::decode(decoder)?))
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
    impl ::sidex::Shaped for BundleIdx {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::BundleIdx",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<usize>),
        )
        .with_docs("Uniquely identifies a bundle in an IR.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Copy"),
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for BundleIdx {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for BundleIdx {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<usize as ::sidex::Decode>::decode(decoder)?))
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
    impl ::sidex::Shaped for SchemaIdx {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::SchemaIdx",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<usize>),
        )
        .with_docs("Uniquely identifies a schema in an IR.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Copy"),
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SchemaIdx {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for SchemaIdx {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<usize as ::sidex::Decode>::decode(decoder)?))
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
    impl ::sidex::Shaped for DefIdx {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::DefIdx",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<usize>),
        )
        .with_docs("Uniquely identifies a definition in an IR.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Copy"),
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefIdx {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for DefIdx {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<usize as ::sidex::Decode>::decode(decoder)?))
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
    impl ::sidex::Shaped for TypeVarIdx {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::TypeVarIdx",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<usize>),
        )
        .with_docs("Uniquely identifies a type variable in a definition.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Copy"),
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for TypeVarIdx {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for TypeVarIdx {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<usize as ::sidex::Decode>::decode(decoder)?))
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
    #[automatically_derived]
    impl ::sidex::Shaped for DefRef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::DefRef",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("bundle", ::sidex::shape::shape_of::<BundleIdx>)
                    .with_docs("The bundle containing the definition.\n"),
                ::sidex::shape::FieldShape::new("schema", ::sidex::shape::shape_of::<SchemaIdx>)
                    .with_docs("The schema containing the definition.\n"),
                ::sidex::shape::FieldShape::new("def", ::sidex::shape::shape_of::<DefIdx>)
                    .with_docs("The definition.\n"),
            ])),
        )
        .with_docs("References a definition.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Copy"),
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefRef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for DefRef {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.bundle)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.schema)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.def)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefRef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<BundleIdx>::None;
            let mut __field_1 = ::std::option::Option::<SchemaIdx>::None;
            let mut __field_2 = ::std::option::Option::<DefIdx>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <BundleIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <SchemaIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <DefIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                bundle: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bundle`"))?,
                schema: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `schema`"))?,
                def: __field_2.ok_or_else(|| ::sidex::DecodeError::new("missing field `def`"))?,
            })
        }
    }
    impl DefRef {
        #[doc = "Creates a new [`DefRef`]."]
        pub fn new(bundle: BundleIdx, schema: SchemaIdx, def: DefIdx) -> Self {
            Self {
                bundle: bundle,
                schema: schema,
                def: def,
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
    #[automatically_derived]
    impl ::sidex::Shaped for Ir {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::Ir" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("bundles" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Bundle , > > ,) . with_docs ("The bundles of the IR.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("schemas" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Schema , > > ,) . with_docs ("The schemas of the IR.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("defs" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Def , > > ,) . with_docs ("The definitions of the IR.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("sources" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Source , > > ,) . with_docs ("The sources of the IR.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("root" , :: sidex :: shape :: shape_of :: < BundleIdx > ,) . with_docs ("The bundle this IR was assembled around. Codegens typically only\nemit code for definitions belonging to schemas in this bundle.\n")]))) . with_docs ("Sidex IR — a fully-resolved, mostly-processed representation that\ndownstream code generators consume.\n\nEvery [`DefRef`] reachable from the IR resolves to a real entry in\n[`Ir.defs`]. Codegens walk this graph without needing to think about\nnames, imports, or version constraints — those concerns are handled\nbefore the IR is built.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Path ("non_exhaustive")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Ir {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Ir {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.bundles)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.schemas)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.defs)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.sources)),
                4usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.root)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Ir {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::vec::Vec<Bundle>>::None;
            let mut __field_1 = ::std::option::Option::<::std::vec::Vec<Schema>>::None;
            let mut __field_2 = ::std::option::Option::<::std::vec::Vec<Def>>::None;
            let mut __field_3 = ::std::option::Option::<::std::vec::Vec<Source>>::None;
            let mut __field_4 = ::std::option::Option::<BundleIdx>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<Bundle> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::vec::Vec<Schema> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::vec::Vec<Def> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::vec::Vec<Source> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <BundleIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                bundles: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bundles`"))?,
                schemas: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `schemas`"))?,
                defs: __field_2.ok_or_else(|| ::sidex::DecodeError::new("missing field `defs`"))?,
                sources: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `sources`"))?,
                root: __field_4.ok_or_else(|| ::sidex::DecodeError::new("missing field `root`"))?,
            })
        }
    }
    impl Ir {
        #[doc = "Creates a new [`Ir`]."]
        pub fn new(root: BundleIdx) -> Self {
            Self {
                root: root,
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
    #[doc = "A *source* is a chunk of text.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Source {
        #[doc = "The origin of the source, e.g., a filesystem path.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "The optional text of the source.\n\nThis field is optional to allow for programmatically generated schemas.\n"]
        pub text: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Source {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::Source" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("origin" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The origin of the source, e.g., a filesystem path.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("text" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The optional text of the source.\n\nThis field is optional to allow for programmatically generated schemas.\n") . optional ()]))) . with_docs ("A *source* is a chunk of text.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Path ("non_exhaustive")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Source {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Source {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => {
                    self.origin
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                1usize => {
                    self.text
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Source {
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
                origin: __field_0,
                text: __field_1,
            })
        }
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
        #[doc = "The groups declared by the bundle, keyed by name.\n"]
        pub groups: ::std::collections::HashMap<::std::string::String, Group>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Bundle {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::Bundle" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("metadata" , :: sidex :: shape :: shape_of :: < Metadata > ,) . with_docs ("The metadata of the bundle.\n") , :: sidex :: shape :: FieldShape :: new ("schemas" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < SchemaIdx , > > ,) . with_docs ("The schemas of the bundle.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("is_internal" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("Whether the bundle is internal to the compiler.\n\nInternal bundles (the standard library, plugin attribute schemas\nauto-loaded by the compiler) are loaded automatically alongside any\nuser bundle so that the typed-attrs parser can validate plugin\nattributes. Code generators should usually skip them since their\ndefs are not part of the user's API surface.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) . json_name ("isInternal") , :: sidex :: shape :: FieldShape :: new ("groups" , :: sidex :: shape :: shape_of :: < :: std :: collections :: HashMap < :: std :: string :: String , Group , > > ,) . with_docs ("The groups declared by the bundle, keyed by name.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")])]))) . with_docs ("A bundle is a flat collection of schemas evolving together.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Path ("non_exhaustive")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Bundle {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Bundle {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.metadata)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.schemas)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.is_internal)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.groups)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Bundle {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Metadata>::None;
            let mut __field_1 = ::std::option::Option::<::std::vec::Vec<SchemaIdx>>::None;
            let mut __field_2 = ::std::option::Option::<bool>::None;
            let mut __field_3 = ::std::option::Option::<
                ::std::collections::HashMap<::std::string::String, Group>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <Metadata as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::vec::Vec<SchemaIdx> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::collections::HashMap<
                            ::std::string::String,
                            Group,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                metadata: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `metadata`"))?,
                schemas: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `schemas`"))?,
                is_internal: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `isInternal`"))?,
                groups: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `groups`"))?,
            })
        }
    }
    impl Bundle {
        #[doc = "Creates a new [`Bundle`]."]
        pub fn new(metadata: Metadata) -> Self {
            Self {
                metadata: metadata,
                schemas: ::std::default::Default::default(),
                is_internal: ::std::default::Default::default(),
                groups: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `groups`."]
        pub fn set_groups(
            &mut self,
            groups: ::std::collections::HashMap<::std::string::String, Group>,
        ) -> &mut Self {
            self.groups = groups;
            self
        }
        #[doc = "Sets the value of `groups`."]
        pub fn with_groups(
            mut self,
            groups: ::std::collections::HashMap<::std::string::String, Group>,
        ) -> Self {
            self.groups = groups;
            self
        }
    }
    #[doc = "A named set of definitions, typically sharing a compatibility promise.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Group {
        #[doc = "The description of the group, e.g., its compatibility promise.\n"]
        pub description: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Group {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Group",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The description of the group, e.g., its compatibility promise.\n"),
            ])),
        )
        .with_docs("A named set of definitions, typically sharing a compatibility promise.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Group {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Group {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.description)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Group {
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
                description: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `description`"))?,
            })
        }
    }
    impl Group {
        #[doc = "Creates a new [`Group`]."]
        pub fn new(description: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self {
                description: description.into(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.description = description.into();
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.description = description.into();
            self
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
    #[automatically_derived]
    impl ::sidex::Shaped for Metadata {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Metadata",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The name of the bundle.\n"),
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The version of the bundle.\n"),
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The optional description of the bundle.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "authors",
                    ::sidex::shape::shape_of::<::std::vec::Vec<::std::string::String>>,
                )
                .with_docs("The optional authors of the bundle.\n")
                .optional(),
            ])),
        )
        .with_docs("Metadata of a bundle.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
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
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.version)),
                2usize => {
                    self.description
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => {
                    self.authors
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
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
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
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
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
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                version: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `version`"))?,
                description: __field_2,
                authors: __field_3,
            })
        }
    }
    impl Metadata {
        #[doc = "Creates a new [`Metadata`]."]
        pub fn new(
            name: impl ::std::convert::Into<::std::string::String>,
            version: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            Self {
                name: name.into(),
                version: version.into(),
                description: ::std::default::Default::default(),
                authors: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn set_version(
            &mut self,
            version: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.version = version.into();
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn with_version(
            mut self,
            version: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.version = version.into();
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
    #[doc = "An *identifier* with an optional span.\n"]
    #[derive(Clone, Debug)]
    pub struct Ident {
        #[doc = "The identifier.\n"]
        pub name: ::std::string::String,
        #[doc = "The optional span of the identifier.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Ident {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Ident",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The identifier.\n"),
                ::sidex::shape::FieldShape::new("span", ::sidex::shape::shape_of::<Span>)
                    .with_docs("The optional span of the identifier.\n")
                    .optional(),
            ])),
        )
        .with_docs("An *identifier* with an optional span.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for Ident {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Ident {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Ident {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                span: __field_1,
            })
        }
    }
    impl Ident {
        #[doc = "Creates a new [`Ident`]."]
        pub fn new(name: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self {
                name: name.into(),
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.name = name.into();
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
    #[doc = "Documentation attached to an item.\n"]
    #[derive(Clone, Debug)]
    pub struct Docs {
        #[doc = "The text of the documentation.\n"]
        pub text: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Docs {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Docs",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "text",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The text of the documentation.\n"),
            ])),
        )
        .with_docs("Documentation attached to an item.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for Docs {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Docs {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.text)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Docs {
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
                text: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `text`"))?,
            })
        }
    }
    impl Docs {
        #[doc = "Creates a new [`Docs`]."]
        pub fn new(text: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self { text: text.into() }
        }
        #[doc = "Sets the value of `text`."]
        pub fn set_text(
            &mut self,
            text: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.text = text.into();
            self
        }
        #[doc = "Sets the value of `text`."]
        pub fn with_text(mut self, text: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.text = text.into();
            self
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
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        #[doc = "The definitions of the schema.\n"]
        pub defs: ::std::vec::Vec<DefIdx>,
        #[doc = "Names visible in the schema's namespace, keyed by the local name\neach entry is reachable as. Includes same-schema defs, imports\n(including wildcard expansions), and the implicit `core::builtins`\ndefs. Mirrors the table the resolver consults at build time so\npost-build passes (typed-attrs `TypeRef` resolution, lints) can\nlook up a name without re-parsing the imports.\n"]
        pub imports: ::std::collections::HashMap<::std::string::String, ImportTarget>,
        #[doc = "The source of the schema.\n"]
        pub source: ::std::option::Option<SourceIdx>,
        #[doc = "The optional span of the schema.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Schema {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::Schema" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("bundle" , :: sidex :: shape :: shape_of :: < BundleIdx > ,) . with_docs ("The bundle the schema is part of.\n") , :: sidex :: shape :: FieldShape :: new ("name" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("The name of the schema.\n") , :: sidex :: shape :: FieldShape :: new ("docs" , :: sidex :: shape :: shape_of :: < Docs > ,) . with_docs ("The documentation of the schema.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("attrs" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Attr , > > ,) . with_docs ("The attributes of the schema.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("typed_attrs" , :: sidex :: shape :: shape_of :: < :: std :: collections :: HashMap < :: std :: string :: String , Value , > > ,) . with_docs ("The parsed, schema-validated attributes, keyed by plugin id.\nPopulated by the compiler for plugins whose attribute schemas are\nloaded; raw [`attrs`] remains the round-trip form for unknown plugins.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) . json_name ("typedAttrs") , :: sidex :: shape :: FieldShape :: new ("defs" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < DefIdx , > > ,) . with_docs ("The definitions of the schema.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("imports" , :: sidex :: shape :: shape_of :: < :: std :: collections :: HashMap < :: std :: string :: String , ImportTarget , > > ,) . with_docs ("Names visible in the schema's namespace, keyed by the local name\neach entry is reachable as. Includes same-schema defs, imports\n(including wildcard expansions), and the implicit `core::builtins`\ndefs. Mirrors the table the resolver consults at build time so\npost-build passes (typed-attrs `TypeRef` resolution, lints) can\nlook up a name without re-parsing the imports.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("source" , :: sidex :: shape :: shape_of :: < SourceIdx > ,) . with_docs ("The source of the schema.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("span" , :: sidex :: shape :: shape_of :: < Span > ,) . with_docs ("The optional span of the schema.\n") . optional ()]))) . with_docs ("A schema is a collection of definitions.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Path ("non_exhaustive")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Schema {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Schema {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.bundle)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                2usize => {
                    self.docs
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.attrs)),
                4usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.typed_attrs)),
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.defs)),
                6usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.imports)),
                7usize => {
                    self.source
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                8usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Schema {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<BundleIdx>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<Docs>::None;
            let mut __field_3 = ::std::option::Option::<::std::vec::Vec<Attr>>::None;
            let mut __field_4 = ::std::option::Option::<
                ::std::collections::HashMap<::std::string::String, Value>,
            >::None;
            let mut __field_5 = ::std::option::Option::<::std::vec::Vec<DefIdx>>::None;
            let mut __field_6 = ::std::option::Option::<
                ::std::collections::HashMap<::std::string::String, ImportTarget>,
            >::None;
            let mut __field_7 = ::std::option::Option::<SourceIdx>::None;
            let mut __field_8 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <BundleIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<Docs as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::vec::Vec<Attr> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::collections::HashMap<
                            ::std::string::String,
                            Value,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <::std::vec::Vec<DefIdx> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::std::collections::HashMap<
                            ::std::string::String,
                            ImportTarget,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(
                            <SourceIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    8usize => {
                        __field_8 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                bundle: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bundle`"))?,
                name: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                docs: __field_2,
                attrs: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `attrs`"))?,
                typed_attrs: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `typedAttrs`"))?,
                defs: __field_5.ok_or_else(|| ::sidex::DecodeError::new("missing field `defs`"))?,
                imports: __field_6
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `imports`"))?,
                source: __field_7,
                span: __field_8,
            })
        }
    }
    impl Schema {
        #[doc = "Creates a new [`Schema`]."]
        pub fn new(
            bundle: BundleIdx,
            name: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            Self {
                bundle: bundle,
                name: name.into(),
                docs: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                typed_attrs: ::std::default::Default::default(),
                defs: ::std::default::Default::default(),
                imports: ::std::default::Default::default(),
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
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.name = name.into();
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
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
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
        #[doc = "Sets the value of `imports`."]
        pub fn set_imports(
            &mut self,
            imports: ::std::collections::HashMap<::std::string::String, ImportTarget>,
        ) -> &mut Self {
            self.imports = imports;
            self
        }
        #[doc = "Sets the value of `imports`."]
        pub fn with_imports(
            mut self,
            imports: ::std::collections::HashMap<::std::string::String, ImportTarget>,
        ) -> Self {
            self.imports = imports;
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
    #[doc = "What a name in a schema's [`imports`] table resolves to.\n"]
    #[derive(Clone, Debug)]
    pub enum ImportTarget {
        #[doc = "Another bundle, e.g. `import other_bundle`.\n"]
        Bundle(BundleIdx),
        #[doc = "A schema in some bundle, e.g. `import bundle::module`.\n"]
        Schema(SchemaIdx),
        #[doc = "A definition, e.g. `import bundle::module::Foo` or any wildcard\nexpansion that pulled `Foo` in.\n"]
        Def(DefRef),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ImportTarget {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::ImportTarget" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Bundle" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < BundleIdx >) ,) . with_docs ("Another bundle, e.g. `import other_bundle`.\n") , :: sidex :: shape :: CaseShape :: new ("Schema" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < SchemaIdx >) ,) . with_docs ("A schema in some bundle, e.g. `import bundle::module`.\n") , :: sidex :: shape :: CaseShape :: new ("Def" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < DefRef >) ,) . with_docs ("A definition, e.g. `import bundle::module::Foo` or any wildcard\nexpansion that pulled `Foo` in.\n")]))) . with_docs ("What a name in a schema's [`imports`] table resolves to.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("internally") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for ImportTarget {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for ImportTarget {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Bundle(_) => 0usize,
                Self::Schema(_) => 1usize,
                Self::Def(_) => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Bundle(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Schema(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Def(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ImportTarget {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Bundle(<BundleIdx as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Schema(<SchemaIdx as ::sidex::Decode>::decode(decoder)?),
                    2usize => Self::Def(<DefRef as ::sidex::Decode>::decode(decoder)?),
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
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        #[doc = "The names of the groups the definition belongs to, sorted.\n\nIncludes the groups assigned with `#[group(...)]` and the groups of\nall definitions of the same bundle that use this definition.\n"]
        pub groups: ::std::vec::Vec<::std::string::String>,
        #[doc = "The kind of the definition.\n"]
        pub kind: DefKind,
        #[doc = "The optional span of the definition's name.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Def {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::Def" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("schema" , :: sidex :: shape :: shape_of :: < SchemaIdx > ,) . with_docs ("The schema the definition belongs to.\n") , :: sidex :: shape :: FieldShape :: new ("name" , :: sidex :: shape :: shape_of :: < Ident > ,) . with_docs ("The name of the definition.\n") , :: sidex :: shape :: FieldShape :: new ("docs" , :: sidex :: shape :: shape_of :: < Docs > ,) . with_docs ("The documentation of the definition.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("vars" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < TypeVar , > > ,) . with_docs ("The type variables of the definition.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("attrs" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Attr , > > ,) . with_docs ("The attributes of the definition.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("typed_attrs" , :: sidex :: shape :: shape_of :: < :: std :: collections :: HashMap < :: std :: string :: String , Value , > > ,) . with_docs ("The parsed, schema-validated attributes, keyed by plugin id.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) . json_name ("typedAttrs") , :: sidex :: shape :: FieldShape :: new ("groups" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("The names of the groups the definition belongs to, sorted.\n\nIncludes the groups assigned with `#[group(...)]` and the groups of\nall definitions of the same bundle that use this definition.\n") . with_attrs (& [:: sidex :: shape :: Attr :: Path ("default")]) , :: sidex :: shape :: FieldShape :: new ("kind" , :: sidex :: shape :: shape_of :: < DefKind > ,) . with_docs ("The kind of the definition.\n") , :: sidex :: shape :: FieldShape :: new ("span" , :: sidex :: shape :: shape_of :: < Span > ,) . with_docs ("The optional span of the definition's name.\n") . optional ()]))) . with_docs ("A definition.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Path ("non_exhaustive")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Def {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Def {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.schema)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                2usize => {
                    self.docs
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.vars)),
                4usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.attrs)),
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.typed_attrs)),
                6usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.groups)),
                7usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.kind)),
                8usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Def {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<SchemaIdx>::None;
            let mut __field_1 = ::std::option::Option::<Ident>::None;
            let mut __field_2 = ::std::option::Option::<Docs>::None;
            let mut __field_3 = ::std::option::Option::<::std::vec::Vec<TypeVar>>::None;
            let mut __field_4 = ::std::option::Option::<::std::vec::Vec<Attr>>::None;
            let mut __field_5 = ::std::option::Option::<
                ::std::collections::HashMap<::std::string::String, Value>,
            >::None;
            let mut __field_6 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_7 = ::std::option::Option::<DefKind>::None;
            let mut __field_8 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <SchemaIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<Ident as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<Docs as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::vec::Vec<TypeVar> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::std::vec::Vec<Attr> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(<::std::collections::HashMap<
                            ::std::string::String,
                            Value,
                        > as ::sidex::Decode>::decode(
                            decoder
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
                        __field_7 = ::std::option::Option::Some(
                            <DefKind as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    8usize => {
                        __field_8 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                schema: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `schema`"))?,
                name: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                docs: __field_2,
                vars: __field_3.ok_or_else(|| ::sidex::DecodeError::new("missing field `vars`"))?,
                attrs: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `attrs`"))?,
                typed_attrs: __field_5
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `typedAttrs`"))?,
                groups: __field_6
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `groups`"))?,
                kind: __field_7.ok_or_else(|| ::sidex::DecodeError::new("missing field `kind`"))?,
                span: __field_8,
            })
        }
    }
    impl Def {
        #[doc = "Creates a new [`Def`]."]
        pub fn new(schema: SchemaIdx, name: Ident, kind: DefKind) -> Self {
            Self {
                schema: schema,
                name: name,
                kind: kind,
                docs: ::std::default::Default::default(),
                vars: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
                typed_attrs: ::std::default::Default::default(),
                groups: ::std::default::Default::default(),
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
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        ) -> Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `groups`."]
        pub fn set_groups(&mut self, groups: ::std::vec::Vec<::std::string::String>) -> &mut Self {
            self.groups = groups;
            self
        }
        #[doc = "Sets the value of `groups`."]
        pub fn with_groups(mut self, groups: ::std::vec::Vec<::std::string::String>) -> Self {
            self.groups = groups;
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
    #[doc = "A type variable of a definition.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TypeVar {
        #[doc = "The name of the type variable.\n"]
        pub name: Ident,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TypeVar {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::TypeVar",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("name", ::sidex::shape::shape_of::<Ident>)
                    .with_docs("The name of the type variable.\n"),
            ])),
        )
        .with_docs("A type variable of a definition.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for TypeVar {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for TypeVar {
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
    impl ::sidex::Decode for TypeVar {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Ident>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<Ident as ::sidex::Decode>::decode(
                            decoder,
                        )?)
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
    impl TypeVar {
        #[doc = "Creates a new [`TypeVar`]."]
        pub fn new(name: Ident) -> Self {
            Self { name: name }
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
    impl ::sidex::Shaped for DefKind {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::DefKind",
            ::sidex::shape::ShapeKind::Variant(::sidex::shape::VariantShape::new(&[
                ::sidex::shape::CaseShape::new(
                    "TypeAlias",
                    ::std::option::Option::Some(::sidex::shape::shape_of::<TypeAliasDef>),
                )
                .with_docs("Definition of a type alias.\n"),
                ::sidex::shape::CaseShape::new(
                    "OpaqueType",
                    ::std::option::Option::Some(::sidex::shape::shape_of::<OpaqueTypeDef>),
                )
                .with_docs("Definition of an opaque type.\n"),
                ::sidex::shape::CaseShape::new(
                    "RecordType",
                    ::std::option::Option::Some(::sidex::shape::shape_of::<RecordTypeDef>),
                )
                .with_docs("Definition of a record type.\n"),
                ::sidex::shape::CaseShape::new(
                    "VariantType",
                    ::std::option::Option::Some(::sidex::shape::shape_of::<VariantTypeDef>),
                )
                .with_docs("Definition of a variant type.\n"),
                ::sidex::shape::CaseShape::new(
                    "WrapperType",
                    ::std::option::Option::Some(::sidex::shape::shape_of::<WrapperTypeDef>),
                )
                .with_docs("Definition of a wrapper type.\n"),
            ])),
        )
        .with_docs("A definition kind.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "tagged",
                value: ::sidex::shape::AttrValue::Path("internally"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefKind {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for DefKind {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::TypeAlias(_) => 0usize,
                Self::OpaqueType(_) => 1usize,
                Self::RecordType(_) => 2usize,
                Self::VariantType(_) => 3usize,
                Self::WrapperType(_) => 4usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::TypeAlias(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::OpaqueType(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::RecordType(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::VariantType(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::WrapperType(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefKind {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::TypeAlias(<TypeAliasDef as ::sidex::Decode>::decode(decoder)?),
                    1usize => {
                        Self::OpaqueType(<OpaqueTypeDef as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => {
                        Self::RecordType(<RecordTypeDef as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        Self::VariantType(<VariantTypeDef as ::sidex::Decode>::decode(decoder)?)
                    }
                    4usize => {
                        Self::WrapperType(<WrapperTypeDef as ::sidex::Decode>::decode(decoder)?)
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
    #[doc = "A definition of a type alias.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TypeAliasDef {
        #[doc = "The type that is aliased.\n"]
        pub aliased: Type,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TypeAliasDef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::TypeAliasDef",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("aliased", ::sidex::shape::shape_of::<Type>)
                    .with_docs("The type that is aliased.\n"),
            ])),
        )
        .with_docs("A definition of a type alias.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for TypeAliasDef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for TypeAliasDef {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.aliased)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for TypeAliasDef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Type>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 =
                            ::std::option::Option::Some(<Type as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                aliased: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `aliased`"))?,
            })
        }
    }
    impl TypeAliasDef {
        #[doc = "Creates a new [`TypeAliasDef`]."]
        pub fn new(aliased: Type) -> Self {
            Self { aliased: aliased }
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
    #[doc = "A definition of an opaque type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct OpaqueTypeDef {}
    #[automatically_derived]
    impl ::sidex::Shaped for OpaqueTypeDef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::OpaqueTypeDef",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[])),
        )
        .with_docs("A definition of an opaque type.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for OpaqueTypeDef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for OpaqueTypeDef {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for OpaqueTypeDef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {})
        }
    }
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
    #[doc = "A definition of a record type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct RecordTypeDef {
        #[doc = "The fields of the record type.\n"]
        pub fields: ::std::vec::Vec<Field>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RecordTypeDef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::RecordTypeDef",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "fields",
                    ::sidex::shape::shape_of::<::std::vec::Vec<Field>>,
                )
                .with_docs("The fields of the record type.\n")
                .with_attrs(&[::sidex::shape::Attr::Path("default")]),
            ])),
        )
        .with_docs("A definition of a record type.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for RecordTypeDef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RecordTypeDef {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.fields)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RecordTypeDef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::vec::Vec<Field>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<Field> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                fields: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `fields`"))?,
            })
        }
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
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        #[doc = "The type of the field.\n"]
        pub typ: Type,
        #[doc = "Indicates whether the field is optional.\n"]
        pub is_optional: bool,
        #[doc = "The optional span of the field.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Field {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Field",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("name", ::sidex::shape::shape_of::<Ident>)
                    .with_docs("The name of the field.\n"),
                ::sidex::shape::FieldShape::new("docs", ::sidex::shape::shape_of::<Docs>)
                    .with_docs("The documentation of the field.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new(
                    "attrs",
                    ::sidex::shape::shape_of::<::std::vec::Vec<Attr>>,
                )
                .with_docs("The attributes of the field.\n")
                .with_attrs(&[::sidex::shape::Attr::Path("default")]),
                ::sidex::shape::FieldShape::new(
                    "typed_attrs",
                    ::sidex::shape::shape_of::<
                        ::std::collections::HashMap<::std::string::String, Value>,
                    >,
                )
                .with_docs("The parsed, schema-validated attributes, keyed by plugin id.\n")
                .with_attrs(&[::sidex::shape::Attr::Path("default")])
                .json_name("typedAttrs"),
                ::sidex::shape::FieldShape::new("typ", ::sidex::shape::shape_of::<Type>)
                    .with_docs("The type of the field.\n"),
                ::sidex::shape::FieldShape::new("is_optional", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Indicates whether the field is optional.\n")
                    .with_attrs(&[::sidex::shape::Attr::Path("default")])
                    .json_name("isOptional"),
                ::sidex::shape::FieldShape::new("span", ::sidex::shape::shape_of::<Span>)
                    .with_docs("The optional span of the field.\n")
                    .optional(),
            ])),
        )
        .with_docs("A field of a record type.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Field {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Field {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => {
                    self.docs
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.attrs)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.typed_attrs)),
                4usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.typ)),
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.is_optional)),
                6usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Field {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Ident>::None;
            let mut __field_1 = ::std::option::Option::<Docs>::None;
            let mut __field_2 = ::std::option::Option::<::std::vec::Vec<Attr>>::None;
            let mut __field_3 = ::std::option::Option::<
                ::std::collections::HashMap<::std::string::String, Value>,
            >::None;
            let mut __field_4 = ::std::option::Option::<Type>::None;
            let mut __field_5 = ::std::option::Option::<bool>::None;
            let mut __field_6 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<Ident as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<Docs as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::vec::Vec<Attr> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::collections::HashMap<
                            ::std::string::String,
                            Value,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<Type as ::sidex::Decode>::decode(decoder)?)
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    6usize => {
                        __field_6 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                docs: __field_1,
                attrs: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `attrs`"))?,
                typed_attrs: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `typedAttrs`"))?,
                typ: __field_4.ok_or_else(|| ::sidex::DecodeError::new("missing field `typ`"))?,
                is_optional: __field_5
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `isOptional`"))?,
                span: __field_6,
            })
        }
    }
    impl Field {
        #[doc = "Creates a new [`Field`]."]
        pub fn new(name: Ident, typ: Type) -> Self {
            Self {
                name: name,
                typ: typ,
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
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
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
    #[doc = "A definition of a variant type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct VariantTypeDef {
        #[doc = "The variants of the variant type.\n"]
        pub variants: ::std::vec::Vec<Variant>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for VariantTypeDef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::VariantTypeDef",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "variants",
                    ::sidex::shape::shape_of::<::std::vec::Vec<Variant>>,
                )
                .with_docs("The variants of the variant type.\n")
                .with_attrs(&[::sidex::shape::Attr::Path("default")]),
            ])),
        )
        .with_docs("A definition of a variant type.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for VariantTypeDef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for VariantTypeDef {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.variants)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for VariantTypeDef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::vec::Vec<Variant>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<Variant> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                variants: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `variants`"))?,
            })
        }
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
        pub typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        #[doc = "The optional payload type of the variant.\n"]
        pub typ: ::std::option::Option<Type>,
        #[doc = "The optional span of the variant.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Variant {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Variant",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("name", ::sidex::shape::shape_of::<Ident>)
                    .with_docs("The name of the variant.\n"),
                ::sidex::shape::FieldShape::new("docs", ::sidex::shape::shape_of::<Docs>)
                    .with_docs("The documentation of the variant.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new(
                    "attrs",
                    ::sidex::shape::shape_of::<::std::vec::Vec<Attr>>,
                )
                .with_docs("The attributes of the variant.\n")
                .with_attrs(&[::sidex::shape::Attr::Path("default")]),
                ::sidex::shape::FieldShape::new(
                    "typed_attrs",
                    ::sidex::shape::shape_of::<
                        ::std::collections::HashMap<::std::string::String, Value>,
                    >,
                )
                .with_docs("The parsed, schema-validated attributes, keyed by plugin id.\n")
                .with_attrs(&[::sidex::shape::Attr::Path("default")])
                .json_name("typedAttrs"),
                ::sidex::shape::FieldShape::new("typ", ::sidex::shape::shape_of::<Type>)
                    .with_docs("The optional payload type of the variant.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new("span", ::sidex::shape::shape_of::<Span>)
                    .with_docs("The optional span of the variant.\n")
                    .optional(),
            ])),
        )
        .with_docs("A variant of a variant type.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Variant {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Variant {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => {
                    self.docs
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.attrs)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.typed_attrs)),
                4usize => {
                    self.typ
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                5usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Variant {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Ident>::None;
            let mut __field_1 = ::std::option::Option::<Docs>::None;
            let mut __field_2 = ::std::option::Option::<::std::vec::Vec<Attr>>::None;
            let mut __field_3 = ::std::option::Option::<
                ::std::collections::HashMap<::std::string::String, Value>,
            >::None;
            let mut __field_4 = ::std::option::Option::<Type>::None;
            let mut __field_5 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<Ident as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<Docs as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::vec::Vec<Attr> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::collections::HashMap<
                            ::std::string::String,
                            Value,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<Type as ::sidex::Decode>::decode(decoder)?)
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                docs: __field_1,
                attrs: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `attrs`"))?,
                typed_attrs: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `typedAttrs`"))?,
                typ: __field_4,
                span: __field_5,
            })
        }
    }
    impl Variant {
        #[doc = "Creates a new [`Variant`]."]
        pub fn new(name: Ident) -> Self {
            Self {
                name: name,
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
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
        ) -> &mut Self {
            self.typed_attrs = typed_attrs;
            self
        }
        #[doc = "Sets the value of `typed_attrs`."]
        pub fn with_typed_attrs(
            mut self,
            typed_attrs: ::std::collections::HashMap<::std::string::String, Value>,
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
    #[doc = "A definition of a wrapper type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct WrapperTypeDef {
        #[doc = "The type that is wrapped.\n"]
        pub wrapped: Type,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for WrapperTypeDef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::WrapperTypeDef",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("wrapped", ::sidex::shape::shape_of::<Type>)
                    .with_docs("The type that is wrapped.\n"),
            ])),
        )
        .with_docs("A definition of a wrapper type.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for WrapperTypeDef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for WrapperTypeDef {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.wrapped)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for WrapperTypeDef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Type>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 =
                            ::std::option::Option::Some(<Type as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                wrapped: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `wrapped`"))?,
            })
        }
    }
    impl WrapperTypeDef {
        #[doc = "Creates a new [`WrapperTypeDef`]."]
        pub fn new(wrapped: Type) -> Self {
            Self { wrapped: wrapped }
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
    #[doc = "A type.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Type {
        #[doc = "The kind of the type.\n"]
        pub kind: TypeKind,
        #[doc = "The span of the type expression.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Type {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Type",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("kind", ::sidex::shape::shape_of::<TypeKind>)
                    .with_docs("The kind of the type.\n"),
                ::sidex::shape::FieldShape::new("span", ::sidex::shape::shape_of::<Span>)
                    .with_docs("The span of the type expression.\n")
                    .optional(),
            ])),
        )
        .with_docs("A type.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Type {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Type {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.kind)),
                1usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Type {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<TypeKind>::None;
            let mut __field_1 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <TypeKind as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                kind: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `kind`"))?,
                span: __field_1,
            })
        }
    }
    impl Type {
        #[doc = "Creates a new [`Type`]."]
        pub fn new(kind: TypeKind) -> Self {
            Self {
                kind: kind,
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
    #[doc = "An abstract type kind.\n\nBuilt-in types like `[T]`, `[K: V]`, and `()` are represented as\ninstantiations of `core::builtins::Sequence`, `core::builtins::Map`,\nand `core::builtins::unit`. There is no separate variant for them.\n"]
    #[derive(Clone, Debug)]
    pub enum TypeKind {
        #[doc = "A type to be determined via substitution of the respective type variable.\n"]
        TypeVar(TypeVarType),
        #[doc = "An instantiation of a type defined in some schema.\n"]
        Instance(InstanceType),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TypeKind {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::TypeKind" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("TypeVar" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < TypeVarType >) ,) . with_docs ("A type to be determined via substitution of the respective type variable.\n") , :: sidex :: shape :: CaseShape :: new ("Instance" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < InstanceType >) ,) . with_docs ("An instantiation of a type defined in some schema.\n")]))) . with_docs ("An abstract type kind.\n\nBuilt-in types like `[T]`, `[K: V]`, and `()` are represented as\ninstantiations of `core::builtins::Sequence`, `core::builtins::Map`,\nand `core::builtins::unit`. There is no separate variant for them.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("internally") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for TypeKind {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for TypeKind {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::TypeVar(_) => 0usize,
                Self::Instance(_) => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::TypeVar(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Instance(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for TypeKind {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::TypeVar(<TypeVarType as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Instance(<InstanceType as ::sidex::Decode>::decode(decoder)?),
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
    #[doc = "A type to be determined via substitution of the respective type variable.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TypeVarType {
        #[doc = "The index of the type variable in the enclosing definition.\n"]
        pub idx: TypeVarIdx,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TypeVarType {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::TypeVarType",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("idx", ::sidex::shape::shape_of::<TypeVarIdx>)
                    .with_docs("The index of the type variable in the enclosing definition.\n"),
            ])),
        )
        .with_docs("A type to be determined via substitution of the respective type variable.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for TypeVarType {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for TypeVarType {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.idx)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for TypeVarType {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<TypeVarIdx>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <TypeVarIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                idx: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `idx`"))?,
            })
        }
    }
    impl TypeVarType {
        #[doc = "Creates a new [`TypeVarType`]."]
        pub fn new(idx: TypeVarIdx) -> Self {
            Self { idx: idx }
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
    #[doc = "An instantiation of a type defined in some schema.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct InstanceType {
        #[doc = "The definition being instantiated.\n"]
        pub def: DefRef,
        #[doc = "Substitutions for the type variables of the definition.\n"]
        pub subst: ::std::vec::Vec<Type>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for InstanceType {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::InstanceType",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("def", ::sidex::shape::shape_of::<DefRef>)
                    .with_docs("The definition being instantiated.\n"),
                ::sidex::shape::FieldShape::new(
                    "subst",
                    ::sidex::shape::shape_of::<::std::vec::Vec<Type>>,
                )
                .with_docs("Substitutions for the type variables of the definition.\n")
                .with_attrs(&[::sidex::shape::Attr::Path("default")]),
            ])),
        )
        .with_docs("An instantiation of a type defined in some schema.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for InstanceType {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for InstanceType {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.def)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.subst)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for InstanceType {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<DefRef>::None;
            let mut __field_1 = ::std::option::Option::<::std::vec::Vec<Type>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <DefRef as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::vec::Vec<Type> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                def: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `def`"))?,
                subst: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `subst`"))?,
            })
        }
    }
    impl InstanceType {
        #[doc = "Creates a new [`InstanceType`]."]
        pub fn new(def: DefRef) -> Self {
            Self {
                def: def,
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
    #[automatically_derived]
    impl ::sidex::Shaped for Span {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Span",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("src", ::sidex::shape::shape_of::<SourceIdx>)
                    .with_docs("The source.\n"),
                ::sidex::shape::FieldShape::new("start", ::sidex::shape::shape_of::<usize>)
                    .with_docs("The start character.\n"),
                ::sidex::shape::FieldShape::new("end", ::sidex::shape::shape_of::<usize>)
                    .with_docs("The included end character.\n"),
            ])),
        )
        .with_docs("A *span* identifies a range of text in a source.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[
                ::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                        ::sidex::shape::Attr::Path("Hash"),
                    ],
                },
                ::sidex::shape::Attr::List {
                    path: "attr",
                    args: &[::sidex::shape::Attr::Path("non_exhaustive")],
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Span {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Span {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.src)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.start)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.end)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Span {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<SourceIdx>::None;
            let mut __field_1 = ::std::option::Option::<usize>::None;
            let mut __field_2 = ::std::option::Option::<usize>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <SourceIdx as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<usize as ::sidex::Decode>::decode(
                            decoder,
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                src: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `src`"))?,
                start: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `start`"))?,
                end: __field_2.ok_or_else(|| ::sidex::DecodeError::new("missing field `end`"))?,
            })
        }
    }
    impl Span {
        #[doc = "Creates a new [`Span`]."]
        pub fn new(src: SourceIdx, start: usize, end: usize) -> Self {
            Self {
                src: src,
                start: start,
                end: end,
            }
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
    #[doc = "An attribute.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct Attr {
        #[doc = "The kind of the attribute.\n"]
        pub kind: AttrKind,
        #[doc = "The span of the attribute.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Attr {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::Attr",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("kind", ::sidex::shape::shape_of::<AttrKind>)
                    .with_docs("The kind of the attribute.\n"),
                ::sidex::shape::FieldShape::new("span", ::sidex::shape::shape_of::<Span>)
                    .with_docs("The span of the attribute.\n")
                    .optional(),
            ])),
        )
        .with_docs("An attribute.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "attr",
                args: &[::sidex::shape::Attr::Path("non_exhaustive")],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Attr {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Attr {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.kind)),
                1usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Attr {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<AttrKind>::None;
            let mut __field_1 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <AttrKind as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                kind: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `kind`"))?,
                span: __field_1,
            })
        }
    }
    impl Attr {
        #[doc = "Creates a new [`Attr`]."]
        pub fn new(kind: AttrKind) -> Self {
            Self {
                kind: kind,
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
    #[doc = "The kinds of attributes.\n"]
    #[derive(Clone, Debug)]
    pub enum AttrKind {
        #[doc = "A bare `::`-separated path, e.g. `pub`.\n"]
        Path(::std::string::String),
        #[doc = "A list attribute of the form `<PATH> ( <ARGS> )` where `<ARGS>` is a\n`,`-separated sequence of arguments.\n"]
        List(AttrList),
        #[doc = "An assign attribute of the form `<PATH> = <VALUE>`.\n"]
        Assign(AttrAssign),
        #[doc = "An anonymous (positional) value used as a list argument — e.g. the\n`{ … }` body in `#[validate({ 0 <= _ <= 100 })]`. Carries the value\ndirectly, with no path. The typed-attrs parser binds these to a\ndesignated schema field (the first field whose declared type is\nthe [`core::attrs::Tokens`] marker, in source order).\n"]
        Value(AttrValue),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AttrKind {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::AttrKind" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Path" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < :: std :: string :: String >) ,) . with_docs ("A bare `::`-separated path, e.g. `pub`.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "content" , value : :: sidex :: shape :: AttrValue :: String ("path") }] }]) . json_content ("path") , :: sidex :: shape :: CaseShape :: new ("List" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < AttrList >) ,) . with_docs ("A list attribute of the form `<PATH> ( <ARGS> )` where `<ARGS>` is a\n`,`-separated sequence of arguments.\n") , :: sidex :: shape :: CaseShape :: new ("Assign" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < AttrAssign >) ,) . with_docs ("An assign attribute of the form `<PATH> = <VALUE>`.\n") , :: sidex :: shape :: CaseShape :: new ("Value" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < AttrValue >) ,) . with_docs ("An anonymous (positional) value used as a list argument — e.g. the\n`{ … }` body in `#[validate({ 0 <= _ <= 100 })]`. Carries the value\ndirectly, with no path. The typed-attrs parser binds these to a\ndesignated schema field (the first field whose declared type is\nthe [`core::attrs::Tokens`] marker, in source order).\n")]))) . with_docs ("The kinds of attributes.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("internally") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for AttrKind {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for AttrKind {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Path(_) => 0usize,
                Self::List(_) => 1usize,
                Self::Assign(_) => 2usize,
                Self::Value(_) => 3usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Path(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::List(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Assign(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Value(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for AttrKind {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => {
                        Self::Path(<::std::string::String as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => Self::List(<AttrList as ::sidex::Decode>::decode(decoder)?),
                    2usize => Self::Assign(<AttrAssign as ::sidex::Decode>::decode(decoder)?),
                    3usize => Self::Value(<AttrValue as ::sidex::Decode>::decode(decoder)?),
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
    #[doc = "A list attribute of the form `<PATH> ( <ARGS> )`.\n"]
    #[derive(Clone, Debug)]
    pub struct AttrList {
        #[doc = "The path of the attribute.\n"]
        pub path: ::std::string::String,
        #[doc = "The arguments of the attribute.\n"]
        pub args: ::std::vec::Vec<Attr>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AttrList {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::AttrList",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "path",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The path of the attribute.\n"),
                ::sidex::shape::FieldShape::new(
                    "args",
                    ::sidex::shape::shape_of::<::std::vec::Vec<Attr>>,
                )
                .with_docs("The arguments of the attribute.\n"),
            ])),
        )
        .with_docs("A list attribute of the form `<PATH> ( <ARGS> )`.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for AttrList {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for AttrList {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.path)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.args)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for AttrList {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::vec::Vec<Attr>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::vec::Vec<Attr> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                path: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `path`"))?,
                args: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `args`"))?,
            })
        }
    }
    impl AttrList {
        #[doc = "Creates a new [`AttrList`]."]
        pub fn new(
            path: impl ::std::convert::Into<::std::string::String>,
            args: ::std::vec::Vec<Attr>,
        ) -> Self {
            Self {
                path: path.into(),
                args: args,
            }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(
            &mut self,
            path: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.path = path.into();
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.path = path.into();
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
    #[doc = "An assign attribute of the form `<PATH> = <VALUE>`.\n"]
    #[derive(Clone, Debug)]
    pub struct AttrAssign {
        #[doc = "The path of the attribute.\n"]
        pub path: ::std::string::String,
        #[doc = "The assigned value.\n"]
        pub value: AttrValue,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AttrAssign {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::meta::ir::AttrAssign",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "path",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("The path of the attribute.\n"),
                ::sidex::shape::FieldShape::new("value", ::sidex::shape::shape_of::<AttrValue>)
                    .with_docs("The assigned value.\n"),
            ])),
        )
        .with_docs("An assign attribute of the form `<PATH> = <VALUE>`.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for AttrAssign {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for AttrAssign {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.path)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for AttrAssign {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<AttrValue>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <AttrValue as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                path: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `path`"))?,
                value: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `value`"))?,
            })
        }
    }
    impl AttrAssign {
        #[doc = "Creates a new [`AttrAssign`]."]
        pub fn new(
            path: impl ::std::convert::Into<::std::string::String>,
            value: AttrValue,
        ) -> Self {
            Self {
                path: path.into(),
                value: value,
            }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(
            &mut self,
            path: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.path = path.into();
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.path = path.into();
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
        #[doc = "A `{ … }`-form value carrying source verbatim.\n\nPlugins that opt into this form (via a typed-attrs schema field of\ntype `core::attrs::Tokens`) capture the source text and parse it\nwith their own grammar — e.g. the validation extension's predicate\nexpressions.\n"]
        Tokens(TokensValue),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AttrValue {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::AttrValue" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Bool" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < bool >) ,) . with_docs ("A boolean value.\n") , :: sidex :: shape :: CaseShape :: new ("Number" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < :: std :: string :: String >) ,) . with_docs ("A numeric value (kept as a string to avoid lossy conversion).\n") , :: sidex :: shape :: CaseShape :: new ("String" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < :: std :: string :: String >) ,) . with_docs ("A string value.\n") , :: sidex :: shape :: CaseShape :: new ("Path" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < :: std :: string :: String >) ,) . with_docs ("A `::`-separated path.\n") , :: sidex :: shape :: CaseShape :: new ("Tokens" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < TokensValue >) ,) . with_docs ("A `{ … }`-form value carrying source verbatim.\n\nPlugins that opt into this form (via a typed-attrs schema field of\ntype `core::attrs::Tokens`) capture the source text and parse it\nwith their own grammar — e.g. the validation extension's predicate\nexpressions.\n")]))) . with_docs ("The value of an assign attribute.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("internally") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for AttrValue {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for AttrValue {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Bool(_) => 0usize,
                Self::Number(_) => 1usize,
                Self::String(_) => 2usize,
                Self::Path(_) => 3usize,
                Self::Tokens(_) => 4usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Bool(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Number(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::String(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Path(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Tokens(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for AttrValue {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Bool(<bool as ::sidex::Decode>::decode(decoder)?),
                    1usize => {
                        Self::Number(<::std::string::String as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => {
                        Self::String(<::std::string::String as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        Self::Path(<::std::string::String as ::sidex::Decode>::decode(decoder)?)
                    }
                    4usize => Self::Tokens(<TokensValue as ::sidex::Decode>::decode(decoder)?),
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
    #[doc = "A `{ … }`-form attribute value.\n\nCarries the verbatim source text between the `{` and `}` (both excluded)\nalong with the span covering that text. Plugin codegens parse the text\nwith their own grammar.\n"]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TokensValue {
        #[doc = "Verbatim source between the surrounding `{` and `}`, both excluded.\n"]
        pub text: ::std::string::String,
        #[doc = "Span of [`text`] within the originating source.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TokensValue {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::meta::ir::TokensValue" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("text" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Verbatim source between the surrounding `{` and `}`, both excluded.\n") , :: sidex :: shape :: FieldShape :: new ("span" , :: sidex :: shape :: shape_of :: < Span > ,) . with_docs ("Span of [`text`] within the originating source.\n") . optional ()]))) . with_docs ("A `{ … }`-form attribute value.\n\nCarries the verbatim source text between the `{` and `}` (both excluded)\nalong with the span covering that text. Plugin codegens parse the text\nwith their own grammar.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "attr" , args : & [:: sidex :: shape :: Attr :: Path ("non_exhaustive")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for TokensValue {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for TokensValue {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.text)),
                1usize => {
                    self.span
                        .as_ref()
                        .map(|value| ::sidex::Encode::encode(value))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for TokensValue {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<Span>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<Span as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                text: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `text`"))?,
                span: __field_1,
            })
        }
    }
    impl TokensValue {
        #[doc = "Creates a new [`TokensValue`]."]
        pub fn new(text: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self {
                text: text.into(),
                span: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `text`."]
        pub fn set_text(
            &mut self,
            text: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.text = text.into();
            self
        }
        #[doc = "Sets the value of `text`."]
        pub fn with_text(mut self, text: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.text = text.into();
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
}
