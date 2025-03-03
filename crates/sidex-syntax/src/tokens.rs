//! Lexer and representation of tokens.
//!
//! The lexer is based on [Chumsky](https://crates.io/crates/chumsky).
//!
//! 📌 **Note:** The lexer also produces tokens for comments which will later enable
//! automated formatting.

use std::{
    fmt::{self, Display, Write},
    hash::Hash,
    sync::Arc,
};

use chumsky::{Stream, prelude::*};
use sidex_diagnostics::{Diagnostic, Label};
use sidex_ir as ir;

use crate::span::Span;

/// A delimiter symbol like `(`, `[`, or `{`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DelimiterSymbol {
    /// An opening delimiter symbol.
    Open(DelimiterKind),
    /// A closing delimiter symbol.
    Close(DelimiterKind),
}

impl fmt::Display for DelimiterSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DelimiterSymbol::Open(kind) => f.write_str(kind.open()),
            DelimiterSymbol::Close(kind) => f.write_str(kind.close()),
        }
    }
}

/// A type of delimiter.
pub trait Delimiter {
    /// The kind of the delimiter.
    const KIND: DelimiterKind;

    /// The token kind of the opening delimiter.
    const OPEN: TokenKind;
    /// The token kind of the closing delimiter.
    const CLOSE: TokenKind;
}

/// Helper macro generating the lexer and data structures for delimiters.
macro_rules! gen_code_delimiters {
    ($( ( $name:ident , $open:literal , $close:literal , $doc:literal ) $(,)? )*) => {
        /// A kind of delimiters like `(`, `[`, or `{`.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum DelimiterKind {
            $(
                #[doc = $doc]
                $name,
            )*
        }

        impl DelimiterKind {
            /// The opening symbol.
            fn open(&self) -> &'static str {
                match self {
                    $(
                        Self::$name => $open,
                    )*
                }
            }

            /// The closing symbol.
            fn close(&self) -> &'static str {
                match self {
                    $(
                        Self::$name => $close,
                    )*
                }
            }
        }

        /// Delimiters like `(`, `[`, or `{`.
        pub mod delimiters {
            use super::*;
            $(
                #[doc = $doc]
                pub struct $name(());

                impl Delimiter for $name {
                    const KIND: DelimiterKind = DelimiterKind::$name;

                    const OPEN: TokenKind = TokenKind::Delimiter(DelimiterSymbol::Open(Self::KIND));
                    const CLOSE: TokenKind = TokenKind::Delimiter(DelimiterSymbol::Close(Self::KIND));
                }
            )*
        }

        /// Construct a lexer for delimiters.
        fn delimiter_lexer() -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
            choice((
                $(
                    just($open).to(delimiters::$name::OPEN),
                    just($close).to(delimiters::$name::CLOSE),
                )*
            ))
        }
    };
}

gen_code_delimiters![
    (
        Parenthesis,
        "(",
        ")",
        "A parenthesis delimiter, i.e., `(` or `)`."
    ),
    (Bracket, "[", "]", "A bracket delimiter, i.e., `[` or `]`."),
    (Brace, "{", "}", "A brace delimiter, i.e., `{` or `}`."),
];

/// A punctuation symbol like `+`, `.`, or `:`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PunctuationSymbol {
    /// The kind of punctuation.
    pub kind: PunctuationKind,

    /// Indicates whether the punctuation is followed by another punctuation.
    ///
    /// This is useful to parse composed operators such `>=`.
    pub is_composed: bool,
}

impl fmt::Display for PunctuationSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

/// A type of punctuation.
pub trait Punctuation {
    /// The kind of the punctuation.
    const KIND: PunctuationKind;

    /// The standalone version of the punctuation.
    const ALONE: TokenKind;
    /// The punctuation followed by another punctuation.
    const COMPOSED: TokenKind;
}

impl PunctuationSymbol {
    /// Creates a new punctuation symbol.
    const fn new(kind: PunctuationKind, is_composed: bool) -> Self {
        Self { kind, is_composed }
    }
}

/// Helper macro generating the lexer and data structures for punctuations.
macro_rules! gen_code_punctuations {
    ($( ( $name:ident , $symbol:literal, $doc:literal ) $(,)? )*) => {
        /// A kind of punctuation like `+`, `.`, or `:`.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum PunctuationKind {
            $(
                #[doc = $doc]
                $name,
            )*
        }

        impl fmt::Display for PunctuationKind {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Self::$name => f.write_str($symbol),
                    )*
                }
            }
        }

        /// Punctuations like `+`, `.`, or `:`.
        pub mod punctuations {
            use super::*;
            $(
                #[doc = $doc]
                pub struct $name(());

                impl Punctuation for $name {
                    const KIND: PunctuationKind = PunctuationKind::$name;

                    const ALONE: TokenKind = TokenKind::Punctuation(PunctuationSymbol::new(Self::KIND, false));
                    const COMPOSED: TokenKind = TokenKind::Punctuation(PunctuationSymbol::new(Self::KIND, true));
                }
            )*
        }

        /// Construct a lexer for punctuations.
        fn punctuation_lexer() -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
            let kind = choice((
                $(
                    just($symbol).to(PunctuationKind::$name),
                )*
            ));
            kind
                .clone()
                .then(kind.or_not().rewind())
                .map(|(kind, next)| {
                    TokenKind::Punctuation(PunctuationSymbol::new(kind, next.is_some()))
                })
        }
    };
}

gen_code_punctuations![
    (Plus, "+", "A plus `+`."),
    (Minus, "-", "A minus `-`."),
    (Percent, "%", "A percent `%`."),
    (Slash, "/", "A slash `/`."),
    (Asterisk, "*", "An asterisk `*`."),
    (Hash, "#", "A hash `#`."),
    (Dot, ".", "A dot `.`."),
    (Comma, ",", "A comma `,`."),
    (Semicolon, ";", "A semicolon `;`."),
    (Dollar, "$", "A dollar `$`."),
    (Colon, ":", "A colon `:`."),
    (Equals, "=", "An equals sign `=`."),
    (Circumflex, "^", "A circumflex `^`."),
    (Ampersand, "&", "An ampersand `&`."),
    (ExclamationMark, "!", "An exclamation mark `!`."),
    (QuestionMark, "?", "A question mark `?`."),
    (AngleOpen, "<", "An opening angle `<`."),
    (AngleClose, ">", "A closing angle `>`."),
];

/// Keywords like `opaque` or `variant`.
pub mod keywords {
    use super::*;

    /// Turns a `&'static str` keyword into [`TokenKind::Identifier`].
    const fn keyword(keyword: &'static str) -> TokenKind {
        TokenKind::Identifier(Str::Static(keyword))
    }

    /// Helper macro for defining keywords.
    macro_rules! define {
        ($( ( $name:ident , $keyword:literal , $doc:literal ) $(,)? )*) => {
            $(
                #[doc = $doc]
                pub const $name: TokenKind = keyword($keyword);
            )*
        }
    }

    define![
        (ALIAS, "alias", "The `alias` keyword."),
        (OPAQUE, "opaque", "The `opaque` keyword."),
        (RECORD, "record", "The `record` keyword."),
        (VARIANT, "variant", "The `variant` keyword."),
        (WRAPPER, "wrapper", "The `wrapper` keyword."),
        (DERIVED, "derived", "The `derived` keyword."),
        (FUN, "fun", "The `fun` keyword."),
        (INTERFACE, "interface", "The `interface` keyword."),
        (IMPORT, "import", "The `import` keyword."),
    ];
}

/// Indicates the type of a comment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommentKind {
    /// The comment is a line comment.
    Line,
    /// The comment is a block comment.
    Block,
}

/// Indicates the type of documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DocKind {
    /// The documentation is inline.
    Inline,
    /// The documentation is preceding.
    Preceding,
}

/// A literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    /// An numeric literal.
    Numeric {
        /// Whether the literal starts with a minus.
        has_minus: bool,
        /// The integral part of the number.
        integral: Arc<String>,
        /// The optional fractional part of the number.
        fractional: Option<Arc<String>>,
    },
    /// A string literal.
    String(Arc<String>),
    /// A boolean literal.
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Numeric {
                has_minus,
                integral,
                fractional,
            } => {
                if *has_minus {
                    f.write_char('-')?;
                }
                f.write_str(integral)?;
                if let Some(fractional) = fractional {
                    f.write_char('.')?;
                    f.write_str(fractional)?;
                }
                Ok(())
            }
            Literal::String(string) => write!(f, "\"{}\"", string.as_str()),
            Literal::Boolean(boolean) => f.write_str(if *boolean { "true" } else { "false" }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Str {
    /// A heap-allocated identifier.
    Heap(Arc<str>),
    /// A statically-allocated identifier.
    Static(&'static str),
}

impl Str {
    pub fn as_str(&self) -> &str {
        match self {
            Str::Heap(ident) => ident.as_ref(),
            Str::Static(ident) => *ident,
        }
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Str {}

impl Hash for Str {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

/// A kind of token.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// A delimiter token.
    Delimiter(DelimiterSymbol),
    /// A punctuation token.
    Punctuation(PunctuationSymbol),
    /// A literal token.
    Literal(Literal),
    /// An identifier token.
    Identifier(Str),
    /// A comment token.
    Comment {
        comment: Arc<String>,
        kind: CommentKind,
    },
    /// A documentation token.
    Doc { doc: Arc<String>, kind: DocKind },
}

impl TokenKind {
    /// Indicates whether the token kind is not a delimiter.
    pub fn is_not_delimiter(&self) -> bool {
        !matches!(self, TokenKind::Delimiter(_))
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Delimiter(delimiter) => delimiter.fmt(f),
            TokenKind::Punctuation(punctuation) => punctuation.fmt(f),
            TokenKind::Literal(literal) => literal.fmt(f),
            TokenKind::Identifier(identifier) => f.write_str(identifier.as_str()),
            TokenKind::Comment { comment, kind } => {
                match kind {
                    CommentKind::Line => {
                        write!(f, "//{comment}\n")
                    }
                    CommentKind::Block => {
                        write!(f, "/*{comment}*/")
                    }
                }
            }
            TokenKind::Doc { doc, kind } => {
                match kind {
                    DocKind::Inline => {
                        write!(f, "//!{doc}\n")
                    }
                    DocKind::Preceding => {
                        write!(f, "///{doc}\n")
                    }
                }
            }
        }
    }
}

/// A token.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The span of the token.
    pub(crate) span: Span,
}

impl Token {
    /// The id of the source the token originates from.
    pub fn src(&self) -> &ir::SourceIdx {
        &self.span.0.src
    }

    /// The start position of the token.
    pub fn start(&self) -> usize {
        self.span.start()
    }

    /// The end position of the token.
    pub fn end(&self) -> usize {
        self.span.end()
    }

    /// Token is separated by a space from its successor.
    pub fn is_separated(&self) -> bool {
        match self.kind {
            TokenKind::Punctuation(symbol) => !symbol.is_composed,
            _ => true,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

/// Construct a lexer for identifiers and keywords.
fn identifier_lexer() -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
    text::ident()
        .map(<Arc<str>>::from)
        .map(Str::Heap)
        .map(TokenKind::Identifier)
}

/// Construct a lexer for comments.
fn comment_lexer<O>(
    start: &'static str,
    until: impl Parser<char, O, Error = Simple<char, Span>>,
    kind: CommentKind,
) -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
    just(start)
        .ignore_then(take_until(until))
        .map(move |(comment, _)| {
            TokenKind::Comment {
                comment: Arc::new(comment.into_iter().collect()),
                kind,
            }
        })
}

/// Construct a lexer for documentation.
fn doc_lexer(
    start: &'static str,
    kind: DocKind,
) -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
    just(start)
        .ignore_then(take_until(choice((text::newline(), end()))))
        .map(move |(doc, _)| {
            TokenKind::Doc {
                doc: Arc::new(doc.into_iter().collect()),
                kind,
            }
        })
}

/// Construct a lexer for string literals.
fn literal_string_lexer() -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
    just('"')
        .ignore_then(
            filter(|c| *c != '\\' && *c != '"' && *c != '\n')
                .or(
                    // Escape sequences.
                    just('\\').ignore_then(choice((just('\\'), just('"')))),
                )
                .repeated(),
        )
        .then_ignore(just('"'))
        .collect::<String>()
        .map(|string| TokenKind::Literal(Literal::String(Arc::new(string))))
}

/// Construct a lexer for numeric literals.
fn literal_numeric_lexer() -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
    // 🔮 In the future, we may want to support integers with other bases than 10.
    just("-")
        .or_not()
        .then(text::int(10))
        .then(just(".").ignore_then(text::int(10)).or_not())
        .map(|((minus, integral), fractional)| {
            TokenKind::Literal(Literal::Numeric {
                has_minus: minus.is_some(),
                integral: Arc::new(integral),
                fractional: fractional.map(Arc::new),
            })
        })
}

/// Construct a lexer for literals.
fn literal_lexer() -> impl Parser<char, TokenKind, Error = Simple<char, Span>> {
    choice((
        just("true").to(TokenKind::Literal(Literal::Boolean(true))),
        just("false").to(TokenKind::Literal(Literal::Boolean(false))),
        literal_string_lexer(),
        literal_numeric_lexer(),
    ))
}

/// Construct a lexer for tokens by combining the other lexers.
fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char, Span>> {
    choice((
        delimiter_lexer(),
        doc_lexer("//!", DocKind::Inline),
        doc_lexer("///", DocKind::Preceding),
        comment_lexer("//", choice((text::newline(), end())), CommentKind::Line),
        comment_lexer("/*", just("*/"), CommentKind::Block),
        literal_lexer(),
        punctuation_lexer(),
        identifier_lexer(),
    ))
    .map_with_span(|kind, span| Token { kind, span })
    .padded()
    .repeated()
    .then_ignore(end())
}

pub(crate) fn diagnostic_from_error(error: Simple<String, Span>) -> Diagnostic {
    match error.reason() {
        chumsky::error::SimpleReason::Unclosed { span, delimiter } => {
            Diagnostic::error(format!("Unclosed delimiter {}.", delimiter))
                .with_span(Some(error.span().into()))
                .with_label(Label::new(
                    span.clone().into(),
                    format!("Unclosed delimiter {}.", delimiter),
                ))
                .with_label(Label::new(
                    error.span().into(),
                    format!(
                        "Must be closed before {}.",
                        error.found().unwrap_or(&"end of file".to_string())
                    ),
                ))
        }
        chumsky::error::SimpleReason::Unexpected => {
            Diagnostic::error(if error.found().is_some() {
                "Unexpected token in input.".to_owned()
            } else {
                "Unexpected end of input.".to_owned()
            })
            .with_span(Some(error.span().into()))
            .with_label(Label::new(
                error.span().into(),
                format!(
                    "Unexpected token {}",
                    error.found().unwrap_or(&"end of file".to_string())
                ),
            ))
        }
        chumsky::error::SimpleReason::Custom(msg) => {
            Diagnostic::error(msg).with_span(Some(error.span().into()))
        }
    }
}

/// Tokenize a source.
pub fn tokenize(source: &ir::Source) -> Option<Vec<Token>> {
    if let Some(text) = &source.text {
        let stream = Stream::from_iter(
            Span::from(source.end()),
            text.chars()
                .enumerate()
                .map(|(pos, c)| (c, source.span_at(pos).into())),
        );
        let (tokens, errors) = lexer().parse_recovery(stream);

        for error in errors {
            diagnostic_from_error(error.map(|c| c.to_string())).emit();
        }

        tokens
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_test_from_file {
        ($name:ident, $path:literal) => {
            #[test]
            fn $name() {
                let mut storage = ir::SourceStorage::new();
                let id = storage.insert(include_str!($path).to_owned(), None);
                let result = tokenize(&storage[id]);
                insta::assert_debug_snapshot!(stringify!($name), result);
            }
        };
    }

    make_test_from_file!(
        test_todo_list_api_manager,
        "../../../examples/todo-list/todo_list_api/schemas/manager.sidex"
    );

    make_test_from_file!(
        test_todo_list_data_ids,
        "../../../examples/todo-list/todo_list_data/schemas/ids.sidex"
    );

    make_test_from_file!(
        test_todo_list_data_person,
        "../../../examples/todo-list/todo_list_data/schemas/person.sidex"
    );

    make_test_from_file!(
        test_todo_list_data_task,
        "../../../examples/todo-list/todo_list_data/schemas/task.sidex"
    );
}
