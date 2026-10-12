//! Shared maps.

use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;
use std::ops::Index;
use std::sync::Arc;

use indexmap::IndexMap;

use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::encode::Encode;
use crate::encode::EncodeError;
use crate::encode::MapView;
use crate::encode::View;
use crate::shape::Shape;
use crate::shape::ShapeKind;
use crate::shape::Shaped;
use crate::shape::shape_of;

/// Map that preserves the insertion order of its entries and is cheap to
/// clone.
///
/// Clones share the entries. Mutation copies the entries first if they are
/// shared ("copy on write").
pub struct Map<K, V>(Arc<IndexMap<K, V>>);

impl<K, V> Map<K, V> {
    /// Creates an empty map.
    pub fn new() -> Self {
        Self(Arc::new(IndexMap::new()))
    }

    /// Creates an empty map with space for at least `capacity` entries.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(IndexMap::with_capacity(capacity)))
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Indicates whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the entries in order.
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter(self.0.iter())
    }

    /// Iterates over the keys in order.
    pub fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.0.keys()
    }

    /// Iterates over the values in order.
    pub fn values(&self) -> impl ExactSizeIterator<Item = &V> {
        self.0.values()
    }

    /// Removes all entries.
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<K: Hash + Eq, V> Map<K, V> {
    /// Returns the value for `key`.
    pub fn get<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.0.get(key)
    }

    /// Indicates whether the map contains `key`.
    pub fn contains_key<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
    {
        self.0.contains_key(key)
    }
}

impl<K: Hash + Eq + Clone, V: Clone> Map<K, V> {
    /// Returns the value for `key` mutably, copying the entries if they are
    /// shared.
    pub fn get_mut<Q: Hash + Eq + ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
    {
        Arc::make_mut(&mut self.0).get_mut(key)
    }

    /// Inserts an entry and returns the previous value for the key.
    ///
    /// Replacing a value keeps the entry's position. Copies the entries if
    /// they are shared.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        Arc::make_mut(&mut self.0).insert(key, value)
    }

    /// Removes the entry for `key` and returns its value.
    ///
    /// Later entries move forward to preserve the order. Copies the entries if
    /// they are shared.
    pub fn remove<Q: Hash + Eq + ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        Arc::make_mut(&mut self.0).shift_remove(key)
    }

    /// Keeps only the entries for which `keep` returns `true`.
    ///
    /// Copies the entries if they are shared.
    pub fn retain(&mut self, mut keep: impl FnMut(&K, &mut V) -> bool) {
        Arc::make_mut(&mut self.0).retain(|key, value| keep(key, value));
    }
}

impl<K, V> Clone for Map<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Hash + Eq, V: PartialEq> PartialEq for Map<K, V> {
    /// Compares the entries regardless of their order.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: Hash + Eq, V: Eq> Eq for Map<K, V> {}

impl<K: Hash + Eq + Borrow<Q>, Q: Hash + Eq + ?Sized, V> Index<&Q> for Map<K, V> {
    type Output = V;

    /// Returns the value for `key`.
    ///
    /// # Panics
    ///
    /// Panics if the map does not contain `key`.
    fn index(&self, key: &Q) -> &V {
        &self.0[key]
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(Arc::new(iter.into_iter().collect()))
    }
}

impl<K: Hash + Eq, V, const N: usize> From<[(K, V); N]> for Map<K, V> {
    fn from(entries: [(K, V); N]) -> Self {
        entries.into_iter().collect()
    }
}

impl<K: Hash + Eq, V, S> From<std::collections::HashMap<K, V, S>> for Map<K, V> {
    fn from(entries: std::collections::HashMap<K, V, S>) -> Self {
        entries.into_iter().collect()
    }
}

impl<K: Hash + Eq, V> From<std::collections::BTreeMap<K, V>> for Map<K, V> {
    fn from(entries: std::collections::BTreeMap<K, V>) -> Self {
        entries.into_iter().collect()
    }
}

impl<K: Hash + Eq + Clone, V: Clone> Extend<(K, V)> for Map<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        Arc::make_mut(&mut self.0).extend(iter);
    }
}

impl<'m, K, V> IntoIterator for &'m Map<K, V> {
    type Item = (&'m K, &'m V);
    type IntoIter = Iter<'m, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K: Clone, V: Clone> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    /// Iterates over the entries, copying them if they are shared.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(Arc::unwrap_or_clone(self.0).into_iter())
    }
}

#[cfg(feature = "indexmap-2")]
impl<K, V> From<IndexMap<K, V>> for Map<K, V> {
    fn from(entries: IndexMap<K, V>) -> Self {
        Self(Arc::new(entries))
    }
}

#[cfg(feature = "indexmap-2")]
impl<K: Clone, V: Clone> From<Map<K, V>> for IndexMap<K, V> {
    /// Converts the map, copying the entries if they are shared.
    fn from(entries: Map<K, V>) -> Self {
        Arc::unwrap_or_clone(entries.0)
    }
}

/// Iterator over the entries of a [`Map`].
pub struct Iter<'m, K, V>(indexmap::map::Iter<'m, K, V>);

impl<'m, K, V> Iterator for Iter<'m, K, V> {
    type Item = (&'m K, &'m V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {}

/// Owning iterator over the entries of a [`Map`].
pub struct IntoIter<K, V>(indexmap::map::IntoIter<K, V>);

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {}

impl<K: Shaped, V: Shaped> Shaped for Map<K, V> {
    const SHAPE: &'static Shape = &Shape::new(
        "::core::builtins::Map",
        ShapeKind::Map(shape_of::<K>, shape_of::<V>),
    )
    .with_args(&[shape_of::<K>, shape_of::<V>]);
}

impl<K: Encode, V: Encode> Encode for Map<K, V> {
    fn encode(&self) -> View<'_> {
        View::Map(self)
    }
}

impl<K: Encode, V: Encode> MapView for Map<K, V> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn each(
        &self,
        f: &mut dyn FnMut(View<'_>, View<'_>) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError> {
        self.iter()
            .try_for_each(|(key, value)| f(key.encode(), value.encode()))
    }
}

impl<K: Decode + Hash + Eq, V: Decode> Decode for Map<K, V> {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        let mut entries = IndexMap::new();
        decoder.map(&mut |key, value| {
            entries.insert(K::decode(key)?, V::decode(value)?);
            Ok(())
        })?;
        Ok(Self(Arc::new(entries)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mutating a clone leaves the original unchanged, and entries keep their
    /// insertion order, also after removals.
    #[test]
    fn copy_on_write_and_order() {
        let mut original = Map::new();
        original.insert("b", 1);
        original.insert("a", 2);
        original.insert("c", 3);
        let mut clone = original.clone();
        clone.remove("a");
        clone.insert("b", 4);
        assert_eq!(
            original.iter().collect::<Vec<_>>(),
            [(&"b", &1), (&"a", &2), (&"c", &3)]
        );
        assert_eq!(clone.iter().collect::<Vec<_>>(), [(&"b", &4), (&"c", &3)]);
    }
}
