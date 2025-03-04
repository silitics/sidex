//! Data structures for RPC requests.

use crate::{
    Method, Never,
    codec::TryDecodeInto,
    error::{CallError, InvalidRequestError, InvalidRequestErrorKind},
    extensions::Extensions,
    response::Response,
};

/// RPC request.
#[derive(Debug, Clone)]
pub struct Request<I> {
    /// Parts of the request.
    pub parts: RequestParts,
    /// Input.
    pub input: I,
}

impl<I> Request<I> {
    /// Create a [`Request`] from the provided parts and input.
    pub fn from_parts(parts: RequestParts, input: I) -> Self {
        Self { parts, input }
    }

    /// Convert this request into its parts and input.
    pub fn into_parts(self) -> (RequestParts, I) {
        (self.parts, self.input)
    }

    /// Decode the request.
    pub fn decode<T>(self) -> Result<Request<T>, (RequestParts, I::Error, Response<Never>)>
    where
        I: TryDecodeInto<T>,
    {
        let (parts, input) = self.into_parts();
        match input.try_decode_into() {
            Ok(input) => Ok(Request::from_parts(parts, input)),
            Err(error) => {
                let response =
                    Response::from_result(Err(CallError::Invalid(InvalidRequestError::new(
                        InvalidRequestErrorKind::Malformed,
                        format_args!("unable to decode request: {error}"),
                    ))));
                Err((parts, error, response))
            }
        }
    }
}

/// Parts of a request.
#[derive(Debug, Clone)]
pub struct RequestParts {
    /// RPC method.
    pub method: Method,
    /// Extensions.
    pub extensions: Extensions,
    /// Private field for future extendability.
    _private: (),
}

impl RequestParts {
    pub fn new(method: Method) -> Self {
        Self {
            method,
            extensions: Extensions::default(),
            _private: (),
        }
    }
}
