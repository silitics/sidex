//! Built-in types represented by Rust's primitives and standard library.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;

use crate::decode::Decode;
use crate::decode::DecodeError;
use crate::decode::Decoder;
use crate::encode::Encode;
use crate::encode::EncodeError;
use crate::encode::SeqView;
use crate::encode::View;
use crate::shape::Scalar;
use crate::shape::Shape;
use crate::shape::ShapeKind;
use crate::shape::Shaped;
use crate::shape::shape_of;

impl_scalar!(bool, "bool", Bool, bool);
impl_scalar!(i8, "i8", I8, i8);
impl_scalar!(i16, "i16", I16, i16);
impl_scalar!(i32, "i32", I32, i32);
impl_scalar!(i64, "i64", I64, i64);
impl_scalar!(u8, "u8", U8, u8);
impl_scalar!(u16, "u16", U16, u16);
impl_scalar!(u32, "u32", U32, u32);
impl_scalar!(u64, "u64", U64, u64);
impl_scalar!(usize, "idx", Idx, idx);
impl_scalar!(f32, "f32", F32, f32);
impl_scalar!(f64, "f64", F64, f64);

impl Shaped for str {
    const SHAPE: &'static Shape = &Shape::new(
        "::core::builtins::string",
        ShapeKind::Scalar(Scalar::String),
    );
}

impl Encode for str {
    fn encode(&self) -> View<'_> {
        View::String(self)
    }
}

impl Shaped for String {
    const SHAPE: &'static Shape = str::SHAPE;
}

impl Encode for String {
    fn encode(&self) -> View<'_> {
        View::String(self)
    }
}

impl Decode for String {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decoder.string()
    }
}

impl<T: Shaped> Shaped for Vec<T> {
    const SHAPE: &'static Shape = &Shape::new(
        "::core::builtins::Sequence",
        ShapeKind::Sequence(shape_of::<T>),
    )
    .with_args(&[shape_of::<T>]);
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self) -> View<'_> {
        View::Seq(self)
    }
}

impl<T: Encode> SeqView for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn each(
        &self,
        f: &mut dyn FnMut(View<'_>) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError> {
        self.iter().try_for_each(|item| f(item.encode()))
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        let mut items = Vec::new();
        decoder.seq(&mut |item| {
            items.push(T::decode(item)?);
            Ok(())
        })?;
        Ok(items)
    }
}

/// Implements the traits for a map type with an `insert` method.
macro_rules! impl_map {
    ($map:ident<K, V $(, $hasher:ident)?>, $($bounds:tt)*) => {
        impl<K: $crate::Shaped, V: $crate::Shaped $(, $hasher)?> $crate::Shaped
            for $map<K, V $(, $hasher)?>
        {
            const SHAPE: &'static $crate::Shape = &$crate::Shape::new(
                "::core::builtins::Map",
                $crate::shape::ShapeKind::Map(
                    $crate::shape::shape_of::<K>,
                    $crate::shape::shape_of::<V>,
                ),
            )
            .with_args(&[$crate::shape::shape_of::<K>, $crate::shape::shape_of::<V>]);
        }

        impl<K: $crate::Encode, V: $crate::Encode $(, $hasher)?> $crate::Encode
            for $map<K, V $(, $hasher)?>
        {
            fn encode(&self) -> $crate::View<'_> {
                $crate::View::Map(self)
            }
        }

        impl<K: $crate::Encode, V: $crate::Encode $(, $hasher)?> $crate::encode::MapView
            for $map<K, V $(, $hasher)?>
        {
            fn len(&self) -> usize {
                self.len()
            }

            fn each(
                &self,
                f: &mut dyn FnMut(
                    $crate::View<'_>,
                    $crate::View<'_>,
                ) -> Result<(), $crate::EncodeError>,
            ) -> Result<(), $crate::EncodeError> {
                self.iter()
                    .try_for_each(|(key, value)| f(key.encode(), value.encode()))
            }
        }

        impl<
            K: $crate::Decode + $($bounds)*,
            V: $crate::Decode
            $(, $hasher: ::std::hash::BuildHasher + Default)?
        > $crate::Decode for $map<K, V $(, $hasher)?>
        {
            fn decode(
                decoder: &mut dyn $crate::Decoder,
            ) -> Result<Self, $crate::DecodeError> {
                let mut entries = Self::default();
                decoder.map(&mut |key, value| {
                    entries.insert(K::decode(key)?, V::decode(value)?);
                    Ok(())
                })?;
                Ok(entries)
            }
        }
    };
}

impl_map!(HashMap<K, V, S>, Eq + Hash);
impl_map!(BTreeMap<K, V>, Ord);

/// Implements the traits for a pointer type that is transparent in Sidex.
macro_rules! impl_pointer {
    ($pointer:ident) => {
        impl<T: Shaped + ?Sized> Shaped for $pointer<T> {
            const SHAPE: &'static Shape = T::SHAPE;
        }

        impl<T: Encode + ?Sized> Encode for $pointer<T> {
            fn encode(&self) -> View<'_> {
                (**self).encode()
            }
        }

        impl<T: Decode> Decode for $pointer<T> {
            fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
                T::decode(decoder).map($pointer::new)
            }
        }
    };
}

impl_pointer!(Box);
impl_pointer!(Arc);
impl_pointer!(Rc);

/// Implements the traits for a standard library type represented as a string,
/// using its `Display` and `FromStr` implementations.
macro_rules! impl_std_text {
    ($($ty:ty => $path:literal,)*) => {
        $(
            impl Shaped for $ty {
                const SHAPE: &'static Shape = &Shape::new(
                    $path,
                    ShapeKind::Opaque(
                        crate::shape::OpaqueShape::new()
                            .json_types(&[crate::formats::json::Type::String]),
                    ),
                );
            }

            impl Encode for $ty {
                fn encode(&self) -> View<'_> {
                    View::Json(std::borrow::Cow::Owned(crate::types::JsonValue::String(
                        self.to_string().into(),
                    )))
                }
            }

            impl Decode for $ty {
                fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
                    decoder.string()?.parse().map_err(DecodeError::new)
                }
            }
        )*
    };
}

impl_std_text! {
    std::net::IpAddr => "::std::net::IpAddr",
    std::net::Ipv4Addr => "::std::net::Ipv4Addr",
    std::net::Ipv6Addr => "::std::net::Ipv6Addr",
    std::net::SocketAddr => "::std::net::SocketAddr",
}

/// Paths are strings. Paths that are not valid Unicode are encoded lossily.
impl Shaped for std::path::PathBuf {
    const SHAPE: &'static Shape = &Shape::new(
        "::std::path::PathBuf",
        ShapeKind::Opaque(
            crate::shape::OpaqueShape::new().json_types(&[crate::formats::json::Type::String]),
        ),
    );
}

impl Encode for std::path::PathBuf {
    fn encode(&self) -> View<'_> {
        match self.to_str() {
            Some(path) => View::String(path),
            None => {
                View::Json(std::borrow::Cow::Owned(crate::types::JsonValue::String(
                    self.to_string_lossy().into(),
                )))
            }
        }
    }
}

impl Decode for std::path::PathBuf {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
        decoder.string().map(Self::from)
    }
}

/// Implements the traits for a non-zero integer type with the shape of the
/// underlying integer type.
macro_rules! impl_non_zero {
    ($($ty:ident => $int:ident, $scalar:ident;)*) => {
        $(
            impl Shaped for std::num::$ty {
                const SHAPE: &'static Shape = <$int>::SHAPE;
            }

            impl Encode for std::num::$ty {
                fn encode(&self) -> View<'_> {
                    View::$scalar(self.get())
                }
            }

            impl Decode for std::num::$ty {
                fn decode(decoder: &mut dyn Decoder) -> Result<Self, DecodeError> {
                    Self::new(decoder.$int()?).ok_or_else(|| DecodeError::new("expected a non-zero integer"))
                }
            }
        )*
    };
}

impl_non_zero! {
    NonZeroU8 => u8, U8;
    NonZeroU16 => u16, U16;
    NonZeroU32 => u32, U32;
    NonZeroU64 => u64, U64;
    NonZeroI8 => i8, I8;
    NonZeroI16 => i16, I16;
    NonZeroI32 => i32, I32;
    NonZeroI64 => i64, I64;
}
