#![doc = include_str!("../README.md")]

use std::{
    any::Any,
    borrow::Borrow,
    collections::HashMap,
    fmt::{Debug, Formatter},
    ops::Deref,
    sync::Arc,
};

use serde::{Deserialize, Deserializer, Serialize};

pub struct InvalidPath;

pub struct Error;

pub struct BoxedDeserializer<'de>(Box<dyn erased_serde::Deserializer<'de>>);

pub struct BoxedSerializer(Box<dyn erased_serde::Serializer>);

pub unsafe trait ValueLayout {
    /// The minimum alignment of the value.
    fn align(&self) -> usize {
        std::mem::align_of_val(self)
    }

    /// The size of the value.
    fn size(&self) -> usize {
        std::mem::size_of_val(self)
    }

    /// The memory layout of the value.
    fn layout(&self) -> std::alloc::Layout {
        std::alloc::Layout::for_value(&self)
    }
}

unsafe impl<T: ?Sized> ValueLayout for T {}

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Sized> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait ValueBase: AsAny + ValueLayout {
    fn get_inner_mut(
        &mut self,
        first: &PathSegment,
        tail: &[PathSegment],
    ) -> Result<&mut dyn Value, Error>;

    fn remove(&mut self, path: &[PathSegment]) -> Result<Box<dyn Value>, Error> {
        Err(Error)
    }
}

pub trait Value: ValueBase {
    fn get_mut(&mut self, path: &[PathSegment]) -> Result<&mut dyn Value, Error>;

    fn swap(&mut self, x: &[PathSegment], y: &[PathSegment]) -> Result<(), Error>;

    fn deserialize_into(
        &mut self,
        deserializer: Box<dyn erased_serde::Deserializer>,
    ) -> Result<(), Error> {
        Err(Error)
    }

    fn set(&mut self, value: Box<dyn Value>);

    /// Serializes the value.
    fn serialize(&self, serializer: BoxedSerializer);

    /// Clones the value into a box.
    fn box_clone(&self) -> Box<dyn Value>;

    /// Formats the value using the given formatter.
    fn debug_fmt(&self, f: &mut Formatter) -> std::fmt::Result;
}

impl Debug for dyn Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}

impl<T: ValueBase + std::fmt::Debug + Clone + Sized + for<'de> Deserialize<'de> + Serialize> Value
    for T
{
    fn get_mut(&mut self, path: &[PathSegment]) -> Result<&mut dyn Value, Error> {
        if let Some((first, tail)) = path.split_first() {
            self.get_inner_mut(first, tail)
        } else {
            Ok(self)
        }
    }

    fn swap(&mut self, x: &[PathSegment], y: &[PathSegment]) -> Result<(), Error> {
        let x = self.get_mut(x)? as *mut dyn Value;
        let y = self.get_mut(y)? as *mut dyn Value;
        drop(self);
        unsafe { swap_values(x, y) }
    }

    fn deserialize_into(
        &mut self,
        deserializer: Box<dyn erased_serde::Deserializer>,
    ) -> Result<(), Error> {
        let result = Self::deserialize(deserializer).map_err(|_| Error)?;
        *self = result;
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }

    fn debug_fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }

    fn serialize(&self, mut serializer: BoxedSerializer) {
        erased_serde::Serialize::erased_serialize(&self, &mut serializer.0);
    }

    fn set(&mut self, mut value: Box<dyn Value>) {
        // Swap the value in the box with the value of self.
        unsafe { swap_values(self, &mut *value) };
        // Box is dropped here.
    }
}

pub type Boxed = Box<dyn Value>;

pub enum PathSegment<'p> {
    Field(&'p str),
    Index(usize),
    Key(&'p dyn Value),
}

unsafe fn swap_values(x: *mut dyn Value, y: *mut dyn Value) -> Result<(), Error> {
    if x == y {
        // Cannot swap because pointers are equal.
        return Ok(());
    }
    let x_val = &*x;
    let y_val = &*y;
    if x_val.type_id() != y_val.type_id() {
        // Cannot swap because types do not match.
        return Err(Error);
    }
    let size = x_val.size();
    if size != y_val.size() {
        // Cannot swap because size does not match.
        return Err(Error);
    }
    if x_val.align() != y_val.align() {
        // Cannot swap because alignment does not match.
        return Err(Error);
    }
    drop(x_val);
    drop(y_val);
    let x = x as *mut u8;
    let y = y as *mut u8;
    let x_end = x.offset(size as isize);
    let y_end = y.offset(size as isize);
    if !(x_end <= y) && !(y_end <= x) {
        // Memory overlaps.
        return Err(Error);
    }
    std::ptr::swap_nonoverlapping(x, y, size);
    Ok(())
}

// use std::any::{Any, TypeId};

// trait SizedValue: 'static {
//     fn type_id(&self) -> TypeId {
//         TypeId::of::<Self>()
//     }

//     fn is(&self, other: &dyn SizedValue) -> bool {
//         self.type_id() == other.type_id()
//     }

//     fn size(&self) -> usize {
//         std::mem::size_of_val(self)
//     }

//     fn align(&self) -> usize {
//         std::mem::align_of_val(self)
//     }
// }

// impl<T: 'static> SizedValue for T {}

// fn swap_x(x: *mut dyn SizedValue, y: *mut dyn SizedValue) -> Result<(), Error> {
//     if x != y {
//         let x_obj = unsafe { &*x };
//         let y_obj = unsafe { &*y };
//         if x_obj.is(y_obj) && x_obj.size() == y_obj.size() && x_obj.align() ==
// y_obj.align() {             let size = x_obj.size();
//             drop(x_obj);
//             drop(y_obj);
//             unsafe { std::ptr::swap_nonoverlapping(x as *mut u8, y as *mut u8, size) };
//             Ok(())
//         } else {
//             // eprintln!(
//             //     "Mismatch! {} {:?} {:?} {} {} {} {}",
//             //     SizedValue::type_id(x_obj),
//             //     SizedValue::type_id(y_obj),
//             //     x.size(),
//             //     y.size(),
//             //     x.align(),
//             //     y.align(),
//             // );
//             Err(Error)
//         }
//     } else {
//         Ok(())
//     }
// }

impl ValueBase for i32 {
    fn get_inner_mut(
        &mut self,
        first: &PathSegment,
        tail: &[PathSegment],
    ) -> Result<&mut dyn Value, Error> {
        Err(Error)
    }
}

impl ValueBase for String {
    fn get_inner_mut(
        &mut self,
        first: &PathSegment,
        tail: &[PathSegment],
    ) -> Result<&mut dyn Value, Error> {
        Err(Error)
    }
}

impl<V: Value> ValueBase for HashMap<String, V> {
    fn get_inner_mut(
        &mut self,
        first: &PathSegment,
        tail: &[PathSegment],
    ) -> Result<&mut dyn Value, Error> {
        match first {
            PathSegment::Field(key) => {
                match self.get_mut(*key) {
                    Some(value) => value.get_mut(tail),
                    None => Err(Error),
                }
            }
            _ => todo!(),
        }
    }

    fn remove(&mut self, path: &[PathSegment]) -> Result<Box<dyn Value>, Error> {
        if let Some((first, tail)) = path.split_first() {
            match first {
                PathSegment::Field(key) => {
                    if tail.is_empty() {
                        match self.remove(*key) {
                            Some(value) => Ok(Box::new(value)),
                            None => Err(Error),
                        }
                    } else {
                        match self.get_mut(*key) {
                            Some(value) => value.remove(tail),
                            None => Err(Error),
                        }
                    }
                }
                _ => todo!(),
            }
        } else {
            Err(Error)
        }
    }
}

/// A zero-copy buffer.
pub struct Buffer(Arc<Vec<u8>>);

impl Buffer {
    fn make_ref<T: ?Sized>(&self, value: &T) -> Ref<T> {
        let buffer = Arc::into_raw(self.0.clone());
        Ref { buffer, value }
    }

    pub fn bytes(&self, offset: usize, len: usize) -> Ref<[u8]> {
        self.make_ref(&self.0.as_ref()[offset..offset + len])
    }

    pub fn str(&self, offset: usize, len: usize) -> Result<Ref<str>, std::str::Utf8Error> {
        let value = std::str::from_utf8(&self.0.as_ref()[offset..offset + len])?;
        Ok(self.make_ref(value))
    }
}

pub enum Cow<T: ?Sized + ToOwned> {
    Owned(T::Owned),
    Ref(Ref<T>),
}

impl<T: ?Sized + ToOwned> Deref for Cow<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Cow::Owned(owned) => owned.borrow(),
            Cow::Ref(buffered) => buffered.deref(),
        }
    }
}

pub struct Ref<T: ?Sized> {
    buffer: *const Vec<u8>,
    value: *const T,
}

impl<T: ?Sized> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<T: ?Sized> Drop for Ref<T> {
    fn drop(&mut self) {
        let buffer = unsafe { Arc::from_raw(self.buffer) };
        drop(buffer)
    }
}

pub fn set_from_deserializer<'de, D: 'static + Deserializer<'de>>(
    root: &mut dyn Value,
    path: &[PathSegment],
    deserializer: D,
) -> Result<(), Error> {
    let value = root.get_mut(path)?;
    let boxed = Box::new(<dyn erased_serde::Deserializer>::erase(deserializer));
    value.deserialize_into(boxed)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Serialize, Clone, Debug)]
    struct X {
        x: i32,
        y: i32,
        z: String,
    }

    impl ValueBase for X {
        fn get_inner_mut(
            &mut self,
            first: &PathSegment,
            tail: &[PathSegment],
        ) -> Result<&mut dyn Value, Error> {
            match first {
                PathSegment::Field(field) => {
                    match *field {
                        "x" => self.x.get_mut(tail),
                        "y" => self.y.get_mut(tail),
                        "z" => self.z.get_mut(tail),
                        _ => Err(Error),
                    }
                }
                _ => todo!(),
            }
        }
    }

    #[test]
    pub fn test_swap_x() {
        let mut x = 5;
        let mut y = 8;
        assert!(unsafe { swap_values(&mut x, &mut y) }.is_ok());
        assert_eq!(x, 8);
        assert_eq!(y, 5);

        //assert!(y.swap("", "").is_ok());

        let mut x = X {
            x: 42,
            y: 405,
            z: "abc".to_owned(),
        };

        let x_path = [PathSegment::Field("x")];
        let y_path = [PathSegment::Field("y")];
        let z_path = [PathSegment::Field("z")];

        assert!(x.swap(&x_path, &y_path).is_ok());
        assert_eq!(x.x, 405);
        assert_eq!(x.y, 42);

        assert!(x.swap(&x_path, &z_path).is_err());

        // let mut abc = [1, 2, 3, 4];

        // unsafe { swap_values() }
        // println!("{:?}", abc);
    }
}

// pub struct Error;

// pub enum PrimitiveMut<'v> {
//     Bool(&'v mut bool),

//     U8(&'v mut u8),
//     U16(&'v mut u16),
//     U32(&'v mut u32),
//     U64(&'v mut u64),

//     I8(&'v mut i8),
//     I16(&'v mut i16),
//     I32(&'v mut i32),
//     I64(&'v mut i64),

//     Idx(&'v mut usize),
// }

// pub unsafe fn swap_any(x: *mut dyn Any, y: *mut dyn Any) {
//     if x != y {
//         let x = unsafe { &mut *x };
//         let y = unsafe { &mut *y };
//     }
// }

// pub trait Value: 'static {
//     fn get_mut(&mut self, path: &str) -> Result<&mut dyn Value, Error> {
//         Err(Error)
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any;

//     fn try_as_primitive_mut(&mut self) -> Result<PrimitiveMut, Error> {
//         Err(Error)
//     }

//     fn try_swap(&mut self, other: &mut dyn Value) -> Result<(), Error> {
//         Err(Error)
//     }
// }

// pub fn swap(v: &mut dyn Value, p1: &str, p2: &str) -> Result<(), Error> {
//     let x = v.get_mut(p1)? as *mut dyn Value;
//     let y = v.get_mut(p2)? as *mut dyn Value;
//     if x != y {
//         let x = unsafe { &mut *x };
//         let y = unsafe { &mut *y };
//         x.try_swap(y)?;
//     }
//     Ok(())
// }

// impl Value for i8 {
//     fn try_as_primitive_mut(&mut self) -> Result<PrimitiveMut, Error> {
//         Ok(PrimitiveMut::I8(self))
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }

//     fn try_swap(&mut self, other: &mut dyn Value) -> Result<(), Error> {
//         match other.as_any_mut().downcast_mut::<Self>() {
//             Some(other) => {
//                 std::mem::swap(self, other);
//                 Ok(())
//             }
//             None => todo!(),
//         }
//     }
// }

// pub trait Transformation {
//     fn transform(&self, value: &mut dyn Value) -> Result<(), Error>;
// }

// pub struct Increment;

// impl Transformation for Increment {
//     fn transform(&self, value: &mut dyn Value) -> Result<(), Error> {
//         match value.try_as_primitive_mut()? {
//             PrimitiveMut::U8(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::U16(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::U32(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::U64(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::I8(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::I16(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::I32(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::I64(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             PrimitiveMut::Idx(v) => {
//                 *v = v.wrapping_add(1);
//             }
//             _ => return Err(Error),
//         };
//         Ok(())
//     }
// }

// pub struct Increment;

// pub trait Operation<T> {
//     fn apply(&self, value: &mut T);
// }

// pub struct DynOp<'o, T> {
//     op: &'o dyn Operation<T>,
// }

// trait Apply {
//     fn apply(&self, value: &mut dyn Any);
// }

// impl<'o, T: Any> Apply for DynOp<'o, T> {
//     fn apply(&self, value: &mut dyn Any) {
//         value.downcast_mut::<T>().map(|value| self.op.apply(value));
//     }
// }

// pub enum PathSegment<'p> {
//     String(&'p str),
//     Index(usize),
//     Key(Value<'p>),
// }

// pub trait Next<T> {
//     fn next(&mut self) -> Option<T>;
// }

// type Path<'p> = &'p mut dyn Next<&'p str>;

// pub struct Value<'v>(&'v dyn Any);

// impl<'v> Value<'v> {
//     pub fn new<T: Any>(value: &'v T) -> Self {
//         Self(value)
//     }

//     pub fn make<T: Any + Clone>(&self) -> Option<T> {
//         self.0.downcast_ref::<T>().cloned()
//     }
// }

// pub trait SidexType {
//     fn get(&self, path: Path) -> Option<Value<'_>> {
//         todo!()
//     }
// }

// impl SidexType for i8 {
//     fn get(&self, path: Path) -> Option<Value<'_>> {
//         match path.next() {
//             Some(_) => todo!(),
//             None => Some(Value::new(self)),
//         }
//     }
// }

// pub struct InvalidOperation;

// pub enum Primitive<'v> {
//     I8(i8),
//     I16(i16),
//     I32(i32),
//     I64(i64),

//     Str(&'v str),
// }

// impl Value for i8 {
//     fn as_primitive(&self) -> Result<Primitive<'_>, InvalidOperation> {
//         Ok(Primitive::I8(*self))
//     }
// }

// pub trait Value {
//     fn as_primitive(&self) -> Result<Primitive<'_>, InvalidOperation> {
//         Err(InvalidOperation)
//     }

//     #[allow(unused)]
//     fn for_each(&self, visit: &mut dyn FnMut(&dyn Value)) -> Result<(),
// InvalidOperation> {         Err(InvalidOperation)
//     }
// }

// impl<T: Value> Value for Vec<T> {
//     fn for_each(&self, visit: &mut dyn FnMut(&dyn Value)) -> Result<(),
// InvalidOperation> {         self.iter().for_each(|element| visit(element));
//         Ok(())
//     }
// }

// pub type X<'v> = &'v dyn Value;

// pub struct Any<'v>(&'v dyn Value);

// // pub struct Value<'v> {
// //     base_type: &'static str,
// //     data: Data<'v>,
// // }

// // enum Data<'v> {
// //     Any(Box<dyn Any + Send>),

// //     Str(&'v str),
// // }

// // impl<'v> Data<'v> {
// //     pub fn downcast<T: 'static>(self) -> Result<T, Self> {
// //         match self {
// //             Data::Any(boxed) => {
// //                 match boxed.downcast() {
// //                     Ok(inner) => Ok(*inner),
// //                     Err(inner) => Err(Self::Any(inner)),
// //                 }
// //             }
// //             _ => Err(self),
// //         }
// //     }
// // }

// // trait ForEach {
// //     fn for_each(&self, func: &mut dyn FnMut(Value<'_>));
// // }

// // trait ToValue {
// //     fn to_value(&self) -> Value<'_>;
// // }

// // impl<T: ToValue> ForEach for Vec<T> {
// //     fn for_each(&self, func: &mut dyn FnMut(Value<'_>)) {
// //         self.iter()
// //             .map(|element| element.to_value())
// //             .for_each(|element| func(element))
// //     }
// // }

// // struct ValueSequence<'d>(&'d dyn ForEach);
