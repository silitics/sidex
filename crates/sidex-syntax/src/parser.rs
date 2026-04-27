//! Hand-written recursive-descent parser for Sidex schemas.
//!
//! The parser drives the [`crate::cst::Builder`] to produce a lossless CST
//! and, in the same pass, builds the typed [`crate::ast`] tree consumed by
//! semantic passes. Both trees are returned together by [`parse`].

use std::sync::Arc;

use sidex_diagnostics::{Diagnostic, Label};
use sidex_ir as ir;

use crate::{
    ast,
    cst::{Builder, SyntaxKind, SyntaxNode},
    lexer,
    tokens::{self, DocKind, PunctuationKind, Token, TokenKind},
};

/// A parsed source: the lossless concrete syntax tree and the lowered AST.
#[derive(Clone)]
pub struct Parsed {
    /// The CST root, retaining whitespace, comments, and any error tokens.
    pub cst: Arc<SyntaxNode>,
    /// The lowered AST consumed by the IR transformer.
    pub schema: ast::Schema,
}

/// Parse a source into a CST and an AST.
pub fn parse_full(source: &ir::Source) -> Option<Parsed> {
    let text = source.text.as_ref()?;
    let mut tokens = lexer::lex(source.idx, text);
    // Filter error tokens out for the parser's grammar layer — they remain
    // surfaced through the diagnostics emitted during lexing.
    tokens.retain(|t| !matches!(t.kind, TokenKind::Error));
    let parser = Parser::new(source.idx, tokens);
    Some(parser.run())
}

/// Backwards-compatible entry point: returns just the AST.
pub fn parse(source: &ir::Source) -> Option<ast::Schema> {
    parse_full(source).map(|p| p.schema)
}

struct Parser {
    src: ir::SourceIdx,
    tokens: Vec<Token>,
    /// Cursor into `tokens`. Always points at the next *unconsumed* token,
    /// which may be trivia.
    cursor: usize,
    builder: Builder,
}

impl Parser {
    fn new(src: ir::SourceIdx, tokens: Vec<Token>) -> Self {
        Self {
            src,
            tokens,
            cursor: 0,
            builder: Builder::new(src),
        }
    }

    fn run(mut self) -> Parsed {
        self.builder.start_node(SyntaxKind::Schema);
        let docs = self.parse_docs(DocKind::Inline);
        let mut items = Vec::new();
        loop {
            self.skip_trivia();
            if self.is_at_end() {
                break;
            }
            if let Some(item) = self.parse_item() {
                items.push(item);
            } else {
                // Couldn't make progress — recover by bumping a single token.
                self.bump_any();
            }
        }
        // Emit any final trivia inside the schema node.
        self.skip_trivia();
        self.builder.finish_node();
        let cst = self.builder.finish();
        Parsed {
            cst,
            schema: ast::Schema {
                attrs: Vec::new(),
                docs,
                items,
            },
        }
    }

    // --- Cursor primitives ---------------------------------------------------

    fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    /// Skip and emit trivia tokens at the current cursor. Trivia = whitespace
    /// + non-doc comments. Doc comments are part of the grammar.
    fn skip_trivia(&mut self) {
        while let Some(tok) = self.tokens.get(self.cursor) {
            if matches!(
                tok.kind,
                TokenKind::Whitespace | TokenKind::Comment { .. } | TokenKind::Error
            ) {
                let tok = tok.clone();
                self.builder.token(tok);
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    /// Returns a reference to the next significant token without consuming
    /// trivia. Useful when peeking is read-only.
    fn peek_significant(&self) -> Option<&Token> {
        let mut i = self.cursor;
        while let Some(tok) = self.tokens.get(i) {
            if matches!(
                tok.kind,
                TokenKind::Whitespace | TokenKind::Comment { .. } | TokenKind::Error
            ) {
                i += 1;
            } else {
                return Some(tok);
            }
        }
        None
    }

    fn at_eof(&self) -> bool {
        self.peek_significant().is_none()
    }

    /// Consumes the next token regardless of kind. Used for error recovery.
    fn bump_any(&mut self) {
        if let Some(tok) = self.tokens.get(self.cursor) {
            let tok = tok.clone();
            self.builder.token(tok);
            self.cursor += 1;
        }
    }

    /// Bumps the next significant token. Caller must have verified it exists.
    fn bump(&mut self) -> Token {
        self.skip_trivia();
        let tok = self.tokens[self.cursor].clone();
        self.builder.token(tok.clone());
        self.cursor += 1;
        tok
    }

    // --- Predicates ----------------------------------------------------------

    fn at_punct(&self, kind: PunctuationKind) -> bool {
        matches!(
            self.peek_significant().map(|t| &t.kind),
            Some(TokenKind::Punctuation(s)) if s.kind == kind
        )
    }

    fn at_keyword(&self, name: &str) -> bool {
        match self.peek_significant().map(|t| &t.kind) {
            Some(TokenKind::Identifier(s)) => s.as_str() == name,
            _ => false,
        }
    }

    fn at_doc(&self, kind: DocKind) -> bool {
        matches!(
            self.peek_significant().map(|t| &t.kind),
            Some(TokenKind::Doc { kind: k, .. }) if *k == kind
        )
    }

    fn at_open(&self, expected: tokens::DelimiterKind) -> bool {
        matches!(
            self.peek_significant().map(|t| &t.kind),
            Some(TokenKind::Delimiter(tokens::DelimiterSymbol::Open(k))) if *k == expected
        )
    }

    fn at_close(&self, expected: tokens::DelimiterKind) -> bool {
        matches!(
            self.peek_significant().map(|t| &t.kind),
            Some(TokenKind::Delimiter(tokens::DelimiterSymbol::Close(k))) if *k == expected
        )
    }

    /// Eats a punctuation if present. Returns whether it matched.
    fn eat_punct(&mut self, kind: PunctuationKind) -> bool {
        if self.at_punct(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_keyword(&mut self, name: &str) -> bool {
        if self.at_keyword(name) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect_punct(&mut self, kind: PunctuationKind) -> bool {
        if self.at_punct(kind) {
            self.bump();
            true
        } else {
            self.error(&format!("Expected `{}`.", kind));
            false
        }
    }

    fn expect_open(&mut self, kind: tokens::DelimiterKind) -> bool {
        if self.at_open(kind) {
            self.bump();
            true
        } else {
            self.error(&format!("Expected `{}`.", kind.open()));
            false
        }
    }

    fn expect_close(&mut self, kind: tokens::DelimiterKind) -> bool {
        if self.at_close(kind) {
            self.bump();
            true
        } else {
            self.error(&format!("Expected `{}`.", kind.close()));
            false
        }
    }

    fn error(&self, msg: &str) {
        let span = if let Some(tok) = self.peek_significant() {
            tok.span().clone()
        } else if let Some(last) = self.tokens.last() {
            ir::Span::new(self.src, last.end(), last.end())
        } else {
            ir::Span::new(self.src, 0, 0)
        };
        Diagnostic::error(msg.to_string())
            .with_span(Some(span.clone()))
            .with_label(Label::new(span, msg.to_string()))
            .emit();
    }

    // --- Composed-token helpers ----------------------------------------------

    /// Match `::`. Returns true and consumes both colons if present.
    fn eat_double_colon(&mut self) -> bool {
        let mut i = self.cursor;
        // Skip trivia.
        while let Some(tok) = self.tokens.get(i) {
            if matches!(
                tok.kind,
                TokenKind::Whitespace | TokenKind::Comment { .. } | TokenKind::Error
            ) {
                i += 1;
            } else {
                break;
            }
        }
        let Some(t1) = self.tokens.get(i) else {
            return false;
        };
        let TokenKind::Punctuation(s1) = &t1.kind else {
            return false;
        };
        if s1.kind != PunctuationKind::Colon || !s1.is_composed {
            return false;
        }
        let Some(t2) = self.tokens.get(i + 1) else {
            return false;
        };
        let TokenKind::Punctuation(s2) = &t2.kind else {
            return false;
        };
        if s2.kind != PunctuationKind::Colon {
            return false;
        }
        // Consume both — including any leading trivia.
        self.bump();
        self.bump();
        true
    }

    /// Match a closing angle bracket `>`. We accept either standalone or
    /// composed (e.g. when followed by another `>` or `=`), so that nested
    /// generics like `Box<Vec<T>>` parse.
    fn eat_angle_close(&mut self) -> bool {
        if matches!(
            self.peek_significant().map(|t| &t.kind),
            Some(TokenKind::Punctuation(s)) if s.kind == PunctuationKind::AngleClose
        ) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn at_angle_close(&self) -> bool {
        matches!(
            self.peek_significant().map(|t| &t.kind),
            Some(TokenKind::Punctuation(s)) if s.kind == PunctuationKind::AngleClose
        )
    }

    // --- Grammar -------------------------------------------------------------

    fn parse_docs(&mut self, kind: DocKind) -> ast::Docs {
        let mut docs = Vec::new();
        loop {
            self.eat_non_doc_trivia();
            if self.at_doc(kind) {
                let tok = self.bump();
                if let TokenKind::Doc { doc, .. } = tok.kind {
                    docs.push(doc);
                }
            } else {
                break;
            }
        }
        ast::Docs(docs)
    }

    /// Skip whitespace and non-doc comments without crossing a doc token.
    fn eat_non_doc_trivia(&mut self) {
        while let Some(tok) = self.tokens.get(self.cursor) {
            if matches!(
                tok.kind,
                TokenKind::Whitespace | TokenKind::Comment { .. } | TokenKind::Error
            ) {
                let tok = tok.clone();
                self.builder.token(tok);
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    fn parse_item(&mut self) -> Option<ast::Item> {
        // Decide between an import directive and a definition.
        // Both can be preceded by docs and attrs (defs), but imports cannot.
        // We peek past docs and attrs to find the keyword.
        let saved = self.cursor;

        // Lookahead skipping trivia, docs, and balanced attribute groups.
        let kw = self.lookahead_item_keyword();
        let _ = saved;
        match kw {
            Some(ItemKw::Import) => self.parse_import().map(ast::Item::Import),
            Some(ItemKw::Def) => self.parse_def().map(ast::Item::Def),
            None => None,
        }
    }

    /// Looks ahead past leading docs/attrs and returns which keyword starts
    /// the next item.
    fn lookahead_item_keyword(&self) -> Option<ItemKw> {
        let mut i = self.cursor;
        loop {
            let Some(tok) = self.tokens.get(i) else {
                return None;
            };
            match &tok.kind {
                TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Error => i += 1,
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Hash => {
                    // Attribute: skip past `#[...]`.
                    i += 1;
                    // Optional whitespace before `[`.
                    while let Some(t) = self.tokens.get(i) {
                        if matches!(
                            t.kind,
                            TokenKind::Whitespace | TokenKind::Comment { .. } | TokenKind::Error
                        ) {
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    if !matches!(
                        self.tokens.get(i).map(|t| &t.kind),
                        Some(TokenKind::Delimiter(tokens::DelimiterSymbol::Open(
                            tokens::DelimiterKind::Bracket
                        )))
                    ) {
                        return None;
                    }
                    i += 1;
                    let mut depth = 1usize;
                    while depth > 0 {
                        match self.tokens.get(i).map(|t| &t.kind) {
                            None => return None,
                            Some(TokenKind::Delimiter(tokens::DelimiterSymbol::Open(_))) => {
                                depth += 1
                            }
                            Some(TokenKind::Delimiter(tokens::DelimiterSymbol::Close(_))) => {
                                depth -= 1
                            }
                            _ => {}
                        }
                        i += 1;
                    }
                }
                TokenKind::Identifier(s) => {
                    return match s.as_str() {
                        "import" => Some(ItemKw::Import),
                        "record" | "variant" | "alias" | "opaque" | "wrapper" => Some(ItemKw::Def),
                        _ => None,
                    };
                }
                _ => return None,
            }
        }
    }

    fn parse_def(&mut self) -> Option<ast::Def> {
        self.builder.start_node(SyntaxKind::Def);
        let docs = self.parse_docs(DocKind::Preceding);
        let attrs = self.parse_attrs();

        self.skip_trivia();
        let kw = match self.peek_significant().map(|t| &t.kind) {
            Some(TokenKind::Identifier(s)) => {
                match s.as_str() {
                    "record" => DefKw::Record,
                    "variant" => DefKw::Variant,
                    "alias" => DefKw::Alias,
                    "opaque" => DefKw::Opaque,
                    "wrapper" => DefKw::Wrapper,
                    _ => {
                        self.error(
                            "Expected `record`, `variant`, `alias`, `opaque`, or `wrapper`.",
                        );
                        self.builder.finish_node();
                        return None;
                    }
                }
            }
            _ => {
                self.error("Expected a definition keyword.");
                self.builder.finish_node();
                return None;
            }
        };
        self.bump(); // keyword

        let Some(name) = self.parse_identifier() else {
            self.builder.finish_node();
            return None;
        };
        let vars = self.parse_type_vars();

        let kind = match kw {
            DefKw::Record => {
                self.expect_open(tokens::DelimiterKind::Brace);
                let fields = self.parse_separated(
                    PunctuationKind::Comma,
                    |p| p.at_close(tokens::DelimiterKind::Brace) || p.at_eof(),
                    |p| p.parse_field(),
                );
                self.expect_close(tokens::DelimiterKind::Brace);
                ast::DefKind::RecordType(ast::RecordTypeDef { fields })
            }
            DefKw::Variant => {
                self.expect_open(tokens::DelimiterKind::Brace);
                let variants = self.parse_separated(
                    PunctuationKind::Comma,
                    |p| p.at_close(tokens::DelimiterKind::Brace) || p.at_eof(),
                    |p| p.parse_variant(),
                );
                self.expect_close(tokens::DelimiterKind::Brace);
                ast::DefKind::VariantType(ast::VariantTypeDef { variants })
            }
            DefKw::Alias => {
                self.expect_punct(PunctuationKind::Colon);
                let aliased = self.parse_type_expr().unwrap_or(ast::TypeExpr::Unit);
                ast::DefKind::Alias(ast::AliasDef { aliased })
            }
            DefKw::Opaque => ast::DefKind::OpaqueType(ast::OpaqueTypeDef {}),
            DefKw::Wrapper => {
                self.expect_punct(PunctuationKind::Colon);
                let wrapped = self.parse_type_expr().unwrap_or(ast::TypeExpr::Unit);
                ast::DefKind::WrapperType(ast::WrapperTypeDef { wrapped })
            }
        };

        self.builder.finish_node();
        Some(ast::Def {
            name,
            docs,
            vars,
            attrs,
            kind,
        })
    }

    fn parse_field(&mut self) -> Option<ast::Field> {
        self.builder.start_node(SyntaxKind::Field);
        let docs = self.parse_docs(DocKind::Preceding);
        let attrs = self.parse_attrs();
        let Some(name) = self.parse_identifier() else {
            self.builder.finish_node();
            return None;
        };
        let is_optional = self.eat_punct(PunctuationKind::QuestionMark);
        self.expect_punct(PunctuationKind::Colon);
        let typ = self.parse_type_expr().unwrap_or(ast::TypeExpr::Unit);
        self.builder.finish_node();
        Some(ast::Field {
            name,
            docs,
            attrs,
            typ,
            is_optional,
        })
    }

    fn parse_variant(&mut self) -> Option<ast::Variant> {
        self.builder.start_node(SyntaxKind::Variant);
        let docs = self.parse_docs(DocKind::Preceding);
        let attrs = self.parse_attrs();
        let Some(name) = self.parse_identifier() else {
            self.builder.finish_node();
            return None;
        };
        let typ = if self.eat_punct(PunctuationKind::Colon) {
            self.parse_type_expr()
        } else {
            None
        };
        self.builder.finish_node();
        Some(ast::Variant {
            name,
            docs,
            attrs,
            typ,
        })
    }

    fn parse_type_vars(&mut self) -> Vec<ast::TypeVar> {
        if !self.at_punct(PunctuationKind::AngleOpen) {
            return Vec::new();
        }
        self.builder.start_node(SyntaxKind::TypeVars);
        self.bump(); // <
        let mut vars = Vec::new();
        while !self.at_eof() && !self.at_angle_close() {
            self.builder.start_node(SyntaxKind::TypeVar);
            let name_opt = self.parse_identifier();
            self.builder.finish_node();
            if let Some(name) = name_opt {
                vars.push(ast::TypeVar { name });
            } else {
                break;
            }
            if !self.eat_punct(PunctuationKind::Comma) {
                break;
            }
        }
        if !self.eat_angle_close() {
            self.error("Expected `>` to close type variable list.");
        }
        self.builder.finish_node();
        vars
    }

    fn parse_identifier(&mut self) -> Option<ast::Identifier> {
        self.skip_trivia();
        let tok = self.peek_significant()?;
        let TokenKind::Identifier(s) = &tok.kind else {
            self.error("Expected an identifier.");
            return None;
        };
        // Keywords share the identifier token kind; whether `record` etc.
        // act as a keyword is decided contextually at item positions. In
        // every other position (field, variant, type-variable, and path
        // segment) they behave as ordinary identifiers.
        let text = s.clone();
        let span = tok.span().clone();
        self.bump();
        Some(ast::Identifier { text, span })
    }

    fn parse_path(&mut self) -> Option<ast::Path> {
        self.builder.start_node(SyntaxKind::Path);
        let is_absolute = self.eat_double_colon();
        let mut segments = Vec::new();
        if let Some(name) = self.parse_identifier() {
            segments.push(name);
        } else {
            self.builder.finish_node();
            return None;
        }
        while self.eat_double_colon() {
            if let Some(name) = self.parse_identifier() {
                segments.push(name);
            } else {
                break;
            }
        }
        self.builder.finish_node();
        Some(ast::Path {
            segments,
            is_absolute,
        })
    }

    fn parse_type_expr(&mut self) -> Option<ast::TypeExpr> {
        self.builder.start_node(SyntaxKind::TypeExpr);
        let res = self.parse_type_expr_inner();
        self.builder.finish_node();
        res
    }

    fn parse_type_expr_inner(&mut self) -> Option<ast::TypeExpr> {
        self.skip_trivia();
        if self.at_open(tokens::DelimiterKind::Bracket) {
            self.bump(); // [
            let left = self.parse_type_expr()?;
            let expr = if self.eat_punct(PunctuationKind::Colon) {
                let right = self.parse_type_expr()?;
                ast::TypeExpr::Map(ast::MapTypeExpr {
                    key: Box::new(left),
                    value: Box::new(right),
                })
            } else {
                ast::TypeExpr::Sequence(ast::SequenceTypeExpr {
                    element: Box::new(left),
                })
            };
            self.expect_close(tokens::DelimiterKind::Bracket);
            return Some(expr);
        }
        // Otherwise an instance: PATH (`<` typeexpr (`,` typeexpr)* `,`? `>`)?
        let path = self.parse_path()?;
        let subst = if self.at_punct(PunctuationKind::AngleOpen) {
            self.builder.start_node(SyntaxKind::TypeSubst);
            self.bump(); // <
            let mut subst = Vec::new();
            while !self.at_eof() && !self.at_angle_close() {
                let Some(t) = self.parse_type_expr() else {
                    break;
                };
                subst.push(t);
                if !self.eat_punct(PunctuationKind::Comma) {
                    break;
                }
            }
            if !self.eat_angle_close() {
                self.error("Expected `>` to close type substitution list.");
            }
            self.builder.finish_node();
            subst
        } else {
            Vec::new()
        };
        Some(ast::TypeExpr::Instance(ast::InstanceTypeExpr {
            path,
            subst,
        }))
    }

    fn parse_attrs(&mut self) -> Vec<ast::Attr> {
        let mut attrs = Vec::new();
        loop {
            self.eat_non_doc_trivia();
            if !self.at_punct(PunctuationKind::Hash) {
                break;
            }
            // Composed-or-not Hash: doesn't matter here, we just want `#[`.
            // Look ahead one significant token after `#` to confirm `[`.
            let mut i = self.cursor + 1;
            while let Some(t) = self.tokens.get(i) {
                if matches!(
                    t.kind,
                    TokenKind::Whitespace | TokenKind::Comment { .. } | TokenKind::Error
                ) {
                    i += 1;
                } else {
                    break;
                }
            }
            if !matches!(
                self.tokens.get(i).map(|t| &t.kind),
                Some(TokenKind::Delimiter(tokens::DelimiterSymbol::Open(
                    tokens::DelimiterKind::Bracket
                )))
            ) {
                break;
            }
            self.builder.start_node(SyntaxKind::Attr);
            self.bump(); // #
            self.bump(); // [
            let attr = self.parse_attr_body();
            self.expect_close(tokens::DelimiterKind::Bracket);
            self.builder.finish_node();
            if let Some(attr) = attr {
                attrs.push(attr);
            }
        }
        attrs
    }

    fn parse_attr_body(&mut self) -> Option<ast::Attr> {
        self.skip_trivia();
        if !self.starts_with_path() {
            // Non-path value (literal, etc.) — capture a single token tree.
            return Some(ast::Attr {
                kind: ast::AttrKind::Tokens(self.collect_one_token_tree()),
            });
        }
        let path = self.parse_path()?;
        // After the path, decide between assignment, list, or bare path.
        if self.at_punct(PunctuationKind::Equals) {
            self.bump();
            let value = self.parse_attr_body()?;
            Some(ast::Attr {
                kind: ast::AttrKind::Assign(ast::AttrAssign {
                    path,
                    value: Box::new(value),
                }),
            })
        } else if self.at_open(tokens::DelimiterKind::Parenthesis) {
            self.bump(); // (
            let elements = self.parse_separated(
                PunctuationKind::Comma,
                |p| p.at_close(tokens::DelimiterKind::Parenthesis) || p.at_eof(),
                |p| p.parse_attr_body(),
            );
            self.expect_close(tokens::DelimiterKind::Parenthesis);
            Some(ast::Attr {
                kind: ast::AttrKind::List(ast::AttrList { path, elements }),
            })
        } else {
            Some(ast::Attr {
                kind: ast::AttrKind::Path(path),
            })
        }
    }

    /// Capture a single token tree: either one non-delimiter token or one
    /// balanced delimiter group. Used for free-form attribute values.
    fn collect_one_token_tree(&mut self) -> ast::TokenStream {
        let mut tokens = Vec::new();
        self.skip_trivia();
        let Some(tok) = self.peek_significant() else {
            return ast::TokenStream(tokens);
        };
        match &tok.kind {
            TokenKind::Delimiter(tokens::DelimiterSymbol::Open(_)) => {
                let mut depth: i32 = 0;
                loop {
                    self.skip_trivia();
                    let Some(t) = self.peek_significant() else {
                        break;
                    };
                    let is_open = matches!(
                        t.kind,
                        TokenKind::Delimiter(tokens::DelimiterSymbol::Open(_))
                    );
                    let is_close = matches!(
                        t.kind,
                        TokenKind::Delimiter(tokens::DelimiterSymbol::Close(_))
                    );
                    let consumed = self.bump();
                    tokens.push(consumed);
                    if is_open {
                        depth += 1;
                    } else if is_close {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                }
            }
            TokenKind::Delimiter(_) => {
                // A bare closing delimiter — nothing to capture.
            }
            _ => {
                tokens.push(self.bump());
            }
        }
        ast::TokenStream(tokens)
    }

    /// Whether the next significant token starts a path (identifier or `::`).
    fn starts_with_path(&self) -> bool {
        match self.peek_significant().map(|t| &t.kind) {
            Some(TokenKind::Identifier(_)) => true,
            Some(TokenKind::Punctuation(s))
                if s.kind == PunctuationKind::Colon && s.is_composed =>
            {
                true
            }
            _ => false,
        }
    }

    fn parse_import(&mut self) -> Option<ast::Import> {
        self.builder.start_node(SyntaxKind::Import);
        if !self.eat_keyword("import") {
            self.error("Expected `import`.");
            self.builder.finish_node();
            return None;
        }
        let tree = self.parse_import_tree();
        self.builder.finish_node();
        tree.map(|tree| ast::Import { tree })
    }

    fn parse_import_tree(&mut self) -> Option<ast::ImportTree> {
        self.builder.start_node(SyntaxKind::ImportTree);
        let res = self.parse_import_tree_inner();
        self.builder.finish_node();
        res
    }

    fn parse_import_tree_inner(&mut self) -> Option<ast::ImportTree> {
        self.skip_trivia();
        if self.at_punct(PunctuationKind::Asterisk) {
            self.bump();
            return Some(ast::ImportTree::Wildcard);
        }
        // Path with optional `::{...}` or `::*` group.
        let is_absolute = self.eat_double_colon();
        let mut segments = Vec::new();
        loop {
            let Some(name) = self.parse_identifier() else {
                if segments.is_empty() {
                    return None;
                }
                break;
            };
            segments.push(name);
            // Need to peek for `::` and decide whether to continue or branch.
            if !self.eat_double_colon() {
                break;
            }
            // After `::`, decide between continuing path, `{...}` group, or `*`.
            if self.at_open(tokens::DelimiterKind::Brace)
                || self.at_punct(PunctuationKind::Asterisk)
            {
                break;
            }
        }
        // After path: maybe `{...}` or `*` for group/wildcard, else plain Path.
        if self.at_open(tokens::DelimiterKind::Brace) {
            self.bump(); // {
            let trees = self.parse_separated(
                PunctuationKind::Comma,
                |p| p.at_close(tokens::DelimiterKind::Brace) || p.at_eof(),
                |p| p.parse_import_tree(),
            );
            self.expect_close(tokens::DelimiterKind::Brace);
            return Some(ast::ImportTree::Group {
                path: ast::Path {
                    segments,
                    is_absolute,
                },
                trees,
            });
        }
        if self.at_punct(PunctuationKind::Asterisk) {
            self.bump();
            return Some(ast::ImportTree::Group {
                path: ast::Path {
                    segments,
                    is_absolute,
                },
                trees: vec![ast::ImportTree::Wildcard],
            });
        }
        Some(ast::ImportTree::Path(ast::Path {
            segments,
            is_absolute,
        }))
    }

    /// Generic separated-list parser. Stops at the terminator predicate or
    /// when the element parser returns `None`.
    fn parse_separated<T, Term, ParseEl>(
        &mut self,
        sep: PunctuationKind,
        term: Term,
        mut parse_el: ParseEl,
    ) -> Vec<T>
    where
        Term: Fn(&Self) -> bool,
        ParseEl: FnMut(&mut Self) -> Option<T>,
    {
        let mut out = Vec::new();
        loop {
            self.skip_trivia();
            if term(self) {
                break;
            }
            let Some(el) = parse_el(self) else {
                // Recovery: bump until separator, terminator, or EOF.
                while !term(self) && !self.at_punct(sep) && !self.at_eof() {
                    self.bump_any();
                }
                if !self.eat_punct(sep) {
                    break;
                }
                continue;
            };
            out.push(el);
            self.skip_trivia();
            if !self.eat_punct(sep) {
                break;
            }
        }
        out
    }
}

enum ItemKw {
    Import,
    Def,
}

#[derive(Clone, Copy)]
enum DefKw {
    Record,
    Variant,
    Alias,
    Opaque,
    Wrapper,
}
