/// **🚧 TODO:** Generate this module from the `reflect` Sidex schema.
pub mod reflect {
    use serde::{Deserialize, Serialize};

    /// Uniquely identifies a bundle in a transformation unit.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct BundleIdx(pub(crate) usize);

    /// Uniquely identifies a schema in bundle.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct SchemaIdx(pub(crate) usize);

    /// Uniquely identifies a definition in a schema.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct DefIdx(pub(crate) usize);

    /// Uniquely identifies a type variable in a definition.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct TypeVarIdx(pub(crate) usize);

    /// A transformation unit is a collection of schemas organized into bundles.
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    #[non_exhaustive]
    pub struct Unit {
        /// The bundles of the unit.
        pub bundles: Vec<Bundle>,
    }

    /// A bundle is a flat collection of schemas evolving together.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Bundle {
        /// The index of the bundle.
        pub idx: BundleIdx,
        /// The metadata of the bundle.
        pub metadata: Metadata,
        /// The dependencies of the bundle.
        pub dependencies: Vec<Dependency>,
        /// The schemas of the bundle.
        pub schemas: Vec<Schema>,
    }

    /// A dependency of a bundle.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Dependency {
        /// The name of the dependency.
        pub name: String,
        /// The bundle.
        pub bundle: BundleIdx,
    }

    /// Metadata of a bundle.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Metadata {
        /// The name of the bundle.
        pub name: String,
        /// The version of the bundle.
        pub version: String,
        /// The optional description of the bundle.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// The optional authors of the bundle.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authors: Option<Vec<String>>,
    }

    /// A schema is a collection of definitions.
    ///
    /// Note that imports have already been processed and resolved.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Schema {
        /// The index of the schema.
        pub idx: SchemaIdx,
        /// The name of the schema.
        pub name: String,
        /// The documentation of the schema.
        pub docs: String,
        /// The attributes of the schema.
        pub attrs: Vec<Attr>,
        /// The definitions of the schema.
        pub defs: Vec<Def>,
    }

    /// A definition.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Def {
        /// The name of the definition.
        pub name: String,
        /// The documentation of the definition.
        pub docs: String,
        /// The type variables of the definition.
        pub vars: Vec<TypeVar>,
        /// The attributes of the definition.
        pub attrs: Vec<Attr>,
        /// The kind of the definition.
        pub kind: DefKind,
    }

    /// A type variable of a definition.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct TypeVar {
        /// Name of the type variable.
        pub name: String,
    }

    /// A definition kind.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "tag")]
    pub enum DefKind {
        /// Definition of a type alias.
        TypeAlias(TypeAliasDef),
        /// Definition of an opaque type.
        OpaqueType(OpaqueTypeDef),
        /// Definition of a record type.
        RecordType(RecordTypeDef),
        /// Definition of a variant type.
        VariantType(VariantTypeDef),
        /// Definition of a wrapper type.
        WrapperType(WrapperTypeDef),
        /// Definition of a service.
        Service(ServiceDef),
    }

    /// A definition of a type alias.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct TypeAliasDef {
        /// The type that is aliased.
        pub aliased: Type,
    }

    /// A definition of an opaque type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct OpaqueTypeDef {}

    /// A definition of a record type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct RecordTypeDef {
        /// The fields of the record type.
        pub fields: Vec<Field>,
    }

    /// A field of a record type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Field {
        /// The name of the field.
        pub name: String,
        /// The documentation of the field.
        pub docs: String,
        /// The attributes of the field.
        pub attrs: Vec<Attr>,
        /// The type of the field.
        pub typ: Type,
        /// Indicates whether the field is optional.
        pub is_optional: bool,
    }

    /// A definition of a variant type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct VariantTypeDef {
        /// The variants of the variant type.
        pub variants: Vec<Variant>,
    }

    /// A variant of a variant type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Variant {
        /// The name of the variant.
        pub name: String,
        /// The documentation of the variant.
        pub docs: String,
        /// The attributes of the variant.
        pub attrs: Vec<Attr>,
        /// The optional type of the variant.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub typ: Option<Type>,
    }

    /// A definition of a wrapper type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct WrapperTypeDef {
        /// The type that is wrapped.
        pub wrapped: Type,
    }

    /// A definition of a service.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct ServiceDef {
        /// The methods provided by the service.
        pub methods: Vec<Method>,
    }

    /// A method of a service definition.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Method {
        /// The name of the method.
        pub name: String,
        /// The documentation of the method.
        pub docs: String,
        /// The attributes of the method.
        pub attrs: Vec<Attr>,
        /// The parameters of the method.
        pub parameters: Vec<MethodParam>,
        /// The optional return type of the method.
        pub returns: Option<Type>,
    }

    /// A parameter of a method.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct MethodParam {
        /// The name of the parameter.
        pub name: String,
        /// The type of the parameter.
        pub typ: Type,
        /// Indicates whether the parameter is optional.
        pub is_optional: bool,
    }

    /// An abstract type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "kind")]
    pub enum Type {
        /// A type to be determined via substitution of the respective type variable.
        TypeVar(TypeVarType),
        /// An instantiation of a type defined in some schema.
        Instance(InstanceType),
    }

    /// A type to be determined via substitution of the respective type variable.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TypeVarType {
        /// The index of the type variable in the enclosing definition.
        pub idx: TypeVarIdx,
    }

    /// An instantiation of a type defined in some schema of some bundle.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct InstanceType {
        /// The bundle containing the schema containing the definition.
        pub bundle: BundleIdx,
        /// The schema containing the definition.
        pub schema: SchemaIdx,
        /// The actual definition.
        pub def: DefIdx,
        /// Substitutions for the type variables of the definition.
        pub subst: Vec<Type>,
    }

    /// An attribute.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Attr {
        /// The name of the attribute.
        pub name: String,
        /// The free-form arguments of the attribute.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub args: Option<String>,
    }
}
