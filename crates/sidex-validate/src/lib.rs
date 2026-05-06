//! Runtime for Sidex's validation extension.
//!
//! Sidex codegens emit calls into this crate for each `#[validate(expr =
//! { ... })]` rule on a generated type. A validator returns a
//! [`ValidationReport`] of zero or more [`ValidationError`]s rather than
//! throwing — multiple errors per call surface every problem at once,
//! which is what UI forms want.
//!
//! ## Helpers
//!
//! Every comparison helper has the same signature shape:
//!
//! ```ignore
//! fn min_inclusive<T: PartialOrd + Display>(
//!     value: T, bound: T,
//!     code: &'static str,
//!     message: Option<&'static str>,
//!     path: &Path,
//! ) -> ValidationReport;
//! ```
//!
//! `code` carries the semantic identifier (`"min"`, `"min_length"`,
//! `"format:email"`, …); the recognizer in `sidex-validate-expr` decides
//! which one to pass based on the source shape. `message` overrides the
//! default; codegens thread the user's `message = "..."` attribute through
//! verbatim or pass `None` for the default.

#![forbid(unsafe_code)]

use std::borrow::Cow;
use std::fmt::Display;

pub use regex::Regex;

// --- Path -------------------------------------------------------------------

/// JSON-pointer-style path identifying the location of a value within the
/// validated structure. Reported on every [`ValidationError`] so frontends
/// can map errors back to specific fields, sequence indices, or map keys.
///
/// The display form matches RFC 6901 ("/users/3/email", "/" for the root).
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Path {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Segment {
    Field(Cow<'static, str>),
    Index(usize),
    Key(String),
}

impl Path {
    pub const fn root() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn field(&self, name: impl Into<Cow<'static, str>>) -> Self {
        let mut next = self.clone();
        next.segments.push(Segment::Field(name.into()));
        next
    }

    pub fn index(&self, idx: usize) -> Self {
        let mut next = self.clone();
        next.segments.push(Segment::Index(idx));
        next
    }

    pub fn key(&self, key: impl Into<String>) -> Self {
        let mut next = self.clone();
        next.segments.push(Segment::Key(key.into()));
        next
    }

    pub fn is_root(&self) -> bool {
        self.segments.is_empty()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.segments.is_empty() {
            return f.write_str("/");
        }
        for seg in &self.segments {
            f.write_str("/")?;
            match seg {
                Segment::Field(name) => write_pointer_token(f, name)?,
                Segment::Index(idx) => write!(f, "{idx}")?,
                Segment::Key(key) => write_pointer_token(f, key)?,
            }
        }
        Ok(())
    }
}

fn write_pointer_token(f: &mut std::fmt::Formatter<'_>, raw: &str) -> std::fmt::Result {
    // RFC 6901: `~` → `~0`, `/` → `~1`.
    for ch in raw.chars() {
        match ch {
            '~' => f.write_str("~0")?,
            '/' => f.write_str("~1")?,
            _ => f.write_str(ch.encode_utf8(&mut [0u8; 4]))?,
        }
    }
    Ok(())
}

// --- Errors / report --------------------------------------------------------

/// One validation failure. `code` is a stable identifier the application
/// can map to a localized string; `message` is the human-readable form
/// (either the default keyed on `code` or a user override from the
/// `message = "..."` attribute).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub path: Path,
    pub code: Cow<'static, str>,
    pub message: Cow<'static, str>,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}: {}", self.code, self.path, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Bag of validation errors with composition primitives. An empty report
/// means "valid"; converting a non-empty report to a `Result` gives back
/// the report itself as the error value.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    errors: Vec<ValidationError>,
}

impl ValidationReport {
    pub fn ok() -> Self {
        Self::default()
    }

    pub fn from_error(error: ValidationError) -> Self {
        Self {
            errors: vec![error],
        }
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    pub fn into_errors(self) -> Vec<ValidationError> {
        self.errors
    }

    pub fn push(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Drain `other` into `self`. Cheap with `Vec::append`.
    pub fn merge(&mut self, mut other: ValidationReport) {
        self.errors.append(&mut other.errors);
    }

    /// Convert into `Result<(), ValidationReport>`. Empty reports become
    /// `Ok(())`; non-empty reports become `Err(self)`.
    pub fn into_result(self) -> Result<(), ValidationReport> {
        if self.is_ok() { Ok(()) } else { Err(self) }
    }
}

impl Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, error) in self.errors.iter().enumerate() {
            if idx != 0 {
                f.write_str("\n")?;
            }
            write!(f, "{error}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

impl IntoIterator for ValidationReport {
    type Item = ValidationError;
    type IntoIter = std::vec::IntoIter<ValidationError>;
    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

// --- Validate trait ---------------------------------------------------------

/// Implemented by every generated type that has at least one validation
/// rule. Codegens emit `validate_at` and provide `validate` as a thin
/// wrapper rooted at `Path::root()`.
pub trait Validate {
    fn validate_at(&self, path: &Path) -> ValidationReport;

    fn validate(&self) -> ValidationReport {
        self.validate_at(&Path::root())
    }
}

// --- Default messages -------------------------------------------------------

/// Pick the default human-readable message for `code`. Codegens pass this
/// through when the user didn't supply a `message = "..."` override.
pub fn default_message(code: &str) -> &'static str {
    match code {
        "min" => "Value is below the minimum.",
        "max" => "Value is above the maximum.",
        "min_size" => "Value is below the minimum size.",
        "max_size" => "Value exceeds the maximum size.",
        "eq" => "Value does not match the expected value.",
        "ne" => "Value matches a forbidden value.",
        "regex" => "Value does not match the required pattern.",
        "predicate" => "Value does not satisfy the predicate.",
        _ => "Value is not valid.",
    }
}

fn pick_message(message: Option<&'static str>, code: &'static str) -> Cow<'static, str> {
    match message {
        Some(m) => Cow::Borrowed(m),
        None => Cow::Borrowed(default_message(code)),
    }
}

// --- Helper builders --------------------------------------------------------

fn report_one(path: &Path, code: &'static str, message: Option<&'static str>) -> ValidationReport {
    ValidationReport::from_error(ValidationError {
        path: path.clone(),
        code: Cow::Borrowed(code),
        message: pick_message(message, code),
    })
}

// --- Comparison helpers -----------------------------------------------------

pub fn min_inclusive<T: PartialOrd>(
    value: T,
    bound: T,
    code: &'static str,
    message: Option<&'static str>,
    path: &Path,
) -> ValidationReport {
    if value >= bound {
        ValidationReport::ok()
    } else {
        report_one(path, code, message)
    }
}

pub fn min_exclusive<T: PartialOrd>(
    value: T,
    bound: T,
    code: &'static str,
    message: Option<&'static str>,
    path: &Path,
) -> ValidationReport {
    if value > bound {
        ValidationReport::ok()
    } else {
        report_one(path, code, message)
    }
}

pub fn max_inclusive<T: PartialOrd>(
    value: T,
    bound: T,
    code: &'static str,
    message: Option<&'static str>,
    path: &Path,
) -> ValidationReport {
    if value <= bound {
        ValidationReport::ok()
    } else {
        report_one(path, code, message)
    }
}

pub fn max_exclusive<T: PartialOrd>(
    value: T,
    bound: T,
    code: &'static str,
    message: Option<&'static str>,
    path: &Path,
) -> ValidationReport {
    if value < bound {
        ValidationReport::ok()
    } else {
        report_one(path, code, message)
    }
}

pub fn eq<T: PartialEq>(
    value: T,
    expected: T,
    code: &'static str,
    message: Option<&'static str>,
    path: &Path,
) -> ValidationReport {
    if value == expected {
        ValidationReport::ok()
    } else {
        report_one(path, code, message)
    }
}

pub fn ne<T: PartialEq>(
    value: T,
    forbidden: T,
    code: &'static str,
    message: Option<&'static str>,
    path: &Path,
) -> ValidationReport {
    if value != forbidden {
        ValidationReport::ok()
    } else {
        report_one(path, code, message)
    }
}

// --- Accessors --------------------------------------------------------------

/// Length of a string in Unicode code points (chars). The runtime helper
/// `_.size` lowers to in the recognizer when validating a `string` field.
/// Pinned across targets — JavaScript spread iteration (`[...str].length`)
/// gives the same answer as Rust's `chars().count()`. Different from the
/// byte count and from JavaScript's native `str.length` (which counts
/// UTF-16 code units).
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Length of a string in bytes (UTF-8 encoded).
pub fn byte_count(s: &str) -> usize {
    s.len()
}

// --- Regex ------------------------------------------------------------------

/// Run a precompiled regex against `s`. Codegens emit a `OnceLock<Regex>`
/// per call site and pass the compiled pattern in here.
pub fn regex_match(
    s: &str,
    pattern: &Regex,
    code: &'static str,
    message: Option<&'static str>,
    path: &Path,
) -> ValidationReport {
    if pattern.is_match(s) {
        ValidationReport::ok()
    } else {
        report_one(path, code, message)
    }
}

// --- Tests ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_helpers_emit_no_errors() {
        let path = Path::root().field("age");
        assert!(min_inclusive(5, 0, "min", None, &path).is_ok());
        assert!(max_inclusive(5, 100, "max", None, &path).is_ok());
        assert!(eq(5, 5, "eq", None, &path).is_ok());
    }

    #[test]
    fn helpers_emit_path_code_and_message() {
        let path = Path::root().field("age");
        let report = min_inclusive(0u32, 1, "min", Some("must be positive"), &path);
        assert!(report.is_err());
        let err = &report.errors()[0];
        assert_eq!(err.path.to_string(), "/age");
        assert_eq!(err.code, "min");
        assert_eq!(err.message, "must be positive");
    }

    #[test]
    fn default_message_falls_back_to_code() {
        let path = Path::root().field("v");
        let report = max_inclusive(101u32, 100, "max", None, &path);
        let err = &report.errors()[0];
        assert_eq!(err.message, "Value is above the maximum.");
    }

    #[test]
    fn merge_appends_errors() {
        let path = Path::root().field("x");
        let mut a = min_inclusive(0u32, 1, "min", None, &path);
        let b = max_inclusive(10u32, 5, "max", None, &path);
        a.merge(b);
        assert_eq!(a.errors().len(), 2);
    }

    #[test]
    fn path_renders_jsonpointer_with_escapes() {
        let p = Path::root().field("u").index(3).key("a/b~c");
        // RFC 6901: `/` → `~1`, `~` → `~0`.
        assert_eq!(p.to_string(), "/u/3/a~1b~0c");
    }

    #[test]
    fn char_count_handles_unicode() {
        assert_eq!(char_count("hello"), 5);
        // 'é' is one code point even though it can be encoded as two bytes.
        assert_eq!(char_count("café"), 4);
        // Surrogate-pair-encoded char in UTF-16 is one code point.
        assert_eq!(char_count("a😀b"), 3);
    }

    #[test]
    fn regex_match_succeeds_and_fails() {
        let pattern = Regex::new("^[A-Z]+$").unwrap();
        let path = Path::root().field("code");
        assert!(regex_match("ABC", &pattern, "regex", None, &path).is_ok());
        assert!(regex_match("abc", &pattern, "regex", None, &path).is_err());
    }
}
