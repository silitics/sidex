//! Beautiful error reporting with [Ariadne](https://crates.io/crates/ariadne).

use ariadne::{Color, Fmt, Label, Report, ReportKind};

use crate::{
    errors::SyntaxError,
    source::{SourceId, SourceStorage, Span},
};

// Required for error reporting with Ariadne.
impl ariadne::Span for Span {
    type SourceId = SourceId;

    fn source(&self) -> &Self::SourceId {
        self.src()
    }

    fn start(&self) -> usize {
        self.start()
    }

    fn end(&self) -> usize {
        self.end()
    }
}

/// Source cache for Ariadne.
pub(crate) struct Cache<'s> {
    storage: &'s SourceStorage,
    cache: Vec<Option<ariadne::Source>>,
}

impl<'s> Cache<'s> {
    /// Creates a new cache backed by the given storage.
    pub fn new(storage: &'s SourceStorage) -> Self {
        let mut cache = Vec::with_capacity(storage.sources.len());
        for _ in 0..storage.sources.len() {
            cache.push(None);
        }
        Self { storage, cache }
    }
}

impl<'s> ariadne::Cache<SourceId> for Cache<'s> {
    fn fetch(&mut self, id: &SourceId) -> Result<&ariadne::Source, Box<dyn std::fmt::Debug + '_>> {
        let cached = &mut self.cache[id.0];
        Ok(cached.get_or_insert_with(|| ariadne::Source::from(&self.storage[*id].text)))
    }

    fn display<'a>(&self, id: &'a SourceId) -> Option<Box<dyn std::fmt::Display + 'a>> {
        let source = &self.storage[*id];
        let name = source
            .path
            .as_ref()
            .and_then(|path| path.file_name())
            .map(|filename| filename.to_string_lossy().into_owned());
        match name {
            Some(name) => Some(Box::new(name)),
            None => None,
        }
    }
}

/// Print syntax errors on stderr.
pub fn eprint_errors<'e, I: Iterator<Item = &'e SyntaxError>>(
    storage: &SourceStorage,
    id: SourceId,
    errors: I,
) {
    errors.for_each(|error| {
        let report = Report::build(ReportKind::Error, id, error.0.span().start());

        let report = match error.0.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow)
                ))
                .with_label(
                    Label::new(span.clone())
                        .with_message(format!(
                            "Unclosed delimiter {}",
                            delimiter.fg(Color::Yellow)
                        ))
                        .with_color(Color::Yellow),
                )
                .with_label(
                    Label::new(error.0.span())
                        .with_message(format!(
                            "Must be closed before this {}",
                            error
                                .0
                                .found()
                                .unwrap_or(&"end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            chumsky::error::SimpleReason::Unexpected => report
                .with_message(if error.0.found().is_some() {
                    "Unexpected token in input.".to_owned()
                } else {
                    "Unexpected end of input.".to_owned()
                })
                .with_label(
                    Label::new(error.0.span())
                        .with_message(format!(
                            "Unexpected token {}",
                            error
                                .0
                                .found()
                                .unwrap_or(&"end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                Label::new(error.0.span())
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red),
            ),
        };

        report.finish().eprint(Cache::new(storage)).unwrap();
    });
}
