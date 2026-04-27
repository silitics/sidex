//! Lexer output: tokens, token kinds, and the public [`tokenize`] entry point.
//!
//! The lexer is hand-written and lives in [`crate::lexer`]. This module
//! defines the data types that consumers see, plus the convenience function
//! used by the language server and other tools that only need a token stream.

use std::{
    fmt::{self, Display, Write},
    hash::Hash,
    sync::Arc,
};

use sidex_ir as ir;

use crate::lexer;

/// A delimiter symbol like `(`, `[`, or `{`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DelimiterSymbol {
    /// An opening delimiter.
    Open(DelimiterKind),
    /// A closing delimiter.
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

/// Trait describing a specific delimiter pair.
pub trait Delimiter {
    /// The kind of the delimiter.
    const KIND: DelimiterKind;
    /// The token kind of the opening delimiter.
    const OPEN: TokenKind;
    /// The token kind of the closing delimiter.
    const CLOSE: TokenKind;
}

macro_rules! gen_delimiters {
    ($( ( $name:ident , $open:literal , $close:literal , $doc:literal ) $(,)? )*) => {
        /// A kind of delimiter.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum DelimiterKind {
            $( #[doc = $doc] $name, )*
        }

        impl DelimiterKind {
            /// The opening character.
            pub fn open(&self) -> &'static str {
                match self { $( Self::$name => $open, )* }
            }
            /// The closing character.
            pub fn close(&self) -> &'static str {
                match self { $( Self::$name => $close, )* }
            }
        }

        /// Marker types for individual delimiters.
        pub mod delimiters {
            use super::*;
            $(
                #[doc = $doc]
                pub struct $name(());

                impl Delimiter for $name {
                    const KIND: DelimiterKind = DelimiterKind::$name;
                    const OPEN: TokenKind =
                        TokenKind::Delimiter(DelimiterSymbol::Open(Self::KIND));
                    const CLOSE: TokenKind =
                        TokenKind::Delimiter(DelimiterSymbol::Close(Self::KIND));
                }
            )*
        }
    };
}

gen_delimiters![
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
    /// Indicates whether the punctuation is followed by another punctuation
    /// without intervening whitespace. Used to parse composed operators
    /// such as `::` or `>=`.
    pub is_composed: bool,
}

impl PunctuationSymbol {
    pub(crate) const fn new(kind: PunctuationKind, is_composed: bool) -> Self {
        Self { kind, is_composed }
    }
}

impl fmt::Display for PunctuationSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

/// Trait describing a specific punctuation symbol.
pub trait Punctuation {
    /// The kind of the punctuation.
    const KIND: PunctuationKind;
    /// The standalone version.
    const ALONE: TokenKind;
    /// The composed version (followed by another punctuation).
    const COMPOSED: TokenKind;
}

macro_rules! gen_punctuations {
    ($( ( $name:ident , $symbol:literal, $doc:literal ) $(,)? )*) => {
        /// A kind of punctuation.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum PunctuationKind {
            $( #[doc = $doc] $name, )*
        }

        impl PunctuationKind {
            /// The single character of this punctuation.
            pub fn ch(&self) -> char {
                match self { $( Self::$name => $symbol.chars().next().unwrap(), )* }
            }
            /// Try to construct a punctuation kind from a character.
            pub fn from_char(c: char) -> Option<Self> {
                match c {
                    $( ch if ch == $symbol.chars().next().unwrap() => Some(Self::$name), )*
                    _ => None,
                }
            }
        }

        impl fmt::Display for PunctuationKind {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self { $( Self::$name => f.write_str($symbol), )* }
            }
        }

        /// Marker types for individual punctuation symbols.
        pub mod punctuations {
            use super::*;
            $(
                #[doc = $doc]
                pub struct $name(());

                impl Punctuation for $name {
                    const KIND: PunctuationKind = PunctuationKind::$name;
                    const ALONE: TokenKind =
                        TokenKind::Punctuation(PunctuationSymbol::new(Self::KIND, false));
                    const COMPOSED: TokenKind =
                        TokenKind::Punctuation(PunctuationSymbol::new(Self::KIND, true));
                }
            )*
        }
    };
}

gen_punctuations![
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

/// Keywords like `record`, `variant`, or `import`.
pub mod keywords {
    use super::*;

    const fn keyword(keyword: &'static str) -> TokenKind {
        TokenKind::Identifier(Str::Static(keyword))
    }

    macro_rules! define {
        ($( ( $name:ident , $keyword:literal , $doc:literal ) $(,)? )*) => {
            $( #[doc = $doc] pub const $name: TokenKind = keyword($keyword); )*

            /// Returns true if `s` is a recognised keyword.
            pub fn is_keyword(s: &str) -> bool {
                matches!(s, $( $keyword )|*)
            }
        }
    }

    define![
        (ALIAS, "alias", "The `alias` keyword."),
        (OPAQUE, "opaque", "The `opaque` keyword."),
        (RECORD, "record", "The `record` keyword."),
        (VARIANT, "variant", "The `variant` keyword."),
        (WRAPPER, "wrapper", "The `wrapper` keyword."),
        (IMPORT, "import", "The `import` keyword."),
    ];
}

/// Indicates the type of a comment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommentKind {
    /// A line comment, i.e., `// ...`.
    Line,
    /// A block comment, i.e., `/* ... */`.
    Block,
}

/// Indicates the type of documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DocKind {
    /// Inline documentation `//! ...`.
    Inline,
    /// Preceding documentation `/// ...`.
    Preceding,
}

/// A literal value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    /// A numeric literal.
    Numeric {
        /// Whether the literal starts with a minus.
        has_minus: bool,
        /// The integral part.
        integral: Arc<String>,
        /// Optional fractional part.
        fractional: Option<Arc<String>>,
    },
    /// A string literal (without surrounding quotes; escapes resolved).
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

/// A reference to a string that may be heap-allocated or static.
#[derive(Debug, Clone)]
pub enum Str {
    /// A heap-allocated string.
    Heap(Arc<str>),
    /// A statically-allocated string slice.
    Static(&'static str),
}

impl Str {
    /// Borrows the string as a `&str`.
    pub fn as_str(&self) -> &str {
        match self {
            Str::Heap(s) => s.as_ref(),
            Str::Static(s) => s,
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

/// The kind of a token.
///
/// All tokens carry their kind, including trivia (comments and docs).
/// Whitespace is represented by [`TokenKind::Whitespace`] when retained in the
/// CST, but is filtered out by the public [`tokenize`] function.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// A delimiter token.
    Delimiter(DelimiterSymbol),
    /// A punctuation token.
    Punctuation(PunctuationSymbol),
    /// A literal token.
    Literal(Literal),
    /// An identifier or keyword token.
    Identifier(Str),
    /// A comment token.
    Comment {
        /// The body of the comment, excluding the leading marker.
        comment: Arc<String>,
        /// The kind of comment.
        kind: CommentKind,
    },
    /// A documentation token.
    Doc {
        /// The body of the doc comment.
        doc: Arc<String>,
        /// The kind of doc.
        kind: DocKind,
    },
    /// Whitespace (spaces, tabs, newlines).
    Whitespace,
    /// An erroneous token (lexer recovery).
    Error,
}

impl TokenKind {
    /// True if the token is not a delimiter.
    pub fn is_not_delimiter(&self) -> bool {
        !matches!(self, TokenKind::Delimiter(_))
    }

    /// True if the token is trivia (whitespace or non-doc comment).
    pub fn is_trivia(&self) -> bool {
        matches!(self, TokenKind::Whitespace | TokenKind::Comment { .. })
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Delimiter(d) => d.fmt(f),
            TokenKind::Punctuation(p) => p.fmt(f),
            TokenKind::Literal(l) => l.fmt(f),
            TokenKind::Identifier(i) => f.write_str(i.as_str()),
            TokenKind::Comment { comment, kind } => {
                match kind {
                    CommentKind::Line => write!(f, "//{comment}\n"),
                    CommentKind::Block => write!(f, "/*{comment}*/"),
                }
            }
            TokenKind::Doc { doc, kind } => {
                match kind {
                    DocKind::Inline => write!(f, "//!{doc}\n"),
                    DocKind::Preceding => write!(f, "///{doc}\n"),
                }
            }
            TokenKind::Whitespace => Ok(()),
            TokenKind::Error => Ok(()),
        }
    }
}

/// A token, paired with its source span.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The span of the token.
    pub(crate) span: ir::Span,
}

impl Token {
    /// Creates a new token.
    pub(crate) fn new(kind: TokenKind, span: ir::Span) -> Self {
        Self { kind, span }
    }

    /// The span of the token.
    pub fn span(&self) -> &ir::Span {
        &self.span
    }

    /// The id of the source the token originates from.
    pub fn src(&self) -> &ir::SourceIdx {
        &self.span.src
    }

    /// The character offset where the token starts.
    pub fn start(&self) -> usize {
        self.span.start
    }

    /// The character offset just past the end of the token.
    pub fn end(&self) -> usize {
        self.span.end
    }

    /// Whether the token is separated from its successor by whitespace.
    ///
    /// Composed punctuation (`::`, `>=`, ...) returns `false` for the leading
    /// piece; everything else returns `true`.
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

/// Tokenize a source.
///
/// The returned vector contains tokens in source order excluding whitespace,
/// matching the legacy chumsky-based output. Comment and doc tokens are
/// preserved.
pub fn tokenize(source: &ir::Source) -> Option<Vec<Token>> {
    let text = source.text.as_ref()?;
    let mut tokens = lexer::lex(source.idx, text);
    tokens.retain(|t| !matches!(t.kind, TokenKind::Whitespace));
    Some(tokens)
}
