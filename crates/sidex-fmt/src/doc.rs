//! Wadler/Lindig-style document IR.
//!
//! The convert pass turns a CST into a [`Doc`]. The render pass turns a
//! [`Doc`] into a string, picking flat or broken layout per [`Doc::Group`]
//! based on the available width.

use std::borrow::Cow;

/// A document — a structural description of formatted output.
#[derive(Debug, Clone)]
pub enum Doc {
    /// The empty document.
    Nil,
    /// A literal string fragment. Must not contain `\n`; use [`Doc::HardLine`]
    /// instead, or split into separate `Text` and `HardLine` parts.
    Text(Cow<'static, str>),
    /// A sequence of documents, rendered back-to-back.
    Concat(Vec<Doc>),
    /// Adds `n` columns of indentation to any line break inside `inner`.
    Nest(usize, Box<Doc>),
    /// A breakable space: `" "` when the enclosing group is flat, otherwise a
    /// newline followed by the current indent.
    Line,
    /// Always a newline followed by the current indent. Forces the enclosing
    /// group to break.
    HardLine,
    /// Empty when the enclosing group is flat, otherwise a newline followed
    /// by the current indent.
    SoftLine,
    /// Try to render `inner` flat (single line); fall back to broken layout
    /// if it does not fit within the configured width.
    Group(Box<Doc>),
}

impl Doc {
    /// Convenience: a static string.
    pub fn text(s: &'static str) -> Self {
        Doc::Text(Cow::Borrowed(s))
    }

    /// Convenience: an owned string.
    pub fn string(s: impl Into<String>) -> Self {
        Doc::Text(Cow::Owned(s.into()))
    }

    /// Concat helper.
    pub fn concat(parts: impl IntoIterator<Item = Doc>) -> Self {
        let parts: Vec<Doc> = parts.into_iter().filter(|d| !matches!(d, Doc::Nil)).collect();
        if parts.is_empty() {
            Doc::Nil
        } else if parts.len() == 1 {
            parts.into_iter().next().unwrap()
        } else {
            Doc::Concat(parts)
        }
    }

    /// Group helper.
    pub fn group(self) -> Self {
        Doc::Group(Box::new(self))
    }

    /// Nest helper.
    pub fn nest(self, n: usize) -> Self {
        Doc::Nest(n, Box::new(self))
    }
}

/// Layout options for rendering.
#[derive(Debug, Clone)]
pub struct LayoutOptions {
    /// Maximum line width before a [`Doc::Group`] breaks.
    pub max_width: usize,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self { max_width: 80 }
    }
}

/// Renders a document into a string.
pub fn render(doc: &Doc, opts: &LayoutOptions) -> String {
    let mut out = String::new();
    let mut stack: Vec<(usize, Mode, &Doc)> = vec![(0, Mode::Break, doc)];
    let mut col: usize = 0;
    while let Some((indent, mode, d)) = stack.pop() {
        match d {
            Doc::Nil => {}
            Doc::Text(s) => {
                out.push_str(s);
                col += s.chars().count();
            }
            Doc::Concat(ds) => {
                for d in ds.iter().rev() {
                    stack.push((indent, mode, d));
                }
            }
            Doc::Nest(n, inner) => stack.push((indent + n, mode, inner)),
            Doc::Line => match mode {
                Mode::Flat => {
                    out.push(' ');
                    col += 1;
                }
                Mode::Break => {
                    out.push('\n');
                    push_indent(&mut out, indent);
                    col = indent;
                }
            },
            Doc::SoftLine => match mode {
                Mode::Flat => {}
                Mode::Break => {
                    out.push('\n');
                    push_indent(&mut out, indent);
                    col = indent;
                }
            },
            Doc::HardLine => {
                out.push('\n');
                push_indent(&mut out, indent);
                col = indent;
            }
            Doc::Group(inner) => {
                let mode = if fits(inner, opts.max_width.saturating_sub(col)) {
                    Mode::Flat
                } else {
                    Mode::Break
                };
                stack.push((indent, mode, inner));
            }
        }
    }
    out
}

fn push_indent(out: &mut String, n: usize) {
    for _ in 0..n {
        out.push(' ');
    }
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Flat,
    Break,
}

/// Returns true if the document fits in the given width when rendered flat.
fn fits(doc: &Doc, mut remaining: usize) -> bool {
    let mut stack: Vec<(Mode, &Doc)> = vec![(Mode::Flat, doc)];
    while let Some((mode, d)) = stack.pop() {
        match d {
            Doc::Nil => {}
            Doc::Text(s) => {
                let len = s.chars().count();
                if len > remaining {
                    return false;
                }
                remaining -= len;
            }
            Doc::Concat(ds) => {
                for d in ds.iter().rev() {
                    stack.push((mode, d));
                }
            }
            Doc::Nest(_, inner) => stack.push((mode, inner)),
            Doc::Line => match mode {
                Mode::Flat => {
                    if remaining == 0 {
                        return false;
                    }
                    remaining -= 1;
                }
                Mode::Break => return true,
            },
            Doc::SoftLine => match mode {
                Mode::Flat => {}
                Mode::Break => return true,
            },
            Doc::HardLine => return false,
            Doc::Group(inner) => stack.push((mode, inner)),
        }
    }
    true
}
