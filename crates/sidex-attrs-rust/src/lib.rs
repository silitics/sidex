use std::{ops::Deref, str::FromStr};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sidex_syntax::ast;

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
}

pub struct Inner {
    pub visibility: Visibility,
}

#[derive(Debug, Default)]
pub struct TypeAttrs {
    pub derive: Derive,
}

impl TryFrom<ast::Meta> for TypeAttrs {
    type Error = ();

    fn try_from(value: ast::Meta) -> Result<Self, Self::Error> {
        let mut attrs = TypeAttrs::default();

        let mut stack = vec![&value];
        while let Some(top) = stack.pop() {
            match top {
                ast::Meta::List(other) => stack.extend(other),
                ast::Meta::Invocation { identifier, args } => {
                    match identifier.as_str() {
                        "derive" => {
                            match args.deref() {
                                ast::Meta::Identifier(identifier) => {
                                    attrs
                                        .derive
                                        .positive
                                        .push(TokenStream::from_str(identifier.as_str()).unwrap());
                                }
                                ast::Meta::List(traits) => {
                                    for t in traits {
                                        match t {
                                            ast::Meta::Identifier(identifier) => {
                                                attrs.derive.positive.push(
                                                    TokenStream::from_str(identifier.as_str())
                                                        .unwrap(),
                                                );
                                            }
                                            _ => continue,
                                        }
                                    }
                                }
                                _ => continue,
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
