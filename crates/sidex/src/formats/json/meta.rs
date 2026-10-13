//! JSON-specific metadata carried by shapes.

use crate::Shape;

/// JSON representation of an opaque type.
#[derive(Debug)]
#[non_exhaustive]
pub struct Opaque {
    /// JSON types of the representation, or `None` for any JSON value.
    pub types: Option<&'static [Type]>,
}

impl Opaque {
    pub(crate) const ANY: Opaque = Opaque { types: None };

    /// Indicates whether the representation may be a value of the type.
    pub fn admits(&self, ty: Type) -> bool {
        self.types.is_none_or(|types| types.contains(&ty))
    }
}

/// JSON type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    /// The `null` value.
    Null,
    /// Boolean.
    Boolean,
    /// Number.
    Number,
    /// String.
    String,
    /// Array.
    Array,
    /// Object.
    Object,
}

/// JSON representation of a record field.
#[derive(Debug)]
#[non_exhaustive]
pub struct Field {
    /// Name of the field in JSON objects.
    pub name: &'static str,
    /// Indicates whether the fields of the field's record are merged into the
    /// enclosing map.
    pub inline: bool,
    /// Indicates whether 64-bit integers are numbers instead of strings.
    pub number: bool,
}

impl Field {
    pub(crate) const fn new(name: &'static str) -> Self {
        Self {
            name,
            inline: false,
            number: false,
        }
    }
}

/// JSON tagging of a variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tagging {
    /// The tag is a field of a map that also holds the payload.
    ///
    /// The fields of record payloads are merged into the map, unless their case
    /// sets a content field. Other payloads use the content field.
    Internal {
        /// Name of the tag field.
        tag: &'static str,
        /// Name of the field holding payloads that are not merged.
        content: &'static str,
    },
    /// The tag and the payload are separate fields of a map.
    Adjacent {
        /// Name of the tag field.
        tag: &'static str,
        /// Name of the field holding the payload.
        content: &'static str,
    },
    /// The case is the single key of a map holding the payload. Cases without
    /// payload are strings.
    External,
    /// The payload is represented directly. Cases without payload are strings.
    /// Decoding selects the first case in declaration order that matches.
    Implicit,
}

/// JSON representation of a variant case.
#[derive(Debug)]
#[non_exhaustive]
pub struct Case {
    /// Name of the case in JSON.
    pub name: &'static str,
    /// Field holding the payload of an internally or adjacently tagged case,
    /// replacing the content field of the tagging.
    pub content: Option<&'static str>,
    /// Indicates whether a 64-bit integer payload is a number instead of a
    /// string.
    pub number: bool,
}

impl Case {
    pub(crate) const fn new(name: &'static str) -> Self {
        Self {
            name,
            content: None,
            number: false,
        }
    }

    /// Returns the field holding a payload of the given shape, or `None` if the
    /// payload's fields are merged into the map holding the tag.
    pub(crate) fn content_field(&self, tagging: Tagging, payload: &Shape) -> Option<&'static str> {
        match (self.content, tagging) {
            (Some(content), _) => Some(content),
            (None, Tagging::Internal { .. }) if payload.as_record().is_some() => None,
            (None, Tagging::Internal { content, .. } | Tagging::Adjacent { content, .. }) => {
                Some(content)
            }
            (None, Tagging::External | Tagging::Implicit) => None,
        }
    }
}
