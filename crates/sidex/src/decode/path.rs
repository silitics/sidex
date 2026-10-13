//! Paths to items within values.

use std::fmt;

use crate::types::Str;

/// Path from the root of a value to one of its items.
///
/// Record fields appear under their JSON names, so paths match the
/// representation that users and other languages see.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Path {
    segments: Vec<PathSegment>,
}

impl Path {
    /// Segments from the root to the item.
    pub fn segments(&self) -> &[PathSegment] {
        &self.segments
    }

    /// Indicates whether the path points to the root.
    pub fn is_root(&self) -> bool {
        self.segments.is_empty()
    }

    /// Prepends a segment.
    pub(crate) fn prepend(&mut self, segment: PathSegment) {
        self.segments.insert(0, segment);
    }
}

impl FromIterator<PathSegment> for Path {
    fn from_iter<I: IntoIterator<Item = PathSegment>>(iter: I) -> Self {
        Self {
            segments: iter.into_iter().collect(),
        }
    }
}

impl fmt::Display for Path {
    /// Formats the path like `devices[3].timeoutMs`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            match segment {
                PathSegment::Key(key) if index == 0 => f.write_str(key)?,
                PathSegment::Key(key) => write!(f, ".{key}")?,
                PathSegment::Index(position) => write!(f, "[{position}]")?,
            }
        }
        Ok(())
    }
}

/// Segment of a [`Path`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    /// Entry of a map or field of a record with the given key.
    Key(Str),
    /// Element of a sequence with the given index.
    Index(usize),
}
