#![doc = include_str!("../README.md")]

use std::{fmt::Debug, panic::Location};

use parking_lot::Mutex;
use scoped_tls::scoped_thread_local;

scoped_thread_local!(
    /// Scoped thread-local storage for a diagnostic context.
    static CTX: DiagnosticCtx
);

/// Mutex-guarded inner fields of a diagnostic context.
struct DiagnosticCtxInner {
    /// The diagnostics emitted.
    diagnostics: Vec<Diagnostic>,
    /// Indicates whether an error diagnostic has been emitted.
    has_error: bool,
}

/// A *diagnostic context*.
pub struct DiagnosticCtx {
    /// The mutex-guarded inner fields.
    inner: Mutex<DiagnosticCtxInner>,
}

impl DiagnosticCtx {
    /// Adds a diagnostic to the context.
    ///
    /// Consumers of the API should use [`Diagnostic::emit`]
    fn emit(&self, diagnostic: Diagnostic) -> DiagnosticId {
        let mut inner = self.inner.lock();
        inner.has_error |= diagnostic.is_error();
        let id = DiagnosticId(inner.diagnostics.len());
        inner.diagnostics.push(diagnostic);
        id
    }

    /// Executes the given *closure* with this diagnostic context.
    pub fn exec_within<R, C: FnOnce() -> R>(&self, closure: C) -> R {
        CTX.set(self, || closure())
    }
}

/// A *diagnostic id* uniquely identifying a diagnostic in a diagnostic context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DiagnosticId(usize);

impl DiagnosticId {
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

impl From<Diagnostic> for DiagnosticId {
    fn from(diagnostic: Diagnostic) -> Self {
        diagnostic.emit().unwrap_or_else(|err| err)
    }
}

impl std::error::Error for DiagnosticId {}

impl std::fmt::Display for DiagnosticId {
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

/// A *diagnostic* with a *severity* and a *message*.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// The diagnostic severity.
    severity: Severity,
    /// The message.
    message: String,
    /// The location where the diagnostic has been emitted.
    location: Option<Location<'static>>,
}

impl Diagnostic {
    /// Creates a new diagnostic with the given *severity* and *message*.
    #[must_use]
    pub fn new<M: ToString>(severity: Severity, message: M) -> Self {
        Self {
            severity,
            message: message.to_string(),
            location: None,
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
    pub fn error<M: ToString>(message: M) -> Self {
        Self::new(Severity::Error, message)
    }

    /// Creates a new warning diagnostic with the given *message*.
    #[must_use]
    pub fn warning<M: ToString>(message: M) -> Self {
        Self::new(Severity::Warning, message)
    }

    /// Creates a new note diagnostic with the given *message*.
    #[must_use]
    pub fn note<M: ToString>(message: M) -> Self {
        Self::new(Severity::Note, message)
    }

    /// Emits the diagnostic in the active diagnostic context.
    ///
    /// # Panics
    ///
    /// This function panics when called outside of a diagnostic context.
    #[track_caller]
    pub fn emit(mut self) -> Result<DiagnosticId, DiagnosticId> {
        self.location = Some(Location::caller().clone());
        if CTX.is_set() {
            let severity = self.severity;
            let id = CTX.with(|ctx| ctx.emit(self));
            if matches!(severity, Severity::Error) {
                Err(id.into())
            } else {
                Ok(id)
            }
        } else {
            panic!("Cannot emit diagnostic outside of diagnostic context!");
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.severity(), self.message()))
    }
}

impl std::error::Error for Diagnostic {}
