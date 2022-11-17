use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sidex_ir as ir;

/// `type = "<PATH>"`
pub struct Type {
    pub path: syn::Path,
}

/// `derive(...)`
#[derive(Debug, Default)]
pub struct Derive {
    pub positive: Vec<TokenStream>,
}

/// `attr(...)`
pub struct Attr {
    pub attribute: TokenStream,
}

pub enum Visibility {
    Pub,
    Crate,
    Super,
    Private,
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Visibility::Pub => tokens.extend(quote! { pub }),
            Visibility::Crate => tokens.extend(quote! { pub(crate) }),
            Visibility::Super => tokens.extend(quote! { pub(super) }),
            Visibility::Private => {
                // Do nothing. Private by default.
            }
        }
    }
}

/// `wrap = "<PATH>"`
pub struct FieldAttrs {
    pub visibility: Visibility,
    pub attrs: Vec<TokenStream>,
    pub boxed: bool,
}

impl TryFrom<&[ir::Attr]> for FieldAttrs {
    type Error = ();

    fn try_from(value: &[ir::Attr]) -> Result<Self, Self::Error> {
        let visibility = Visibility::Pub;
        let attrs = Vec::new();
        let mut boxed = false;

        let stack = value
            .iter()
            .filter_map(|attr| {
                match &attr.kind {
                    ir::AttrKind::List(list) => {
                        if list.path.as_str() == "rust" {
                            Some(list.elements.iter())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        for attr in stack {
            match &attr.kind {
                ir::AttrKind::Path(path) => {
                    if path.as_str() == "box" {
                        boxed = true
                    }
                }
                _ => continue,
            }
        }

        Ok(Self {
            visibility,
            attrs,
            boxed,
        })
    }
}

pub struct Inner {
    pub visibility: Visibility,
}

#[derive(Debug, Default)]
pub struct TypeAttrs {
    pub derive: Derive,
}

impl TryFrom<&[ir::Attr]> for TypeAttrs {
    type Error = ();

    fn try_from(value: &[ir::Attr]) -> Result<Self, Self::Error> {
        let mut attrs = TypeAttrs::default();

        let mut stack = value
            .iter()
            .filter_map(|attr| {
                match &attr.kind {
                    ir::AttrKind::List(list) => {
                        if list.path.as_str() == "rust" {
                            Some(list.elements.iter())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        while let Some(top) = stack.pop() {
            match &top.kind {
                ir::AttrKind::List(list) => {
                    match list.path.as_str() {
                        "derive" => {
                            for element in &list.elements {
                                if let ir::AttrKind::Path(path) = &element.kind {
                                    attrs
                                        .derive
                                        .positive
                                        .push(TokenStream::from_str(path.as_str()).unwrap())
                                }
                            }
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }

        Ok(attrs)
    }
}
