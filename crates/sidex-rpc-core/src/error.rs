//! RPC error types.

use std::fmt::Display;

/// Result of an RPC call.
pub type CallResult<O> = Result<O, CallError>;

/// Error fulfilling an RPC request.
#[derive(Debug, Clone)]
pub enum CallError {
    /// Request is invalid and cannot be fulfilled (caller's fault).
    ///
    /// Corresponds to HTTP status codes `4xx`.
    Invalid(InvalidRequestError),
    /// Request seems valid but cannot be fulfilled (callee's fault).
    ///
    /// Corresponds to HTTP status codes `5xx`.
    Internal(InternalError),
}

/// Request is invalid and cannot be fulfilled (caller's fault).
#[derive(Debug, Clone)]
pub struct InvalidRequestError {
    /// Error kind.
    kind: InvalidRequestErrorKind,
    /// Error message.
    message: String,
}

impl InvalidRequestError {
    /// Create a new [`InvalidRequestError`] with the given kind and message.
    pub fn new<M: Display>(kind: InvalidRequestErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }

    /// Error kind.
    pub fn kind(&self) -> &InvalidRequestErrorKind {
        &self.kind
    }

    /// Error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Kind of error with a request.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum InvalidRequestErrorKind {
    /// Method not found.
    NotFound,
    /// Provided input is invalid.
    Input,
    /// Request is forbidden.
    Forbidden,
    /// Request requires authorization.
    Unauthorized,
    /// Request is malformed and could not have been decoded.
    Malformed,
    /// Other error.
    Other,
}

/// Provided method is invalid.
#[derive(Debug)]
pub struct InvalidMethodError {
    reason: InvalidMethodErrorReason,
}

impl InvalidMethodError {
    pub(crate) fn empty() -> Self {
        Self {
            reason: InvalidMethodErrorReason::Empty,
        }
    }

    pub(crate) fn invalid_character(character: char, position: usize) -> Self {
        Self {
            reason: InvalidMethodErrorReason::InvalidCharacter {
                character,
                position,
            },
        }
    }
}

impl std::fmt::Display for InvalidMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            InvalidMethodErrorReason::Empty => f.write_str("method must not be empty"),
            InvalidMethodErrorReason::InvalidCharacter {
                character,
                position,
            } => {
                write!(
                    f,
                    "invalid character {character} in method at position {position}"
                )
            }
        }
    }
}

#[derive(Debug)]
enum InvalidMethodErrorReason {
    Empty,
    InvalidCharacter { character: char, position: usize },
}

impl From<InvalidMethodError> for InvalidRequestError {
    fn from(value: InvalidMethodError) -> Self {
        InvalidRequestError::new(
            InvalidRequestErrorKind::Malformed,
            format!("unable to decode method: {value}"),
        )
    }
}

/// Request seems valid but cannot be fulfilled (callee's fault).
#[derive(Debug, Clone)]
pub struct InternalError {
    /// Error message.
    message: String,
}

impl InternalError {
    /// Create a new [`InternalError`] with the provided message.
    pub fn new<M: Display>(message: M) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    /// Error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}
