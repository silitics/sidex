//! _Abstract Syntax Tree_ (AST) for Sidex schemas.

use std::{
    fmt::{self, Display, Write},
    ops::Deref,
    sync::Arc,
};

use sidex_ir as ir;

use crate::{span::Span, tokens::Token};

/// A stream of tokens.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenStream(pub(crate) Vec<Token>);

impl TokenStream {
    /// An iterator over the tokens.
    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }
}

impl Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;

    type IntoIter = TokenStreamIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        TokenStreamIntoIter(self.0.into_iter())
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

/// An iterator that moves out of a token stream.
pub struct TokenStreamIntoIter(pub(crate) std::vec::IntoIter<Token>);

impl Iterator for TokenStreamIntoIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// A node of the AST behaving as a smart-pointer to `T`.
#[derive(Clone, Debug)]
pub struct Node<T> {
    /// The inner value.
    pub(crate) inner: T,
    /// The span of the node.
    pub(crate) span: Span,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Node<T> {
    /// Construct a new AST node from a given value and span.
    pub(crate) fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }

    /// The source id of the node.
    pub fn src(this: &Self) -> &ir::SourceIdx {
        &this.span.0.src
    }

    /// The start position of the node.
    pub fn start(this: &Self) -> usize {
        this.span.0.start
    }

    /// The end position of the node.
    pub fn end(this: &Self) -> usize {
        this.span.0.end
    }
}

/// An identifier.
#[derive(Clone, Debug)]
pub struct Identifier(pub(crate) Node<Arc<String>>);

impl Identifier {
    /// The identifier as a `&str`.
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
#[non_exhaustive]
pub struct Path {
    /// The segments of the path.
    pub segments: Vec<Identifier>,
    /// Indicates whether the path is absolute, i.e., starts with `::`.
    pub is_absolute: bool,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_absolute {
            f.write_str("::")?;
        }
        for idx in 0..self.segments.len() {
            f.write_str(self.segments[idx].as_str())?;
            if idx < self.segments.len() - 1 {
                f.write_str("::")?;
            }
        }
        Ok(())
    }
}

/// A schema is a collection of items.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Schema {
    /// The attributes of the schema.
    pub attrs: Vec<Attr>,
    /// The documentation of the schema.
    pub docs: Docs,
    /// The items of the schema.
    pub items: Vec<Item>,
}

/// An item of a schema.
#[derive(Clone, Debug)]
pub enum Item {
    /// An import directive.
    Import(Import),
    /// A definition.
    Def(Def),
}

/// An import directive.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Import {
    /// A tree of paths to import.
    pub tree: ImportTree,
}

/// An import tree.
#[derive(Clone, Debug)]
pub enum ImportTree {
    /// A path to import.
    Path(Path),
    /// An import wildcard.
    Wildcard,
    /// An import group.
    Group { path: Path, trees: Vec<ImportTree> },
}

/// A definition.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Def {
    /// The name of the definition.
    pub name: Identifier,
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
    pub name: Identifier,
}

/// A definition kind.
#[derive(Clone, Debug)]
pub enum DefKind {
    /// Definition of a type alias.
    Alias(AliasDef),
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

/// Definition of a type alias.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AliasDef {
    /// The type expression of the aliased type.
    pub aliased: TypeExpr,
}

/// Definition of an opaque type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct OpaqueTypeDef {}

/// Definition of a variant type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct VariantTypeDef {
    /// The variants of the variant type.
    pub variants: Vec<Variant>,
}

/// A variant of a variant type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Variant {
    /// The name of the variant.
    pub name: Identifier,
    /// The documentation of the variant.
    pub docs: Docs,
    /// The attributes of the variant.
    pub attrs: Vec<Attr>,
    /// An optional type expression describing the type of the variant.
    pub typ: Option<TypeExpr>,
}

/// Definition of a record type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct RecordTypeDef {
    /// The fields of the record type.
    pub fields: Vec<Field>,
}

/// A field of a record type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Field {
    /// The name of the field.
    pub name: Identifier,
    /// The documentation of the field.
    pub docs: Docs,
    /// The attributes of the field.
    pub attrs: Vec<Attr>,
    /// The type expression describing the type of the field.
    pub typ: TypeExpr,
    /// Indicates whether the field is optional.
    pub is_optional: bool,
}

/// Definition of a wrapper type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WrapperTypeDef {
    /// The type expression describing the wrapped type.
    pub wrapped: TypeExpr,
}

/// Definition of a service.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ServiceDef {
    /// The methods provided by the service.
    pub methods: Vec<Method>,
}

/// A method of a service definition.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Method {
    /// The name of the method.
    pub name: Identifier,
    /// The documentation of the method.
    pub docs: Docs,
    /// The attributes of the method.
    pub attrs: Vec<Attr>,
    /// The parameters of the method.
    pub params: Vec<MethodParam>,
    /// An optional type expression describing the return type of the method.
    pub returns: Option<TypeExpr>,
}

/// A parameter of a method.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct MethodParam {
    /// The name of the parameter.
    pub name: Identifier,
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
    /// A sequence type expression.
    Sequence(SequenceTypeExpr),
    /// A map type expression.
    Map(MapTypeExpr),
    /// A unit type expression.
    Unit,
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

/// A sequence type expression.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SequenceTypeExpr {
    /// The element type of the sequence type.
    pub element: Box<TypeExpr>,
}

/// A map type expression.
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
    pub kind: AttrKind,
}

#[derive(Clone, Debug)]
pub enum AttrKind {
    Path(Path),
    List(AttrList),
    Assign(AttrAssign),
    Tokens(TokenStream),
}

#[derive(Clone, Debug)]
pub struct AttrList {
    pub path: Path,
    pub elements: Vec<Attr>,
}

#[derive(Clone, Debug)]
pub struct AttrAssign {
    pub path: Path,
    pub value: Box<Attr>,
}
