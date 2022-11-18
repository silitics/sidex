//! A thin wrapper around [`ir::Span`] implementing [`chumsky::Span`].

use std::ops::Range;

use sidex_ir as ir;

/// A thin wrapper around [`ir::Span`] implementing [`chumsky::Span`].
///
/// This is private because we implement [`chumsky::Span`] for it and we do not want
/// [`chumsky`] to become a public dependency.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Span(pub(crate) ir::Span);

impl Span {
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.0.start = range.start;
        self.0.end = range.end;
        self
    }
}

impl chumsky::Span for Span {
    type Context = ir::SourceIdx;

    type Offset = usize;

    fn new(context: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self(ir::Span::new(context, range.start, range.end))
    }

    fn context(&self) -> Self::Context {
        self.0.src
    }

    fn start(&self) -> Self::Offset {
        self.0.start
    }

    fn end(&self) -> Self::Offset {
        self.0.end
    }
}

impl From<ir::Span> for Span {
    fn from(span: ir::Span) -> Self {
        Span(span)
    }
}

impl From<Span> for ir::Span {
    fn from(span: Span) -> Self {
        span.0
    }
}
