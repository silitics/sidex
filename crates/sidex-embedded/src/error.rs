//! Errors of decoding and encoding.

use core::fmt;

/// Error decoding a value.
///
/// Errors are small and `Copy`. Instead of a path to the offending value, they
/// record its byte offset in the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeError {
    kind: DecodeErrorKind,
    offset: usize,
}

impl DecodeError {
    /// Creates an error of the given kind at the given byte offset.
    pub const fn new(kind: DecodeErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    /// Kind of the error.
    pub const fn kind(&self) -> DecodeErrorKind {
        self.kind
    }

    /// Byte offset of the offending value in the input.
    pub const fn offset(&self) -> usize {
        self.offset
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.kind, self.offset)
    }
}

impl core::error::Error for DecodeError {}

/// Kind of a [`DecodeError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DecodeErrorKind {
    /// The input is not valid JSON.
    Syntax,
    /// Values are nested deeper than the reader's maximum depth.
    DepthLimitExceeded,
    /// The value does not have the expected representation, which the payload
    /// describes, for instance, `a 32-bit unsigned integer`.
    Unexpected(&'static str),
    /// A required field is missing.
    MissingField(&'static str),
    /// A field is present under more than one of its names.
    DuplicateField(&'static str),
    /// The tag of a variant names no case.
    UnknownCase,
    /// No case of an implicitly tagged variant matches the value.
    NoMatchingCase,
}

impl fmt::Display for DecodeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax => f.write_str("invalid JSON"),
            Self::DepthLimitExceeded => f.write_str("maximum nesting depth exceeded"),
            Self::Unexpected(expected) => write!(f, "expected {expected}"),
            Self::MissingField(name) => write!(f, "missing field `{name}`"),
            Self::DuplicateField(name) => write!(f, "duplicate field `{name}`"),
            Self::UnknownCase => f.write_str("unknown case"),
            Self::NoMatchingCase => f.write_str("no case matches"),
        }
    }
}

/// Error encoding a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EncodeError {
    /// The output buffer is too small.
    BufferFull,
    /// A [`Sink`](crate::json::Sink) failed to write.
    Write,
    /// The value of an inline field or a merged variant payload is not a record.
    NotARecord,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::BufferFull => "output buffer is full",
            Self::Write => "failed to write output",
            Self::NotARecord => "value of an inline field is not a record",
        })
    }
}

impl core::error::Error for EncodeError {}

/// Error copying data into a buffer that is too small.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferTooSmall;

impl fmt::Display for BufferTooSmall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("buffer is too small")
    }
}

impl core::error::Error for BufferTooSmall {}
