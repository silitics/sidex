//! **🚧 TODO:** Generate this module from the `$reflect` Sidex model.

pub(crate) mod reflect {
    use serde::{Deserialize, Serialize};

    /// Uniquely identifies a model in a compilation unit.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct ModelIdx(pub(crate) usize);

    /// Uniquely identifies a schema in a model.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct SchemaIdx(pub(crate) usize);

    /// Uniquely identifies a definition in a schema.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct DefIdx(pub(crate) usize);

    /// Uniquely identifies a type variable in a definition.
    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub struct TypeVarIdx(pub(crate) usize);

    /// A compilation unit is a collection of schemas organized into models.
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct Unit {
        /// The models of the unit.
        pub models: Vec<Model>,
    }

    /// A model is a flat collection of schemas evolving together.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Model {
        /// The metadata of the model.
        pub metadata: Metadata,
        /// The schemas of the model.
        pub schemas: Vec<Schema>,
    }

    /// Metadata of a model.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Metadata {
        /// The name of the model.
        pub name: String,
        /// The version of the model.
        pub version: String,
        /// An optional description of the model.
        #[serde(default)]
        pub description: Option<String>,
        /// The authors of the model.
        #[serde(default)]
        pub authors: Vec<String>,
    }

    /// A schema is a collection of definitions.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Schema {
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

    /// A type variable.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TypeVar {
        /// The name of the type variable.
        pub name: String,
    }

    /// Different types of definitions.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "$tag")]
    pub enum DefKind {
        /// Definition of an opaque type.
        Opaque(OpaqueDef),
        /// Definition of a type alias.
        Alias(AliasDef),
        /// Definition of an enumeration type.
        Enum(EnumDef),
        /// Definition of a struct type.
        Struct(StructDef),
        /// Definition of a service.
        Service(ServiceDef),
    }

    /// Definition of an opaque type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct OpaqueDef {}

    /// Definition of a type alias.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AliasDef {
        /// The type that is aliased.
        pub aliased: Type,
    }

    /// Definition of an enumeration type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct EnumDef {
        /// The variants of the enumeration type.
        pub variants: Vec<EnumVariant>,
    }

    /// A variant of an enumeration type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct EnumVariant {
        /// The name of the variant.
        pub name: String,
        /// The documentation of the variant.
        pub docs: String,
        /// The attributes of the variant.
        pub attrs: Vec<Attr>,
        /// The type of the variant (must be a struct type).
        pub typ: Option<Type>,
    }

    /// Definition of a struct type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct StructDef {
        /// The fields of the struct type.
        pub fields: Vec<StructField>,
    }

    /// A field of a struct type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct StructField {
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

    /// Definition of a service.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ServiceDef {
        /// The functions provided by the service.
        pub functions: Vec<Function>,
    }

    /// A function of a service definition.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Function {
        /// The name of the function.
        pub name: String,
        /// The documentation of the function.
        pub docs: String,
        /// The attributes of the function.
        pub attrs: Vec<Attr>,
        /// The parameters of the function.
        pub parameters: Vec<FunctionParam>,
        /// The return type of the function.
        pub returns: Type,
    }

    /// A parameter of a function.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct FunctionParam {
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
        /// An anonymous sequence type.
        Sequence(SequenceType),
        /// An anonymous map type.
        Map(MapType),
    }

    /// A type to be determined via substitution of the respective type variable.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TypeVarType {
        /// The index of the type variable in the enclosing definition.
        pub idx: TypeVarIdx,
    }

    /// An instantiation of a type defined in some schema.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct InstanceType {
        /// The model containing the schema containing the definition.
        pub model: ModelIdx,
        /// The schema containing the definition.
        pub schema: SchemaIdx,
        /// The actual definition.
        pub def: DefIdx,
        /// Substitutions for the type variables of the definition.
        pub subst: Vec<Type>,
    }

    /// An anonymous sequence type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SequenceType {
        /// The element type of the sequence type.
        pub element: Box<Type>,
    }

    /// An anonymous map type.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct MapType {
        /// The key type of the map type.
        pub key: Box<Type>,
        /// The value type of the map type.
        pub value: Box<Type>,
    }

    /// An attribute.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Attr {
        /// The name of the attribute.
        pub name: String,
        /// The free-form arguments of the attribute.
        pub args: Option<String>,
    }
}
