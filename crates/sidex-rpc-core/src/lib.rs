#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
//! Core data structures and abstractions for Sidex's RPC facilities.
//!
//! This crate provides two core types: [`Request`], representing an RPC request, and
//! [`Response`], representing an RPC response. In addition, it provides the [`AsyncCall`]
//! trait, an abstraction for asynchronous processing of RPC requests into RPC responses,
//! and the [`codec`] module for decoding and encoding of RPC requests and responses.
//!
//! RPC requests consist of a _method_ and an _input_. The method is a string produced by
//! the following grammar:
//!
//! ```plain
//! ⟨method⟩    ::=  [ ⟨method⟩ '.' ] ⟨fragment⟩
//! ⟨fragment⟩  ::=  /[_a-zA-Z][_0-9a-zA-Z]*/
//! ```

use std::pin::Pin;

use bytes::{Buf, Bytes};

use crate::{error::InvalidMethodError, request::Request, response::Response};

pub mod codec;
pub mod error;
pub mod extensions;
pub mod request;
pub mod response;

pub type BoxedCallFuture<O> = Pin<Box<dyn Future<Output = Response<O>> + Send>>;

/// Asynchronously process an RPC [`Request`] into a [`Response`].
pub trait AsyncCall<I, O>: 'static + Send + Sync {
    /// Future type for processing requests.
    type Future: 'static + Future<Output = Response<O>> + Send;

    /// Return a future processing the RPC [`Request`] into a [`Response`].
    fn call(&self, request: Request<I>) -> Self::Future;
}

/// Never type `!`.
type Never = std::convert::Infallible;

/// RPC method.
#[derive(Debug, Clone)]
pub struct Method {
    // INVARIANT: bytes are valid UTF-8
    raw: Bytes,
}

impl Method {
    /// Create [`Method`] from the provided bytes without checking for validity.
    pub(crate) fn unchecked_from_bytes(raw: Bytes) -> Self {
        Self { raw }
    }

    /// Split the method at the last fragment, returning the prefix and the last fragment
    /// as [`Method`].
    pub fn split_last(&self) -> (&str, Method) {
        let (tail, _) = self
            .as_str()
            .rsplit_once('.')
            .unwrap_or(("", self.as_str()));
        let mut head = self.raw.clone();
        head.advance(tail.len() + 1);
        (tail, Method::unchecked_from_bytes(head))
    }

    /// Method string.
    pub fn as_str(&self) -> &str {
        // SAFETY: bytes are valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.raw) }
    }

    /// Method fragments.
    pub fn fragments(&self) -> impl Iterator<Item = &str> {
        self.as_str().split('.')
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::str::FromStr for Method {
    type Err = InvalidMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(InvalidMethodError::empty());
        }
        let mut fragment_start = true;
        for (position, character) in s.char_indices() {
            if fragment_start {
                if !character.is_ascii_alphabetic() && character != '_' {
                    return Err(InvalidMethodError::invalid_character(character, position));
                }
                fragment_start = false;
            } else if character == '.' {
                fragment_start = true;
            } else if !character.is_ascii_alphanumeric() && character != '_' {
                return Err(InvalidMethodError::invalid_character(character, position));
            }
        }
        Ok(Self::unchecked_from_bytes(Bytes::copy_from_slice(
            s.as_bytes(),
        )))
    }
}
