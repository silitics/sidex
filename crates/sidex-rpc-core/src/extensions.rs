//! Type map for extending requests and responses with auxiliary data.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

/// Type map for extending requests and responses with auxiliary data.
#[derive(Debug, Clone, Default)]
pub struct Extensions {
    map: HashMap<TypeId, Box<dyn Extension>>,
}

impl Extensions {
    /// Create an empty map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an extension into the map.
    pub fn insert<T: Extension>(&mut self, ext: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(ext));
    }

    /// Get a reference to an extension.
    pub fn get<T: Extension>(&self) -> Option<&T> {
        self.map
            .get(&T::type_id())
            .map(|ext| ext.as_any().downcast_ref().expect("type must match"))
    }

    /// Get a mutable reference to an extension.
    pub fn get_mut<T: Extension>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&T::type_id())
            .map(|ext| ext.as_any_mut().downcast_mut().expect("type must match"))
    }

    /// Remove an extension.
    pub fn remove<T: Extension>(&mut self) -> Option<T> {
        self.map
            .remove(&T::type_id())
            .map(|ext| *ext.into_any().downcast().expect("type must match"))
    }

    /// Clear all extensions.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Return whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Return the number of extensions in the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

/// Value that can be used as an extension.
pub trait Extension: 'static + Debug + Send + Sync {
    /// [`TypeId`] of the type.
    fn type_id() -> TypeId
    where
        Self: Sized;

    /// Clone the extension into a box.
    fn clone_boxed(&self) -> Box<dyn Extension>;

    /// Cast a reference to the extension to `&dyn Any`.
    fn as_any(&self) -> &dyn Any;
    /// Cast a mutable reference to the extension to `&mut dyn Any`.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Convert the extension into `Box<dyn Any>`.
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: 'static + Clone + Debug + Send + Sync> Extension for T {
    fn type_id() -> TypeId
    where
        Self: Sized,
    {
        std::any::TypeId::of::<T>()
    }

    fn clone_boxed(&self) -> Box<dyn Extension> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Clone for Box<dyn Extension> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}
