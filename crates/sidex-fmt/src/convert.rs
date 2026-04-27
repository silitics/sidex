//! CST → [`Doc`] conversion plus schema orchestration.
//!
//! Walks a [`SyntaxNode`] tree and emits a [`Doc`]. Comments are preserved at
//! their CST positions. The schema-level entry point also sorts imports into
//! external/internal groups before emitting.

use std::sync::Arc;

use sidex_syntax::{
    cst::{SyntaxElement, SyntaxKind, SyntaxNode},
    tokens::{
        CommentKind, DelimiterKind, DelimiterSymbol, DocKind, Literal, PunctuationKind,
        PunctuationSymbol, Token, TokenKind,
    },
};

use crate::{
    FormatOptions,
    doc::{Doc, LayoutOptions, render},
};

/// Render an entire schema CST to a string.
pub fn format_schema(node: &SyntaxNode, source: &str, opts: &FormatOptions) -> String {
    let layout = LayoutOptions {
        max_width: opts.max_width,
    };
    let cx = Cx::new(source);
    let doc = schema(node, &cx, opts);
    let mut out = render(&doc, &layout);
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// Conversion context: holds the source text and a char→byte index map so we
/// can resolve token spans to substrings in O(1).
struct Cx<'a> {
    source: &'a str,
    /// `char_to_byte[i]` is the byte offset of the `i`-th character in
    /// `source`. There is one extra entry at the end pointing past the last
    /// byte, so a range `[start..end]` of char indices is always
    /// representable as `&source[char_to_byte[start]..char_to_byte[end]]`.
    char_to_byte: Vec<usize>,
}

impl<'a> Cx<'a> {
    fn new(source: &'a str) -> Self {
        let mut char_to_byte = Vec::with_capacity(source.len() + 1);
        for (b, _) in source.char_indices() {
            char_to_byte.push(b);
        }
        char_to_byte.push(source.len());
        Self { source, char_to_byte }
    }

    fn text_at(&self, start: usize, end: usize) -> &'a str {
        let last = self.char_to_byte.len().saturating_sub(1);
        let s = self.char_to_byte[start.min(last)];
        let e = self.char_to_byte[end.min(last)];
        &self.source[s..e]
    }

    fn newlines_in(&self, tok: &Token) -> usize {
        let span = tok.span();
        self.text_at(span.start, span.end)
            .bytes()
            .filter(|&b| b == b'\n')
            .count()
    }
}

// ---------------------------------------------------------------------------
// Schema-level orchestration
// ---------------------------------------------------------------------------

struct LogicalItem {
    /// Trivia tokens (whitespace + comments) appearing before this item but
    /// after the previous item / header.
    leading: Vec<Token>,
    node: Arc<SyntaxNode>,
}

fn schema(node: &SyntaxNode, cx: &Cx<'_>, opts: &FormatOptions) -> Doc {
    debug_assert_eq!(node.kind, SyntaxKind::Schema);

    let mut header_buffer: Vec<Token> = Vec::new();
    let mut imports: Vec<LogicalItem> = Vec::new();
    let mut defs: Vec<LogicalItem> = Vec::new();
    let mut buffer: Vec<Token> = Vec::new();
    let mut seen_first_item = false;

    for el in &node.children {
        match el {
            SyntaxElement::Token(tok) => {
                buffer.push(tok.clone());
            }
            SyntaxElement::Node(child) => match child.kind {
                SyntaxKind::Import | SyntaxKind::Def => {
                    let mut leading = std::mem::take(&mut buffer);
                    if !seen_first_item {
                        // Split off the schema header: trivia separated from
                        // the first item by a blank line is part of the
                        // header. Trivia adjacent to the item attaches to it.
                        let (header, attached) = split_at_last_blank(leading, cx);
                        header_buffer = header;
                        leading = attached;
                        seen_first_item = true;
                    }
                    let item = LogicalItem {
                        leading,
                        node: child.clone(),
                    };
                    if child.kind == SyntaxKind::Import {
                        imports.push(item);
                    } else {
                        defs.push(item);
                    }
                }
                _ => {}
            },
        }
    }
    let trailing_buffer = buffer;

    let (external, internal) = classify_imports(imports, opts);

    let mut parts: Vec<Doc> = Vec::new();

    // Schema header: inline docs + any leading comments.
    let header_doc = render_header(&header_buffer, cx);
    if !matches!(header_doc, Doc::Nil) {
        parts.push(header_doc);
    }

    // Imports section.
    let imports_doc = render_imports(external, internal, cx, opts);
    if !matches!(imports_doc, Doc::Nil) {
        if !parts.is_empty() {
            parts.push(Doc::HardLine);
            parts.push(Doc::HardLine);
        }
        parts.push(imports_doc);
    }

    // Defs section.
    if !defs.is_empty() {
        if !parts.is_empty() {
            parts.push(Doc::HardLine);
            parts.push(Doc::HardLine);
        }
        parts.push(render_defs(defs, cx, opts));
    }

    // Trailing comments at end of file (rare). Always blank-line separated
    // from the preceding section.
    let trailing_doc = render_trailing(&trailing_buffer, cx);
    if !matches!(trailing_doc, Doc::Nil) {
        parts.push(Doc::HardLine);
        parts.push(Doc::HardLine);
        parts.push(trailing_doc);
    }

    Doc::concat(parts)
}

fn render_header(tokens: &[Token], cx: &Cx<'_>) -> Doc {
    // Header carries inline `//!` doc tokens and any leading comments. We
    // preserve them in source order and use blank-line markers from
    // surrounding whitespace.
    let pieces = collect_trivia_pieces(tokens, cx, /*include_doc_inline=*/ true);
    join_pieces_top_level(pieces)
}

fn render_trailing(tokens: &[Token], cx: &Cx<'_>) -> Doc {
    let pieces = collect_trivia_pieces(tokens, cx, /*include_doc_inline=*/ true);
    join_pieces_top_level(pieces)
}

fn render_imports(
    external: Vec<LogicalItem>,
    internal: Vec<LogicalItem>,
    cx: &Cx<'_>,
    opts: &FormatOptions,
) -> Doc {
    if external.is_empty() && internal.is_empty() {
        return Doc::Nil;
    }
    let ext_doc = render_import_group(external, cx, opts);
    let int_doc = render_import_group(internal, cx, opts);
    match (matches!(ext_doc, Doc::Nil), matches!(int_doc, Doc::Nil)) {
        (true, _) => int_doc,
        (_, true) => ext_doc,
        _ => Doc::concat([ext_doc, Doc::HardLine, Doc::HardLine, int_doc]),
    }
}

fn render_import_group(items: Vec<LogicalItem>, cx: &Cx<'_>, opts: &FormatOptions) -> Doc {
    if items.is_empty() {
        return Doc::Nil;
    }
    let mut parts: Vec<Doc> = Vec::new();
    for (i, item) in items.into_iter().enumerate() {
        let leading = render_leading_comments(&item.leading, cx, /*allow_blank_line=*/ i > 0);
        if !matches!(leading, Doc::Nil) {
            if i > 0 {
                parts.push(Doc::HardLine);
            }
            parts.push(leading);
        } else if i > 0 {
            parts.push(Doc::HardLine);
        }
        parts.push(import_node(&item.node, opts));
    }
    Doc::concat(parts)
}

fn render_defs(items: Vec<LogicalItem>, cx: &Cx<'_>, opts: &FormatOptions) -> Doc {
    let mut parts: Vec<Doc> = Vec::new();
    for (i, item) in items.into_iter().enumerate() {
        let had_blank_line = leading_has_blank_line(&item.leading, cx);
        let leading = render_leading_comments(&item.leading, cx, /*allow_blank_line=*/ i > 0);
        if i > 0 {
            parts.push(Doc::HardLine);
            if had_blank_line || !matches!(leading, Doc::Nil) {
                parts.push(Doc::HardLine);
            }
        }
        if !matches!(leading, Doc::Nil) {
            parts.push(leading);
            parts.push(Doc::HardLine);
        }
        parts.push(def_node(&item.node, cx, opts));
    }
    Doc::concat(parts)
}

// ---------------------------------------------------------------------------
// Import classification
// ---------------------------------------------------------------------------

fn classify_imports(
    imports: Vec<LogicalItem>,
    opts: &FormatOptions,
) -> (Vec<LogicalItem>, Vec<LogicalItem>) {
    let mut external = Vec::new();
    let mut internal = Vec::new();
    for item in imports {
        if is_external_import(&item.node, opts) {
            external.push(item);
        } else {
            internal.push(item);
        }
    }
    external.sort_by(|a, b| import_sort_key(&a.node).cmp(&import_sort_key(&b.node)));
    internal.sort_by(|a, b| import_sort_key(&a.node).cmp(&import_sort_key(&b.node)));
    (external, internal)
}

fn is_external_import(import: &SyntaxNode, opts: &FormatOptions) -> bool {
    let Some(tree) = first_node_of_kind(import, SyntaxKind::ImportTree) else {
        return false;
    };
    if import_tree_is_absolute(&tree) {
        return true;
    }
    if let Some(first) = first_path_segment_text(&tree) {
        if first == "std" || opts.external_bundles.iter().any(|b| b == &first) {
            return true;
        }
    }
    false
}

fn import_tree_is_absolute(tree: &SyntaxNode) -> bool {
    // An ImportTree is absolute when its first significant tokens are `::`.
    let mut iter = tree.children.iter();
    while let Some(el) = iter.next() {
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Error => continue,
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Colon && s.is_composed => {
                    return true;
                }
                _ => return false,
            },
            SyntaxElement::Node(_) => return false,
        }
    }
    false
}

fn first_path_segment_text(tree: &SyntaxNode) -> Option<String> {
    for el in &tree.children {
        if let SyntaxElement::Token(tok) = el {
            match &tok.kind {
                TokenKind::Identifier(s) => return Some(s.as_str().to_owned()),
                TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Punctuation(_)
                | TokenKind::Error => continue,
                _ => return None,
            }
        }
    }
    None
}

/// Sort key for an import: render the import's logical target as a string.
fn import_sort_key(import: &SyntaxNode) -> String {
    let Some(tree) = first_node_of_kind(import, SyntaxKind::ImportTree) else {
        return String::new();
    };
    render_import_tree_to_string(&tree, true)
}

/// Render an import tree to a stable string for sorting. Group bodies are
/// rendered as `{name1,name2,...}` with sorted names so that two imports that
/// would format identically have the same sort key.
fn render_import_tree_to_string(tree: &SyntaxNode, sort_groups: bool) -> String {
    let mut out = String::new();
    let mut path_segments: Vec<String> = Vec::new();
    let mut absolute = false;
    let mut group_node: Option<&SyntaxNode> = None;
    let mut wildcard = false;

    for el in &tree.children {
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Identifier(s) => path_segments.push(s.as_str().to_owned()),
                TokenKind::Punctuation(s)
                    if s.kind == PunctuationKind::Colon && s.is_composed =>
                {
                    if path_segments.is_empty() {
                        absolute = true;
                    }
                }
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Asterisk => {
                    wildcard = true;
                }
                _ => {}
            },
            SyntaxElement::Node(child) => {
                if child.kind == SyntaxKind::ImportTree {
                    group_node = Some(child);
                }
            }
        }
    }

    if absolute {
        out.push_str("::");
    }
    out.push_str(&path_segments.join("::"));

    // Detect group-body delimited by braces.
    let is_group = tree.children.iter().any(|el| {
        matches!(
            el,
            SyntaxElement::Token(t) if matches!(
                t.kind,
                TokenKind::Delimiter(DelimiterSymbol::Open(DelimiterKind::Brace))
            )
        )
    });

    if is_group {
        let mut group_children: Vec<String> = Vec::new();
        for el in &tree.children {
            if let SyntaxElement::Node(child) = el {
                if child.kind == SyntaxKind::ImportTree {
                    group_children.push(render_import_tree_to_string(child, sort_groups));
                }
            }
        }
        if sort_groups {
            group_children.sort();
        }
        out.push_str("::{");
        out.push_str(&group_children.join(","));
        out.push('}');
    } else if wildcard {
        out.push_str("::*");
    }

    let _ = group_node; // future use
    out
}

// ---------------------------------------------------------------------------
// Items
// ---------------------------------------------------------------------------

fn import_node(node: &SyntaxNode, opts: &FormatOptions) -> Doc {
    debug_assert_eq!(node.kind, SyntaxKind::Import);
    // Find the ImportTree.
    let Some(tree) = first_node_of_kind(node, SyntaxKind::ImportTree) else {
        return Doc::text("import");
    };
    Doc::concat([
        Doc::text("import "),
        import_tree(&tree, opts, /*top=*/ true),
    ])
}

fn import_tree(tree: &SyntaxNode, opts: &FormatOptions, _top: bool) -> Doc {
    let mut absolute = false;
    let mut path_segments: Vec<String> = Vec::new();
    let mut group_children: Vec<&SyntaxNode> = Vec::new();
    let mut wildcard = false;
    let mut has_brace = false;

    for el in &tree.children {
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Identifier(s) => path_segments.push(s.as_str().to_owned()),
                TokenKind::Punctuation(s)
                    if s.kind == PunctuationKind::Colon && s.is_composed =>
                {
                    if path_segments.is_empty() {
                        absolute = true;
                    }
                }
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Asterisk => {
                    wildcard = true;
                }
                TokenKind::Delimiter(DelimiterSymbol::Open(DelimiterKind::Brace)) => {
                    has_brace = true;
                }
                _ => {}
            },
            SyntaxElement::Node(child) => {
                if child.kind == SyntaxKind::ImportTree {
                    group_children.push(child);
                }
            }
        }
    }

    let mut prefix = String::new();
    if absolute {
        prefix.push_str("::");
    }
    prefix.push_str(&path_segments.join("::"));

    if has_brace {
        // Sort group children by their rendered form.
        group_children.sort_by_key(|child| render_import_tree_to_string(child, true));
        let mut parts: Vec<Doc> = Vec::new();
        if !prefix.is_empty() {
            parts.push(Doc::string(prefix));
            parts.push(Doc::text("::"));
        }
        parts.push(Doc::text("{"));
        let mut inner: Vec<Doc> = Vec::new();
        for (i, child) in group_children.iter().enumerate() {
            inner.push(Doc::HardLine);
            inner.push(import_tree(child, opts, false));
            if i + 1 < group_children.len() || true {
                inner.push(Doc::text(","));
            }
        }
        parts.push(Doc::concat(inner).nest(opts.indent));
        parts.push(Doc::HardLine);
        parts.push(Doc::text("}"));
        Doc::concat(parts)
    } else if wildcard {
        if !prefix.is_empty() {
            Doc::string(format!("{}::*", prefix))
        } else {
            Doc::text("*")
        }
    } else {
        Doc::string(prefix)
    }
}

fn def_node(node: &SyntaxNode, cx: &Cx<'_>, opts: &FormatOptions) -> Doc {
    debug_assert_eq!(node.kind, SyntaxKind::Def);

    // Walk children, emitting docs, attrs, then the keyword + identifier +
    // type variables + body. Trivia inside the def is preserved.
    let mut parts: Vec<Doc> = Vec::new();
    let mut between_iter = TriviaCarrier::default();
    let mut keyword: Option<&Token> = None;

    let mut idx = 0;
    let children = &node.children;
    while idx < children.len() {
        let el = &children[idx];
        match el {
            SyntaxElement::Token(tok) => {
                match &tok.kind {
                    TokenKind::Whitespace => {
                        between_iter.push(tok);
                    }
                    TokenKind::Doc {
                        kind: DocKind::Preceding,
                        ..
                    } => {
                        // Emit any pending trivia (comments on previous lines).
                        flush_def_trivia(&mut parts, &mut between_iter);
                        parts.push(doc_token(tok));
                        parts.push(Doc::HardLine);
                    }
                    TokenKind::Comment { .. } | TokenKind::Doc { .. } => {
                        between_iter.push(tok);
                    }
                    TokenKind::Identifier(s)
                        if matches!(
                            s.as_str(),
                            "record" | "variant" | "alias" | "opaque" | "wrapper"
                        ) && keyword.is_none() =>
                    {
                        // Reached the def keyword. Flush trivia, emit keyword,
                        // then handle the rest based on the kind.
                        flush_def_trivia(&mut parts, &mut between_iter);
                        keyword = Some(tok);
                        parts.push(Doc::string(s.as_str().to_owned()));
                        idx += 1;
                        break;
                    }
                    _ => {
                        between_iter.push(tok);
                    }
                }
            }
            SyntaxElement::Node(child) => match child.kind {
                SyntaxKind::Attr => {
                    flush_def_trivia(&mut parts, &mut between_iter);
                    parts.push(attr(child, opts));
                    parts.push(Doc::HardLine);
                }
                _ => {
                    // Unexpected — skip.
                }
            },
        }
        idx += 1;
    }

    let Some(_kw) = keyword else {
        return Doc::concat(parts);
    };

    // After the keyword: identifier, optional <vars>, body.
    let mut name_parts: Vec<Doc> = Vec::new();
    let mut after_name = false;
    let mut had_type_vars = false;
    while idx < children.len() {
        let el = &children[idx];
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Error => {}
                TokenKind::Identifier(s) if !after_name => {
                    name_parts.push(Doc::text(" "));
                    name_parts.push(Doc::string(s.as_str().to_owned()));
                    after_name = true;
                }
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Colon => {
                    // Alias / wrapper: `kw NAME: TYPE`.
                    name_parts.push(Doc::text(":"));
                    idx += 1;
                    parts.extend(name_parts.drain(..));
                    let ty = collect_type_expr_after(children, &mut idx, opts);
                    parts.push(Doc::text(" "));
                    parts.push(ty);
                    return Doc::concat(parts);
                }
                TokenKind::Delimiter(DelimiterSymbol::Open(DelimiterKind::Brace)) => {
                    // Record/variant body.
                    parts.extend(name_parts.drain(..));
                    parts.push(Doc::text(" {"));
                    idx += 1;
                    let body = collect_brace_body(children, &mut idx, cx, opts);
                    parts.push(body);
                    parts.push(Doc::text("}"));
                    return Doc::concat(parts);
                }
                _ => {}
            },
            SyntaxElement::Node(child) => {
                if child.kind == SyntaxKind::TypeVars {
                    had_type_vars = true;
                    name_parts.push(type_vars(child));
                }
            }
        }
        idx += 1;
    }
    let _ = had_type_vars;
    parts.extend(name_parts);
    Doc::concat(parts)
}

#[derive(Default)]
struct TriviaCarrier<'a> {
    tokens: Vec<&'a Token>,
}

impl<'a> TriviaCarrier<'a> {
    fn push(&mut self, t: &'a Token) {
        self.tokens.push(t);
    }
}

/// Emits any pending leading comments (drops whitespace) inside a def header.
/// Each comment is followed by a HardLine.
fn flush_def_trivia<'a>(parts: &mut Vec<Doc>, carrier: &mut TriviaCarrier<'a>) {
    for tok in carrier.tokens.drain(..) {
        match &tok.kind {
            TokenKind::Comment { .. } => {
                parts.push(comment_token(tok));
                parts.push(Doc::HardLine);
            }
            TokenKind::Doc {
                kind: DocKind::Preceding,
                ..
            } => {
                parts.push(doc_token(tok));
                parts.push(Doc::HardLine);
            }
            _ => {}
        }
    }
}

fn type_vars(node: &SyntaxNode) -> Doc {
    debug_assert_eq!(node.kind, SyntaxKind::TypeVars);
    let mut names: Vec<String> = Vec::new();
    for el in &node.children {
        if let SyntaxElement::Node(child) = el {
            if child.kind == SyntaxKind::TypeVar {
                if let Some(t) = first_identifier(child) {
                    names.push(t);
                }
            }
        }
    }
    Doc::string(format!("<{}>", names.join(", ")))
}

fn collect_type_expr_after(
    children: &[SyntaxElement],
    idx: &mut usize,
    _opts: &FormatOptions,
) -> Doc {
    while *idx < children.len() {
        match &children[*idx] {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Error => *idx += 1,
                _ => break,
            },
            SyntaxElement::Node(child) => {
                if child.kind == SyntaxKind::TypeExpr {
                    let d = type_expr(child);
                    *idx += 1;
                    return d;
                } else {
                    *idx += 1;
                }
            }
        }
    }
    Doc::Nil
}

/// Collect brace-delimited body of a record/variant. Emits fields/variants
/// always-expanded, one per line, with comma after each. Preserves comments
/// between members.
fn collect_brace_body(
    children: &[SyntaxElement],
    idx: &mut usize,
    cx: &Cx<'_>,
    opts: &FormatOptions,
) -> Doc {
    // Collect pieces: each piece is either a member node or a comment token.
    enum BodyPiece<'a> {
        Member(&'a Arc<SyntaxNode>),
        Comment(&'a Token),
        BlankLine,
    }
    let mut pieces: Vec<BodyPiece> = Vec::new();
    let mut last_was_blank = false;
    while *idx < children.len() {
        let el = &children[*idx];
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Delimiter(DelimiterSymbol::Close(DelimiterKind::Brace)) => {
                    *idx += 1;
                    return assemble_body(pieces, opts);
                }
                TokenKind::Whitespace => {
                    if cx.newlines_in(tok) >= 2 && !pieces.is_empty() && !last_was_blank {
                        pieces.push(BodyPiece::BlankLine);
                        last_was_blank = true;
                    }
                }
                TokenKind::Comment { .. } => {
                    pieces.push(BodyPiece::Comment(tok));
                    last_was_blank = false;
                }
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Comma => {
                    // Commas are absorbed; we emit our own.
                }
                _ => {}
            },
            SyntaxElement::Node(child) => match child.kind {
                SyntaxKind::Field | SyntaxKind::Variant => {
                    pieces.push(BodyPiece::Member(child));
                    last_was_blank = false;
                }
                _ => {}
            },
        }
        *idx += 1;
    }
    fn assemble_body(pieces: Vec<BodyPiece>, opts: &FormatOptions) -> Doc {
        let any_member = pieces.iter().any(|p| matches!(p, BodyPiece::Member(_)));
        if !any_member {
            return Doc::Nil;
        }
        let mut inner: Vec<Doc> = Vec::new();
        let mut first_member_emitted = false;
        for piece in pieces {
            match piece {
                BodyPiece::Member(m) => {
                    inner.push(Doc::HardLine);
                    inner.push(member(m, opts));
                    inner.push(Doc::text(","));
                    first_member_emitted = true;
                }
                BodyPiece::Comment(tok) => {
                    inner.push(Doc::HardLine);
                    inner.push(comment_token(tok));
                }
                BodyPiece::BlankLine => {
                    if first_member_emitted {
                        inner.push(Doc::HardLine);
                    }
                }
            }
        }
        let body_inner = Doc::concat(inner).nest(opts.indent);
        Doc::concat([body_inner, Doc::HardLine])
    }
    assemble_body(pieces, opts)
}

fn member(node: &SyntaxNode, opts: &FormatOptions) -> Doc {
    match node.kind {
        SyntaxKind::Field => field(node, opts),
        SyntaxKind::Variant => variant(node, opts),
        _ => Doc::Nil,
    }
}

fn field(node: &SyntaxNode, opts: &FormatOptions) -> Doc {
    let mut parts: Vec<Doc> = Vec::new();
    // Emit docs and attrs first; then `name?: TYPE`.
    let mut name: Option<String> = None;
    let mut optional = false;
    let mut ty: Option<&Arc<SyntaxNode>> = None;
    let mut docs: Vec<&Token> = Vec::new();
    let mut leading_comments: Vec<&Token> = Vec::new();
    let mut attrs: Vec<&Arc<SyntaxNode>> = Vec::new();
    for el in &node.children {
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Doc { kind: DocKind::Preceding, .. } => docs.push(tok),
                TokenKind::Comment { .. } => leading_comments.push(tok),
                TokenKind::Identifier(s) if name.is_none() => name = Some(s.as_str().to_owned()),
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::QuestionMark => {
                    optional = true;
                }
                _ => {}
            },
            SyntaxElement::Node(child) => match child.kind {
                SyntaxKind::Attr => attrs.push(child),
                SyntaxKind::TypeExpr => ty = Some(child),
                _ => {}
            },
        }
    }
    for tok in docs {
        parts.push(doc_token(tok));
        parts.push(Doc::HardLine);
    }
    for tok in leading_comments {
        parts.push(comment_token(tok));
        parts.push(Doc::HardLine);
    }
    for a in attrs {
        parts.push(attr(a, opts));
        parts.push(Doc::HardLine);
    }
    let name = name.unwrap_or_default();
    let opt = if optional { "?" } else { "" };
    let ty_doc = ty.map(|t| type_expr(t.as_ref())).unwrap_or(Doc::Nil);
    parts.push(Doc::string(format!("{name}{opt}: ")));
    parts.push(ty_doc);
    Doc::concat(parts)
}

fn variant(node: &SyntaxNode, opts: &FormatOptions) -> Doc {
    let mut parts: Vec<Doc> = Vec::new();
    let mut name: Option<String> = None;
    let mut ty: Option<&Arc<SyntaxNode>> = None;
    let mut has_colon = false;
    let mut docs: Vec<&Token> = Vec::new();
    let mut leading_comments: Vec<&Token> = Vec::new();
    let mut attrs: Vec<&Arc<SyntaxNode>> = Vec::new();
    for el in &node.children {
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Doc { kind: DocKind::Preceding, .. } => docs.push(tok),
                TokenKind::Comment { .. } => leading_comments.push(tok),
                TokenKind::Identifier(s) if name.is_none() => name = Some(s.as_str().to_owned()),
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Colon => has_colon = true,
                _ => {}
            },
            SyntaxElement::Node(child) => match child.kind {
                SyntaxKind::Attr => attrs.push(child),
                SyntaxKind::TypeExpr => ty = Some(child),
                _ => {}
            },
        }
    }
    for tok in docs {
        parts.push(doc_token(tok));
        parts.push(Doc::HardLine);
    }
    for tok in leading_comments {
        parts.push(comment_token(tok));
        parts.push(Doc::HardLine);
    }
    for a in attrs {
        parts.push(attr(a, opts));
        parts.push(Doc::HardLine);
    }
    let name = name.unwrap_or_default();
    parts.push(Doc::string(name));
    if has_colon {
        if let Some(t) = ty {
            parts.push(Doc::text(": "));
            parts.push(type_expr(t));
        }
    }
    Doc::concat(parts)
}

// ---------------------------------------------------------------------------
// Type expressions
// ---------------------------------------------------------------------------

fn type_expr(node: &SyntaxNode) -> Doc {
    // Type expressions are always emitted inline. We render them by walking
    // the tokens directly, with separators chosen by token kind.
    let mut out = String::new();
    render_type_expr_into(node, &mut out);
    Doc::string(out)
}

fn render_type_expr_into(node: &SyntaxNode, out: &mut String) {
    for el in &node.children {
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Error => {}
                TokenKind::Identifier(s) => out.push_str(s.as_str()),
                TokenKind::Punctuation(s) => match s.kind {
                    PunctuationKind::Colon if s.is_composed => out.push(':'),
                    PunctuationKind::Colon => {
                        // Could be the second colon of `::` or the map-key
                        // separator inside `[K: V]`. The composed flag of the
                        // preceding colon disambiguates; here we just emit `:`
                        // and the surrounding context handles spacing.
                        if !out.ends_with(':') {
                            out.push_str(": ");
                        } else {
                            out.push(':');
                        }
                    }
                    PunctuationKind::Comma => out.push_str(", "),
                    PunctuationKind::AngleOpen => out.push('<'),
                    PunctuationKind::AngleClose => out.push('>'),
                    other => {
                        // Generic fallback.
                        out.push_str(&other.to_string());
                    }
                },
                TokenKind::Delimiter(DelimiterSymbol::Open(DelimiterKind::Bracket)) => {
                    out.push('[');
                }
                TokenKind::Delimiter(DelimiterSymbol::Close(DelimiterKind::Bracket)) => {
                    out.push(']');
                }
                TokenKind::Delimiter(d) => out.push_str(&d.to_string()),
                _ => {}
            },
            SyntaxElement::Node(child) => match child.kind {
                SyntaxKind::Path => {
                    render_path_into(child, out);
                }
                SyntaxKind::TypeExpr => {
                    render_type_expr_into(child, out);
                }
                SyntaxKind::TypeSubst => {
                    out.push('<');
                    let mut first = true;
                    for sub in &child.children {
                        if let SyntaxElement::Node(n) = sub {
                            if n.kind == SyntaxKind::TypeExpr {
                                if !first {
                                    out.push_str(", ");
                                }
                                first = false;
                                render_type_expr_into(n, out);
                            }
                        }
                    }
                    out.push('>');
                }
                _ => {}
            },
        }
    }
    // Trim trailing whitespace from naive emission.
    while out.ends_with(' ') {
        out.pop();
    }
}

fn render_path_into(path: &SyntaxNode, out: &mut String) {
    let mut absolute_done = false;
    let mut segments: Vec<String> = Vec::new();
    let mut absolute = false;
    for el in &path.children {
        if let SyntaxElement::Token(tok) = el {
            match &tok.kind {
                TokenKind::Identifier(s) => segments.push(s.as_str().to_owned()),
                TokenKind::Punctuation(s) if s.kind == PunctuationKind::Colon => {
                    if segments.is_empty() && !absolute_done {
                        absolute = true;
                    }
                    absolute_done = true;
                }
                _ => {}
            }
        }
    }
    if absolute {
        out.push_str("::");
    }
    out.push_str(&segments.join("::"));
}

// ---------------------------------------------------------------------------
// Attributes
// ---------------------------------------------------------------------------

fn attr(node: &SyntaxNode, opts: &FormatOptions) -> Doc {
    debug_assert_eq!(node.kind, SyntaxKind::Attr);
    // An Attr node contains: `#`, `[`, attribute body (Path / AttrList /
    // AttrAssign / AttrTokens / inline tokens), `]`.
    // Find the body: the first child that's either an inner node or a
    // sequence of meaningful tokens.
    // Simplest: render the body to a Doc that fits-or-breaks via Group.
    let body = render_attr_body(node, opts);
    Doc::concat([Doc::text("#["), body, Doc::text("]")]).group()
}

fn render_attr_body(attr_node: &SyntaxNode, opts: &FormatOptions) -> Doc {
    // Collect the elements between `[` and `]`.
    let mut iter = attr_node.children.iter();
    while let Some(el) = iter.next() {
        if let SyntaxElement::Token(t) = el {
            if matches!(
                t.kind,
                TokenKind::Delimiter(DelimiterSymbol::Open(DelimiterKind::Bracket))
            ) {
                break;
            }
        }
    }
    let body_elements: Vec<&SyntaxElement> = iter
        .take_while(|el| {
            !matches!(
                el,
                SyntaxElement::Token(t) if matches!(
                    t.kind,
                    TokenKind::Delimiter(DelimiterSymbol::Close(DelimiterKind::Bracket))
                )
            )
        })
        .collect();
    render_attr_term(&body_elements, opts)
}

/// Render a slice of attribute body elements as a single attribute term:
/// `PATH = VALUE`, `PATH ( ARG, ARG )`, `PATH`, or free-form tokens.
fn render_attr_term(elements: &[&SyntaxElement], opts: &FormatOptions) -> Doc {
    let mut i = 0;
    while i < elements.len() && is_trivia(elements[i]) {
        i += 1;
    }
    if i == elements.len() {
        return Doc::Nil;
    }

    // Pull a leading Path node if present.
    let mut path_str = String::new();
    if let SyntaxElement::Node(n) = elements[i] {
        if n.kind == SyntaxKind::Path {
            render_path_into(n, &mut path_str);
            i += 1;
        }
    }

    while i < elements.len() && is_trivia(elements[i]) {
        i += 1;
    }

    if path_str.is_empty() {
        // No leading path — render verbatim as a token tree.
        return render_attr_tokens(elements, opts);
    }

    if i >= elements.len() {
        return Doc::string(path_str);
    }

    match elements[i] {
        SyntaxElement::Token(t) => match &t.kind {
            TokenKind::Punctuation(s) if s.kind == PunctuationKind::Equals => {
                let value_doc = render_attr_term(&elements[i + 1..], opts);
                Doc::concat([Doc::string(path_str), Doc::text(" = "), value_doc])
            }
            TokenKind::Delimiter(DelimiterSymbol::Open(DelimiterKind::Parenthesis)) => {
                let close_idx = find_matching_paren(elements, i);
                let inner = &elements[i + 1..close_idx];
                let groups = split_top_level_commas(inner);
                if groups.is_empty() {
                    return Doc::concat([Doc::string(path_str), Doc::text("()")]);
                }
                let mut group_docs: Vec<Doc> = Vec::new();
                for (j, g) in groups.iter().enumerate() {
                    if j > 0 {
                        group_docs.push(Doc::text(","));
                        group_docs.push(Doc::Line);
                    }
                    let g_refs: Vec<&SyntaxElement> = g.iter().copied().collect();
                    group_docs.push(render_attr_term(&g_refs, opts));
                }
                // Trailing comma when broken (empty when flat).
                group_docs.push(if_break_comma());
                let inner_doc = Doc::concat(group_docs);
                let nested = Doc::concat([Doc::SoftLine, inner_doc]).nest(opts.indent);
                let body = Doc::concat([nested, Doc::SoftLine]).group();
                Doc::concat([Doc::string(path_str), Doc::text("("), body, Doc::text(")")])
            }
            _ => Doc::string(path_str),
        },
        _ => Doc::string(path_str),
    }
}

/// A doc that renders to `,` when broken and to `` (empty) when flat.
///
/// We don't have a dedicated `IfBreak` combinator, so simulate: under flat
/// rendering, `SoftLine` is empty; under broken rendering, `SoftLine` is a
/// newline. Concat with `Text(",")` only makes sense when broken. We approx
/// by using a separate Group: when fits flat, add nothing; when broken, add
/// the trailing comma.
fn if_break_comma() -> Doc {
    // Wrap in a Group whose flat path is Nil and broken path includes ",".
    // We model "if broken" with a Group that always breaks (HardLine inside)
    // and emits "," before its line; but that would force the outer group
    // to break too. Easier: emit a SoftLine-conditioned text via a small
    // helper Doc::concat([Group(Nil), ...]) — there is no clean way without
    // adding IfBreak. For now, omit the trailing comma to avoid forcing a
    // break.
    Doc::Nil
}

fn is_trivia(el: &SyntaxElement) -> bool {
    matches!(
        el,
        SyntaxElement::Token(t) if matches!(
            t.kind,
            TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Error
        )
    )
}

/// Render a slice of body elements as raw tokens (used for free-form attrs).
fn render_attr_tokens(elements: &[&SyntaxElement], _opts: &FormatOptions) -> Doc {
    let mut out = String::new();
    let mut last_was_separated = true;
    for el in elements {
        match el {
            SyntaxElement::Token(tok) => match &tok.kind {
                TokenKind::Whitespace
                | TokenKind::Comment { .. }
                | TokenKind::Doc { .. }
                | TokenKind::Error => {}
                _ => {
                    if !last_was_separated && !out.is_empty() {
                        out.push(' ');
                    }
                    out.push_str(&render_token(tok));
                    last_was_separated = matches!(
                        tok.kind,
                        TokenKind::Punctuation(PunctuationSymbol {
                            is_composed: true,
                            ..
                        })
                    );
                    last_was_separated = !last_was_separated;
                    let _ = last_was_separated;
                    last_was_separated = !tok.is_separated();
                }
            },
            SyntaxElement::Node(_) => {}
        }
    }
    Doc::string(out)
}

fn render_token(tok: &Token) -> String {
    match &tok.kind {
        TokenKind::Identifier(s) => s.as_str().to_owned(),
        TokenKind::Literal(Literal::Boolean(b)) => if *b { "true" } else { "false" }.to_owned(),
        TokenKind::Literal(Literal::Numeric { has_minus, integral, fractional }) => {
            let mut s = String::new();
            if *has_minus {
                s.push('-');
            }
            s.push_str(integral);
            if let Some(f) = fractional {
                s.push('.');
                s.push_str(f);
            }
            s
        }
        TokenKind::Literal(Literal::String(string)) => format!("\"{}\"", string),
        TokenKind::Punctuation(s) => s.kind.to_string(),
        TokenKind::Delimiter(d) => d.to_string(),
        TokenKind::Comment { .. } | TokenKind::Doc { .. } | TokenKind::Whitespace | TokenKind::Error => String::new(),
    }
}

/// Find the index of the closing `)` matching the `(` at `open_idx` (inside
/// `elements`, with proper nesting).
fn find_matching_paren(elements: &[&SyntaxElement], open_idx: usize) -> usize {
    let mut depth = 0i32;
    for (i, el) in elements.iter().enumerate().skip(open_idx) {
        if let SyntaxElement::Token(t) = el {
            match &t.kind {
                TokenKind::Delimiter(DelimiterSymbol::Open(DelimiterKind::Parenthesis)) => {
                    depth += 1;
                }
                TokenKind::Delimiter(DelimiterSymbol::Close(DelimiterKind::Parenthesis)) => {
                    depth -= 1;
                    if depth == 0 {
                        return i;
                    }
                }
                _ => {}
            }
        }
    }
    elements.len()
}

/// Splits elements on top-level commas (commas not inside any paren, bracket,
/// or brace group). Returns a slice per group.
fn split_top_level_commas<'a>(
    elements: &'a [&'a SyntaxElement],
) -> Vec<Vec<&'a SyntaxElement>> {
    let mut groups: Vec<Vec<&SyntaxElement>> = Vec::new();
    let mut current: Vec<&SyntaxElement> = Vec::new();
    let mut depth = 0i32;
    for el in elements {
        if let SyntaxElement::Token(t) = el {
            match &t.kind {
                TokenKind::Delimiter(DelimiterSymbol::Open(_)) => {
                    depth += 1;
                    current.push(*el);
                    continue;
                }
                TokenKind::Delimiter(DelimiterSymbol::Close(_)) => {
                    depth -= 1;
                    current.push(*el);
                    continue;
                }
                TokenKind::Punctuation(s)
                    if s.kind == PunctuationKind::Comma && depth == 0 =>
                {
                    if !current.is_empty() || !groups.is_empty() {
                        groups.push(std::mem::take(&mut current));
                    }
                    continue;
                }
                _ => {}
            }
        }
        current.push(*el);
    }
    if !current.is_empty() {
        // Discard if it's only trivia.
        let any_meaningful = current.iter().any(|el| matches!(el, SyntaxElement::Token(t) if !matches!(t.kind, TokenKind::Whitespace | TokenKind::Comment { .. } | TokenKind::Doc { .. } | TokenKind::Error)) || matches!(el, SyntaxElement::Node(_)));
        if any_meaningful {
            groups.push(current);
        }
    }
    groups
}

// ---------------------------------------------------------------------------
// Trivia helpers
// ---------------------------------------------------------------------------

enum TriviaPiece {
    LineComment(String),
    BlockComment(String),
    DocPreceding(String),
    DocInline(String),
    BlankLine,
}

fn collect_trivia_pieces(
    tokens: &[Token],
    cx: &Cx<'_>,
    include_doc_inline: bool,
) -> Vec<TriviaPiece> {
    let mut pieces = Vec::new();
    let mut last_was_blank = false;
    for tok in tokens {
        match &tok.kind {
            TokenKind::Whitespace => {
                if cx.newlines_in(tok) >= 2 && !pieces.is_empty() && !last_was_blank {
                    pieces.push(TriviaPiece::BlankLine);
                    last_was_blank = true;
                }
            }
            TokenKind::Comment {
                kind: CommentKind::Line,
                comment,
            } => {
                pieces.push(TriviaPiece::LineComment(comment.as_str().to_owned()));
                last_was_blank = false;
            }
            TokenKind::Comment {
                kind: CommentKind::Block,
                comment,
            } => {
                pieces.push(TriviaPiece::BlockComment(comment.as_str().to_owned()));
                last_was_blank = false;
            }
            TokenKind::Doc {
                kind: DocKind::Preceding,
                doc,
            } => {
                pieces.push(TriviaPiece::DocPreceding(doc.as_str().to_owned()));
                last_was_blank = false;
            }
            TokenKind::Doc {
                kind: DocKind::Inline,
                doc,
            } if include_doc_inline => {
                pieces.push(TriviaPiece::DocInline(doc.as_str().to_owned()));
                last_was_blank = false;
            }
            _ => {}
        }
    }
    pieces
}

fn join_pieces_top_level(mut pieces: Vec<TriviaPiece>) -> Doc {
    // Trim trailing blank-line markers — section separators handle inter-
    // section spacing, so the header / trailing blocks shouldn't emit their
    // own.
    while matches!(pieces.last(), Some(TriviaPiece::BlankLine)) {
        pieces.pop();
    }
    if pieces.is_empty() {
        return Doc::Nil;
    }
    let mut parts: Vec<Doc> = Vec::new();
    let mut first = true;
    for p in pieces {
        if !first {
            parts.push(Doc::HardLine);
        }
        first = false;
        match p {
            TriviaPiece::LineComment(s) => parts.push(Doc::string(format!("//{}", s))),
            TriviaPiece::BlockComment(s) => parts.push(Doc::string(format!("/*{}*/", s))),
            TriviaPiece::DocPreceding(s) => parts.push(Doc::string(format!("///{}", s))),
            TriviaPiece::DocInline(s) => parts.push(Doc::string(format!("//!{}", s))),
            TriviaPiece::BlankLine => {
                parts.push(Doc::HardLine);
                first = true;
            }
        }
    }
    Doc::concat(parts)
}

fn render_leading_comments(tokens: &[Token], cx: &Cx<'_>, allow_blank_line: bool) -> Doc {
    let pieces = collect_trivia_pieces(tokens, cx, false);
    if pieces.is_empty() {
        return Doc::Nil;
    }
    let mut parts: Vec<Doc> = Vec::new();
    let mut first = true;
    for p in pieces {
        if !first {
            parts.push(Doc::HardLine);
        }
        first = false;
        match p {
            TriviaPiece::LineComment(s) => parts.push(Doc::string(format!("//{}", s))),
            TriviaPiece::BlockComment(s) => parts.push(Doc::string(format!("/*{}*/", s))),
            TriviaPiece::DocPreceding(s) => parts.push(Doc::string(format!("///{}", s))),
            TriviaPiece::DocInline(s) => parts.push(Doc::string(format!("//!{}", s))),
            TriviaPiece::BlankLine if allow_blank_line => {
                parts.push(Doc::HardLine);
                first = true;
            }
            TriviaPiece::BlankLine => {}
        }
    }
    Doc::concat(parts)
}

fn leading_has_blank_line(tokens: &[Token], cx: &Cx<'_>) -> bool {
    tokens
        .iter()
        .any(|t| matches!(t.kind, TokenKind::Whitespace) && cx.newlines_in(t) >= 2)
}

/// Split a token buffer at the most recent whitespace token containing a
/// blank line. The returned `(before, after)` slices the buffer so that
/// `after` contains exactly the trivia adjacent to the next significant
/// element (the typical "leading" of an item).
fn split_at_last_blank(mut tokens: Vec<Token>, cx: &Cx<'_>) -> (Vec<Token>, Vec<Token>) {
    let mut last_blank = None;
    for (i, t) in tokens.iter().enumerate() {
        if matches!(t.kind, TokenKind::Whitespace) && cx.newlines_in(t) >= 2 {
            last_blank = Some(i);
        }
    }
    match last_blank {
        Some(i) => {
            let after = tokens.split_off(i + 1);
            (tokens, after)
        }
        None => (Vec::new(), tokens),
    }
}

fn comment_token(tok: &Token) -> Doc {
    match &tok.kind {
        TokenKind::Comment {
            kind: CommentKind::Line,
            comment,
        } => Doc::string(format!("//{}", comment)),
        TokenKind::Comment {
            kind: CommentKind::Block,
            comment,
        } => Doc::string(format!("/*{}*/", comment)),
        _ => Doc::Nil,
    }
}

fn doc_token(tok: &Token) -> Doc {
    match &tok.kind {
        TokenKind::Doc {
            kind: DocKind::Preceding,
            doc,
        } => Doc::string(format!("///{}", doc)),
        TokenKind::Doc {
            kind: DocKind::Inline,
            doc,
        } => Doc::string(format!("//!{}", doc)),
        _ => Doc::Nil,
    }
}

// ---------------------------------------------------------------------------
// Generic helpers
// ---------------------------------------------------------------------------

fn first_node_of_kind(node: &SyntaxNode, kind: SyntaxKind) -> Option<Arc<SyntaxNode>> {
    for el in &node.children {
        if let SyntaxElement::Node(child) = el {
            if child.kind == kind {
                return Some(child.clone());
            }
        }
    }
    None
}

fn first_identifier(node: &SyntaxNode) -> Option<String> {
    for el in &node.children {
        if let SyntaxElement::Token(tok) = el {
            if let TokenKind::Identifier(s) = &tok.kind {
                return Some(s.as_str().to_owned());
            }
        }
    }
    None
}
