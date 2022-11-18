//! Rendering of diagnostics and reports with [Ariadne][ariadne].

use std::collections::HashMap;

use sidex_ir as ir;

use crate::{Diagnostic, Severity};

type SourceId = Option<ir::SourceIdx>;

pub(crate) struct Cache<'s> {
    sources: &'s ir::SourceStorage,
    cache: HashMap<SourceId, ariadne::Source>,
}

impl<'s> Cache<'s> {
    pub fn new(sources: &'s ir::SourceStorage) -> Self {
        Self {
            sources,
            cache: Default::default(),
        }
    }
}

impl<'s> ariadne::Cache<SourceId> for Cache<'s> {
    fn fetch(&mut self, id: &SourceId) -> Result<&ariadne::Source, Box<dyn std::fmt::Debug + '_>> {
        Ok(self.cache.entry(*id).or_insert_with(|| {
            match id {
                Some(idx) => ariadne::Source::from(&self.sources[*idx].text),
                None => ariadne::Source::from(""),
            }
        }))
    }

    fn display<'a>(&self, id: &'a SourceId) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match id {
            Some(idx) => {
                match &self.sources[*idx].origin {
                    Some(origin) => Some(Box::new(origin.clone())),
                    None => None,
                }
            }
            None => None,
        }
    }
}

struct Span {
    src: SourceId,
    start: usize,
    end: usize,
}

impl From<ir::Span> for Span {
    fn from(span: ir::Span) -> Self {
        Self {
            src: Some(span.src),
            start: span.start,
            end: span.end,
        }
    }
}

impl ariadne::Span for Span {
    type SourceId = SourceId;

    fn source(&self) -> &Self::SourceId {
        &self.src
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}

pub(crate) fn report_kind(severity: &Severity) -> ariadne::ReportKind {
    match severity {
        Severity::Error => ariadne::ReportKind::Error,
        Severity::Warning => ariadne::ReportKind::Warning,
        Severity::Note => ariadne::ReportKind::Advice,
    }
}

pub(crate) fn render<'u, W: std::io::Write>(
    cache: &mut Cache<'u>,
    diagnostic: &Diagnostic,
    writer: &mut W,
) -> std::io::Result<()> {
    let src_id = diagnostic.span.as_ref().map(|span| span.src);
    let offset = diagnostic
        .span
        .as_ref()
        .map(|span| span.start)
        .unwrap_or_default();
    let mut builder =
        ariadne::Report::<Span>::build(report_kind(&diagnostic.severity), src_id, offset)
            .with_message(diagnostic.message());
    builder.add_labels(diagnostic.labels.iter().map(|label| {
        ariadne::Label::new(label.span().clone().into()).with_message(&label.message)
    }));
    let mut note = String::new();
    if !diagnostic.errors.is_empty() {
        note.push_str("Associated Errors:\n");
        for error in diagnostic.errors() {
            note.push_str("\n");
            note.push_str(&error.to_string());
        }
    }
    if !note.is_empty() {
        builder.set_note(note);
    }
    builder.finish().write(cache, writer)
}
