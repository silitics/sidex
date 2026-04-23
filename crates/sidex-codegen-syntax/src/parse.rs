use crate::ast::{Fragment, IterMode, Template};

/// A template parse error.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parses a template string into a [`Template`].
pub fn parse(input: &str) -> Result<Template, ParseError> {
    let dedented = dedent(input);
    let mut parser = Parser::new(&dedented);
    let fragments = parser.parse_fragments(false)?;
    Ok(Template { fragments })
}

/// Strips leading/trailing blank lines and common indentation.
fn dedent(input: &str) -> String {
    let mut text = input;

    // Strip leading newline (the newline right after the opening `"`).
    if text.starts_with('\n') {
        text = &text[1..];
    } else if text.starts_with("\r\n") {
        text = &text[2..];
    }

    // Strip trailing whitespace-only line (the indentation before the closing `"`).
    if let Some(last_newline) = text.rfind('\n') {
        let after = &text[last_newline + 1..];
        if after.chars().all(|c| c == ' ' || c == '\t') {
            text = &text[..last_newline];
        }
    }

    if text.is_empty() {
        return String::new();
    }

    // Compute common indent: minimum leading whitespace of non-empty lines.
    let common_indent = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    // Strip common indent from each line and rejoin.
    text.lines()
        .map(|line| {
            if line.len() >= common_indent {
                &line[common_indent..]
            } else {
                // Empty or whitespace-only line shorter than common indent.
                ""
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    column: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            column: 0,
        }
    }

    fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.column = 0;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            message: message.into(),
            position: self.pos,
        }
    }

    fn parse_fragments(&mut self, in_iteration: bool) -> Result<Vec<Fragment>, ParseError> {
        let mut fragments = Vec::new();
        let mut literal = String::new();

        while let Some(ch) = self.peek() {
            if in_iteration && ch == ')' {
                break;
            }

            if ch == '@' {
                if !literal.is_empty() {
                    fragments.push(Fragment::Literal(std::mem::take(&mut literal)));
                }
                fragments.push(self.parse_at()?);
            } else if ch == '\n' {
                if !literal.is_empty() {
                    fragments.push(Fragment::Literal(std::mem::take(&mut literal)));
                }
                self.advance();
                fragments.push(Fragment::Newline);
            } else {
                literal.push(ch);
                self.advance();
            }
        }

        if !literal.is_empty() {
            fragments.push(Fragment::Literal(literal));
        }

        Ok(fragments)
    }

    fn parse_at(&mut self) -> Result<Fragment, ParseError> {
        let at_column = self.column;
        let at_pos = self.pos;
        self.advance(); // consume '@'

        match self.peek() {
            Some('@') => {
                self.advance();
                Ok(Fragment::Literal("@".to_owned()))
            }
            Some('(') => {
                // Iteration block.
                self.advance(); // consume '('
                let saved_column = self.column;
                self.column = 0;
                let body = self.parse_fragments(true)?;
                self.column = saved_column;
                if self.peek() != Some(')') {
                    return Err(self.error("unclosed `@(`"));
                }
                self.advance(); // consume ')'

                // Parse separator (everything between ')' and '*' or '+').
                let mut separator = String::new();
                loop {
                    match self.peek() {
                        Some('*') => {
                            self.advance();
                            return Ok(Fragment::Iteration {
                                body,
                                mode: IterMode::Vertical,
                                separator,
                                column: at_column,
                            });
                        }
                        Some('+') => {
                            self.advance();
                            return Ok(Fragment::Iteration {
                                body,
                                mode: IterMode::Horizontal,
                                separator,
                                column: at_column,
                            });
                        }
                        Some(ch) => {
                            separator.push(ch);
                            self.advance();
                        }
                        None => {
                            return Err(self.error("expected `*` or `+` after `@(...)`"));
                        }
                    }
                }
            }
            Some(ch) if ch == '_' || ch.is_ascii_alphabetic() => {
                let var = self.parse_ident();
                Ok(Fragment::Interpolation {
                    var,
                    column: at_column,
                })
            }
            _ => {
                Err(ParseError {
                    message: "expected identifier, `(`, or `@` after `@`".to_owned(),
                    position: at_pos,
                })
            }
        }
    }

    fn parse_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch == '_' || ch.is_ascii_alphanumeric() {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Fragment, IterMode};

    #[test]
    fn test_dedent_basic() {
        let input = "\n    hello\n    world\n  ";
        let result = dedent(input);
        assert_eq!(result, "hello\nworld");
    }

    #[test]
    fn test_dedent_preserves_relative_indent() {
        let input = "\n    hello\n      indented\n    back\n  ";
        let result = dedent(input);
        assert_eq!(result, "hello\n  indented\nback");
    }

    #[test]
    fn test_dedent_blank_lines() {
        let input = "\n    hello\n\n    world\n  ";
        let result = dedent(input);
        assert_eq!(result, "hello\n\nworld");
    }

    #[test]
    fn test_parse_literal() {
        let t = parse("hello world").unwrap();
        assert_eq!(t.fragments.len(), 1);
        assert!(matches!(&t.fragments[0], Fragment::Literal(s) if s == "hello world"));
    }

    #[test]
    fn test_parse_interpolation() {
        let t = parse("hello @name!").unwrap();
        assert_eq!(t.fragments.len(), 3);
        assert!(matches!(&t.fragments[0], Fragment::Literal(s) if s == "hello "));
        assert!(
            matches!(&t.fragments[1], Fragment::Interpolation { var, column } if var == "name" && *column == 6)
        );
        assert!(matches!(&t.fragments[2], Fragment::Literal(s) if s == "!"));
    }

    #[test]
    fn test_parse_escaped_at() {
        let t = parse("costs @@5").unwrap();
        assert_eq!(t.fragments.len(), 3);
        assert!(matches!(&t.fragments[0], Fragment::Literal(s) if s == "costs "));
        assert!(matches!(&t.fragments[1], Fragment::Literal(s) if s == "@"));
        assert!(matches!(&t.fragments[2], Fragment::Literal(s) if s == "5"));
    }

    #[test]
    fn test_parse_vertical_iteration() {
        let t = parse("@(@items)*").unwrap();
        assert_eq!(t.fragments.len(), 1);
        match &t.fragments[0] {
            Fragment::Iteration {
                body,
                mode,
                separator,
                column,
            } => {
                assert_eq!(*mode, IterMode::Vertical);
                assert_eq!(separator, "");
                assert_eq!(*column, 0);
                assert_eq!(body.len(), 1);
                assert!(matches!(&body[0], Fragment::Interpolation { var, .. } if var == "items"));
            }
            _ => panic!("expected iteration"),
        }
    }

    #[test]
    fn test_parse_vertical_with_separator() {
        let t = parse("@(@items),*").unwrap();
        match &t.fragments[0] {
            Fragment::Iteration {
                mode, separator, ..
            } => {
                assert_eq!(*mode, IterMode::Vertical);
                assert_eq!(separator, ",");
            }
            _ => panic!("expected iteration"),
        }
    }

    #[test]
    fn test_parse_horizontal_iteration() {
        let t = parse("@(@args), +").unwrap();
        match &t.fragments[0] {
            Fragment::Iteration {
                mode, separator, ..
            } => {
                assert_eq!(*mode, IterMode::Horizontal);
                assert_eq!(separator, ", ");
            }
            _ => panic!("expected iteration"),
        }
    }

    #[test]
    fn test_parse_newlines() {
        let t = parse("a\nb").unwrap();
        assert_eq!(t.fragments.len(), 3);
        assert!(matches!(&t.fragments[0], Fragment::Literal(s) if s == "a"));
        assert!(matches!(&t.fragments[1], Fragment::Newline));
        assert!(matches!(&t.fragments[2], Fragment::Literal(s) if s == "b"));
    }

    #[test]
    fn test_parse_multiline_template() {
        let t = parse("\n  def foo(@args):\n    @body\n").unwrap();
        // After dedent: "def foo(@args):\n  @body"
        let frags = &t.fragments;
        assert!(matches!(&frags[0], Fragment::Literal(s) if s == "def foo("));
        assert!(
            matches!(&frags[1], Fragment::Interpolation { var, column } if var == "args" && *column == 8)
        );
        assert!(matches!(&frags[2], Fragment::Literal(s) if s == "):"));
        assert!(matches!(&frags[3], Fragment::Newline));
        assert!(matches!(&frags[4], Fragment::Literal(s) if s == "  "));
        assert!(
            matches!(&frags[5], Fragment::Interpolation { var, column } if var == "body" && *column == 2)
        );
    }

    #[test]
    fn test_parse_iteration_column() {
        let t = parse("\n    class Foo:\n      @(@methods)*\n  ").unwrap();
        // After dedent: "class Foo:\n  @(@methods)*"
        let frags = &t.fragments;
        match &frags[3] {
            Fragment::Iteration { column, .. } => assert_eq!(*column, 2),
            _ => panic!("expected iteration at index 3"),
        }
    }

    #[test]
    fn test_parse_iteration_with_body_pattern() {
        let t = parse("@(@name: @typ)*").unwrap();
        match &t.fragments[0] {
            Fragment::Iteration { body, .. } => {
                assert_eq!(body.len(), 3);
                assert!(matches!(&body[0], Fragment::Interpolation { var, .. } if var == "name"));
                assert!(matches!(&body[1], Fragment::Literal(s) if s == ": "));
                assert!(matches!(&body[2], Fragment::Interpolation { var, .. } if var == "typ"));
            }
            _ => panic!("expected iteration"),
        }
    }

    #[test]
    fn test_parse_error_unclosed_iteration() {
        let result = parse("@(@items");
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("unclosed"));
    }

    #[test]
    fn test_parse_error_missing_mode() {
        let result = parse("@(@items)");
        assert!(result.is_err());
    }
}
