#![doc = include_str!("../README.md")]
//!
//! ## Diagnostics
//!
//! *Diagnostics* such as errors, warnings, or hints are represented by [`Diagnostic`].
//! Every such diagnostic consists of a message and a severity ([`Severity`]) which is
//! either [`Error`][Severity::Error], [`Warning`][Severity::Warning], or
//! [`Hint`][Severity::Hint]. In addition, diagnostics track:
//!
//! - The [`Location`] where they have been created (useful for debugging).
//! - The [`Location`] where they have been emitted (useful for debugging).
//! - A list of [errors][std::error::Error] which have been attached to the diagnostic.
//! - An optional [`ir::Span`] indicating the primary source span the diagnostic is about.
//! - A list of [labels][Label] indicating contextual source spans the diagnostic is
//!   about.
//!
//! Here is an example for the creation of an error diagnostic:
//!
//! ```
//! # use sidex_diagnostics::Diagnostic;
//! Diagnostic::error("This is a custom error message!");
//! ```
//!
//! For convenience, diagnostics can also be created from any [`Error`][std::error::Error]
//! with the [`Into`] conversion trait. In particular, this enable the usage of `?` to
//! turn arbitrary errors which may occur into diagnostics. Note that the resulting
//! diagnostics do not have a primary source span. In fact, they represent an error
//! independent of any Sidex source file.
//!
//!
//! ## Emitting Diagnostics
//!
//! Diagnostics are *emitted* with the [`emit`][Diagnostic::emit] method. Emitting
//! requires an active *diagnostic context* ([`DiagnosticCtx`]). A diagnostic context is
//! responsible for collecting multiple diagnostics. Here is an example:
//!
//! ```
//! # use sidex_diagnostics::{Diagnostic, DiagnosticCtx};
//! let ctx = DiagnosticCtx::new();
//! ctx.exec(|| Diagnostic::error("This is a custom error message!").emit());
//! ```
//!
//! Merely creating a diagnostic will do nothing if the diagnostic is not emitted. Among
//! other things, this means that intermittent error diagnostics can be handled as usual
//! via [`Result`] without them being collected and displayed.
//!
//! Trying to emit a diagnostic outside of an active diagnostic context will panic.
//!
//! Most functionality of Sidex assumes to be executed in an active diagnostic context.
//!
//!
//! ## Reports
//!
//! To display diagnostics a *report* [`Report`] is produced from a context.
//!
//! A report can then be rendered to the standard error output using its
//! [`eprint`][Report::eprint] method.

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

/// A *diagnostic context* for collecting diagnostics.
#[derive(Default)]
pub struct DiagnosticCtx {
    /// The mutex-guarded inner fields.
    inner: Mutex<DiagnosticCtxInner>,
}

impl DiagnosticCtx {
    /// Creates a new diagnosis context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a diagnostic to the context.
    ///
    /// Consumers of the API should use [`Diagnostic::emit`]
    fn emit(&self, diagnostic: Diagnostic) -> DiagnosticIdx {
        let mut inner = self.inner.lock();
        inner.has_error |= diagnostic.is_error();
        let id = DiagnosticIdx(inner.diagnostics.len());
        inner.diagnostics.push(diagnostic);
        id
    }

    /// Executes the given *closure* within this diagnostic context.
    pub fn exec<R, C: FnOnce() -> R>(&self, closure: C) -> R {
        CTX.set(self, || closure())
    }

    /// Turns the context into a report.
    pub fn report(self) -> Report {
        Report {
            inner: self.inner.into_inner(),
        }
    }
}

/// A *diagnostic report* is a collection of diagnostics.
///
/// Can only be created via a diagnosis context.
pub struct Report {
    inner: DiagnosticCtxInner,
}

impl Report {
    /// The diagnostics of the report.
    pub fn diagnostics(&self) -> impl Iterator<Item = &Diagnostic> {
        self.inner.diagnostics.iter()
    }

    /// Indicates whether there has been an error.
    pub fn has_error(&self) -> bool {
        self.inner.has_error
    }

    /// Writes the complete report to the given *writer*.
    fn write_to<W: std::io::Write>(&self, sources: &ir::SourceStorage, mut writer: W) {
        let mut cache = render::Cache::new(sources);
        for diagnostic in self.diagnostics() {
            let _ = render::render(&mut cache, diagnostic, &mut writer);
        }
    }

    /// Prints the report on the standard error output.
    pub fn eprint(&self, sources: &ir::SourceStorage) {
        self.write_to(sources, std::io::stderr());
    }

    /// Prints the report on the standard output.
    pub fn print(&self, sources: &ir::SourceStorage) {
        self.write_to(sources, std::io::stdout());
    }
}

/// A *diagnostic index* uniquely identifying a diagnostic in a diagnostic context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DiagnosticIdx(usize);

impl DiagnosticIdx {
    /// Tries to retrieve the diagnostic from the active context.
    ///
    /// Invokes the closure with [`None`] if there is no active context.
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

impl From<Diagnostic> for DiagnosticIdx {
    fn from(diagnostic: Diagnostic) -> Self {
        diagnostic.emit()
    }
}

impl std::error::Error for DiagnosticIdx {}

impl std::fmt::Display for DiagnosticIdx {
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
    /// The diagnostic is a hint.
    Hint,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// A *diagnostic* with a *severity* and a *message*.
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
    /// The primary source span the diagnostic is about.
    span: Option<ir::Span>,
    /// Labels attached to contextually relevant source spans.
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

    /// Creates a new hint diagnostic with the given *message*.
    #[must_use]
    #[track_caller]
    pub fn hint<M: ToString>(message: M) -> Self {
        Self::new(Severity::Hint, message)
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

    /// Attaches a label to the diagnostic.
    pub fn attach_label(&mut self, label: Label) {
        self.labels.push(label)
    }

    /// Attaches a label to the diagnostic.
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
    pub fn emit(mut self) -> DiagnosticIdx {
        self.emitted_at = Some(Location::caller().clone());
        if CTX.is_set() {
            CTX.with(|ctx| ctx.emit(self))
        } else {
            panic!("Cannot emit diagnostic outside of diagnostic context!");
        }
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

/// A *diagnostic label* attached to a source span.
#[derive(Debug)]
pub struct Label {
    /// The source span.
    span: ir::Span,
    /// The message.
    message: String,
}

impl Label {
    /// Creates a new label.
    pub fn new<M: ToString>(span: ir::Span, message: M) -> Self {
        Self {
            span,
            message: message.to_string(),
        }
    }

    /// The span of the label.
    pub fn span(&self) -> &ir::Span {
        &self.span
    }

    /// The message of the label.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

/// A result type with [`Diagnostic`] as error type.
pub type Result<T> = std::result::Result<T, Diagnostic>;
