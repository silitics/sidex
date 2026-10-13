//! Decoding typed values.
//!
//! Types implement [`Decode`] by requesting the data they expect from a
//! [`Decoder`]. Scalars are returned directly. Compound values are decoded
//! through callbacks that receive a decoder for each element, entry, field, or
//! payload, so nested decoders live on the stack.
//!
//! Decoders are provided by formats, such as [`json`], which translate the
//! requests into reads of a JSON value. Errors record the path to the offending
//! item while they propagate through the callbacks.
//!
//! [`json`]: crate::formats::json

use std::fmt;

use crate::JsonValue;
use crate::shape::Shape;
use crate::types::Date;
use crate::types::Timestamp;
use crate::types::Uuid;
use crate::value::Path;
use crate::value::PathSegment;

/// Types that can be decoded.
pub trait Decode: Sized {
    /// Decodes a value.
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError>;
}

/// Source of a typed value.
///
/// Each method decodes the value as the requested type. A decoder decodes at
/// most one value.
pub trait Decoder {
    /// Decodes a `bool`.
    fn bool(&mut self) -> Result<bool, DecodeError>;

    /// Decodes an `i8`.
    fn i8(&mut self) -> Result<i8, DecodeError>;

    /// Decodes an `i16`.
    fn i16(&mut self) -> Result<i16, DecodeError>;

    /// Decodes an `i32`.
    fn i32(&mut self) -> Result<i32, DecodeError>;

    /// Decodes an `i64`.
    fn i64(&mut self) -> Result<i64, DecodeError>;

    /// Decodes a `u8`.
    fn u8(&mut self) -> Result<u8, DecodeError>;

    /// Decodes a `u16`.
    fn u16(&mut self) -> Result<u16, DecodeError>;

    /// Decodes a `u32`.
    fn u32(&mut self) -> Result<u32, DecodeError>;

    /// Decodes a `u64`.
    fn u64(&mut self) -> Result<u64, DecodeError>;

    /// Decodes an `idx`.
    fn idx(&mut self) -> Result<usize, DecodeError>;

    /// Decodes an `f32`.
    fn f32(&mut self) -> Result<f32, DecodeError>;

    /// Decodes an `f64`.
    fn f64(&mut self) -> Result<f64, DecodeError>;

    /// Decodes a `string`.
    fn string(&mut self) -> Result<String, DecodeError>;

    /// Decodes `bytes`.
    fn bytes(&mut self) -> Result<Vec<u8>, DecodeError>;

    /// Decodes a `uuid`.
    fn uuid(&mut self) -> Result<Uuid, DecodeError>;

    /// Decodes a `timestamp`.
    fn timestamp(&mut self) -> Result<Timestamp, DecodeError>;

    /// Decodes a `date`.
    fn date(&mut self) -> Result<Date, DecodeError>;

    /// Decodes a sequence, calling `f` with a decoder for each element.
    fn seq(
        &mut self,
        f: &mut dyn FnMut(&mut dyn Decoder) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError>;

    /// Decodes a map, calling `f` with a key decoder and a value decoder for
    /// each entry.
    #[expect(
        clippy::type_complexity,
        reason = "spelling out callbacks keeps the trait readable in its documentation"
    )]
    fn map(
        &mut self,
        f: &mut dyn FnMut(&mut dyn Decoder, &mut dyn Decoder) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError>;

    /// Decodes a record of the given shape, calling `f` with the index of each
    /// present field and a decoder for its value.
    ///
    /// Absent optional fields are skipped. Missing required fields are errors.
    fn record(
        &mut self,
        shape: &'static Shape,
        f: &mut dyn FnMut(usize, &mut dyn Decoder) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError>;

    /// Decodes a variant of the given shape, calling `f` with the index of the
    /// case and a decoder for its payload.
    ///
    /// For cases without payload, the decoder must not be used.
    ///
    /// Some formats cannot tell the case from the input alone, for instance,
    /// JSON for implicitly tagged variants. They try the cases in
    /// declaration order by calling `f` for each, until a call succeeds. Hence,
    /// `f` must only have effects if it succeeds.
    fn variant(
        &mut self,
        shape: &'static Shape,
        f: &mut dyn FnMut(usize, &mut dyn Decoder) -> Result<(), DecodeError>,
    ) -> Result<(), DecodeError>;

    /// Decodes a JSON value.
    fn json(&mut self) -> Result<JsonValue, DecodeError>;
}

/// Error decoding a value.
#[derive(Debug, Clone)]
pub struct DecodeError {
    path: Path,
    message: String,
    unknown_field: bool,
}

impl DecodeError {
    /// Creates an error with the given message at the current position.
    pub fn new(message: impl fmt::Display) -> Self {
        Self {
            path: Path::default(),
            message: message.to_string(),
            unknown_field: false,
        }
    }

    /// Creates an error for an item of the wrong kind.
    pub fn unexpected(found: &str, expected: &str) -> Self {
        Self::new(format_args!("expected {expected}, found {found}"))
    }

    /// Creates an error for a field that the record does not define.
    ///
    /// The error's path points to the field.
    pub fn unknown_field(key: &str) -> Self {
        Self {
            unknown_field: true,
            ..Self::new(format_args!("unknown field `{key}`"))
        }
        .within(PathSegment::Key(key.into()))
    }

    /// Indicates whether the error rejects a field unknown to a record.
    ///
    /// Other messages may quote offending input. The path and message of an
    /// unknown field error only contain keys and indices, so applications can
    /// show them without revealing values, such as secrets in configuration
    /// files.
    pub fn is_unknown_field(&self) -> bool {
        self.unknown_field
    }

    /// Prepends a segment to the error's path.
    ///
    /// Decoders call this when an error propagates out of a nested item.
    pub fn within(mut self, segment: PathSegment) -> Self {
        self.path.prepend(segment);
        self
    }

    /// Path from the root to the offending item.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Message describing the error.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.path.is_root() {
            write!(f, "{}: ", self.path)?;
        }
        f.write_str(&self.message)
    }
}

impl std::error::Error for DecodeError {}
