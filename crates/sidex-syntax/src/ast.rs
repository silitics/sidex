//! Typed _Abstract Syntax Tree_ (AST) for Sidex schemas.
//!
//! The AST is the lowered representation produced from the [`crate::cst`].
//! Semantic passes — name resolution, type checking, IR generation — work
//! against this tree. The AST is owned data; navigating between siblings is
//! done by indexing rather than pointer chasing.

use std::{
    fmt::{self, Display, Write},
    ops::Deref,
    sync::Arc,
};

use sidex_ir as ir;

use crate::tokens::Token;

/// A stream of tokens carried inside an attribute's free-form body.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenStream(pub(crate) Vec<Token>);

impl TokenStream {
    /// Iterates over the tokens.
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

/// An identifier with its source span.
#[derive(Clone, Debug)]
pub struct Identifier {
    pub(crate) text: Arc<str>,
    pub(crate) span: ir::Span,
}

impl Identifier {
    /// The identifier as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The source span.
    pub fn span(&self) -> &ir::Span {
        &self.span
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A vector of doc strings.
#[derive(Clone, Debug, Default)]
pub struct Docs(pub(crate) Vec<Arc<String>>);

impl Docs {
    /// Iterates over the doc strings.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|s| s.as_str())
    }

    /// True if there are no doc strings.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Docs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for doc in &self.0 {
            f.write_str(doc.trim())?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

/// A `::`-separated list of identifiers.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Path {
    /// The segments of the path.
    pub segments: Vec<Identifier>,
    /// True if the path begins with `::` (absolute).
    pub is_absolute: bool,
}

impl Display for Path {
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
    /// A single path.
    Path(Path),
    /// A wildcard `*`.
    Wildcard,
    /// A grouped import `path::{ ... }`.
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

/// A kind of definition.
#[derive(Clone, Debug)]
pub enum DefKind {
    /// `alias`.
    Alias(AliasDef),
    /// `opaque`.
    OpaqueType(OpaqueTypeDef),
    /// `record`.
    RecordType(RecordTypeDef),
    /// `variant`.
    VariantType(VariantTypeDef),
    /// `wrapper`.
    WrapperType(WrapperTypeDef),
}

/// Definition of a type alias.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AliasDef {
    /// The aliased type expression.
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
    /// The variants.
    pub variants: Vec<Variant>,
}

/// A variant of a variant type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Variant {
    /// The name.
    pub name: Identifier,
    /// The documentation.
    pub docs: Docs,
    /// The attributes.
    pub attrs: Vec<Attr>,
    /// Optional payload type.
    pub typ: Option<TypeExpr>,
}

/// Definition of a record type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct RecordTypeDef {
    /// The fields.
    pub fields: Vec<Field>,
}

/// A field of a record type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Field {
    /// The name.
    pub name: Identifier,
    /// The documentation.
    pub docs: Docs,
    /// The attributes.
    pub attrs: Vec<Attr>,
    /// The type expression.
    pub typ: TypeExpr,
    /// Whether the field is optional.
    pub is_optional: bool,
}

/// Definition of a wrapper type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WrapperTypeDef {
    /// The wrapped type expression.
    pub wrapped: TypeExpr,
}

/// A type expression.
#[derive(Clone, Debug)]
pub enum TypeExpr {
    /// Instantiation of a named type.
    Instance(InstanceTypeExpr),
    /// Sequence type `[T]`.
    Sequence(SequenceTypeExpr),
    /// Map type `[K: V]`.
    Map(MapTypeExpr),
    /// Unit type `()`.
    Unit,
}

/// An instantiation of a named type.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct InstanceTypeExpr {
    /// The path of the instantiated type.
    pub path: Path,
    /// Substitutions for type variables.
    pub subst: Vec<TypeExpr>,
}

/// A sequence type expression.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SequenceTypeExpr {
    /// The element type.
    pub element: Box<TypeExpr>,
}

/// A map type expression.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct MapTypeExpr {
    /// The key type.
    pub key: Box<TypeExpr>,
    /// The value type.
    pub value: Box<TypeExpr>,
}

/// An attribute.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Attr {
    /// The kind of the attribute.
    pub kind: AttrKind,
}

/// The kind of an attribute.
#[derive(Clone, Debug)]
pub enum AttrKind {
    /// A bare path, e.g. `inline`.
    Path(Path),
    /// A list, e.g. `json(rename = "x")`.
    List(AttrList),
    /// An assignment, e.g. `rename = "x"`.
    Assign(AttrAssign),
    /// A free-form token stream.
    Tokens(TokenStream),
}

/// A list-form attribute body.
#[derive(Clone, Debug)]
pub struct AttrList {
    /// The path being applied.
    pub path: Path,
    /// The arguments.
    pub elements: Vec<Attr>,
}

/// An assignment-form attribute body.
#[derive(Clone, Debug)]
pub struct AttrAssign {
    /// The left-hand path.
    pub path: Path,
    /// The right-hand value (recursively another attribute).
    pub value: Box<Attr>,
}
