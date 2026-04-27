//! Hand-written lexer for Sidex schemas.
//!
//! Iterates the source by Unicode scalar values (not bytes), so the spans it
//! produces are character-indexed — matching the convention used elsewhere in
//! the IR.

use std::sync::Arc;

use sidex_diagnostics::{Diagnostic, Label};
use sidex_ir as ir;

use crate::tokens::{
    CommentKind, DelimiterKind, DelimiterSymbol, DocKind, Literal, PunctuationKind,
    PunctuationSymbol, Str, Token, TokenKind,
};

/// Lex `text` and return a vector of tokens including whitespace and trivia.
///
/// Errors are emitted via the active diagnostic context. The lexer always
/// makes progress: unexpected characters produce a single-character
/// [`TokenKind::Error`] token and the lexer continues.
pub fn lex(src: ir::SourceIdx, text: &str) -> Vec<Token> {
    let chars: Vec<char> = text.chars().collect();
    let mut lx = Lexer {
        src,
        chars: &chars,
        pos: 0,
        tokens: Vec::new(),
    };
    while lx.pos < lx.chars.len() {
        lx.next_token();
    }
    // Mark composed punctuation: a punctuation immediately followed by another
    // punctuation (no intervening whitespace or other token) is "composed".
    mark_composed(&mut lx.tokens);
    lx.tokens
}

struct Lexer<'a> {
    src: ir::SourceIdx,
    chars: &'a [char],
    pos: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn span(&self, start: usize, end: usize) -> ir::Span {
        ir::Span::new(self.src, start, end)
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    fn push(&mut self, kind: TokenKind, start: usize, end: usize) {
        self.tokens.push(Token::new(kind, self.span(start, end)));
    }

    fn next_token(&mut self) {
        let start = self.pos;
        let Some(c) = self.peek(0) else {
            return;
        };

        if c.is_whitespace() {
            self.lex_whitespace(start);
            return;
        }

        if c == '/' {
            if self.peek(1) == Some('/') {
                // Line comment or doc comment (`//`, `///`, `//!`).
                self.lex_line_comment(start);
                return;
            } else if self.peek(1) == Some('*') {
                self.lex_block_comment(start);
                return;
            }
        }

        if c.is_ascii_digit() {
            self.lex_number(start, false);
            return;
        }

        if c == '-' && self.peek(1).is_some_and(|n| n.is_ascii_digit()) {
            self.pos += 1;
            self.lex_number(start, true);
            return;
        }

        if c == '"' {
            self.lex_string(start);
            return;
        }

        if is_ident_start(c) {
            self.lex_ident(start);
            return;
        }

        if let Some(kind) = delimiter_kind(c) {
            self.pos += 1;
            self.push(kind, start, self.pos);
            return;
        }

        if let Some(kind) = PunctuationKind::from_char(c) {
            self.pos += 1;
            self.push(
                TokenKind::Punctuation(PunctuationSymbol::new(kind, false)),
                start,
                self.pos,
            );
            return;
        }

        // Unknown character: emit an error token and a diagnostic, then move on.
        self.pos += 1;
        let span = self.span(start, self.pos);
        Diagnostic::error(format!("Unexpected character {:?}.", c))
            .with_span(Some(span.clone()))
            .with_label(Label::new(span.clone(), "Unexpected character."))
            .emit();
        self.push(TokenKind::Error, start, self.pos);
    }

    fn lex_whitespace(&mut self, start: usize) {
        while self.peek(0).is_some_and(char::is_whitespace) {
            self.pos += 1;
        }
        self.push(TokenKind::Whitespace, start, self.pos);
    }

    /// Consumes a `//`-prefixed comment. Decides whether it's a doc or normal
    /// comment based on the third character.
    fn lex_line_comment(&mut self, start: usize) {
        // Skip the leading `//`.
        self.pos += 2;
        // Determine the comment variety.
        let third = self.peek(0);
        let kind: CommentVariety = match third {
            Some('/') => {
                // `///` — preceding doc.
                self.pos += 1;
                CommentVariety::Doc(DocKind::Preceding)
            }
            Some('!') => {
                // `//!` — inline doc.
                self.pos += 1;
                CommentVariety::Doc(DocKind::Inline)
            }
            _ => CommentVariety::Line,
        };
        let body_start = self.pos;
        while let Some(c) = self.peek(0) {
            if c == '\n' {
                break;
            }
            self.pos += 1;
        }
        let body: String = self.chars[body_start..self.pos].iter().collect();
        // Match the legacy lexer: the comment body does NOT include the
        // terminating newline (which is consumed as whitespace in a later
        // iteration).
        let kind_token = match kind {
            CommentVariety::Doc(doc_kind) => {
                TokenKind::Doc {
                    doc: Arc::new(body),
                    kind: doc_kind,
                }
            }
            CommentVariety::Line => {
                TokenKind::Comment {
                    comment: Arc::new(body),
                    kind: CommentKind::Line,
                }
            }
        };
        self.push(kind_token, start, self.pos);
    }

    fn lex_block_comment(&mut self, start: usize) {
        // Skip the leading `/*`.
        self.pos += 2;
        let body_start = self.pos;
        let mut closed = false;
        while let Some(c) = self.peek(0) {
            if c == '*' && self.peek(1) == Some('/') {
                let body: String = self.chars[body_start..self.pos].iter().collect();
                self.pos += 2;
                self.push(
                    TokenKind::Comment {
                        comment: Arc::new(body),
                        kind: CommentKind::Block,
                    },
                    start,
                    self.pos,
                );
                closed = true;
                break;
            }
            self.pos += 1;
        }
        if !closed {
            let body: String = self.chars[body_start..self.pos].iter().collect();
            let span = self.span(start, self.pos);
            Diagnostic::error("Unclosed block comment.")
                .with_span(Some(span.clone()))
                .with_label(Label::new(span.clone(), "Unclosed block comment."))
                .emit();
            self.push(
                TokenKind::Comment {
                    comment: Arc::new(body),
                    kind: CommentKind::Block,
                },
                start,
                self.pos,
            );
        }
    }

    /// Lex a numeric literal. `start` is the offset of the leading minus or
    /// digit; `has_minus` is whether a minus has already been consumed.
    fn lex_number(&mut self, start: usize, has_minus: bool) {
        let int_start = self.pos;
        while self.peek(0).is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        let integral: String = self.chars[int_start..self.pos].iter().collect();

        let mut fractional = None;
        if self.peek(0) == Some('.') && self.peek(1).is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1; // dot
            let frac_start = self.pos;
            while self.peek(0).is_some_and(|c| c.is_ascii_digit()) {
                self.pos += 1;
            }
            let frac: String = self.chars[frac_start..self.pos].iter().collect();
            fractional = Some(Arc::new(frac));
        }

        self.push(
            TokenKind::Literal(Literal::Numeric {
                has_minus,
                integral: Arc::new(integral),
                fractional,
            }),
            start,
            self.pos,
        );
    }

    fn lex_string(&mut self, start: usize) {
        // Skip the opening quote.
        self.pos += 1;
        let mut value = String::new();
        let mut closed = false;
        while let Some(c) = self.peek(0) {
            match c {
                '"' => {
                    self.pos += 1;
                    closed = true;
                    break;
                }
                '\n' => {
                    // Unterminated string: stop at the newline so we can recover.
                    break;
                }
                '\\' => {
                    self.pos += 1;
                    match self.peek(0) {
                        Some('\\') => {
                            value.push('\\');
                            self.pos += 1;
                        }
                        Some('"') => {
                            value.push('"');
                            self.pos += 1;
                        }
                        Some(other) => {
                            // Unknown escape — emit a diagnostic and keep both
                            // characters verbatim so spans stay sane.
                            let escape_span = self.span(self.pos - 1, self.pos + 1);
                            Diagnostic::error(format!("Unknown escape sequence `\\{}`.", other))
                                .with_span(Some(escape_span.clone()))
                                .with_label(Label::new(escape_span, "Unknown escape sequence."))
                                .emit();
                            value.push('\\');
                            value.push(other);
                            self.pos += 1;
                        }
                        None => break,
                    }
                }
                other => {
                    value.push(other);
                    self.pos += 1;
                }
            }
        }
        if !closed {
            let span = self.span(start, self.pos);
            Diagnostic::error("Unclosed string literal.")
                .with_span(Some(span.clone()))
                .with_label(Label::new(span, "Unclosed string literal."))
                .emit();
        }
        self.push(
            TokenKind::Literal(Literal::String(Arc::new(value))),
            start,
            self.pos,
        );
    }

    fn lex_ident(&mut self, start: usize) {
        while self.peek(0).is_some_and(is_ident_continue) {
            self.pos += 1;
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        let kind = match text.as_str() {
            "true" => TokenKind::Literal(Literal::Boolean(true)),
            "false" => TokenKind::Literal(Literal::Boolean(false)),
            _ => TokenKind::Identifier(Str::Heap(Arc::from(text.as_str()))),
        };
        self.push(kind, start, self.pos);
    }
}

enum CommentVariety {
    Line,
    Doc(DocKind),
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

fn delimiter_kind(c: char) -> Option<TokenKind> {
    use DelimiterKind::*;
    let (kind, open) = match c {
        '(' => (Parenthesis, true),
        ')' => (Parenthesis, false),
        '[' => (Bracket, true),
        ']' => (Bracket, false),
        '{' => (Brace, true),
        '}' => (Brace, false),
        _ => return None,
    };
    Some(TokenKind::Delimiter(if open {
        DelimiterSymbol::Open(kind)
    } else {
        DelimiterSymbol::Close(kind)
    }))
}

/// Walks the produced token stream and sets the `is_composed` flag on every
/// punctuation that is immediately followed by another punctuation in the
/// source — i.e., with no whitespace, comment, or other token between them.
fn mark_composed(tokens: &mut [Token]) {
    for i in 0..tokens.len().saturating_sub(1) {
        if let TokenKind::Punctuation(sym) = &tokens[i].kind {
            // Only consider directly adjacent tokens (no trivia between).
            if tokens[i].end() == tokens[i + 1].start() {
                if let TokenKind::Punctuation(_) = &tokens[i + 1].kind {
                    let composed = PunctuationSymbol::new(sym.kind, true);
                    tokens[i].kind = TokenKind::Punctuation(composed);
                }
            }
        }
    }
}
