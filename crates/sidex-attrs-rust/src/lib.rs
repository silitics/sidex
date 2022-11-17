use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sidex_attrs::{accept, reject, TryApplyAttr, TryFromAttr};
use sidex_diagnostics::Diagnostic;
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

/// A *visibility*.
#[derive(Clone, Copy, Debug, Default)]
pub enum Visibility {
    /// Visibility `pub`.
    #[default]
    Pub,
    Crate,
    Super,
    Private,
}

impl TryFromAttr for Visibility {
    fn try_from_attr(attr: &ir::Attr) -> Result<Self, sidex_attrs::Error> {
        match &attr.kind {
            ir::AttrKind::Path(path) => {
                match path.as_str() {
                    "pub" => accept!(Self::Pub),
                    "private" => accept!(Self::Private),
                    _ => {}
                }
            }
            ir::AttrKind::List(list) if list.path.as_str() == "pub" && list.elements.len() == 1 => {
                if let ir::AttrKind::Path(path) = &list.elements[0].kind {
                    match path.as_str() {
                        "crate" => accept!(Self::Crate),
                        "super" => accept!(Self::Super),
                        _ => {}
                    }
                }
            }
            _ => {}
        };
        reject!(
            attr,
            "Expected visibility: `pub`, `pub(crate)`, `pub(super)`, or `private`."
        )
    }
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

#[derive(Clone, Debug)]
pub struct Wrapper {
    pub wrapper: String,
}

impl Wrapper {
    fn new<S: ToString>(wrapper: S) -> Self {
        Self {
            wrapper: wrapper.to_string(),
        }
    }
}

impl TryFromAttr for Wrapper {
    fn try_from_attr(attr: &ir::Attr) -> Result<Self, sidex_attrs::Error> {
        match &attr.kind {
            ir::AttrKind::Path(path) => {
                match path.as_str() {
                    "box" => accept!(Self::new("::std::boxed::Box")),
                    "arc" => accept!(Self::new("::std::sync::Arc")),
                    "rc" => accept!(Self::new("::std::rc::Rc")),
                    _ => {}
                }
            }
            ir::AttrKind::Assign(assign) if assign.path.as_str() == "wrap" => {
                if let ir::AttrKind::Path(path) = &assign.value.kind {
                    accept!(Self::new(path.as_str()))
                }
            }
            _ => {}
        }
        reject!(
            attr,
            "Expected wrap attribute: `box`, `arc`, `rc`, or `wrap = \"<PATH>\"."
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct FieldAttrs {
    pub visibility: Visibility,
    pub wrappers: Vec<Wrapper>,
}

impl TryApplyAttr for FieldAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> Result<(), sidex_attrs::Error> {
        if let ir::AttrKind::List(list) = &attr.kind {
            if list.path.as_str() == "rust" {
                for attr in &list.elements {
                    if let Ok(visibility) = Visibility::try_from_attr(attr) {
                        self.visibility = visibility;
                    } else if let Ok(wrapper) = Wrapper::try_from_attr(attr) {
                        self.wrappers.push(wrapper)
                    } else {
                        reject!(attr, "Expected Rust field attributes.")
                    }
                }
            }
        }
        accept!()
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
