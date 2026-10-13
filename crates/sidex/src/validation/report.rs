//! Results of validation.

use std::borrow::Cow;
use std::fmt;

use crate::decode::Path;

/// Violation of a validation rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub(super) path: Path,
    pub(super) code: &'static str,
    pub(super) message: Cow<'static, str>,
}

impl ValidationError {
    /// Path to the value violating the rule.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Code identifying the kind of violation, for instance, `min`.
    pub fn code(&self) -> &'static str {
        self.code
    }

    /// Message describing the violation.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.path.is_root() {
            write!(f, "{}: ", self.path)?;
        }
        f.write_str(&self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Violations found by [`validate`](super::validate).
///
/// A report contains at least one error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub(super) errors: Vec<ValidationError>,
}

impl ValidationReport {
    /// Errors in the order they were found.
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }
}

impl IntoIterator for ValidationReport {
    type Item = ValidationError;
    type IntoIter = std::vec::IntoIter<ValidationError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, error) in self.errors.iter().enumerate() {
            if index > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{error}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}
