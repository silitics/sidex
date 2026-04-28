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
//! This crate defines traits for converting [`ir::Attr`] (whole attributes) and
//! [`ir::AttrValue`] (the rhs of an `Assign`) to native Rust structures.

use std::str::FromStr;

#[doc(hidden)]
pub use sidex_diagnostics as diagnostics;
use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Result;
#[doc(hidden)]
pub use sidex_ir as ir;

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

/// Tries to convert an attribute *value* (the rhs of an `Assign`) to `Self`.
pub trait TryFromAttrValue: Sized {
    /// `attr` is the enclosing attribute, used to attach a span to errors.
    fn try_from_attr_value(value: &ir::AttrValue, attr: &ir::Attr) -> Result<Self>;
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
        {
            let attr = $attr;
            return Err($crate::diagnostics::Diagnostic::error(format!($($arg)*)).with_span(attr.span.clone()))
        }
    };
}

/// Helper macro for *accepting* an attribute in a conversion.
#[macro_export]
macro_rules! accept {
    () => {
        return Ok(())
    };
    ($value:expr) => {
        return Ok($value)
    };
}

// --- Primitive AttrValue conversions ---

impl TryFromAttrValue for String {
    fn try_from_attr_value(value: &ir::AttrValue, attr: &ir::Attr) -> Result<Self> {
        match value {
            ir::AttrValue::String(s) => Ok(s.clone()),
            _ => Err(Diagnostic::error("Expected a string value.").with_span(attr.span.clone())),
        }
    }
}

impl TryFromAttrValue for bool {
    fn try_from_attr_value(value: &ir::AttrValue, attr: &ir::Attr) -> Result<Self> {
        match value {
            ir::AttrValue::Bool(b) => Ok(*b),
            _ => Err(Diagnostic::error("Expected a boolean value.").with_span(attr.span.clone())),
        }
    }
}

mod _sealed {
    pub trait Sealed {}

    impl Sealed for super::ir::Attr {}
}

/// Sealed extension trait implemented on [`ir::Attr`] for converting attributes.
pub trait AttrConvertExt: _sealed::Sealed {
    fn convert<T: TryFromAttr>(&self) -> Result<T>;

    fn is_path<P: AsRef<str>>(&self, path: P) -> bool;

    /// The path of a bare-path attribute (`pub`, `derive`, etc.).
    fn expect_path(&self) -> Result<&str>;

    fn expect_list(&self) -> Result<&ir::AttrList>;
    fn expect_list_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrList>;

    fn expect_assign(&self) -> Result<&ir::AttrAssign>;
    fn expect_assign_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrAssign>;

    /// Convenience: extract a string literal from an `Assign` attribute, regardless of path.
    fn expect_string_literal(&self) -> Result<&str>;

    fn expect_from_string<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: Into<Diagnostic>;
}

impl AttrConvertExt for ir::Attr {
    fn convert<T: TryFromAttr>(&self) -> Result<T> {
        T::try_from_attr(self)
    }

    fn is_path<P: AsRef<str>>(&self, expected: P) -> bool {
        match &self.kind {
            ir::AttrKind::Path(path) => path == expected.as_ref(),
            _ => false,
        }
    }

    fn expect_path(&self) -> Result<&str> {
        match &self.kind {
            ir::AttrKind::Path(path) => Ok(path),
            _ => reject!(self, "Expected a path."),
        }
    }

    fn expect_list(&self) -> Result<&ir::AttrList> {
        match &self.kind {
            ir::AttrKind::List(list) => Ok(list),
            _ => reject!(self, "Expected a list attribute."),
        }
    }

    fn expect_list_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrList> {
        let path = path.as_ref();
        self.expect_list().and_then(|list| {
            if list.path == path {
                Ok(list)
            } else {
                reject!(self, "Expected a list attribute with path `{path}`.")
            }
        })
    }

    fn expect_assign(&self) -> Result<&ir::AttrAssign> {
        match &self.kind {
            ir::AttrKind::Assign(assign) => Ok(assign),
            _ => reject!(self, "Expected an assign attribute."),
        }
    }

    fn expect_assign_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrAssign> {
        let path = path.as_ref();
        self.expect_assign().and_then(|assign| {
            if assign.path == path {
                Ok(assign)
            } else {
                reject!(self, "Expected an assign attribute with path `{path}`.")
            }
        })
    }

    fn expect_string_literal(&self) -> Result<&str> {
        let assign = self.expect_assign()?;
        match &assign.value {
            ir::AttrValue::String(s) => Ok(s.as_str()),
            _ => reject!(self, "Expected a string literal."),
        }
    }

    fn expect_from_string<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: Into<Diagnostic>,
    {
        self.expect_string_literal()
            .and_then(|s| T::from_str(s).map_err(Into::into))
    }
}

#[macro_export]
macro_rules! new_assign_attr {
    (
        $(#[$($meta:tt)*])*
        $vis:vis struct $ident:ident[$name:literal]($inner_vis:vis $typ:ty)
    ) => {
        $(#[$($meta)*])*
        #[derive(Debug, Clone)]
        $vis struct $ident($inner_vis $typ);

        impl<T: ::std::convert::Into<$typ>>  ::std::convert::From<T> for $ident {
            fn from(value: T) -> Self {
                Self(value.into())
            }
        }

        impl $crate::TryFromAttr for $ident {
            fn try_from_attr(attr: &$crate::ir::Attr) -> $crate::diagnostics::Result<Self> {
                let assign = $crate::AttrConvertExt::expect_assign_with(attr, $name)?;
                Ok(Self(
                    <$typ as $crate::TryFromAttrValue>::try_from_attr_value(&assign.value, attr)?
                ))
            }
        }
    };
}
