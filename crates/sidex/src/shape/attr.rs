//! Raw attributes.

/// Raw attribute as written in the schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attr {
    /// Bare path, for instance, `default`.
    Path(&'static str),
    /// List of the form `path(args)`.
    List {
        /// Path of the attribute.
        path: &'static str,
        /// Arguments of the list.
        args: &'static [Attr],
    },
    /// Assignment of the form `path = value`.
    Assign {
        /// Path of the attribute.
        path: &'static str,
        /// Assigned value.
        value: AttrValue,
    },
    /// Positional value inside a list.
    Value(AttrValue),
}

/// Value of a raw attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrValue {
    /// Boolean.
    Bool(bool),
    /// Number as written in the schema.
    Number(&'static str),
    /// String.
    String(&'static str),
    /// Path.
    Path(&'static str),
    /// Verbatim source of a `{ ... }` block without the braces.
    Tokens(&'static str),
}
