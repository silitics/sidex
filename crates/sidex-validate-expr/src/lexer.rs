//! A tiny lexer for the validation expression DSL.
//!
//! Independent of `sidex-syntax` — the validate plugin captures the body
//! text from the IR and re-tokenizes it locally, both because the body
//! is whitespace-collapsed lex-equivalent text (not byte-verbatim) and
//! because the DSL uses operators (`==`, `!=`, …) that aren't first-class
//! Sidex tokens.

use std::iter::Peekable;
use std::str::CharIndices;

/// Tokens emitted by the DSL lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// `_`.
    Underscore,
    /// An identifier — letters/digits/underscores, can't start with a digit.
    Ident(String),
    /// A numeric literal. Stored as a string to preserve the user's
    /// spelling (sign, integer/fractional).
    Number(String),
    /// A string literal. The lexer resolves `\\` and `\"` escapes; other
    /// escapes (e.g. `\n`) are passed through verbatim — the DSL is small
    /// enough not to need full string-escape semantics.
    String(String),
    /// `(`.
    LParen,
    /// `)`.
    RParen,
    /// `,`.
    Comma,
    /// `.`.
    Dot,
    /// One of the comparison operators.
    Lt,
    Le,
    Gt,
    Ge,
    EqEq,
    BangEq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned {
    pub token: Token,
    /// Byte offset within the source body where the token starts.
    pub start: usize,
    /// Byte offset within the source body where the token ends (exclusive).
    pub end: usize,
}

/// A lexer error. Carries enough information for the parser to produce a
/// `Diagnostic` with a span pointing at the offending byte range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

/// Lex `input` into a vector of spanned tokens. Whitespace is dropped.
pub fn lex(input: &str) -> Result<Vec<Spanned>, LexError> {
    let mut out = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some(&(idx, ch)) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch.is_ascii_digit() || (ch == '-' && peek_is_digit(&mut chars.clone(), idx)) {
            out.push(lex_number(&mut chars, input)?);
            continue;
        }
        if ch == '_' || ch.is_ascii_alphabetic() {
            out.push(lex_ident_or_underscore(&mut chars, input));
            continue;
        }
        if ch == '"' {
            out.push(lex_string(&mut chars)?);
            continue;
        }
        // Operators / punctuation.
        let single = match ch {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            _ => None,
        };
        if let Some(tok) = single {
            chars.next();
            out.push(Spanned {
                token: tok,
                start: idx,
                end: idx + ch.len_utf8(),
            });
            continue;
        }
        // Multi-char operators.
        if let Some(tok) = lex_operator(&mut chars, input)? {
            out.push(tok);
            continue;
        }
        return Err(LexError {
            message: format!("Unexpected character `{ch}`."),
            start: idx,
            end: idx + ch.len_utf8(),
        });
    }
    Ok(out)
}

fn peek_is_digit(chars: &mut Peekable<CharIndices<'_>>, sign_idx: usize) -> bool {
    chars.next();
    matches!(chars.peek(), Some(&(idx, c)) if idx > sign_idx && c.is_ascii_digit())
}

fn lex_number(chars: &mut Peekable<CharIndices<'_>>, input: &str) -> Result<Spanned, LexError> {
    let start = chars.peek().map(|&(i, _)| i).unwrap();
    if chars.peek().map(|&(_, c)| c) == Some('-') {
        chars.next();
    }
    while let Some(&(_, c)) = chars.peek() {
        if c.is_ascii_digit() {
            chars.next();
        } else {
            break;
        }
    }
    if chars.peek().map(|&(_, c)| c) == Some('.') {
        // Need to distinguish "1.5" (number) from "_.length" (member access on
        // `_`); we only consume the dot if it's followed by a digit.
        let mut peek = chars.clone();
        peek.next();
        if matches!(peek.peek(), Some(&(_, c)) if c.is_ascii_digit()) {
            chars.next(); // consume dot
            while let Some(&(_, c)) = chars.peek() {
                if c.is_ascii_digit() {
                    chars.next();
                } else {
                    break;
                }
            }
        }
    }
    let end = chars.peek().map(|&(i, _)| i).unwrap_or(input.len());
    Ok(Spanned {
        token: Token::Number(input[start..end].to_owned()),
        start,
        end,
    })
}

fn lex_ident_or_underscore(chars: &mut Peekable<CharIndices<'_>>, input: &str) -> Spanned {
    let (start, _) = chars.peek().copied().unwrap();
    while let Some(&(_, c)) = chars.peek() {
        if c == '_' || c.is_ascii_alphanumeric() {
            chars.next();
        } else {
            break;
        }
    }
    let end = chars.peek().map(|&(i, _)| i).unwrap_or(input.len());
    let text = &input[start..end];
    let token = if text == "_" {
        Token::Underscore
    } else {
        Token::Ident(text.to_owned())
    };
    Spanned { token, start, end }
}

fn lex_string(chars: &mut Peekable<CharIndices<'_>>) -> Result<Spanned, LexError> {
    let (start, _) = chars.next().unwrap(); // consume opening "
    let mut value = String::new();
    loop {
        let Some(&(idx, ch)) = chars.peek() else {
            return Err(LexError {
                message: "Unterminated string literal.".to_owned(),
                start,
                end: start + 1,
            });
        };
        chars.next();
        let end_after = idx + ch.len_utf8();
        match ch {
            '"' => {
                return Ok(Spanned {
                    token: Token::String(value),
                    start,
                    end: end_after,
                });
            }
            '\\' => {
                if let Some(&(_, esc)) = chars.peek() {
                    chars.next();
                    match esc {
                        '"' => value.push('"'),
                        '\\' => value.push('\\'),
                        // Pass through other escape sequences verbatim — the
                        // regex engine and other consumers re-interpret them.
                        other => {
                            value.push('\\');
                            value.push(other);
                        }
                    }
                }
            }
            _ => value.push(ch),
        }
    }
}

fn lex_operator(
    chars: &mut Peekable<CharIndices<'_>>,
    _input: &str,
) -> Result<Option<Spanned>, LexError> {
    let (start, ch) = match chars.peek() {
        Some(&(i, c)) => (i, c),
        None => return Ok(None),
    };
    match ch {
        '<' => {
            chars.next();
            if matches!(chars.peek(), Some(&(_, '='))) {
                chars.next();
                Ok(Some(Spanned {
                    token: Token::Le,
                    start,
                    end: start + 2,
                }))
            } else {
                Ok(Some(Spanned {
                    token: Token::Lt,
                    start,
                    end: start + 1,
                }))
            }
        }
        '>' => {
            chars.next();
            if matches!(chars.peek(), Some(&(_, '='))) {
                chars.next();
                Ok(Some(Spanned {
                    token: Token::Ge,
                    start,
                    end: start + 2,
                }))
            } else {
                Ok(Some(Spanned {
                    token: Token::Gt,
                    start,
                    end: start + 1,
                }))
            }
        }
        '=' => {
            chars.next();
            if matches!(chars.peek(), Some(&(_, '='))) {
                chars.next();
                Ok(Some(Spanned {
                    token: Token::EqEq,
                    start,
                    end: start + 2,
                }))
            } else {
                Err(LexError {
                    message: "Expected `==` (single `=` is not valid).".to_owned(),
                    start,
                    end: start + 1,
                })
            }
        }
        '!' => {
            chars.next();
            if matches!(chars.peek(), Some(&(_, '='))) {
                chars.next();
                Ok(Some(Spanned {
                    token: Token::BangEq,
                    start,
                    end: start + 2,
                }))
            } else {
                Err(LexError {
                    message: "Expected `!=`.".to_owned(),
                    start,
                    end: start + 1,
                })
            }
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(input: &str) -> Vec<Token> {
        lex(input).unwrap().into_iter().map(|s| s.token).collect()
    }

    #[test]
    fn lexes_basic_comparison() {
        assert_eq!(
            kinds("0 <= _ <= 100"),
            vec![
                Token::Number("0".into()),
                Token::Le,
                Token::Underscore,
                Token::Le,
                Token::Number("100".into()),
            ]
        );
    }

    #[test]
    fn lexes_member_access() {
        assert_eq!(
            kinds("_.length"),
            vec![Token::Underscore, Token::Dot, Token::Ident("length".into())]
        );
        assert_eq!(
            kinds("_ . length"),
            vec![Token::Underscore, Token::Dot, Token::Ident("length".into())]
        );
    }

    #[test]
    fn lexes_call_with_string() {
        assert_eq!(
            kinds(r#"matches(_, "^[A-Z]+$")"#),
            vec![
                Token::Ident("matches".into()),
                Token::LParen,
                Token::Underscore,
                Token::Comma,
                Token::String("^[A-Z]+$".into()),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn lexes_negative_and_decimal_numbers() {
        assert_eq!(kinds("-5"), vec![Token::Number("-5".into())]);
        assert_eq!(kinds("3.14"), vec![Token::Number("3.14".into())]);
    }

    #[test]
    fn lexes_eq_and_neq() {
        assert_eq!(
            kinds("_ == 5"),
            vec![Token::Underscore, Token::EqEq, Token::Number("5".into())]
        );
        assert_eq!(
            kinds("_ != 5"),
            vec![Token::Underscore, Token::BangEq, Token::Number("5".into())]
        );
    }
}
