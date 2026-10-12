//! Shared strings.

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::encode::Encode;
use crate::encode::View;
use crate::shape::Shape;
use crate::shape::Shaped;

/// Immutable string that is cheap to clone.
///
/// Clones share the string. Static strings are used without allocation.
#[derive(Clone)]
pub struct Str(Repr);

/// Backing of a [`Str`].
///
/// The backing is private, so that further backings can be added, for
/// instance, slices of a shared buffer.
#[derive(Clone)]
enum Repr {
    Static(&'static str),
    Shared(Arc<str>),
}

impl Str {
    /// Creates a string from a static string without allocating.
    pub const fn from_static(string: &'static str) -> Self {
        Self(Repr::Static(string))
    }

    /// Returns the string as a [`str`].
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Repr::Static(string) => string,
            Repr::Shared(string) => string,
        }
    }
}

impl Default for Str {
    fn default() -> Self {
        Self::from_static("")
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Str {}

impl PartialEq<str> for Str {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Str {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Str {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for Str {
    /// Hashes like [`str`], as required by `Borrow<str>`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl From<&str> for Str {
    fn from(string: &str) -> Self {
        Self(Repr::Shared(string.into()))
    }
}

impl From<String> for Str {
    fn from(string: String) -> Self {
        Self(Repr::Shared(string.into()))
    }
}

impl From<Str> for String {
    fn from(string: Str) -> Self {
        string.as_str().to_owned()
    }
}

impl FromStr for Str {
    type Err = std::convert::Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(string.into())
    }
}

impl Shaped for Str {
    const SHAPE: &'static Shape = str::SHAPE;
}

impl Encode for Str {
    fn encode(&self) -> View<'_> {
        View::String(self.as_str())
    }
}

impl Decode for Str {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decoder.string().map(Str::from)
    }
}
