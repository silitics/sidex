#![doc = include_str!("../README.md")]

use std::{fmt::Debug, panic::Location};

use parking_lot::Mutex;
use scoped_tls::scoped_thread_local;
use sidex_ir as ir;

mod render;

scoped_thread_local!(
    /// Scoped thread-local storage for the active diagnostic context.
    static CTX: DiagnosticCtx
);

/// Mutex-guarded inner fields of a diagnostic context.
#[derive(Default)]
struct DiagnosticCtxInner {
    /// The diagnostics emitted.
    diagnostics: Vec<Diagnostic>,
    /// Indicates whether an error diagnostic has been emitted.
    has_error: bool,
}

/// A *diagnostic report* is a collection of diagnostic records.
pub struct Report {
    inner: DiagnosticCtxInner,
}

impl Report {
    pub fn diagnostics(&self) -> impl Iterator<Item = &Diagnostic> {
        self.inner.diagnostics.iter()
    }

    pub fn has_error(&self) -> bool {
        self.inner.has_error
    }

    fn write_to<W: std::io::Write>(&self, sources: &ir::SourceStorage, writer: &mut W) {
        let mut cache = render::Cache::new(sources);
        for diagnostic in self.diagnostics() {
            let _ = render::render(&mut cache, diagnostic, writer);
        }
    }

    pub fn eprint(&self, sources: &ir::SourceStorage) {
        self.write_to(sources, &mut std::io::stderr());
    }

    pub fn print(&self, sources: &ir::SourceStorage) {
        self.write_to(sources, &mut std::io::stdout());
    }
}

/// A *diagnostic context* for emitting diagnostics records.
#[derive(Default)]
pub struct DiagnosticCtx {
    /// The mutex-guarded inner fields.
    inner: Mutex<DiagnosticCtxInner>,
}

impl DiagnosticCtx {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a diagnostic to the context.
    ///
    /// Consumers of the API should use [`Diagnostic::emit`]
    fn emit(&self, diagnostic: Diagnostic) -> RecordId {
        let mut inner = self.inner.lock();
        inner.has_error |= diagnostic.is_error();
        let id = RecordId(inner.diagnostics.len());
        inner.diagnostics.push(diagnostic);
        id
    }

    /// Executes the given *closure* within this diagnostic context.
    pub fn exec<R, C: FnOnce() -> R>(&self, closure: C) -> R {
        CTX.set(self, || closure())
    }

    pub fn report(self) -> Report {
        Report {
            inner: self.inner.into_inner(),
        }
    }
}

/// A *record id* uniquely identifying a diagnostic record in a diagnostic context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RecordId(usize);

impl RecordId {
    fn with<R, C: FnOnce(Option<&Diagnostic>) -> R>(&self, closure: C) -> R {
        if CTX.is_set() {
            CTX.with(|ctx| {
                let inner = ctx.inner.lock();
                closure(Some(&inner.diagnostics[self.0]))
            })
        } else {
            closure(None)
        }
    }
}

impl From<Diagnostic> for RecordId {
    fn from(diagnostic: Diagnostic) -> Self {
        diagnostic.emit()
    }
}

impl std::error::Error for RecordId {}

impl std::fmt::Display for RecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.with(|diagnostic| {
            if let Some(diagnostic) = diagnostic {
                std::fmt::Display::fmt(diagnostic, f)
            } else {
                std::fmt::Debug::fmt(self, f)
            }
        })
    }
}

/// A *diagnostic severity* indicating the severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Severity {
    /// The diagnostic is an error.
    Error,
    /// The diagnostic is a warning.
    Warning,
    /// The diagnostic is a note.
    Note,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct Label {
    span: ir::Span,
    message: String,
}

impl Label {
    pub fn new<M: ToString>(span: ir::Span, message: M) -> Self {
        Self {
            span,
            message: message.to_string(),
        }
    }

    pub fn span(&self) -> &ir::Span {
        &self.span
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

/// A *diagnostic record* with a *severity* and a *message*.
#[derive(Debug)]
pub struct Diagnostic {
    /// The diagnostic severity.
    severity: Severity,
    /// The message.
    message: String,
    /// The location where the diagnostic has been created.
    created_at: Location<'static>,
    /// The location where the diagnostic has been emitted.
    emitted_at: Option<Location<'static>>,
    /// Errors attached to the diagnostic.
    errors: Vec<anyhow::Error>,
    /// The span the record corresponds to.
    span: Option<ir::Span>,
    /// Labels attached to the record.
    labels: Vec<Label>,
}

impl Diagnostic {
    /// Creates a new diagnostic with the given *severity* and *message*.
    #[must_use]
    #[track_caller]
    pub fn new<M: ToString>(severity: Severity, message: M) -> Self {
        Self {
            severity,
            message: message.to_string(),
            created_at: Location::caller().clone(),
            emitted_at: None,
            errors: Default::default(),
            span: None,
            labels: Default::default(),
        }
    }

    /// The severity of the diagnostic.
    #[must_use]
    pub fn severity(&self) -> Severity {
        self.severity
    }

    /// The message of the diagnostic.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Indicates whether the diagnostic is an error.
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self.severity, Severity::Error)
    }

    /// Creates a new error diagnostic with the given *message*.
    #[must_use]
    #[track_caller]
    pub fn error<M: ToString>(message: M) -> Self {
        Self::new(Severity::Error, message)
    }

    /// Creates a new warning diagnostic with the given *message*.
    #[must_use]
    #[track_caller]
    pub fn warning<M: ToString>(message: M) -> Self {
        Self::new(Severity::Warning, message)
    }

    /// Creates a new note diagnostic with the given *message*.
    #[must_use]
    #[track_caller]
    pub fn note<M: ToString>(message: M) -> Self {
        Self::new(Severity::Note, message)
    }

    /// The errors attached to the diagnostic.
    #[must_use]
    pub fn errors(&self) -> impl Iterator<Item = &dyn std::error::Error> {
        self.errors.iter().map(|error| error.as_ref())
    }

    /// Sets the span of the diagnostic.
    pub fn set_span(&mut self, span: Option<ir::Span>) {
        self.span = span;
    }

    /// Sets the span of the diagnostic.
    #[must_use]
    pub fn with_span(mut self, span: Option<ir::Span>) -> Self {
        self.set_span(span);
        self
    }

    /// Attaches an error to the diagnostic.
    pub fn attach_error<E: 'static + Send + Sync + std::error::Error>(&mut self, error: E) {
        self.errors.push(error.into())
    }

    /// Attaches an error to the diagnostic.
    #[must_use]
    pub fn with_error<E: 'static + Send + Sync + std::error::Error>(mut self, error: E) -> Self {
        self.attach_error(error);
        self
    }

    /// Attaches a label to the diagnostic record.
    pub fn attach_label(&mut self, label: Label) {
        self.labels.push(label)
    }

    /// Attaches a label to the diagnostic record.
    #[must_use]
    pub fn with_label(mut self, label: Label) -> Self {
        self.attach_label(label);
        self
    }

    /// Emits the diagnostic in the active diagnostic context.
    ///
    /// # Panics
    ///
    /// This function panics when called outside of a diagnostic context.
    #[track_caller]
    pub fn emit(mut self) -> RecordId {
        self.emitted_at = Some(Location::caller().clone());
        if CTX.is_set() {
            CTX.with(|ctx| ctx.emit(self))
        } else {
            panic!("Cannot emit diagnostic outside of diagnostic context!");
        }
    }

    /// Format the diagnostic.
    pub(crate) fn _fmt(
        &self,
        _unit: &ir::Unit,
        _writer: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.severity(), self.message()))?;
        f.write_fmt(format_args!("\nCreated At: {}", self.created_at))?;
        if !self.errors.is_empty() {
            f.write_str("\n==== ATTACHED ERRORS ===")?;
            for error in &self.errors {
                std::fmt::Display::fmt(error, f)?;
            }
        }
        Ok(())
    }
}

impl<E: 'static + Send + Sync + std::error::Error> From<E> for Diagnostic {
    fn from(error: E) -> Self {
        Self::error(&error).with_error(error)
    }
}

/// An *error* is a diagnostic record.
pub type Error = Diagnostic;

/// A result type based on [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
