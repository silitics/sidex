//! Rendering of diagnostics and reports with [Ariadne][ariadne].

use std::collections::HashMap;

use sidex_ir as ir;

use crate::{Diagnostic, Severity};

/// Source identifier for identifying an Ariadne source in [`Cache`].
type SourceId = Option<ir::SourceIdx>;

/// A source cache for Ariadne based on [`ir::SourceStorage`].
pub(crate) struct Cache<'s> {
    /// The underlying source storage.
    sources: &'s ir::SourceStorage,
    /// The cache for [`ariadne::Source`].
    cache: HashMap<SourceId, ariadne::Source>,
}

impl<'s> Cache<'s> {
    /// Creates a new cache from the given source storage.
    pub(crate) fn new(sources: &'s ir::SourceStorage) -> Self {
        Self {
            sources,
            cache: HashMap::new(),
        }
    }
}

impl<'s> ariadne::Cache<SourceId> for Cache<'s> {
    fn fetch(&mut self, id: &SourceId) -> Result<&ariadne::Source, Box<dyn std::fmt::Debug + '_>> {
        Ok(self.cache.entry(*id).or_insert_with(|| {
            match id {
                Some(idx) => {
                    ariadne::Source::from(
                        self.sources[*idx]
                            .text
                            .as_ref()
                            .map(String::as_str)
                            .unwrap_or(""),
                    )
                }
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

/// A source span implementing [`ariadne::Span`].
struct Span {
    /// The source id of the span.
    src: SourceId,
    /// The first character of the span.
    start: usize,
    /// The last character of the span.
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

/// Converts a diagnostic severity to [`ariadne::ReportKind`].
///
/// We cannot implement a conversion trait because [`ariadne`] is a private dependency.
fn to_report_kind(severity: Severity) -> ariadne::ReportKind {
    match severity {
        Severity::Error => ariadne::ReportKind::Error,
        Severity::Warning => ariadne::ReportKind::Warning,
        Severity::Hint => ariadne::ReportKind::Advice,
    }
}

/// Renders a diagnostic to the given *writer* using the given *cache*.
pub(crate) fn render<'u, W: std::io::Write>(
    cache: &mut Cache<'u>,
    diagnostic: &Diagnostic,
    mut writer: W,
) -> std::io::Result<()> {
    // 1️⃣ Print the main part of the diagnostic with Ariadne.
    let src_id = diagnostic.span.as_ref().map(|span| span.src);
    let offset = diagnostic
        .span
        .as_ref()
        .map(|span| span.start)
        .unwrap_or_default();
    let mut builder =
        ariadne::Report::<Span>::build(to_report_kind(diagnostic.severity), src_id, offset);
    builder.set_message(diagnostic.message());
    builder.add_labels(diagnostic.labels.iter().map(|label| {
        ariadne::Label::new(label.span().clone().into()).with_message(&label.message)
    }));
    if let Some(help) = &diagnostic.help {
        builder.set_help(help);
    }
    builder.finish().write(cache, &mut writer)?;

    // 2️⃣ Print the associated errors.
    if !diagnostic.errors.is_empty() {
        write!(writer, "\nAssociated Errors:")?;
        for (idx, error) in diagnostic.errors().enumerate() {
            write!(writer, "\n   {}: {}", idx, error)?;
        }
        write!(writer, "\n\n")?;
    };

    // 3️⃣ Print further useful information.
    write!(
        writer,
        "\nCreated At:\n   {}:{}:{}\n\n",
        diagnostic.created_at.file(),
        diagnostic.created_at.line(),
        diagnostic.created_at.column()
    )?;

    Ok(())
}
