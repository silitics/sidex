//! Abstract syntax tree for Sidex schemas.

use std::{
    fmt::{self, Display, Write},
    ops::Deref,
    sync::Arc,
};

use crate::{
    source::{SourceId, Span},
    tokens::Token,
};

/// A stream of tokens used for attributes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenStream(pub(crate) Vec<Token>);

impl TokenStream {
    /// An iterator over the tokens.
    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in self.iter() {
            write!(f, "{}", token)?;
            if token.is_separated() {
                f.write_char(' ')?;
            }
        }
        Ok(())
    }
}

/// A node of the AST behaving as a smart-pointer to `T`.
#[derive(Clone, Debug)]
pub struct Node<T> {
    pub(crate) inner: T,
    pub(crate) span: Span,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Node<T> {
    /// Construct a new AST nod from a given value and span.
    pub(crate) fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }

    /// The source id of the node.
    pub fn src(this: &Self) -> &SourceId {
        this.span.src()
    }

    /// The start position of the node.
    pub fn start(this: &Self) -> usize {
        this.span.start()
    }

    /// The end position of the node.
    pub fn end(this: &Self) -> usize {
        this.span.end()
    }
}

/// An name.
#[derive(Clone, Debug)]
pub struct Name(pub(crate) Node<Arc<String>>);

impl Name {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A vector of doc strings.
#[derive(Clone, Debug, Default)]
pub struct Docs(pub(crate) Vec<Node<Arc<String>>>);

impl fmt::Display for Docs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for doc in &self.0 {
            f.write_str(doc.trim())?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

/// A path is a `::` separated list of names.
#[derive(Clone, Debug)]
pub struct Path {
    /// The segments of the path.
    pub segments: Vec<Name>,
    /// Indicates whether the path is absolute, i.e., starts with `::`.
    pub is_absolute: bool,
}

/// A schema is a collection of syntax items.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Schema {
    /// The documentation of the schema.
    pub docs: Docs,
    /// The items of the schema.
    pub items: Vec<Item>,
}

/// A syntax item of a schema.
#[derive(Clone, Debug)]
pub enum Item {
    /// An import statement.
    Import(Import),
    /// A definition.
    Def(Def),
}

/// An import statement.
///
/// ** 🚧 TODO:** How do import statements look like?
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Import {
    /// A tree of paths to import.
    pub tree: ImportTree,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ImportTree {
    Path(Path),
    Wildcard,
    From { path: Path, trees: Vec<ImportTree> },
}

/// A definition.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Def {
    /// The name of the definition.
    pub name: Name,
    /// The documentation of the definition.
    pub docs: Docs,
    /// The type variables of the definition.
    pub vars: Vec<TypeVar>,
    /// The attributes of the definition.
    pub attrs: Vec<Attr>,
    /// The kind of the definition.
    pub kind: DefKind,
}

/// A type variable.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TypeVar {
    /// The name of the type variable.
    pub name: Name,
}

/// Different types of definitions.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct OpaqueDef {}

/// Definition of a type alias.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AliasDef {
    /// The type expression describing the aliased type.
    pub aliased: TypeExpr,
}

/// Definition of an enumeration type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EnumDef {
    /// The variants of the enumeration type.
    pub variants: Vec<EnumVariant>,
}

/// A variant of an enumeration type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EnumVariant {
    /// The name of the variant.
    pub name: Name,
    /// The documentation of the variant.
    pub docs: Docs,
    /// The attributes of the variant.
    pub attrs: Vec<Attr>,
    /// The type expression describing the type of the variant.
    pub typ: Option<TypeExpr>,
}

/// Definition of a struct type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StructDef {
    /// The fields of the struct type.
    pub fields: Vec<StructField>,
}

/// A field of a struct type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StructField {
    /// The name of the field.
    pub name: Name,
    /// The documentation of the field.
    pub docs: Docs,
    /// The attributes of the field.
    pub attrs: Vec<Attr>,
    /// The type expression describing the type of the field.
    pub typ: TypeExpr,
    /// Indicates whether the field is optional.
    pub is_optional: bool,
}

/// Definition of a service.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ServiceDef {
    /// The functions provided by the service.
    pub functions: Vec<Function>,
}

/// A function of a service definition.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Function {
    /// The name of the function.
    pub name: Name,
    /// The documentation of the function.
    pub docs: Docs,
    /// The attributes of the function.
    pub attrs: Vec<Attr>,
    /// The parameters of the function.
    pub params: Vec<FunctionParam>,
    /// The type expression describing the return type of the function.
    pub returns: TypeExpr,
}

/// A parameter of a function.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FunctionParam {
    /// The name of the parameter.
    pub name: Name,
    /// The type expression describing the type of the parameter.
    pub typ: TypeExpr,
    /// Indicates whether the parameter is optional.
    pub is_optional: bool,
}

/// An expression describing a type.
#[derive(Clone, Debug)]
pub enum TypeExpr {
    /// An instantiation of a type.
    Instance(InstanceTypeExpr),
    /// An anonymous sequence type.
    Sequence(SequenceTypeExpr),
    /// An anonymous map type.
    Map(MapTypeExpr),
}

/// An instantiation of a type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct InstanceTypeExpr {
    /// The path of the instantiated type.
    pub path: Path,
    /// Substitutions for the type variables of the instance.
    pub subst: Vec<TypeExpr>,
}

/// An anonymous sequence type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SequenceTypeExpr {
    /// The element type of the sequence type.
    pub element: Box<TypeExpr>,
}

/// An anonymous map type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct MapTypeExpr {
    /// The key type of the map type.
    pub key: Box<TypeExpr>,
    /// The value type of the map type.
    pub value: Box<TypeExpr>,
}

/// An attribute.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Attr {
    /// The name of the attribute.
    pub name: Name,
    /// The free-form arguments of the attribute.
    pub args: Option<TokenStream>,
}
