//! Lexer output: tokens and token kinds.
//!
//! The lexer is hand-written and lives in [`crate::lexer`]. This module
//! defines the data types that consumers of the lexer and parser see.

use std::{
    fmt::{self, Display, Write},
    sync::Arc,
};

use sidex_ir as ir;

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

macro_rules! gen_punctuations {
    ($( ( $name:ident , $symbol:literal, $doc:literal ) $(,)? )*) => {
        /// A kind of punctuation.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum PunctuationKind {
            $( #[doc = $doc] $name, )*
        }

        impl PunctuationKind {
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

/// The kind of a token.
///
/// All tokens carry their kind, including trivia (comments, docs, and
/// whitespace) which are retained by the lossless CST.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// A delimiter token.
    Delimiter(DelimiterSymbol),
    /// A punctuation token.
    Punctuation(PunctuationSymbol),
    /// A literal token.
    Literal(Literal),
    /// An identifier or keyword token.
    Identifier(Arc<str>),
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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Delimiter(d) => d.fmt(f),
            TokenKind::Punctuation(p) => p.fmt(f),
            TokenKind::Literal(l) => l.fmt(f),
            TokenKind::Identifier(i) => f.write_str(i),
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
