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

use std::{any, fmt, str::FromStr};

#[doc(hidden)]
pub use sidex_diagnostics as diagnostics;
use sidex_diagnostics::{Diagnostic, Result};
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
            let _ = $attr;
            return Err($crate::diagnostics::Diagnostic::error(format!($($arg)*)))
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

impl TryFromAttr for String {
    fn try_from_attr(attr: &ir::Attr) -> Result<Self> {
        Ok(attr.expect_string_literal()?.to_owned())
    }
}

mod _sealed {
    pub trait Sealed {}

    impl Sealed for super::ir::Attr {}
}

/// Sealed extension trait implemented on [`ir::Attr`] for converting attributes.
pub trait AttrConvertExt: _sealed::Sealed {
    fn convert<T: TryFromAttr>(&self) -> Result<T>;

    fn expect_path(&self) -> Result<&ir::Path>;

    fn expect_list(&self) -> Result<&ir::AttrList>;
    fn expect_list_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrList>;

    fn expect_assign(&self) -> Result<&ir::AttrAssign>;
    fn expect_assign_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrAssign>;

    fn expect_literal(&self) -> Result<&ir::Literal>;
    fn expect_string_literal(&self) -> Result<&str>;

    fn expect_from_string<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: Into<Diagnostic>;
}

impl AttrConvertExt for ir::Attr {
    fn expect_path(&self) -> Result<&ir::Path> {
        match &self.kind {
            ir::AttrKind::Path(path) => accept!(path),
            _ => reject!(self, "Expected a path."),
        }
    }

    fn expect_list_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrList> {
        let path = path.as_ref();
        self.expect_list().and_then(|list| {
            if list.path.as_str() == path {
                accept!(list)
            } else {
                reject!(self, "Expected an list attribute with path `{path}`.")
            }
        })
    }

    fn expect_assign_with<P: AsRef<str>>(&self, path: P) -> Result<&ir::AttrAssign> {
        let path = path.as_ref();
        self.expect_assign().and_then(|assign| {
            if assign.path.as_str() == path {
                accept!(assign)
            } else {
                reject!(self, "Expected an assign attribute with path `{path}`.")
            }
        })
    }

    fn expect_list(&self) -> Result<&ir::AttrList> {
        match &self.kind {
            ir::AttrKind::List(list) => accept!(list),
            _ => reject!(self, "Expected a list attribute."),
        }
    }

    fn convert<T: TryFromAttr>(&self) -> Result<T> {
        T::try_from_attr(self)
    }

    fn expect_assign(&self) -> Result<&ir::AttrAssign> {
        match &self.kind {
            ir::AttrKind::Assign(assign) => accept!(assign),
            _ => reject!(self, "Expected an assign attribute."),
        }
    }

    fn expect_literal(&self) -> Result<&ir::Literal> {
        match &self.kind {
            ir::AttrKind::Tokens(tokens) => {
                if tokens.len() == 1 {
                    match &tokens[0].kind {
                        ir::TokenKind::Literal(literal) => accept!(literal),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        reject!(self, "Expected a literal.")
    }

    fn expect_string_literal(&self) -> Result<&str> {
        self.expect_literal().and_then(|literal| {
            match literal {
                ir::Literal::String(string) => accept!(string.as_str()),
                _ => reject!(self, "Expected a string literal."),
            }
        })
    }

    fn expect_from_string<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: Into<Diagnostic>,
    {
        self.expect_string_literal().and_then(|string| {
            match T::from_str(string) {
                Ok(value) => accept!(value),
                Err(err) => Err(err.into()),
            }
        })
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
                Ok(Self(
                    attr
                        .expect_assign_with($name)?
                        .value
                        .convert()?
                ))
            }
        }
    };
}
