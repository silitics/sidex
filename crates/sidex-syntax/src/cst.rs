//! Concrete Syntax Tree (CST) for Sidex schemas.
//!
//! The CST is a lossless tree that retains every token from the source —
//! including whitespace, comments, and erroneous tokens. It is the foundation
//! for autoformatting and other source-preserving tools.
//!
//! Semantic passes typically work through the typed views in [`crate::ast`]
//! rather than walking the CST directly.

use std::sync::Arc;

use sidex_ir as ir;

use crate::tokens::Token;

/// The kind of a syntax tree node.
///
/// Tokens carry their own [`crate::tokens::TokenKind`]; this enum classifies
/// only the structural nodes produced by the parser.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SyntaxKind {
    /// Top-level schema node — the parse root.
    Schema,
    /// An attribute, e.g. `#[json(rename = "x")]`.
    Attr,
    /// The path inside an attribute or type instance.
    Path,
    /// A list-form attribute body, e.g. `(rename = "x", inline)`.
    AttrList,
    /// An assignment-form attribute body, e.g. `rename = "x"`.
    AttrAssign,
    /// A free-form attribute body holding raw tokens.
    AttrTokens,
    /// A definition: record, variant, alias, opaque, or wrapper.
    Def,
    /// The `<...>` type variable list of a definition.
    TypeVars,
    /// A single type variable name.
    TypeVar,
    /// A field of a record type.
    Field,
    /// A variant of a variant type.
    Variant,
    /// A type expression.
    TypeExpr,
    /// The substitution list for a type expression, e.g. `<T, U>`.
    TypeSubst,
    /// An `import` directive.
    Import,
    /// A node within an import tree (path, group, or wildcard).
    ImportTree,
    /// A documentation block (sequence of `///` or `//!` lines).
    Docs,
    /// A node holding a single erroneous token, used for recovery.
    Error,
}

/// An element of a [`SyntaxNode`]: either a child node or a token.
#[derive(Clone, Debug)]
pub enum SyntaxElement {
    /// A child syntax node.
    Node(Arc<SyntaxNode>),
    /// A child token (may be trivia or significant).
    Token(Token),
}

impl SyntaxElement {
    /// The span of this element.
    pub fn span(&self) -> ir::Span {
        match self {
            SyntaxElement::Node(node) => node.span.clone(),
            SyntaxElement::Token(token) => token.span().clone(),
        }
    }
}

/// A node in the concrete syntax tree.
#[derive(Debug)]
pub struct SyntaxNode {
    /// The kind of node.
    pub kind: SyntaxKind,
    /// The span covering this node.
    pub span: ir::Span,
    /// The children of this node, in source order.
    pub children: Vec<SyntaxElement>,
}

impl SyntaxNode {
    /// Iterates over all child nodes (skipping tokens).
    pub fn child_nodes(&self) -> impl Iterator<Item = &Arc<SyntaxNode>> {
        self.children.iter().filter_map(|el| {
            match el {
                SyntaxElement::Node(n) => Some(n),
                SyntaxElement::Token(_) => None,
            }
        })
    }

    /// Iterates over all child tokens (skipping nodes).
    pub fn child_tokens(&self) -> impl Iterator<Item = &Token> {
        self.children.iter().filter_map(|el| {
            match el {
                SyntaxElement::Token(t) => Some(t),
                SyntaxElement::Node(_) => None,
            }
        })
    }
}

/// Incremental builder for syntax trees.
///
/// Use [`start_node`][Self::start_node] / [`token`][Self::token] /
/// [`finish_node`][Self::finish_node] from a recursive-descent parser, then
/// call [`finish`][Self::finish] to obtain the root node.
///
/// The span of each node is derived from its children. An empty node receives
/// a zero-width span at the position the builder reports via
/// [`set_pos`][Self::set_pos]; this lets the parser place anchor markers for
/// recovery and missing-token nodes.
pub struct Builder {
    src: ir::SourceIdx,
    stack: Vec<Pending>,
    completed: Vec<Arc<SyntaxNode>>,
    /// The current text position, used as a fallback span for empty nodes.
    pos: usize,
}

struct Pending {
    kind: SyntaxKind,
    children: Vec<SyntaxElement>,
}

impl Builder {
    /// Creates a new builder for the given source.
    pub fn new(src: ir::SourceIdx) -> Self {
        Self {
            src,
            stack: Vec::new(),
            completed: Vec::new(),
            pos: 0,
        }
    }

    /// Sets the current text position. Used as the fallback span for empty
    /// nodes; the parser should call this whenever it advances or rewinds in
    /// the token stream so that anchor spans line up with the source.
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Starts a new node of the given kind.
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.stack.push(Pending {
            kind,
            children: Vec::new(),
        });
    }

    /// Pushes a token into the currently open node.
    pub fn token(&mut self, token: Token) {
        self.pos = token.end();
        let element = SyntaxElement::Token(token);
        if let Some(top) = self.stack.last_mut() {
            top.children.push(element);
        }
    }

    /// Closes the topmost open node. The closed node is attached as a child
    /// of its parent, or stored as a completed root if it was the outermost
    /// node.
    pub fn finish_node(&mut self) {
        let Pending { kind, children } = self.stack.pop().expect("finish_node without start_node");
        let span = if let (Some(first), Some(last)) = (children.first(), children.last()) {
            let start = first.span().start;
            let end = last.span().end;
            ir::Span::new(self.src, start, end)
        } else {
            ir::Span::new(self.src, self.pos, self.pos)
        };
        let node = Arc::new(SyntaxNode {
            kind,
            span,
            children,
        });
        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(SyntaxElement::Node(node));
        } else {
            self.completed.push(node);
        }
    }

    /// Finishes building and returns the single completed root.
    ///
    /// # Panics
    ///
    /// Panics if more than one root was constructed or none at all.
    pub fn finish(mut self) -> Arc<SyntaxNode> {
        assert!(self.stack.is_empty(), "unfinished node on builder stack");
        assert_eq!(
            self.completed.len(),
            1,
            "builder must produce exactly one root"
        );
        self.completed.pop().unwrap()
    }
}
