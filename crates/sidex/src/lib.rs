#![doc = include_str!("../README.md")]

use std::any::Any;

use ::serde::{de::IntoDeserializer, Deserialize, Serialize};
#[cfg(feature = "macros")]
pub use sidex_macros::*;
#[cfg(feature = "serde")]
pub use sidex_serde as serde;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "rpc")]
pub mod rpc {
    pub use sidex_rpc::*;
}

/// A _data type_.
pub trait DataType: 'static + Clone + Send + for<'de> Deserialize<'de> + Serialize {}

impl<T> DataType for T where T: 'static + Clone + Send + for<'de> Deserialize<'de> + Serialize {}

pub trait Embedded: Sized {
    fn deserialize<T: DataType>(&self) -> Result<T, ()>;
    fn serialize<T: DataType>(value: &T) -> Result<Self, ()>;
}

impl Embedded for serde_json::Value {
    fn deserialize<T: DataType>(&self) -> Result<T, ()> {
        T::deserialize(self.clone().into_deserializer()).map_err(|_| ())
    }

    fn serialize<T: DataType>(value: &T) -> Result<Self, ()> {
        serde_json::to_value(value).map_err(|_| ())
    }
}

impl Embedded for Box<dyn Any + Send> {
    fn deserialize<T: DataType>(&self) -> Result<T, ()> {
        self.downcast_ref().cloned().ok_or_else(|| ())
    }

    fn serialize<T: DataType>(value: &T) -> Result<Self, ()> {
        Ok(Box::new(value.clone()))
    }
}
