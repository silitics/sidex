//! Adapter traits that let codegen-emitted types compose Serde encodings for
//! container types (`Vec`, `HashMap`, `Option`, ...) with custom leaf encodings
//! for primitives whose default Serde encoding doesn't roundtrip cleanly through
//! JavaScript JSON, namely:
//!
//! - 64-bit integers: a JavaScript `Number` is an IEEE-754 double, so values above `2^53`
//!   lose precision when read into a JS runtime. We therefore emit `i64`/`u64` as decimal
//!   strings on the wire for human-readable formats.
//! - Non-finite floats: `NaN` and `±Infinity` have no canonical JSON spelling. We emit
//!   them as `"NaN"` / `"+Infinity"` / `"-Infinity"`.
//! - Bytes: emitted as Base64 strings in human-readable formats.
//!
//! Binary (non-human-readable) formats keep the native Serde encoding.
//!
//! # Pattern
//!
//! Encodings are zero-sized "schema" types that describe how to encode a
//! value at the type level. [`AsSelf`] delegates to the value's own
//! [`Serialize`] / [`Deserialize`] impl; leaf encodings like [`AsU64`],
//! [`AsF64`], or [`AsBytes`] override the wire form for primitives.
//! Container encodings (`Vec<U>`, `Option<U>`, ...) propagate through to
//! their elements via blanket impls, so a field of type `Vec<u64>` is
//! described by the encoding `Vec<AsU64>`.
//!
//! Use [`SerializeAsWrap`] and [`DeserializeAsWrap`] to bridge an encoding
//! to plain Serde's [`Serialize`] / [`Deserialize`] traits when handing
//! values off to a Serde driver.
//!
//! # Example
//!
//! ```ignore
//! use sidex_serde::adapter::*;
//!
//! let xs: Vec<u64> = vec![1, 2, 1u64 << 60];
//! let json = serde_json::to_string(&SerializeAsWrap::<_, Vec<AsU64>>::new(&xs)).unwrap();
//! // -> "[\"1\",\"2\",\"1152921504606846976\"]"
//! ```

use core::fmt;
use core::hash::BuildHasher;
use core::hash::Hash;
use core::marker::PhantomData;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de;
use serde::de::SeqAccess;
use serde::de::Visitor;

use crate::de::sanitize_size_hint;

/// Sentinel string representing `NaN`.
pub const FLOAT_NAN: &str = "NaN";
/// Sentinel string representing positive infinity.
pub const FLOAT_POSITIVE_INFINITY: &str = "+Infinity";
/// Sentinel string representing negative infinity.
pub const FLOAT_NEGATIVE_INFINITY: &str = "-Infinity";

/// JavaScript's `Number.MAX_SAFE_INTEGER` (`2^53 - 1`).
///
/// Integers in `[-MAX_SAFE_INTEGER, MAX_SAFE_INTEGER]` round-trip losslessly
/// through a JS `Number`; values outside this range may lose precision when
/// parsed in JS. `AsI64` / `AsU64` use this threshold to decide whether to
/// emit a value as a JSON number (small values, backward-compatible) or as
/// a decimal string (large values, lossless).
pub const JS_MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// Serializes a value of type `T` using the encoding described by `Self`.
///
/// Implement this trait on an encoding (typically a zero-sized type) to
/// define a Sidex-specific Serde encoding for `T`. Container encodings like
/// `Vec<U>` or `Option<U>` get blanket impls that propagate through to
/// their elements.
pub trait SerializeAs<T: ?Sized> {
    /// Serializes `value` using this encoding.
    fn serialize_as<S: Serializer>(value: &T, serializer: S) -> Result<S::Ok, S::Error>;
}

/// Deserializes a value of type `T` using the encoding described by `Self`.
///
/// Mirrors [`SerializeAs`].
pub trait DeserializeAs<'de, T>: Sized {
    /// Deserializes a `T` using this encoding.
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<T, D::Error>;
}

/// Lifts every Sidex-encodable type to its canonical encoding.
///
/// This is what makes generic types compose: when a generated type is
/// instantiated as `Foo<u64>`, the codegen substitutes
/// `<u64 as SidexType>::Encoding` (= [`AsU64`]) everywhere it sees the type
/// variable — so the inner `u64` is string-encoded even though the outer
/// `Foo` doesn't statically know it's holding a 64-bit integer.
///
/// Primitives, the standard containers (`Option`, `Vec`, `HashMap`,
/// `BTreeMap`, `Box`, `Arc`, `Rc`), and every Sidex-generated type get an
/// impl. For user-defined opaque types (mapped via `#[rust(typ = "...")]`)
/// that you want to use as a generic argument inside a Sidex type, supply
/// the impl yourself with [`impl_sidex_type!`].
pub trait SidexType: Sized {
    /// The Sidex-canonical encoding for `Self`.
    type Encoding: SerializeAs<Self> + for<'de> DeserializeAs<'de, Self>;
}

/// Implements [`SidexType`] for a foreign type.
///
/// Two forms are supported:
///
/// ```ignore
/// // Default encoding is `AsSelf` — delegates to the type's own Serde impl.
/// sidex_serde::impl_sidex_type!(::user::Id);
///
/// // Or pin a specific encoding (e.g., to string-encode an opaque-wrapped u64).
/// sidex_serde::impl_sidex_type!(::user::BigId, sidex_serde::AsU64);
/// ```
#[macro_export]
macro_rules! impl_sidex_type {
    ($t:ty) => {
        impl $crate::SidexType for $t {
            type Encoding = $crate::AsSelf;
        }
    };
    ($t:ty, $encoding:ty) => {
        impl $crate::SidexType for $t {
            type Encoding = $encoding;
        }
    };
}

/// Bridges a [`SerializeAs`] encoding to plain Serde [`Serialize`] by holding
/// a borrowed value alongside its encoding type.
///
/// Pass `&SerializeAsWrap::<T, As>::new(&value)` anywhere a `&impl Serialize`
/// is expected (e.g., to `SerializeMap::serialize_value`).
pub struct SerializeAsWrap<'a, T: ?Sized, As: ?Sized> {
    value: &'a T,
    phantom: PhantomData<As>,
}

impl<'a, T: ?Sized, As: ?Sized> SerializeAsWrap<'a, T, As> {
    /// Wraps `value` so it serializes using `As`.
    #[inline]
    pub fn new(value: &'a T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized, As: ?Sized + SerializeAs<T>> Serialize for SerializeAsWrap<'_, T, As> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        As::serialize_as(self.value, serializer)
    }
}

/// Bridges a [`DeserializeAs`] encoding to plain Serde [`Deserialize`] by
/// holding the deserialized value.
///
/// Use `DeserializeAsWrap::<T, As>::deserialize(deserializer)?.into_inner()` to
/// obtain a `T` decoded with encoding `As`. Most callers can just deserialize
/// `DeserializeAsWrap<T, As>` directly via Serde's normal entry points (e.g.,
/// `MapAccess::next_value::<DeserializeAsWrap<T, As>>()`).
pub struct DeserializeAsWrap<T, As> {
    value: T,
    phantom: PhantomData<As>,
}

impl<T, As> DeserializeAsWrap<T, As> {
    /// Returns the wrapped value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<'de, T, As: DeserializeAs<'de, T>> Deserialize<'de> for DeserializeAsWrap<T, As> {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            value: As::deserialize_as(deserializer)?,
            phantom: PhantomData,
        })
    }
}

/// Encoding that delegates to the value's own [`Serialize`] / [`Deserialize`]
/// impl.
///
/// Use this for any leaf type that's already encoded the way Sidex expects
/// (i.e., everything except 64-bit ints, floats, and bytes).
pub struct AsSelf;

impl<T: ?Sized + Serialize> SerializeAs<T> for AsSelf {
    #[inline]
    fn serialize_as<S: Serializer>(value: &T, serializer: S) -> Result<S::Ok, S::Error> {
        value.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> DeserializeAs<'de, T> for AsSelf {
    #[inline]
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<T, D::Error> {
        T::deserialize(deserializer)
    }
}

/// Encoding for `i64`. Values inside the JS-safe-integer range serialize as
/// JSON numbers; values outside that range serialize as decimal strings.
/// On the way in we accept either form.
pub struct AsI64;

impl SerializeAs<i64> for AsI64 {
    #[inline]
    fn serialize_as<S: Serializer>(value: &i64, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() && value.unsigned_abs() > JS_MAX_SAFE_INTEGER {
            serializer.collect_str(value)
        } else {
            serializer.serialize_i64(*value)
        }
    }
}

impl<'de> DeserializeAs<'de, i64> for AsI64 {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(I64Visitor)
        } else {
            deserializer.deserialize_i64(I64Visitor)
        }
    }
}

/// Encoding for `u64`. Values inside the JS-safe-integer range serialize as
/// JSON numbers; values outside that range serialize as decimal strings. On
/// the way in we accept either form.
pub struct AsU64;

impl SerializeAs<u64> for AsU64 {
    #[inline]
    fn serialize_as<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() && *value > JS_MAX_SAFE_INTEGER {
            serializer.collect_str(value)
        } else {
            serializer.serialize_u64(*value)
        }
    }
}

impl<'de> DeserializeAs<'de, u64> for AsU64 {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(U64Visitor)
        } else {
            deserializer.deserialize_u64(U64Visitor)
        }
    }
}

/// Encoding for `f32`, replacing `NaN` and `±Infinity` with sentinel strings
/// in human-readable formats.
pub struct AsF32;

impl SerializeAs<f32> for AsF32 {
    fn serialize_as<S: Serializer>(value: &f32, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() && !value.is_finite() {
            serializer.serialize_str(non_finite_sentinel(*value as f64))
        } else {
            serializer.serialize_f32(*value)
        }
    }
}

impl<'de> DeserializeAs<'de, f32> for AsF32 {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(F64Visitor).map(|v| v as f32)
        } else {
            deserializer.deserialize_f32(F32Visitor)
        }
    }
}

/// Encoding for `f64`, replacing `NaN` and `±Infinity` with sentinel strings
/// in human-readable formats.
pub struct AsF64;

impl SerializeAs<f64> for AsF64 {
    fn serialize_as<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() && !value.is_finite() {
            serializer.serialize_str(non_finite_sentinel(*value))
        } else {
            serializer.serialize_f64(*value)
        }
    }
}

impl<'de> DeserializeAs<'de, f64> for AsF64 {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(F64Visitor)
        } else {
            deserializer.deserialize_f64(F64Visitor)
        }
    }
}

/// Encoding for `Vec<u8>` as Base64 in human-readable formats and as raw
/// bytes otherwise.
pub struct AsBytes;

impl<T: AsRef<[u8]>> SerializeAs<T> for AsBytes {
    fn serialize_as<S: Serializer>(value: &T, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes = value.as_ref();
        if serializer.is_human_readable() {
            serializer.serialize_str(&base64::encode(bytes))
        } else {
            serializer.serialize_bytes(bytes)
        }
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for AsBytes {
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(BytesVisitor)
        } else {
            deserializer.deserialize_bytes(BytesVisitor)
        }
    }
}

impl<T, U> SerializeAs<Option<T>> for Option<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S: Serializer>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(v) => serializer.serialize_some(&SerializeAsWrap::<T, U>::new(v)),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de, T, U> DeserializeAs<'de, Option<T>> for Option<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Option<T>, D::Error> {
        Option::<DeserializeAsWrap<T, U>>::deserialize(deserializer)
            .map(|opt| opt.map(DeserializeAsWrap::into_inner))
    }
}

impl<T, U> SerializeAs<Vec<T>> for Vec<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S: Serializer>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(value.iter().map(SerializeAsWrap::<T, U>::new))
    }
}

impl<'de, T, U> DeserializeAs<'de, Vec<T>> for Vec<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Vec<T>, D::Error> {
        deserializer.deserialize_seq(SeqVisitor::<T, U>(PhantomData))
    }
}

impl<K, V, KU, VU, S> SerializeAs<HashMap<K, V, S>> for HashMap<KU, VU, S>
where
    KU: SerializeAs<K>,
    VU: SerializeAs<V>,
{
    fn serialize_as<Ser: Serializer>(
        value: &HashMap<K, V, S>,
        serializer: Ser,
    ) -> Result<Ser::Ok, Ser::Error> {
        serializer.collect_map(value.iter().map(|(k, v)| {
            (
                SerializeAsWrap::<K, KU>::new(k),
                SerializeAsWrap::<V, VU>::new(v),
            )
        }))
    }
}

impl<'de, K, V, KU, VU, S> DeserializeAs<'de, HashMap<K, V, S>> for HashMap<KU, VU, S>
where
    KU: DeserializeAs<'de, K>,
    VU: DeserializeAs<'de, V>,
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<HashMap<K, V, S>, D::Error> {
        deserializer.deserialize_map(MapVisitor::<HashMap<K, V, S>, KU, VU>(PhantomData))
    }
}

impl<T, U> SerializeAs<Box<T>> for Box<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S: Serializer>(value: &Box<T>, serializer: S) -> Result<S::Ok, S::Error> {
        SerializeAsWrap::<T, U>::new(value).serialize(serializer)
    }
}

impl<'de, T, U> DeserializeAs<'de, Box<T>> for Box<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Box<T>, D::Error> {
        DeserializeAsWrap::<T, U>::deserialize(deserializer).map(|w| Box::new(w.into_inner()))
    }
}

impl<T, U> SerializeAs<Arc<T>> for Arc<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S: Serializer>(value: &Arc<T>, serializer: S) -> Result<S::Ok, S::Error> {
        SerializeAsWrap::<T, U>::new(value).serialize(serializer)
    }
}

impl<'de, T, U> DeserializeAs<'de, Arc<T>> for Arc<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Arc<T>, D::Error> {
        DeserializeAsWrap::<T, U>::deserialize(deserializer).map(|w| Arc::new(w.into_inner()))
    }
}

impl<T, U> SerializeAs<Rc<T>> for Rc<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S: Serializer>(value: &Rc<T>, serializer: S) -> Result<S::Ok, S::Error> {
        SerializeAsWrap::<T, U>::new(value).serialize(serializer)
    }
}

impl<'de, T, U> DeserializeAs<'de, Rc<T>> for Rc<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<Rc<T>, D::Error> {
        DeserializeAsWrap::<T, U>::deserialize(deserializer).map(|w| Rc::new(w.into_inner()))
    }
}

impl<K, V, KU, VU> SerializeAs<BTreeMap<K, V>> for BTreeMap<KU, VU>
where
    KU: SerializeAs<K>,
    VU: SerializeAs<V>,
{
    fn serialize_as<S: Serializer>(
        value: &BTreeMap<K, V>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_map(value.iter().map(|(k, v)| {
            (
                SerializeAsWrap::<K, KU>::new(k),
                SerializeAsWrap::<V, VU>::new(v),
            )
        }))
    }
}

impl<'de, K, V, KU, VU> DeserializeAs<'de, BTreeMap<K, V>> for BTreeMap<KU, VU>
where
    KU: DeserializeAs<'de, K>,
    VU: DeserializeAs<'de, V>,
    K: Ord,
{
    fn deserialize_as<D: Deserializer<'de>>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error> {
        deserializer.deserialize_map(MapVisitor::<BTreeMap<K, V>, KU, VU>(PhantomData))
    }
}

#[cfg(feature = "indexmap")]
impl<K, V, KU, VU, S> SerializeAs<indexmap::IndexMap<K, V, S>> for indexmap::IndexMap<KU, VU, S>
where
    KU: SerializeAs<K>,
    VU: SerializeAs<V>,
{
    fn serialize_as<Ser: Serializer>(
        value: &indexmap::IndexMap<K, V, S>,
        serializer: Ser,
    ) -> Result<Ser::Ok, Ser::Error> {
        serializer.collect_map(value.iter().map(|(k, v)| {
            (
                SerializeAsWrap::<K, KU>::new(k),
                SerializeAsWrap::<V, VU>::new(v),
            )
        }))
    }
}

#[cfg(feature = "indexmap")]
impl<'de, K, V, KU, VU, S> DeserializeAs<'de, indexmap::IndexMap<K, V, S>>
    for indexmap::IndexMap<KU, VU, S>
where
    KU: DeserializeAs<'de, K>,
    VU: DeserializeAs<'de, V>,
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn deserialize_as<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<indexmap::IndexMap<K, V, S>, D::Error> {
        deserializer.deserialize_map(MapVisitor::<indexmap::IndexMap<K, V, S>, KU, VU>(
            PhantomData,
        ))
    }
}

macro_rules! impl_sidex_primitive {
    ($t:ty, $marker:ty) => {
        impl SidexType for $t {
            type Encoding = $marker;
        }
    };
}

impl_sidex_primitive!(bool, AsSelf);
impl_sidex_primitive!(i8, AsSelf);
impl_sidex_primitive!(i16, AsSelf);
impl_sidex_primitive!(i32, AsSelf);
impl_sidex_primitive!(i64, AsI64);
impl_sidex_primitive!(u8, AsSelf);
impl_sidex_primitive!(u16, AsSelf);
impl_sidex_primitive!(u32, AsSelf);
impl_sidex_primitive!(u64, AsU64);
impl_sidex_primitive!(usize, AsSelf);
impl_sidex_primitive!(isize, AsSelf);
impl_sidex_primitive!(f32, AsF32);
impl_sidex_primitive!(f64, AsF64);
impl_sidex_primitive!(char, AsSelf);
impl_sidex_primitive!(String, AsSelf);
impl_sidex_primitive!((), AsSelf);

impl<T: SidexType> SidexType for Option<T> {
    type Encoding = Option<T::Encoding>;
}

impl<T: SidexType> SidexType for Vec<T> {
    type Encoding = Vec<T::Encoding>;
}

impl<T: SidexType> SidexType for Box<T> {
    type Encoding = Box<T::Encoding>;
}

impl<T: SidexType> SidexType for Arc<T> {
    type Encoding = Arc<T::Encoding>;
}

impl<T: SidexType> SidexType for Rc<T> {
    type Encoding = Rc<T::Encoding>;
}

impl<K, V, S> SidexType for HashMap<K, V, S>
where
    K: SidexType + Eq + Hash,
    V: SidexType,
    S: BuildHasher + Default + 'static,
{
    type Encoding = HashMap<K::Encoding, V::Encoding, S>;
}

impl<K, V> SidexType for BTreeMap<K, V>
where
    K: SidexType + Ord,
    V: SidexType,
{
    type Encoding = BTreeMap<K::Encoding, V::Encoding>;
}

#[cfg(feature = "indexmap")]
impl<K, V, S> SidexType for indexmap::IndexMap<K, V, S>
where
    K: SidexType + Eq + Hash,
    V: SidexType,
    S: BuildHasher + Default + 'static,
{
    type Encoding = indexmap::IndexMap<K::Encoding, V::Encoding, S>;
}

#[cfg(feature = "serde_json")]
impl SidexType for serde_json::Value {
    type Encoding = AsSelf;
}

#[cfg(feature = "serde_json")]
impl SidexType for serde_json::Map<String, serde_json::Value> {
    type Encoding = AsSelf;
}

fn non_finite_sentinel(value: f64) -> &'static str {
    if value.is_nan() {
        FLOAT_NAN
    } else if value.is_sign_positive() {
        FLOAT_POSITIVE_INFINITY
    } else {
        FLOAT_NEGATIVE_INFINITY
    }
}

fn parse_non_finite<E: de::Error>(s: &str) -> Result<f64, E> {
    match s {
        FLOAT_NAN => Ok(f64::NAN),
        FLOAT_POSITIVE_INFINITY => Ok(f64::INFINITY),
        FLOAT_NEGATIVE_INFINITY => Ok(f64::NEG_INFINITY),
        _ => {
            Err(E::invalid_value(
                de::Unexpected::Str(s),
                &"\"NaN\", \"+Infinity\", or \"-Infinity\"",
            ))
        }
    }
}

struct I64Visitor;

impl Visitor<'_> for I64Visitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 64-bit signed integer or its decimal-string form")
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<i64, E> {
        Ok(v)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<i64, E> {
        i64::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Unsigned(v), &self))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<i64, E> {
        v.parse::<i64>()
            .map_err(|_| E::invalid_value(de::Unexpected::Str(v), &self))
    }
}

struct U64Visitor;

impl Visitor<'_> for U64Visitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 64-bit unsigned integer or its decimal-string form")
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<u64, E> {
        u64::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Signed(v), &self))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<u64, E> {
        Ok(v)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<u64, E> {
        v.parse::<u64>()
            .map_err(|_| E::invalid_value(de::Unexpected::Str(v), &self))
    }
}

struct F32Visitor;

impl Visitor<'_> for F32Visitor {
    type Value = f32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 32-bit floating-point number")
    }

    #[inline]
    fn visit_f32<E>(self, v: f32) -> Result<f32, E> {
        Ok(v)
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<f32, E> {
        Ok(v as f32)
    }
}

struct F64Visitor;

impl Visitor<'_> for F64Visitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or one of \"NaN\", \"+Infinity\", \"-Infinity\"")
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<f64, E> {
        Ok(v)
    }

    #[inline]
    fn visit_f32<E>(self, v: f32) -> Result<f64, E> {
        Ok(v as f64)
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<f64, E> {
        Ok(v as f64)
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<f64, E> {
        Ok(v as f64)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<f64, E> {
        parse_non_finite(v)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Base64 string or a byte sequence")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Vec<u8>, E> {
        base64::decode(v).map_err(E::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Vec<u8>, E> {
        Ok(v.to_vec())
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Vec<u8>, E> {
        Ok(v)
    }
}

struct SeqVisitor<T, U>(PhantomData<(T, U)>);

impl<'de, T, U> Visitor<'de> for SeqVisitor<T, U>
where
    U: DeserializeAs<'de, T>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<T>, A::Error> {
        let mut out = Vec::with_capacity(sanitize_size_hint(seq.size_hint()));
        while let Some(elem) = seq.next_element::<DeserializeAsWrap<T, U>>()? {
            out.push(elem.into_inner());
        }
        Ok(out)
    }
}

struct MapVisitor<M, KU, VU>(PhantomData<(M, KU, VU)>);

impl<'de, K, V, KU, VU, S> Visitor<'de> for MapVisitor<HashMap<K, V, S>, KU, VU>
where
    KU: DeserializeAs<'de, K>,
    VU: DeserializeAs<'de, V>,
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Value = HashMap<K, V, S>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut out =
            HashMap::with_capacity_and_hasher(sanitize_size_hint(access.size_hint()), S::default());
        while let Some((k, v)) =
            access.next_entry::<DeserializeAsWrap<K, KU>, DeserializeAsWrap<V, VU>>()?
        {
            out.insert(k.into_inner(), v.into_inner());
        }
        Ok(out)
    }
}

impl<'de, K, V, KU, VU> Visitor<'de> for MapVisitor<BTreeMap<K, V>, KU, VU>
where
    KU: DeserializeAs<'de, K>,
    VU: DeserializeAs<'de, V>,
    K: Ord,
{
    type Value = BTreeMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut out = BTreeMap::new();
        while let Some((k, v)) =
            access.next_entry::<DeserializeAsWrap<K, KU>, DeserializeAsWrap<V, VU>>()?
        {
            out.insert(k.into_inner(), v.into_inner());
        }
        Ok(out)
    }
}

#[cfg(feature = "indexmap")]
impl<'de, K, V, KU, VU, S> Visitor<'de> for MapVisitor<indexmap::IndexMap<K, V, S>, KU, VU>
where
    KU: DeserializeAs<'de, K>,
    VU: DeserializeAs<'de, V>,
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Value = indexmap::IndexMap<K, V, S>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut out = indexmap::IndexMap::with_capacity_and_hasher(
            sanitize_size_hint(access.size_hint()),
            S::default(),
        );
        while let Some((k, v)) =
            access.next_entry::<DeserializeAsWrap<K, KU>, DeserializeAsWrap<V, VU>>()?
        {
            out.insert(k.into_inner(), v.into_inner());
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip_json<T, As>(value: &T) -> T
    where
        As: SerializeAs<T>,
        for<'de> As: DeserializeAs<'de, T>,
    {
        let s = serde_json::to_string(&SerializeAsWrap::<T, As>::new(value)).unwrap();
        serde_json::from_str::<DeserializeAsWrap<T, As>>(&s)
            .unwrap()
            .into_inner()
    }

    #[test]
    fn u64_below_threshold_emits_number() {
        for v in [0u64, 1, 42, JS_MAX_SAFE_INTEGER] {
            let s = serde_json::to_string(&SerializeAsWrap::<u64, AsU64>::new(&v)).unwrap();
            assert_eq!(s, v.to_string(), "value {v}");
        }
    }

    #[test]
    fn u64_above_threshold_emits_string() {
        for v in [JS_MAX_SAFE_INTEGER + 1, 1u64 << 60, u64::MAX] {
            let s = serde_json::to_string(&SerializeAsWrap::<u64, AsU64>::new(&v)).unwrap();
            assert_eq!(s, format!("\"{v}\""), "value {v}");
        }
    }

    #[test]
    fn u64_accepts_string_or_number() {
        let v: u64 = serde_json::from_str::<DeserializeAsWrap<u64, AsU64>>("\"7\"")
            .unwrap()
            .into_inner();
        assert_eq!(v, 7);
        let v: u64 = serde_json::from_str::<DeserializeAsWrap<u64, AsU64>>("9")
            .unwrap()
            .into_inner();
        assert_eq!(v, 9);
        // Large value coming in as a string still round-trips.
        let v: u64 =
            serde_json::from_str::<DeserializeAsWrap<u64, AsU64>>("\"18446744073709551615\"")
                .unwrap()
                .into_inner();
        assert_eq!(v, u64::MAX);
    }

    #[test]
    fn i64_threshold_split() {
        // Inside the safe range — emit numbers.
        for &v in &[
            0i64,
            1,
            -1,
            JS_MAX_SAFE_INTEGER as i64,
            -(JS_MAX_SAFE_INTEGER as i64),
        ] {
            let s = serde_json::to_string(&SerializeAsWrap::<i64, AsI64>::new(&v)).unwrap();
            assert_eq!(s, v.to_string(), "value {v}");
            assert_eq!(roundtrip_json::<i64, AsI64>(&v), v);
        }
        // Outside — emit strings.
        for &v in &[
            (JS_MAX_SAFE_INTEGER + 1) as i64,
            -((JS_MAX_SAFE_INTEGER + 1) as i64),
            i64::MIN,
            i64::MAX,
        ] {
            let s = serde_json::to_string(&SerializeAsWrap::<i64, AsI64>::new(&v)).unwrap();
            assert_eq!(s, format!("\"{v}\""), "value {v}");
            assert_eq!(roundtrip_json::<i64, AsI64>(&v), v);
        }
    }

    #[test]
    fn f64_finite_roundtrip() {
        for &v in &[0.0f64, -0.0, 1.5, -1.5, f64::MIN, f64::MAX] {
            let r = roundtrip_json::<f64, AsF64>(&v);
            assert_eq!(r.to_bits(), v.to_bits(), "value {v}");
        }
    }

    #[test]
    fn f64_non_finite_serializes_as_string() {
        for (v, expected) in [
            (f64::NAN, "\"NaN\""),
            (f64::INFINITY, "\"+Infinity\""),
            (f64::NEG_INFINITY, "\"-Infinity\""),
        ] {
            let s = serde_json::to_string(&SerializeAsWrap::<f64, AsF64>::new(&v)).unwrap();
            assert_eq!(s, expected);
        }
        let nan = roundtrip_json::<f64, AsF64>(&f64::NAN);
        assert!(nan.is_nan());
        assert_eq!(roundtrip_json::<f64, AsF64>(&f64::INFINITY), f64::INFINITY);
        assert_eq!(
            roundtrip_json::<f64, AsF64>(&f64::NEG_INFINITY),
            f64::NEG_INFINITY,
        );
    }

    #[test]
    fn vec_u64_propagates() {
        let xs: Vec<u64> = vec![1, 2, 1u64 << 60];
        let s = serde_json::to_string(&SerializeAsWrap::<_, Vec<AsU64>>::new(&xs)).unwrap();
        // Small values stay as numbers; only the >2^53 element becomes a string.
        assert_eq!(s, "[1,2,\"1152921504606846976\"]");
        let back: Vec<u64> = serde_json::from_str::<DeserializeAsWrap<Vec<u64>, Vec<AsU64>>>(&s)
            .unwrap()
            .into_inner();
        assert_eq!(back, xs);
    }

    #[test]
    fn option_propagates() {
        let some: Option<u64> = Some(1u64 << 60);
        let s = serde_json::to_string(&SerializeAsWrap::<_, Option<AsU64>>::new(&some)).unwrap();
        assert_eq!(s, "\"1152921504606846976\"");
        let back: Option<u64> =
            serde_json::from_str::<DeserializeAsWrap<Option<u64>, Option<AsU64>>>(&s)
                .unwrap()
                .into_inner();
        assert_eq!(back, some);

        let none: Option<u64> = None;
        let s = serde_json::to_string(&SerializeAsWrap::<_, Option<AsU64>>::new(&none)).unwrap();
        assert_eq!(s, "null");
    }

    #[test]
    fn hashmap_propagates() {
        let mut m: HashMap<String, u64> = HashMap::new();
        m.insert("k".to_owned(), u64::MAX);
        let s = serde_json::to_string(&SerializeAsWrap::<
            HashMap<String, u64>,
            HashMap<AsSelf, AsU64>,
        >::new(&m))
        .unwrap();
        assert_eq!(s, "{\"k\":\"18446744073709551615\"}");
        let back: HashMap<String, u64> = serde_json::from_str::<
            DeserializeAsWrap<HashMap<String, u64>, HashMap<AsSelf, AsU64>>,
        >(&s)
        .unwrap()
        .into_inner();
        assert_eq!(back, m);
    }

    #[test]
    fn as_self_passes_through() {
        let s = serde_json::to_string(&SerializeAsWrap::<u32, AsSelf>::new(&7u32)).unwrap();
        assert_eq!(s, "7");
    }

    #[test]
    fn sidex_type_propagates_through_generics() {
        // Stand-in for a Sidex-generated generic type: encoding is `AsSelf`
        // (the type's own Serialize/Deserialize handles the wire form), and
        // the inner field's encoding comes from `<T as SidexType>::Encoding`.
        struct Wrap<T>(T);
        impl<T: SidexType> SidexType for Wrap<T> {
            type Encoding = AsSelf;
        }
        impl<T: SidexType> Serialize for Wrap<T> {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                SerializeAsWrap::<T, T::Encoding>::new(&self.0).serialize(s)
            }
        }
        impl<'de, T: SidexType> Deserialize<'de> for Wrap<T> {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let inner = DeserializeAsWrap::<T, T::Encoding>::deserialize(d)?.into_inner();
                Ok(Wrap(inner))
            }
        }

        let v = Wrap::<u64>(1u64 << 60);
        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(s, "\"1152921504606846976\"");
        let back: Wrap<u64> = serde_json::from_str(&s).unwrap();
        assert_eq!(back.0, v.0);

        // And via a container: Vec<Wrap<u64>> picks AsU64 at the leaf.
        // The small `7` stays a number; the >2^53 value becomes a string.
        let xs = vec![Wrap::<u64>(7), Wrap::<u64>(u64::MAX)];
        let s = serde_json::to_string(&xs).unwrap();
        assert_eq!(s, "[7,\"18446744073709551615\"]");
    }

    #[test]
    fn bytes_base64_roundtrip() {
        let bs: Vec<u8> = vec![0x01, 0x02, 0xff];
        let s = serde_json::to_string(&SerializeAsWrap::<Vec<u8>, AsBytes>::new(&bs)).unwrap();
        assert_eq!(s, "\"AQL/\"");
        let back: Vec<u8> = serde_json::from_str::<DeserializeAsWrap<Vec<u8>, AsBytes>>(&s)
            .unwrap()
            .into_inner();
        assert_eq!(back, bs);
    }
}
