//! Shared sequences.

use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::encode::Encode;
use crate::encode::EncodeError;
use crate::encode::SeqView;
use crate::encode::View;
use crate::shape::Shape;
use crate::shape::Shaped;

/// Sequence that is cheap to clone.
///
/// Clones share the elements. Mutation copies the elements first if they are
/// shared ("copy on write").
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seq<T>(Arc<Vec<T>>);

impl<T> Seq<T> {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        Self(Arc::new(Vec::new()))
    }

    /// Creates an empty sequence with space for at least `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(Vec::with_capacity(capacity)))
    }

    /// Returns the elements as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
}

impl<T: Clone> Seq<T> {
    /// Returns the elements for mutation, copying them if they are shared.
    pub fn to_mut(&mut self) -> &mut Vec<T> {
        Arc::make_mut(&mut self.0)
    }

    /// Appends an element.
    pub fn push(&mut self, item: T) {
        self.to_mut().push(item);
    }
}

impl<T> Clone for Seq<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for Seq<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for Seq<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

impl<T: Clone> DerefMut for Seq<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.to_mut()
    }
}

impl<T: fmt::Debug> fmt::Debug for Seq<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> AsRef<[T]> for Seq<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T> Borrow<[T]> for Seq<T> {
    fn borrow(&self) -> &[T] {
        self
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for Seq<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: PartialEq> PartialEq<[T]> for Seq<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: Clone> From<&[T]> for Seq<T> {
    fn from(items: &[T]) -> Self {
        Self(Arc::new(items.to_vec()))
    }
}

impl<T, const N: usize> From<[T; N]> for Seq<T> {
    fn from(items: [T; N]) -> Self {
        Self(Arc::new(items.into()))
    }
}

impl<T> From<Vec<T>> for Seq<T> {
    fn from(items: Vec<T>) -> Self {
        Self(Arc::new(items))
    }
}

impl<T: Clone> From<Seq<T>> for Vec<T> {
    fn from(items: Seq<T>) -> Self {
        Arc::unwrap_or_clone(items.0)
    }
}

impl<T> FromIterator<T> for Seq<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(Arc::new(iter.into_iter().collect()))
    }
}

impl<T: Clone> Extend<T> for Seq<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.to_mut().extend(iter);
    }
}

impl<'s, T> IntoIterator for &'s Seq<T> {
    type Item = &'s T;
    type IntoIter = std::slice::Iter<'s, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Clone> IntoIterator for Seq<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Iterates over the elements, copying them if they are shared.
    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
    }
}

impl<T: Shaped> Shaped for Seq<T> {
    const SHAPE: &'static Shape = <Vec<T>>::SHAPE;
}

impl<T: Encode> Encode for Seq<T> {
    fn encode(&self) -> View<'_> {
        View::Seq(self)
    }
}

impl<T: Encode> SeqView for Seq<T> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn each(
        &self,
        f: &mut dyn FnMut(View<'_>) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError> {
        self.iter().try_for_each(|item| f(item.encode()))
    }
}

impl<T: Decode> Decode for Seq<T> {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        Vec::decode(decoder).map(Seq::from)
    }
}
