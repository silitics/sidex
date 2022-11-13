//! Representation of text sources.

use std::{ops::Range, path::PathBuf};

/// Uniquely identifies a source in a storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceId(pub(crate) usize);

impl SourceId {
    /// A dummy source identifier primarily for testing purposes.
    #[cfg(test)]
    pub fn dummy() -> Self {
        Self(usize::MAX)
    }
}

/// A text source.
#[derive(Clone, Debug)]
pub struct Source {
    /// The id of the source.
    id: SourceId,
    /// The source text of the source.
    pub text: String,
    /// The path of the source.
    pub path: Option<PathBuf>,
}

impl Source {
    /// The id of the source.
    pub fn id(&self) -> SourceId {
        self.id
    }

    /// The end of the source.
    pub(crate) fn end(&self) -> Span {
        self.span_at(self.text.len())
    }

    /// Create a span at the given position.
    pub(crate) fn span_at(&self, pos: usize) -> Span {
        Span::new(self.id, pos..pos + 1)
    }
}

/// Identifies a range of characters in a given source.
///
/// This is private because we implement [`chumsky::Span`] and [`ariadne::Span`] for it.
/// The [`super::tokens::Token`] implementation exposes the same information as part of
/// its public API.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Span {
    src: SourceId,
    range: Range<usize>,
}

// This is an internal API, hence, we allow dead code.
#[allow(dead_code)]
impl Span {
    /// Construct a new span.
    pub fn new(src: SourceId, range: Range<usize>) -> Self {
        Span { src, range }
    }

    /// Construct a dummy span primarily for testing purposes.
    #[cfg(test)]
    pub fn dummy() -> Self {
        Self {
            src: SourceId::dummy(),
            range: 0..0,
        }
    }

    /// Changes the range of the span.
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.range = range;
        self
    }

    /// The source.
    pub fn src(&self) -> &SourceId {
        &self.src
    }

    /// The start character.
    pub fn start(&self) -> usize {
        self.range.start
    }

    /// The non-included end character.
    pub fn end(&self) -> usize {
        self.range.end
    }
}

// Required for parsing with Chumsky.
impl chumsky::Span for Span {
    type Context = SourceId;
    type Offset = usize;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Span {
            src: context,
            range,
        }
    }

    fn context(&self) -> Self::Context {
        self.src.clone()
    }

    fn start(&self) -> Self::Offset {
        self.range.start
    }

    fn end(&self) -> Self::Offset {
        self.range.end
    }
}

/// A storage for sources.
#[derive(Default)]
pub struct SourceStorage {
    /// The text sources.
    pub(crate) sources: Vec<Source>,
}

impl SourceStorage {
    /// Create a new empty storage.
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert a source into the storage.
    pub fn insert(&mut self, text: String, path: Option<PathBuf>) -> SourceId {
        let id = SourceId(self.sources.len());
        self.sources.push(Source { id, text, path });
        id
    }
}

impl std::ops::Index<SourceId> for SourceStorage {
    type Output = Source;

    fn index(&self, index: SourceId) -> &Self::Output {
        &self.sources[index.0]
    }
}
