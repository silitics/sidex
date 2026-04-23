use std::fmt;

/// A part of a code fragment.
#[derive(Clone, Debug)]
pub enum Part {
    Literal(&'static str),
    Owned(String),
    Newline,
    Indent(usize),
    Dedent,
    Parts(Vec<Part>),
}

/// A code fragment that can be interpolated into templates.
#[derive(Clone, Debug, Default)]
pub struct Code {
    parts: Vec<Part>,
}

/// Trait for types that can be interpolated into a code template.
pub trait ToCode {
    fn to_code(&self) -> Code;
}

impl Code {
    /// Create an empty code fragment.
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    /// Appends a static literal.
    pub fn push_static(&mut self, s: &'static str) {
        self.parts.push(Part::Literal(s));
    }

    /// Appends an owned string.
    pub fn push_str(&mut self, s: &str) {
        self.parts.push(Part::Owned(s.to_owned()));
    }

    /// Appends a newline.
    pub fn push_newline(&mut self) {
        self.parts.push(Part::Newline);
    }

    /// Interpolates a Code value at a given column position.
    pub fn interpolate(&mut self, value: Code, column: usize) {
        if column > 0 {
            self.parts.push(Part::Indent(column));
            self.parts.push(Part::Parts(value.parts));
            self.parts.push(Part::Dedent);
        } else {
            self.parts.push(Part::Parts(value.parts));
        }
    }

    /// Joins items vertically (one per line at `column`), with `separator` after each except the last.
    pub fn join_vertical(
        &mut self,
        items: impl IntoIterator<Item = Code>,
        column: usize,
        separator: &str,
    ) {
        let items: Vec<Code> = items.into_iter().collect();
        let last = items.len().saturating_sub(1);
        self.parts.push(Part::Indent(column));
        for (i, item) in items.into_iter().enumerate() {
            if i > 0 {
                self.parts.push(Part::Newline);
            }
            self.parts.push(Part::Parts(item.parts));
            if !separator.is_empty() && i < last {
                self.parts.push(Part::Owned(separator.to_owned()));
            }
        }
        self.parts.push(Part::Dedent);
    }

    /// Joins items horizontally (inline), with `separator` between each.
    pub fn join_horizontal(&mut self, items: impl IntoIterator<Item = Code>, separator: &str) {
        let mut first = true;
        for item in items {
            if !first && !separator.is_empty() {
                self.parts.push(Part::Owned(separator.to_owned()));
            }
            first = false;
            self.parts.push(Part::Parts(item.parts));
        }
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut indent_stack = Vec::new();
        let mut pending_indent = false;
        render_parts(&self.parts, f, &mut indent_stack, &mut pending_indent)
    }
}

/// Renders parts to a writer. Indent is deferred: spaces are only emitted before
/// the next non-empty content after a newline, so blank lines stay clean.
fn render_parts<W: fmt::Write>(
    parts: &[Part],
    out: &mut W,
    indent_stack: &mut Vec<usize>,
    pending_indent: &mut bool,
) -> fmt::Result {
    for part in parts {
        match part {
            Part::Literal(s) => {
                if !s.is_empty() {
                    emit_pending_indent(out, indent_stack, pending_indent)?;
                    out.write_str(s)?;
                }
            }
            Part::Owned(s) => {
                if !s.is_empty() {
                    emit_pending_indent(out, indent_stack, pending_indent)?;
                    out.write_str(s)?;
                }
            }
            Part::Newline => {
                out.write_char('\n')?;
                *pending_indent = true;
            }
            Part::Indent(n) => indent_stack.push(*n),
            Part::Dedent => {
                indent_stack.pop();
            }
            Part::Parts(inner) => render_parts(inner, out, indent_stack, pending_indent)?,
        }
    }
    Ok(())
}

fn emit_pending_indent<W: fmt::Write>(
    out: &mut W,
    indent_stack: &[usize],
    pending_indent: &mut bool,
) -> fmt::Result {
    if *pending_indent {
        *pending_indent = false;
        let total: usize = indent_stack.iter().sum();
        for _ in 0..total {
            out.write_char(' ')?;
        }
    }
    Ok(())
}

impl From<String> for Code {
    fn from(s: String) -> Self {
        Self {
            parts: string_to_parts(&s),
        }
    }
}

impl From<&str> for Code {
    fn from(s: &str) -> Self {
        Self {
            parts: string_to_parts(s),
        }
    }
}

/// Splits a string on newlines into Parts.
fn string_to_parts(s: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    let mut lines = s.split('\n');
    if let Some(first) = lines.next() {
        if !first.is_empty() {
            parts.push(Part::Owned(first.to_owned()));
        }
    }
    for line in lines {
        parts.push(Part::Newline);
        if !line.is_empty() {
            parts.push(Part::Owned(line.to_owned()));
        }
    }
    parts
}

impl ToCode for Code {
    fn to_code(&self) -> Code {
        self.clone()
    }
}

impl<T: ToCode + ?Sized> ToCode for &T {
    fn to_code(&self) -> Code {
        (*self).to_code()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_single_line() {
        let mut code = Code::new();
        code.push_str("hello ");
        code.interpolate(Code::from("world"), 6);
        assert_eq!(code.to_string(), "hello world");
    }

    #[test]
    fn test_interpolate_multi_line() {
        let mut code = Code::new();
        code.push_str("def foo(");
        code.interpolate(Code::from("x: int,\ny: int,\nz: int"), 8);
        code.push_str("):");
        assert_eq!(
            code.to_string(),
            "def foo(x: int,\n        y: int,\n        z: int):"
        );
    }

    #[test]
    fn test_interpolate_empty() {
        let mut code = Code::new();
        code.push_str("hello ");
        code.interpolate(Code::from(""), 6);
        code.push_str("world");
        assert_eq!(code.to_string(), "hello world");
    }

    #[test]
    fn test_interpolate_with_blank_line() {
        let mut code = Code::new();
        code.interpolate(Code::from("a\n\nb"), 2);
        assert_eq!(code.to_string(), "a\n\n  b");
    }

    #[test]
    fn test_join_vertical_basic() {
        let mut code = Code::new();
        let items = vec![
            Code::from("x = 1"),
            Code::from("y = 2"),
            Code::from("return x + y"),
        ];
        code.join_vertical(items, 2, "");
        assert_eq!(code.to_string(), "x = 1\n  y = 2\n  return x + y");
    }

    #[test]
    fn test_join_vertical_with_separator() {
        let mut code = Code::new();
        let items = vec![
            Code::from("\"apple\""),
            Code::from("\"banana\""),
            Code::from("\"cherry\""),
        ];
        code.join_vertical(items, 2, ",");
        assert_eq!(code.to_string(), "\"apple\",\n  \"banana\",\n  \"cherry\"");
    }

    #[test]
    fn test_join_vertical_single_item() {
        let mut code = Code::new();
        let items = vec![Code::from("only")];
        code.join_vertical(items, 0, ",");
        assert_eq!(code.to_string(), "only");
    }

    #[test]
    fn test_join_vertical_empty() {
        let mut code = Code::new();
        let items: Vec<Code> = vec![];
        code.join_vertical(items, 4, ",");
        assert_eq!(code.to_string(), "");
    }

    #[test]
    fn test_join_horizontal() {
        let mut code = Code::new();
        let items = vec![
            Code::from("x: int"),
            Code::from("y: int"),
            Code::from("z: int"),
        ];
        code.join_horizontal(items, ", ");
        assert_eq!(code.to_string(), "x: int, y: int, z: int");
    }

    #[test]
    fn test_join_horizontal_empty() {
        let mut code = Code::new();
        let items: Vec<Code> = vec![];
        code.join_horizontal(items, ", ");
        assert_eq!(code.to_string(), "");
    }

    #[test]
    fn test_join_vertical_multi_line_items() {
        let mut code = Code::new();
        let items = vec![Code::from("if True:\n    return 1"), Code::from("x = 2")];
        code.join_vertical(items, 2, "");
        assert_eq!(code.to_string(), "if True:\n      return 1\n  x = 2");
    }

    #[test]
    fn test_nested_code() {
        let inner = Code::from("x = 1\ny = 2\nreturn x + y");
        let mut outer = Code::new();
        outer.push_str("def foo():");
        outer.push_newline();
        outer.interpolate(inner, 4);
        assert_eq!(
            outer.to_string(),
            "def foo():\n    x = 1\n    y = 2\n    return x + y"
        );
    }
}
