//! Shared bytes.

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::encode::Encode;
use crate::encode::View;
use crate::shape::Scalar;
use crate::shape::Shape;
use crate::shape::ShapeKind;
use crate::shape::Shaped;

/// Sequence of bytes that is cheap to clone.
///
/// Sidex distinguishes `bytes` from sequences of `u8`, so `bytes` needs a type
/// distinct from [`Seq<u8>`](super::Seq). Clones share the bytes. Mutation
/// copies the bytes first if they are shared ("copy on write").
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(Arc<Vec<u8>>);

impl Bytes {
    /// Creates an empty sequence of bytes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the bytes as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Returns the bytes for mutation, copying them if they are shared.
    pub fn to_mut(&mut self) -> &mut Vec<u8> {
        Arc::make_mut(&mut self.0)
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(Arc::new(bytes))
    }
}

impl From<&[u8]> for Bytes {
    fn from(bytes: &[u8]) -> Self {
        Self::from(bytes.to_vec())
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        Arc::unwrap_or_clone(bytes.0)
    }
}

impl Shaped for Bytes {
    const SHAPE: &'static Shape =
        &Shape::new("::core::builtins::bytes", ShapeKind::Scalar(Scalar::Bytes));
}

impl Encode for Bytes {
    fn encode(&self) -> View<'_> {
        View::Bytes(self.as_slice())
    }
}

impl Decode for Bytes {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decoder.bytes().map(Bytes::from)
    }
}
