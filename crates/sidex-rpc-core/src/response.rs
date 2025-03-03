//! Data structures for RPC responses.

use crate::{
    codec::TryEncodeFrom,
    error::{CallError, CallResult, InternalError},
    extensions::Extensions,
};

/// RPC response.
pub struct Response<O> {
    /// Parts of the response.
    pub parts: ResponseParts,
    /// Result.
    pub result: CallResult<O>,
}

impl<O> Response<O> {
    /// Construct a [`Response`] from the provided [`CallResult`].
    pub fn from_result(result: CallResult<O>) -> Self {
        Self {
            parts: ResponseParts::default(),
            result,
        }
    }

    /// Create a [`Response`] from the provided parts and output.
    pub fn from_parts(parts: ResponseParts, result: CallResult<O>) -> Self {
        Self { parts, result }
    }

    /// Convert this response into its parts and result.
    pub fn into_parts(self) -> (ResponseParts, CallResult<O>) {
        (self.parts, self.result)
    }

    /// Encode the response.
    pub fn encode<T>(self) -> Result<Response<T>, (T::Error, Response<T>)>
    where
        T: TryEncodeFrom<O>,
    {
        let (parts, result) = self.into_parts();
        match result.map(|input| T::try_encode_from(input)) {
            Ok(Ok(value)) => Ok(Response::from_parts(parts, Ok(value))),
            Ok(Err(error)) => {
                let response = Response::from_parts(
                    parts,
                    Err(CallError::Internal(InternalError::new(format_args!(
                        "unable to encode response: {error}"
                    )))),
                );
                Err((error, response))
            }
            Err(error) => Ok(Response::from_parts(parts, Err(error))),
        }
    }
}

/// Parts of a response.
#[derive(Debug, Clone, Default)]
pub struct ResponseParts {
    /// Extensions.
    pub extensions: Extensions,
    /// Private field for future extendability.
    _private: (),
}
