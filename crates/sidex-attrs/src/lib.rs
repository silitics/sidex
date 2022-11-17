#![doc = include_str!("../README.md")]
//!
//! **ℹ️ Note:** You only need this crate, if you want to convert attributes. In most
//! cases, in particular, when writing code generators, you should use this crate via its
//! re-export in [`sidex-gen`](https://docs.rs/sidex-gen/). This ensures that all
//! dependencies stay in sync.
//!
//! This crate exposes the following crates in its public API:
//!
//! - [`sidex-diagnostics`][sidex_diagnostics]: For the result type of fallible
//!   conversions.
//! - [`sidex-ir`][sidex_ir]: For the data structures of Sidex IR attributes.
//!
//!
//! ## Fallible Conversions
//!
//! This crate defines three traits for converting [`ir::Attr`] to native Rust
//! structures.

use sidex_diagnostics::Diagnostic;
use sidex_ir as ir;

/// An error that may appear during conversion.
pub type Error = Diagnostic;

/// A result of a fallible conversion.
pub type Result<T> = std::result::Result<T, Error>;

/// Tries to apply an attribute to an already existing structure.
///
/// The existing structure is modified to take into account the attribute.
pub trait TryApplyAttr {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> Result<()>;
}

/// Tries to convert an attribute to `Self`.
pub trait TryFromAttr: Sized {
    fn try_from_attr(attr: &ir::Attr) -> Result<Self>;
}

/// Tries to convert a sequence of attributes to `Self`.
pub trait TryFromAttrs: Sized {
    fn try_from_attrs<'a, I: IntoIterator<Item = &'a ir::Attr>>(attrs: I) -> Result<Self>;
}

impl<T: Default + TryApplyAttr> TryFromAttr for T {
    fn try_from_attr(attr: &ir::Attr) -> Result<Self> {
        let mut result = Self::default();
        result.try_apply_attr(attr)?;
        Ok(result)
    }
}

impl<T: Default + TryApplyAttr> TryFromAttrs for T {
    fn try_from_attrs<'a, I: IntoIterator<Item = &'a ir::Attr>>(attrs: I) -> Result<Self> {
        let mut result = Self::default();
        for attr in attrs {
            result.try_apply_attr(attr)?;
        }
        Ok(result)
    }
}

/// Helper macro for *rejecting* an attribute in a conversion.
#[macro_export]
macro_rules! reject {
    ($attr:expr, $($arg:tt)*) => {
        return Err(Diagnostic::error(format!($($arg)*)));
    };
}

/// Helper macro for *accepting* an attribute in a conversion.
#[macro_export]
macro_rules! accept {
    () => {
        return Ok(());
    };
    ($value:expr) => {
        return Ok($value);
    };
}
