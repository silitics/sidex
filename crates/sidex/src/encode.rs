//! Encoding typed values.
//!
//! Types implement [`Encode`] by returning a [`View`] of themselves. A view
//! exposes the data of a value in terms of the Sidex data model without copying
//! it: scalars by value, strings and bytes by reference, and compound values as
//! trait objects. Records and variants provide random access to their fields
//! and payload, and sequences and maps provide their items through callbacks.
//!
//! Views know nothing about formats. A format, such as [`json`], walks a view
//! together with the type's [`Shape`] to produce a freeform value.
//!
//! [`json`]: crate::formats::json
//! [`Shape`]: crate::Shape

use std::borrow::Cow;
use std::fmt;

use crate::Value;
use crate::shape::Shape;
use crate::types::Date;
use crate::types::Timestamp;
use crate::types::Uuid;

/// Types that can be encoded.
pub trait Encode {
    /// Returns a view of the value.
    fn encode(&self) -> View<'_>;
}

/// Borrowed view of a typed value.
pub enum View<'a> {
    /// A `unit` value.
    Unit,
    /// A `bool` value.
    Bool(bool),
    /// An `i8` value.
    I8(i8),
    /// An `i16` value.
    I16(i16),
    /// An `i32` value.
    I32(i32),
    /// An `i64` value.
    I64(i64),
    /// A `u8` value.
    U8(u8),
    /// A `u16` value.
    U16(u16),
    /// A `u32` value.
    U32(u32),
    /// A `u64` value.
    U64(u64),
    /// An `idx` value.
    Idx(usize),
    /// An `f32` value.
    F32(f32),
    /// An `f64` value.
    F64(f64),
    /// A `string` value.
    String(&'a str),
    /// A `bytes` value.
    Bytes(&'a [u8]),
    /// A `uuid` value.
    Uuid(Uuid),
    /// A `timestamp` value.
    Timestamp(Timestamp),
    /// A `date` value.
    Date(Date),
    /// Sequence.
    Seq(&'a dyn SeqView),
    /// Map.
    Map(&'a dyn MapView),
    /// Record.
    Record(&'a dyn RecordView),
    /// Variant.
    Variant(&'a dyn VariantView),
    /// Freeform value.
    ///
    /// Owned values let types whose data is not available as a view, such as
    /// types encoded through another serialization framework, produce one.
    Value(Cow<'a, Value>),
}

/// Sequence of a [`View`].
pub trait SeqView {
    /// Number of elements.
    fn len(&self) -> usize;

    /// Indicates whether the sequence is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Calls `f` for each element in order.
    fn each(
        &self,
        f: &mut dyn FnMut(View<'_>) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError>;
}

/// Map of a [`View`].
pub trait MapView {
    /// Number of entries.
    fn len(&self) -> usize;

    /// Indicates whether the map is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Calls `f` for each entry.
    fn each(
        &self,
        f: &mut dyn FnMut(View<'_>, View<'_>) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError>;
}

/// Record of a [`View`].
pub trait RecordView {
    /// Shape of the record.
    fn shape(&self) -> &'static Shape;

    /// Returns the field with the given index, or `None` if the field is
    /// optional and absent.
    fn field(&self, index: usize) -> Option<View<'_>>;
}

/// Variant of a [`View`].
pub trait VariantView {
    /// Shape of the variant.
    fn shape(&self) -> &'static Shape;

    /// Index of the case.
    fn case(&self) -> usize;

    /// Payload of the case, if it has one.
    fn payload(&self) -> Option<View<'_>>;
}

/// Error encoding a value.
#[derive(Debug, Clone)]
pub struct EncodeError {
    message: String,
}

impl EncodeError {
    /// Creates an error with the given message.
    pub fn new(message: impl fmt::Display) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for EncodeError {}
