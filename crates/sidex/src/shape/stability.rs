//! Stability of definitions, fields, and cases.
//!
//! Schemas mark when items were introduced, deprecated, or are not yet stable
//! with the `since`, `deprecated`, and `unstable` attributes. Shapes carry this
//! information as [`Stability`]:
//!
//! ```
//! use sidex::shape::stability::{Deprecated, Stability};
//!
//! const STABILITY: Stability = Stability::new()
//!     .since("1.4")
//!     .deprecated(Deprecated::new().since("2.0").note("Use `display_name`."));
//! ```

/// Stability of a definition, field, or case.
///
/// Reflects the `since`, `deprecated`, and `unstable` attributes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Stability {
    /// Version that introduced the item.
    pub since: Option<&'static str>,
    /// Deprecation of the item.
    pub deprecated: Option<Deprecated>,
    /// Indicates that the item is not yet stable.
    pub unstable: Option<Unstable>,
}

impl Stability {
    /// Creates the stability of an item without stability attributes.
    pub const fn new() -> Self {
        Self {
            since: None,
            deprecated: None,
            unstable: None,
        }
    }

    /// Sets the version that introduced the item.
    pub const fn since(self, version: &'static str) -> Self {
        Self {
            since: Some(version),
            ..self
        }
    }

    /// Marks the item as deprecated.
    pub const fn deprecated(self, deprecated: Deprecated) -> Self {
        Self {
            deprecated: Some(deprecated),
            ..self
        }
    }

    /// Marks the item as unstable.
    pub const fn unstable(self, unstable: Unstable) -> Self {
        Self {
            unstable: Some(unstable),
            ..self
        }
    }
}

/// Deprecation of an item.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Deprecated {
    /// Version that deprecated the item.
    pub since: Option<&'static str>,
    /// Version that will remove the item.
    pub remove_in: Option<&'static str>,
    /// Explanation, for instance, naming a replacement.
    pub note: Option<&'static str>,
}

impl Deprecated {
    /// Creates a deprecation without further details.
    pub const fn new() -> Self {
        Self {
            since: None,
            remove_in: None,
            note: None,
        }
    }

    /// Sets the version that deprecated the item.
    pub const fn since(self, version: &'static str) -> Self {
        Self {
            since: Some(version),
            ..self
        }
    }

    /// Sets the version that will remove the item.
    pub const fn remove_in(self, version: &'static str) -> Self {
        Self {
            remove_in: Some(version),
            ..self
        }
    }

    /// Sets the explanation.
    pub const fn note(self, note: &'static str) -> Self {
        Self {
            note: Some(note),
            ..self
        }
    }
}

/// Instability of an item.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Unstable {
    /// Feature that gates the item.
    pub feature: Option<&'static str>,
    /// Explanation.
    pub note: Option<&'static str>,
    /// Issue tracking the stabilization.
    pub issue: Option<&'static str>,
}

impl Unstable {
    /// Creates an instability without further details.
    pub const fn new() -> Self {
        Self {
            feature: None,
            note: None,
            issue: None,
        }
    }

    /// Sets the feature that gates the item.
    pub const fn feature(self, feature: &'static str) -> Self {
        Self {
            feature: Some(feature),
            ..self
        }
    }

    /// Sets the explanation.
    pub const fn note(self, note: &'static str) -> Self {
        Self {
            note: Some(note),
            ..self
        }
    }

    /// Sets the issue tracking the stabilization.
    pub const fn issue(self, issue: &'static str) -> Self {
        Self {
            issue: Some(issue),
            ..self
        }
    }
}
