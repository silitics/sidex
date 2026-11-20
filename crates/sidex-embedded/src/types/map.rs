//! The map type.

use core::fmt;

use crate::Decode;
use crate::Encode;
use crate::MapKey;
use crate::error::DecodeError;
use crate::error::EncodeError;
use crate::json::Object;
use crate::json::Reader;
use crate::json::Writer;

/// Map, borrowed from the input or from the application.
///
/// Like [`Seq`](crate::types::Seq), decoding checks every entry but keeps only
/// the input, and iterating decodes the entries again. Entries keep their order.
/// Entries with duplicate keys are kept, and [`get`](Self::get) returns the
/// value of the last one.
///
/// ```
/// use sidex_embedded::json;
/// use sidex_embedded::types::{Map, Str};
///
/// let limits: Map<'_, Str<'_>, u32> = json::from_str(r#"{"cpu": 2, "memory": 512}"#).unwrap();
/// assert_eq!(limits.get("memory"), Some(512));
/// assert_eq!(limits.get("disk"), None);
///
/// let entries = [(Str::new("cpu"), 2_u32)];
/// let limits = Map::new(&entries);
/// assert_eq!(limits.len(), 1);
/// ```
pub struct Map<'a, K, V> {
    repr: Repr<'a, K, V>,
}

enum Repr<'a, K, V> {
    /// Object that has been decoded successfully as a map from `K` to `V`.
    Json {
        text: &'a [u8],
        len: usize,
    },
    Slice(&'a [(K, V)]),
}

impl<'a, K, V> Map<'a, K, V> {
    /// Creates a map with the given entries.
    pub const fn new(entries: &'a [(K, V)]) -> Self {
        Self {
            repr: Repr::Slice(entries),
        }
    }

    /// Number of entries.
    pub const fn len(&self) -> usize {
        match self.repr {
            Repr::Json { len, .. } => len,
            Repr::Slice(entries) => entries.len(),
        }
    }

    /// Indicates whether the map is empty.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, K: MapKey<'a> + Clone, V: Decode<'a> + Clone> Map<'a, K, V> {
    /// Returns an iterator over the entries.
    pub fn iter(&self) -> Iter<'a, K, V> {
        Iter {
            repr: match self.repr {
                Repr::Json { text, len } => {
                    let mut reader = Reader::trusted(text);
                    let object = reader.object().expect("decoded maps are objects");
                    IterRepr::Json {
                        reader,
                        object,
                        remaining: len,
                    }
                }
                Repr::Slice(entries) => IterRepr::Slice(entries.iter()),
            },
        }
    }

    /// Returns the value of the last entry with the given key.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<V>
    where
        K: PartialEq<Q>,
    {
        self.iter()
            .filter(|(candidate, _)| candidate == key)
            .last()
            .map(|(_, value)| value)
    }
}

impl<K, V> Clone for Map<'_, K, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, V> Copy for Map<'_, K, V> {}

impl<K, V> Clone for Repr<'_, K, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K, V> Copy for Repr<'_, K, V> {}

impl<K, V> Default for Map<'_, K, V> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a, K, V> From<&'a [(K, V)]> for Map<'a, K, V> {
    fn from(entries: &'a [(K, V)]) -> Self {
        Self::new(entries)
    }
}

impl<'a, K: MapKey<'a> + Clone, V: Decode<'a> + Clone> IntoIterator for Map<'a, K, V> {
    type Item = (K, V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K, V> PartialEq for Map<'a, K, V>
where
    K: MapKey<'a> + Clone + PartialEq,
    V: Decode<'a> + Clone + PartialEq,
{
    /// Compares the entries in order.
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<'a, K, V> fmt::Debug for Map<'a, K, V>
where
    K: MapKey<'a> + Clone + fmt::Debug,
    V: Decode<'a> + Clone + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'a, K: MapKey<'a>, V: Decode<'a>> Decode<'a> for Map<'a, K, V> {
    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        let start = reader.value_start();
        let mut object = reader.object()?;
        let mut len = 0;
        loop {
            let key_reader = reader.clone();
            let Some(key) = object.next_key(reader)? else {
                break;
            };
            if reader.is_trusted() {
                reader.skip()?;
            } else {
                K::decode_key(key).map_err(|kind| key_reader.error(kind))?;
                V::decode(reader)?;
            }
            len += 1;
        }
        Ok(Self {
            repr: Repr::Json {
                text: reader.input_from(start),
                len,
            },
        })
    }
}

impl<'a, K: MapKey<'a> + Clone, V: Decode<'a> + Encode + Clone> Encode for Map<'a, K, V> {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        for (key, value) in self.iter() {
            value.encode(object.key(&key)?)?;
        }
        object.end()
    }
}

/// Iterator over the entries of a [`Map`].
pub struct Iter<'a, K, V> {
    repr: IterRepr<'a, K, V>,
}

enum IterRepr<'a, K, V> {
    Json {
        reader: Reader<'a>,
        object: Object,
        remaining: usize,
    },
    Slice(core::slice::Iter<'a, (K, V)>),
}

impl<'a, K: MapKey<'a> + Clone, V: Decode<'a> + Clone> Iterator for Iter<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        match &mut self.repr {
            IterRepr::Json {
                reader,
                object,
                remaining,
            } => {
                let key = object.next_key(reader).expect("decoded maps are objects")?;
                *remaining -= 1;
                let key = K::decode_key(key).expect("keys of decoded maps decode again");
                let value = V::decode(reader).expect("values of decoded maps decode again");
                Some((key, value))
            }
            IterRepr::Slice(entries) => entries.next().cloned(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match &self.repr {
            IterRepr::Json { remaining, .. } => *remaining,
            IterRepr::Slice(entries) => entries.len(),
        };
        (len, Some(len))
    }
}

impl<'a, K: MapKey<'a> + Clone, V: Decode<'a> + Clone> ExactSizeIterator for Iter<'a, K, V> {}
