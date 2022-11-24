use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sidex_gen::{
    attrs::{accept, reject, AttrConvertExt, TryApplyAttr, TryFromAttr},
    diagnostics::{Diagnostic, Result},
    ir,
};

/// `type = "<PATH>"`
#[derive(Debug, Clone)]
pub struct Type {
    pub path: String,
}

impl TryFromAttr for Type {
    fn try_from_attr(attr: &ir::Attr) -> Result<Self> {
        match &attr.kind {
            ir::AttrKind::Assign(assign) => {
                if assign.path.as_str() == "type" {
                    let path = String::try_from_attr(&assign.value)?;
                    accept!(Self { path })
                }
            }
            _ => {}
        }
        reject!(attr, "Expected type attribute.")
    }
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
    fn try_from_attr(attr: &ir::Attr) -> Result<Self> {
        match &attr.kind {
            ir::AttrKind::Path(path) => {
                match path.as_str() {
                    "pub" => accept!(Self::Pub),
                    "private" => accept!(Self::Private),
                    _ => {}
                }
            }
            ir::AttrKind::List(list) if list.path.as_str() == "pub" && list.args.len() == 1 => {
                if let ir::AttrKind::Path(path) = &list.args[0].kind {
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
    fn try_from_attr(attr: &ir::Attr) -> Result<Self> {
        attr.expect_path()
            .and_then(|path| {
                match path.as_str() {
                    "box" => accept!(Self::new("::std::boxed::Box")),
                    "arc" => accept!(Self::new("::std::sync::Arc")),
                    "rc" => accept!(Self::new("::std::rc::Rc")),
                    _ => reject!(attr, ""),
                }
            })
            .or_else(|_| -> Result<Self> {
                accept!(Self::new(
                    attr.expect_assign_with("wrap")?
                        .value
                        .expect_path()?
                        .as_str()
                ))
            })
            .or_else(|_| {
                reject!(
                    attr,
                    "Expected wrap attribute: `box`, `arc`, `rc`, or `wrap = \"<PATH>\"."
                )
            })
    }
}

#[derive(Clone, Debug, Default)]
pub struct FieldAttrs {
    pub visibility: Visibility,
    pub wrappers: Vec<Wrapper>,
}

impl TryApplyAttr for FieldAttrs {
    fn try_apply_attr(&mut self, attr: &ir::Attr) -> Result<()> {
        if let ir::AttrKind::List(list) = &attr.kind {
            if list.path.as_str() == "rust" {
                for attr in &list.args {
                    if let Ok(visibility) = Visibility::try_from_attr(attr) {
                        self.visibility = visibility;
                    } else if let Ok(wrapper) = Wrapper::try_from_attr(attr) {
                        self.wrappers.push(wrapper)
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
    pub typ: Option<Type>,
    pub derive: Derive,
    pub attrs: Vec<TokenStream>,
}

impl TryFrom<&[ir::Attr]> for TypeAttrs {
    type Error = Diagnostic;

    fn try_from(value: &[ir::Attr]) -> std::result::Result<Self, Self::Error> {
        let mut attrs = TypeAttrs::default();

        let mut stack = value
            .iter()
            .filter_map(|attr| {
                match &attr.kind {
                    ir::AttrKind::List(list) => {
                        if list.path.as_str() == "rust" {
                            Some(list.args.iter())
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
                ir::AttrKind::Assign(_) => {
                    if let Ok(typ) = Type::try_from_attr(top) {
                        attrs.typ = Some(typ)
                    }
                }
                ir::AttrKind::List(list) => {
                    match list.path.as_str() {
                        "derive" => {
                            for element in &list.args {
                                if let ir::AttrKind::Path(path) = &element.kind {
                                    attrs
                                        .derive
                                        .positive
                                        .push(TokenStream::from_str(path.as_str()).unwrap())
                                }
                            }
                        }
                        "attr" => {
                            attrs
                                .attrs
                                .push(TokenStream::from_str(&list.args[0].to_string()).unwrap())
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
