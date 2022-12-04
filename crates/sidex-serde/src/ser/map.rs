use std::cell::RefCell;

use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};

pub struct EntriesList<K, V, I: Iterator<Item = (K, V)>> {
    iterator: RefCell<Option<I>>,
}

impl<K, V, I: Iterator<Item = (K, V)>> EntriesList<K, V, I> {
    pub fn new(iterator: I) -> Self {
        Self {
            iterator: RefCell::new(Some(iterator)),
        }
    }
}

impl<K: Serialize, V: Serialize, I: Iterator<Item = (K, V)>> Serialize for EntriesList<K, V, I> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut iterator = self
            .iterator
            .borrow_mut()
            .take()
            .expect("Cannot serialize entry iterator more than once!");
        let (_, hint) = iterator.size_hint();
        let mut seq = serializer.serialize_seq(hint)?;
        while let Some(entry) = iterator.next() {
            seq.serialize_element(&entry)?;
        }
        seq.end()
    }
}

pub struct Map<K, V, I: Iterator<Item = (K, V)>> {
    iterator: RefCell<Option<I>>,
}

impl<K, V, I: Iterator<Item = (K, V)>> Map<K, V, I> {
    pub fn new(iterator: I) -> Self {
        Self {
            iterator: RefCell::new(Some(iterator)),
        }
    }
}

impl<K: Serialize, V: Serialize, I: Iterator<Item = (K, V)>> Serialize for Map<K, V, I> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut iterator = self
            .iterator
            .borrow_mut()
            .take()
            .expect("Cannot serialize entry iterator more than once!");
        let (_, hint) = iterator.size_hint();
        let mut seq = serializer.serialize_map(hint)?;
        while let Some((key, value)) = iterator.next() {
            seq.serialize_entry(&key, &value)?;
        }
        seq.end()
    }
}
