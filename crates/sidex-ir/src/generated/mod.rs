/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod reflect {
    #[doc = "Uniquely identifies a source in a unit.\n"]
    #[derive(
        :: serde :: Serialize, :: serde :: Deserialize, Clone, Debug, Copy, PartialEq, Eq, Hash,
    )]
    pub struct SourceIdx(pub(crate) usize);
    impl ::std::convert::From<SourceIdx> for usize {
        fn from(wrapped: SourceIdx) -> Self {
            wrapped.0
        }
    }
    #[doc = "Uniquely identifies a bundle in a unit.\n"]
    #[derive(
        :: serde :: Serialize, :: serde :: Deserialize, Clone, Debug, Copy, PartialEq, Eq, Hash,
    )]
    pub struct BundleIdx(pub(crate) usize);
    impl ::std::convert::From<BundleIdx> for usize {
        fn from(wrapped: BundleIdx) -> Self {
            wrapped.0
        }
    }
    #[doc = "Uniquely identifies a schema in bundle.\n"]
    #[derive(
        :: serde :: Serialize, :: serde :: Deserialize, Clone, Debug, Copy, PartialEq, Eq, Hash,
    )]
    pub struct SchemaIdx(pub(crate) usize);
    impl ::std::convert::From<SchemaIdx> for usize {
        fn from(wrapped: SchemaIdx) -> Self {
            wrapped.0
        }
    }
    #[doc = "Uniquely identifies a definition in a schema.\n"]
    #[derive(
        :: serde :: Serialize, :: serde :: Deserialize, Clone, Debug, Copy, PartialEq, Eq, Hash,
    )]
    pub struct DefIdx(pub(crate) usize);
    impl ::std::convert::From<DefIdx> for usize {
        fn from(wrapped: DefIdx) -> Self {
            wrapped.0
        }
    }
    #[doc = "Uniquely identifies a type variable in a definition.\n"]
    #[derive(
        :: serde :: Serialize, :: serde :: Deserialize, Clone, Debug, Copy, PartialEq, Eq, Hash,
    )]
    pub struct TypeVarIdx(pub(crate) usize);
    impl ::std::convert::From<TypeVarIdx> for usize {
        fn from(wrapped: TypeVarIdx) -> Self {
            wrapped.0
        }
    }
    #[doc = "A *unit* is a collection of bundles.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct Unit {
        #[doc = "The bundles of the unit.\n"]
        pub bundles: ::std::vec::Vec<Bundle>,
        #[doc = "The sources of the unit.\n"]
        pub sources: ::std::vec::Vec<Source>,
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
    impl ::std::default::Default for Unit {
        fn default() -> Self {
            Self::new()
        }
    }
    #[doc = "A *source* is simply a chunk of text.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Source {
        #[doc = "The text of the source.\n"]
        pub text: ::std::string::String,
        #[doc = "The origin of the source, e.g., a filesystem path.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
    }
    impl Source {
        #[doc = "Creates a new [`Source`]."]
        pub fn new(text: ::std::string::String) -> Self {
            Self {
                text,
                origin: ::std::default::Default::default(),
            }
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
    }
    #[doc = "A bundle is a flat collection of schemas evolving together.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A dependency of a bundle.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "Metadata of a bundle.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A schema is a collection of definitions.\n\nNote that imports have already been processed and resolved.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct Schema {
        #[doc = "The index of the schema.\n"]
        pub idx: SchemaIdx,
        #[doc = "The name of the schema.\n"]
        pub name: ::std::string::String,
        #[doc = "The documentation of the schema.\n"]
        pub docs: ::std::string::String,
        #[doc = "The attributes of the schema.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The definitions of the schema.\n"]
        pub defs: ::std::vec::Vec<Def>,
        #[doc = "The source of the schema.\n"]
        pub source: ::std::option::Option<SourceIdx>,
    }
    impl Schema {
        #[doc = "Creates a new [`Schema`]."]
        pub fn new(
            idx: SchemaIdx,
            name: ::std::string::String,
            docs: ::std::string::String,
        ) -> Self {
            Self {
                idx,
                name,
                docs,
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
        pub fn set_docs(&mut self, docs: ::std::string::String) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::string::String) -> Self {
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
    #[doc = "A definition.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct Def {
        #[doc = "The name of the definition.\n"]
        pub name: ::std::string::String,
        #[doc = "The documentation of the definition.\n"]
        pub docs: ::std::string::String,
        #[doc = "The type variables of the definition.\n"]
        pub vars: ::std::vec::Vec<TypeVar>,
        #[doc = "The attributes of the definition.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The kind of the definition.\n"]
        pub kind: DefKind,
    }
    impl Def {
        #[doc = "Creates a new [`Def`]."]
        pub fn new(
            name: ::std::string::String,
            docs: ::std::string::String,
            kind: DefKind,
        ) -> Self {
            Self {
                name,
                docs,
                kind,
                vars: ::std::default::Default::default(),
                attrs: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::string::String) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::string::String) -> Self {
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
    #[doc = "A type variable of a definition.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct TypeVar {
        #[doc = "Name of the type variable.\n"]
        pub name: ::std::string::String,
    }
    impl TypeVar {
        #[doc = "Creates a new [`TypeVar`]."]
        pub fn new(name: ::std::string::String) -> Self {
            Self { name }
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
    }
    #[doc = "A definition kind.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A definition of a type alias.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A definition of an opaque type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A definition of a record type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A field of a record type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct Field {
        #[doc = "The name of the field.\n"]
        pub name: ::std::string::String,
        #[doc = "The documentation of the field.\n"]
        pub docs: ::std::string::String,
        #[doc = "The attributes of the field.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The type of the field.\n"]
        pub typ: Type,
        #[doc = "Indicates whether the field is optional.\n"]
        pub is_optional: bool,
    }
    impl Field {
        #[doc = "Creates a new [`Field`]."]
        pub fn new(
            name: ::std::string::String,
            docs: ::std::string::String,
            typ: Type,
            is_optional: bool,
        ) -> Self {
            Self {
                name,
                docs,
                typ,
                is_optional,
                attrs: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::string::String) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::string::String) -> Self {
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
    #[doc = "A definition of a variant type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A variant of a variant type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct Variant {
        #[doc = "The name of the variant.\n"]
        pub name: ::std::string::String,
        #[doc = "The documentation of the variant.\n"]
        pub docs: ::std::string::String,
        #[doc = "The attributes of the variant.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The optional type of the variant.\n"]
        pub typ: ::std::option::Option<Type>,
    }
    impl Variant {
        #[doc = "Creates a new [`Variant`]."]
        pub fn new(name: ::std::string::String, docs: ::std::string::String) -> Self {
            Self {
                name,
                docs,
                attrs: ::std::default::Default::default(),
                typ: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::string::String) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::string::String) -> Self {
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
    #[doc = "A definition of a wrapper type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A definition of a service.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A method of a service definition.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct Method {
        #[doc = "The name of the method.\n"]
        pub name: ::std::string::String,
        #[doc = "The documentation of the method.\n"]
        pub docs: ::std::string::String,
        #[doc = "The attributes of the method.\n"]
        pub attrs: ::std::vec::Vec<Attr>,
        #[doc = "The parameters of the method.\n"]
        pub parameters: ::std::vec::Vec<MethodParam>,
        #[doc = "The optional return type of the method.\n"]
        pub returns: ::std::option::Option<Type>,
    }
    impl Method {
        #[doc = "Creates a new [`Method`]."]
        pub fn new(name: ::std::string::String, docs: ::std::string::String) -> Self {
            Self {
                name,
                docs,
                attrs: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
                returns: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `docs`."]
        pub fn set_docs(&mut self, docs: ::std::string::String) -> &mut Self {
            self.docs = docs;
            self
        }
        #[doc = "Sets the value of `docs`."]
        pub fn with_docs(mut self, docs: ::std::string::String) -> Self {
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
    #[doc = "A parameter of a method.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct MethodParam {
        #[doc = "The name of the parameter.\n"]
        pub name: ::std::string::String,
        #[doc = "The type of the parameter.\n"]
        pub typ: Type,
        #[doc = "Indicates whether the parameter is optional.\n"]
        pub is_optional: bool,
    }
    impl MethodParam {
        #[doc = "Creates a new [`MethodParam`]."]
        pub fn new(name: ::std::string::String, typ: Type, is_optional: bool) -> Self {
            Self {
                name,
                typ,
                is_optional,
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
    #[doc = "A type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "An abstract type kind.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub enum TypeKind {
        #[doc = "A type to be determined via substitution of the respective type variable.\n"]
        TypeVar(TypeVarType),
        #[doc = "An instantiation of a type defined in some schema.\n"]
        Instance(InstanceType),
    }
    #[doc = "A type to be determined via substitution of the respective type variable.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "An instantiation of a type defined in some schema of some bundle.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "An attribute.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A *span* identifies a range of text in a source.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A token.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = ""]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = ""]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub enum Literal {
        #[doc = ""]
        String(::std::string::String),
        #[doc = ""]
        Number(::std::string::String),
        #[doc = ""]
        Bool(bool),
    }
    #[doc = "A stream of tokens.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct TokenStream(pub(crate) ::std::vec::Vec<Token>);
    impl ::std::convert::From<TokenStream> for ::std::vec::Vec<Token> {
        fn from(wrapped: TokenStream) -> Self {
            wrapped.0
        }
    }
    #[doc = "A `::` separated path of identifiers.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Path(pub(crate) ::std::string::String);
    impl ::std::convert::From<Path> for ::std::string::String {
        fn from(wrapped: Path) -> Self {
            wrapped.0
        }
    }
    #[doc = "A compile-time structured attribute.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub enum AttrKind {
        #[doc = "A `::` separated path.\n"]
        Path(Path),
        #[doc = "A list of `,` separated attributes.\n"]
        List(AttrList),
        #[doc = "An assign attribute.\n"]
        Assign(AttrAssign),
        #[doc = "A stream of tokens.\n"]
        Tokens(TokenStream),
    }
    #[doc = ""]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct AttrList {
        #[doc = ""]
        pub path: Path,
        #[doc = ""]
        pub elements: ::std::vec::Vec<Attr>,
    }
    impl AttrList {
        #[doc = "Creates a new [`AttrList`]."]
        pub fn new(path: Path) -> Self {
            Self {
                path,
                elements: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `elements`."]
        pub fn set_elements(&mut self, elements: ::std::vec::Vec<Attr>) -> &mut Self {
            self.elements = elements;
            self
        }
        #[doc = "Sets the value of `elements`."]
        pub fn with_elements(mut self, elements: ::std::vec::Vec<Attr>) -> Self {
            self.elements = elements;
            self
        }
    }
    #[doc = ""]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct AttrAssign {
        #[doc = ""]
        pub path: Path,
        #[doc = ""]
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
}
