//! The sequence type.

use core::fmt;

use crate::Decode;
use crate::Encode;
use crate::error::DecodeError;
use crate::error::EncodeError;
use crate::json::Array;
use crate::json::Reader;
use crate::json::Writer;

/// Sequence, borrowed from the input or from the application.
///
/// Decoding checks every item, but keeps only the input. Iterating decodes the
/// items again, which cannot fail. Items are cheap to decode because they
/// borrow from the input, too.
///
/// ```
/// use sidex_embedded::json;
/// use sidex_embedded::types::{Seq, Str};
///
/// let names: Seq<'_, Str<'_>> = json::from_str(r#"["a", "b"]"#).unwrap();
/// assert_eq!(names.len(), 2);
/// assert!(names.iter().eq(["a", "b"].map(Str::new)));
///
/// let numbers = Seq::new(&[1_u8, 2, 3]);
/// assert_eq!(numbers.get(1), Some(2));
/// ```
pub struct Seq<'a, T> {
    repr: Repr<'a, T>,
}

enum Repr<'a, T> {
    /// Array that has been decoded successfully as a sequence of `T`.
    Json {
        text: &'a [u8],
        len: usize,
    },
    Slice(&'a [T]),
}

impl<'a, T> Seq<'a, T> {
    /// Creates a sequence of the given items.
    pub const fn new(items: &'a [T]) -> Self {
        Self {
            repr: Repr::Slice(items),
        }
    }

    /// Number of items.
    pub const fn len(&self) -> usize {
        match self.repr {
            Repr::Json { len, .. } => len,
            Repr::Slice(items) => items.len(),
        }
    }

    /// Indicates whether the sequence is empty.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T: Decode<'a> + Clone> Seq<'a, T> {
    /// Returns an iterator over the items.
    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            repr: match self.repr {
                Repr::Json { text, len } => {
                    let mut reader = Reader::trusted(text);
                    let array = reader.array().expect("decoded sequences are arrays");
                    IterRepr::Json {
                        reader,
                        array,
                        remaining: len,
                    }
                }
                Repr::Slice(items) => IterRepr::Slice(items.iter()),
            },
        }
    }

    /// Returns the item with the given index.
    ///
    /// For decoded sequences, this decodes the preceding items.
    pub fn get(&self, index: usize) -> Option<T> {
        self.iter().nth(index)
    }
}

impl<T> Clone for Seq<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Seq<'_, T> {}

impl<T> Clone for Repr<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Repr<'_, T> {}

impl<T> Default for Seq<'_, T> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a, T> From<&'a [T]> for Seq<'a, T> {
    fn from(items: &'a [T]) -> Self {
        Self::new(items)
    }
}

impl<'a, T: Decode<'a> + Clone> IntoIterator for Seq<'a, T> {
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T: Decode<'a> + Clone + PartialEq> PartialEq for Seq<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<'a, T: Decode<'a> + Clone + fmt::Debug> fmt::Debug for Seq<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Seq<'a, T> {
    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        let start = reader.value_start();
        let mut array = reader.array()?;
        let mut len = 0;
        while array.next(reader)? {
            if reader.is_trusted() {
                reader.skip()?;
            } else {
                T::decode(reader)?;
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

impl<'a, T: Decode<'a> + Encode + Clone> Encode for Seq<'a, T> {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut array = writer.array()?;
        for item in self.iter() {
            item.encode(array.item()?)?;
        }
        array.end()
    }
}

/// Iterator over the items of a [`Seq`].
pub struct Iter<'a, T> {
    repr: IterRepr<'a, T>,
}

enum IterRepr<'a, T> {
    Json {
        reader: Reader<'a>,
        array: Array,
        remaining: usize,
    },
    Slice(core::slice::Iter<'a, T>),
}

impl<'a, T: Decode<'a> + Clone> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match &mut self.repr {
            IterRepr::Json {
                reader,
                array,
                remaining,
            } => {
                let more = array.next(reader).expect("decoded sequences are arrays");
                if !more {
                    return None;
                }
                *remaining -= 1;
                Some(T::decode(reader).expect("items of decoded sequences decode again"))
            }
            IterRepr::Slice(items) => items.next().cloned(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match &self.repr {
            IterRepr::Json { remaining, .. } => *remaining,
            IterRepr::Slice(items) => items.len(),
        };
        (len, Some(len))
    }
}

impl<'a, T: Decode<'a> + Clone> ExactSizeIterator for Iter<'a, T> {}
