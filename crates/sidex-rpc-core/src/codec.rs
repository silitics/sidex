//! Decoding and encoding of requests and responses.

/// Value that might be _decoded into_ a value of type `T`.
pub trait TryDecodeInto<T> {
    /// Error decoding the value.
    type Error: std::error::Error;

    /// Try to decode the value into a value of type `T`.
    fn try_decode_into(self) -> Result<T, Self::Error>;
}

/// Value that might be _encoded from_ a value of type `T`.
pub trait TryEncodeFrom<T>: Sized {
    /// Error encoding the value.
    type Error: std::error::Error;

    /// Try to encode the value from a value of type `T`.
    fn try_encode_from(value: T) -> Result<Self, Self::Error>;
}

/// JSON encoding and decoding via [`serde`].
#[cfg(feature = "json")]
pub mod json {
    use serde::{Serialize, de::DeserializeOwned};

    use super::{TryDecodeInto, TryEncodeFrom};

    /// JSON byte string.
    #[derive(Debug, Clone)]
    #[repr(transparent)]
    pub struct Json<B>(pub B);

    impl<B: AsRef<[u8]>, T: DeserializeOwned> TryDecodeInto<T> for Json<B> {
        type Error = JsonError;

        fn try_decode_into(self) -> Result<T, Self::Error> {
            serde_json::from_slice(self.0.as_ref()).map_err(JsonError)
        }
    }

    impl<T: Serialize> TryEncodeFrom<T> for Json<Vec<u8>> {
        type Error = JsonError;

        fn try_encode_from(value: T) -> Result<Self, Self::Error> {
            serde_json::to_vec(&value).map(Json).map_err(JsonError)
        }
    }

    impl<T: Serialize> TryEncodeFrom<T> for Json<String> {
        type Error = JsonError;

        fn try_encode_from(value: T) -> Result<Self, Self::Error> {
            serde_json::to_string(&value).map(Json).map_err(JsonError)
        }
    }

    /// Error encoding or decoding JSON.
    #[derive(Debug)]
    pub struct JsonError(serde_json::Error);

    impl std::fmt::Display for JsonError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl std::error::Error for JsonError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.0.source()
        }
    }
}

/// Encoding and decoding for `Box<dyn Any>`.
pub mod any {
    use std::any::Any;

    use super::{TryDecodeInto, TryEncodeFrom};
    use crate::Never;

    impl<T: 'static> TryDecodeInto<T> for Box<dyn Any + Send + Sync> {
        type Error = BoxedAnyDecodeError;

        fn try_decode_into(self) -> Result<T, Self::Error> {
            self.downcast().map(|v| *v).map_err(|v| {
                BoxedAnyDecodeError {
                    original: v,
                    expected: std::any::type_name::<T>(),
                }
            })
        }
    }

    impl<T: 'static + Send + Sync> TryEncodeFrom<T> for Box<dyn Any + Send + Sync> {
        type Error = Never;

        fn try_encode_from(value: T) -> Result<Self, Self::Error> {
            Ok(Box::new(value))
        }
    }

    /// Error decoding `Box<dyn Any>` into some target type.
    #[derive(Debug)]
    pub struct BoxedAnyDecodeError {
        pub original: Box<dyn Any + Send + Sync>,
        expected: &'static str,
    }

    impl std::fmt::Display for BoxedAnyDecodeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "expected value of type {}", self.expected)
        }
    }

    impl std::error::Error for BoxedAnyDecodeError {}
}
