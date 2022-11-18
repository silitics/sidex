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
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug, Default)]
    pub struct Unit {
        #[doc = "The bundles of the unit.\n"]
        pub bundles: ::std::vec::Vec<Bundle>,
        #[doc = "The sources of the unit.\n"]
        pub sources: ::std::vec::Vec<Source>,
    }
    #[doc = "A *source* is simply a chunk of text.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Source {
        #[doc = "The text of the source.\n"]
        pub text: ::std::string::String,
        #[doc = "The origin of the source, e.g., a filesystem path.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
    }
    #[doc = "A bundle is a flat collection of schemas evolving together.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A dependency of a bundle.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Dependency {
        #[doc = "The name of the dependency.\n"]
        pub name: ::std::string::String,
        #[doc = "The bundle.\n"]
        pub bundle: BundleIdx,
    }
    #[doc = "Metadata of a bundle.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A schema is a collection of definitions.\n\nNote that imports have already been processed and resolved.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A definition.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A type variable of a definition.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct TypeVar {
        #[doc = "Name of the type variable.\n"]
        pub name: ::std::string::String,
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
    pub struct TypeAliasDef {
        #[doc = "The type that is aliased.\n"]
        pub aliased: Type,
    }
    #[doc = "A definition of an opaque type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct OpaqueTypeDef {}
    #[doc = "A definition of a record type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct RecordTypeDef {
        #[doc = "The fields of the record type.\n"]
        pub fields: ::std::vec::Vec<Field>,
    }
    #[doc = "A field of a record type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A definition of a variant type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct VariantTypeDef {
        #[doc = "The variants of the variant type.\n"]
        pub variants: ::std::vec::Vec<Variant>,
    }
    #[doc = "A variant of a variant type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A definition of a wrapper type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct WrapperTypeDef {
        #[doc = "The type that is wrapped.\n"]
        pub wrapped: Type,
    }
    #[doc = "A definition of a service.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct ServiceDef {
        #[doc = "The methods provided by the service.\n"]
        pub methods: ::std::vec::Vec<Method>,
    }
    #[doc = "A method of a service definition.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "A parameter of a method.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct MethodParam {
        #[doc = "The name of the parameter.\n"]
        pub name: ::std::string::String,
        #[doc = "The type of the parameter.\n"]
        pub typ: Type,
        #[doc = "Indicates whether the parameter is optional.\n"]
        pub is_optional: bool,
    }
    #[doc = "A type.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Type {
        #[doc = "The kind of the type.\n"]
        pub kind: TypeKind,
        #[doc = "The span of the type expression.\n"]
        pub span: ::std::option::Option<Span>,
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
    pub struct TypeVarType {
        #[doc = "The index of the type variable in the enclosing definition.\n"]
        pub idx: TypeVarIdx,
    }
    #[doc = "An instantiation of a type defined in some schema of some bundle.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
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
    #[doc = "An attribute.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Attr {
        #[doc = "The kind of the attribute.\n"]
        pub kind: AttrKind,
        #[doc = "The span of the attribute.\n"]
        pub span: ::std::option::Option<Span>,
    }
    #[doc = "A *span* identifies a range of text in a source.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Span {
        #[doc = "The source.\n"]
        pub src: SourceIdx,
        #[doc = "The start character.\n"]
        pub start: usize,
        #[doc = "The included end character.\n"]
        pub end: usize,
    }
    #[doc = "A token.\n"]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct Token {
        #[doc = "The token itself.\n"]
        pub kind: TokenKind,
        #[doc = "The span of the token.\n"]
        pub span: ::std::option::Option<Span>,
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
    #[doc = ""]
    #[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, Debug)]
    pub struct AttrAssign {
        #[doc = ""]
        pub path: Path,
        #[doc = ""]
        pub value: ::std::boxed::Box<Attr>,
    }
}
