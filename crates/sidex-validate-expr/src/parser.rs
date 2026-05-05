//! Parser for the validation expression DSL — turns a stream of [`Token`]s
//! into the [`Expr`] AST. Hand-written recursive descent; the grammar is
//! small enough not to need Pratt parsing.

use crate::ast::CmpOp;
use crate::ast::Expr;
use crate::ast::Literal;
use crate::lexer::LexError;
use crate::lexer::Spanned;
use crate::lexer::Token;
use crate::lexer::lex;

/// A parser error. Carries a span (within the body source) for diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        Self {
            message: e.message,
            start: e.start,
            end: e.end,
        }
    }
}

/// Parse a validation expression body. The input must already have outer
/// `{` / `}` stripped — what's stored in `TokensValue.text`.
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let tokens = lex(input)?;
    let mut p = Parser {
        tokens,
        pos: 0,
        end: input.len(),
    };
    let expr = p.parse_expr()?;
    if let Some(extra) = p.peek().cloned() {
        return Err(ParseError {
            message: "Unexpected trailing token.".to_owned(),
            start: extra.start,
            end: extra.end,
        });
    }
    Ok(expr)
}

struct Parser {
    tokens: Vec<Spanned>,
    pos: usize,
    end: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Spanned> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Spanned> {
        let tok = self.tokens.get(self.pos).cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, want: &Token, label: &str) -> Result<Spanned, ParseError> {
        match self.peek() {
            Some(tok) if &tok.token == want => Ok(self.bump().unwrap()),
            Some(other) => {
                Err(ParseError {
                    message: format!("Expected {label}."),
                    start: other.start,
                    end: other.end,
                })
            }
            None => {
                Err(ParseError {
                    message: format!("Expected {label}, found end of input."),
                    start: self.end,
                    end: self.end,
                })
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let first = self.parse_primary()?;
        // Comparison chain.
        if let Some(op) = self.peek_cmp_op() {
            let mut operands = vec![first];
            let mut ops = Vec::new();
            ops.push(op);
            self.bump();
            operands.push(self.parse_primary()?);
            while let Some(op) = self.peek_cmp_op() {
                ops.push(op);
                self.bump();
                operands.push(self.parse_primary()?);
            }
            return Ok(Expr::Compare { operands, ops });
        }
        Ok(first)
    }

    fn peek_cmp_op(&self) -> Option<CmpOp> {
        self.peek().and_then(|s| {
            match s.token {
                Token::Lt => Some(CmpOp::Lt),
                Token::Le => Some(CmpOp::Le),
                Token::EqEq => Some(CmpOp::Eq),
                Token::BangEq => Some(CmpOp::Ne),
                Token::Ge => Some(CmpOp::Ge),
                Token::Gt => Some(CmpOp::Gt),
                _ => None,
            }
        })
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let Some(tok) = self.peek().cloned() else {
            return Err(ParseError {
                message: "Expected an expression, found end of input.".to_owned(),
                start: self.end,
                end: self.end,
            });
        };
        match tok.token {
            Token::Underscore => {
                self.bump();
                self.parse_member_chain()
            }
            Token::Number(n) => {
                self.bump();
                Ok(Expr::Lit(Literal::Number(n)))
            }
            Token::String(s) => {
                self.bump();
                Ok(Expr::Lit(Literal::String(s)))
            }
            Token::Ident(name) => {
                self.bump();
                if matches!(self.peek().map(|s| &s.token), Some(Token::LParen)) {
                    self.parse_call(name)
                } else {
                    Err(ParseError {
                        message: format!(
                            "Bare identifier `{name}` is not a valid expression. \
                             Did you mean to call a function (e.g., `{name}(_)`)?"
                        ),
                        start: tok.start,
                        end: tok.end,
                    })
                }
            }
            _ => {
                Err(ParseError {
                    message: "Expected `_`, a literal, or a function call.".to_owned(),
                    start: tok.start,
                    end: tok.end,
                })
            }
        }
    }

    fn parse_member_chain(&mut self) -> Result<Expr, ParseError> {
        let mut path = Vec::new();
        while matches!(self.peek().map(|s| &s.token), Some(Token::Dot)) {
            self.bump();
            let next = self.bump().ok_or_else(|| {
                ParseError {
                    message: "Expected field name after `.`.".to_owned(),
                    start: self.end,
                    end: self.end,
                }
            })?;
            let Token::Ident(name) = next.token else {
                return Err(ParseError {
                    message: "Expected an identifier after `.`.".to_owned(),
                    start: next.start,
                    end: next.end,
                });
            };
            path.push(name);
        }
        Ok(if path.is_empty() {
            Expr::Underscore
        } else {
            Expr::Member { path }
        })
    }

    fn parse_call(&mut self, name: String) -> Result<Expr, ParseError> {
        self.expect(&Token::LParen, "`(`")?;
        let mut args = Vec::new();
        if !matches!(self.peek().map(|s| &s.token), Some(Token::RParen)) {
            args.push(self.parse_call_arg()?);
            while matches!(self.peek().map(|s| &s.token), Some(Token::Comma)) {
                self.bump();
                args.push(self.parse_call_arg()?);
            }
        }
        self.expect(&Token::RParen, "`)`")?;
        Ok(Expr::Call { name, args })
    }

    fn parse_call_arg(&mut self) -> Result<Expr, ParseError> {
        // For v1, call args are primaries (no nested comparisons). Keeps the
        // grammar honest and avoids ambiguity with `f(a < b, c)`.
        self.parse_primary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_underscore_alone() {
        assert_eq!(parse("_").unwrap(), Expr::Underscore);
    }

    #[test]
    fn parses_member_chain() {
        assert_eq!(
            parse("_.length").unwrap(),
            Expr::Member {
                path: vec!["length".into()],
            }
        );
        assert_eq!(
            parse("_.user.email").unwrap(),
            Expr::Member {
                path: vec!["user".into(), "email".into()],
            }
        );
    }

    #[test]
    fn parses_chained_comparison() {
        let parsed = parse("0 <= _ <= 100").unwrap();
        let Expr::Compare { operands, ops } = parsed else {
            panic!("expected a comparison");
        };
        assert_eq!(ops, vec![CmpOp::Le, CmpOp::Le]);
        assert_eq!(operands.len(), 3);
        assert!(matches!(operands[0], Expr::Lit(Literal::Number(_))));
        assert!(matches!(operands[1], Expr::Underscore));
        assert!(matches!(operands[2], Expr::Lit(Literal::Number(_))));
    }

    #[test]
    fn parses_matches_call() {
        let parsed = parse(r#"matches(_, "^[A-Z]+$")"#).unwrap();
        assert_eq!(
            parsed,
            Expr::Call {
                name: "matches".into(),
                args: vec![
                    Expr::Underscore,
                    Expr::Lit(Literal::String("^[A-Z]+$".into())),
                ],
            }
        );
    }

    #[test]
    fn parses_cross_field_predicate() {
        // `_.min <= _.max` is a comparison with two member-access operands.
        let parsed = parse("_.min <= _.max").unwrap();
        let Expr::Compare { operands, ops } = parsed else {
            panic!("expected a comparison");
        };
        assert_eq!(ops, vec![CmpOp::Le]);
        assert_eq!(operands.len(), 2);
    }

    #[test]
    fn rejects_bare_identifier() {
        let err = parse("foo").unwrap_err();
        assert!(err.message.contains("not a valid expression"));
    }

    #[test]
    fn rejects_trailing_garbage() {
        let err = parse("_ <= 5 6").unwrap_err();
        assert!(err.message.contains("trailing"));
    }
}
