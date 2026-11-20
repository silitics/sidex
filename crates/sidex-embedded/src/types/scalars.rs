//! Booleans, integers, and floats.

use core::str::FromStr;

use crate::Decode;
use crate::Encode;
use crate::MapKey;
use crate::error::DecodeError;
use crate::error::DecodeErrorKind;
use crate::error::EncodeError;
use crate::json::Kind;
use crate::json::Reader;
use crate::json::Writer;
use crate::types::Str;

impl Decode<'_> for bool {
    fn decode(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        reader.bool()
    }
}

impl Encode for bool {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        writer.bool(*self)
    }
}

/// Implements the traits for an integer type.
///
/// 64-bit integers are decimal strings beyond the range in which JavaScript
/// represents integers exactly and are also accepted as strings.
macro_rules! impl_int {
    ($ty:ty, $expected:literal, $wide:literal) => {
        impl Decode<'_> for $ty {
            fn decode(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
                const EXPECTED: DecodeErrorKind = DecodeErrorKind::Unexpected($expected);
                let error = reader.error(EXPECTED);
                let parsed = match reader.peek()? {
                    Kind::Number => parse_int(reader.number()?),
                    Kind::String if $wide => Self::decode_key(reader.string()?).ok(),
                    _ => None,
                };
                parsed.ok_or(error)
            }
        }

        impl Encode for $ty {
            fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
                if $wide && (*self as i128).unsigned_abs() > MAX_SAFE_INTEGER {
                    writer.fmt_str(format_args!("{self}"))
                } else {
                    writer.fmt(format_args!("{self}"))
                }
            }
        }

        impl MapKey<'_> for $ty {
            fn decode_key(key: Str<'_>) -> Result<Self, DecodeErrorKind> {
                key.with_text::<24, _>(|text| text.parse().ok())
                    .flatten()
                    .ok_or(DecodeErrorKind::Unexpected($expected))
            }

            fn encode_key(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
                writer.fmt_str(format_args!("{self}"))
            }
        }
    };
}

impl_int!(i8, "an 8-bit signed integer", false);
impl_int!(i16, "a 16-bit signed integer", false);
impl_int!(i32, "a 32-bit signed integer", false);
impl_int!(i64, "a 64-bit signed integer", true);
impl_int!(u8, "an 8-bit unsigned integer", false);
impl_int!(u16, "a 16-bit unsigned integer", false);
impl_int!(u32, "a 32-bit unsigned integer", false);
impl_int!(u64, "a 64-bit unsigned integer", true);
impl_int!(usize, "an index", false);

/// Largest magnitude up to which JavaScript represents all integers exactly.
const MAX_SAFE_INTEGER: u128 = (1 << 53) - 1;

/// Parses the text of a JSON number if it is an integer in range.
fn parse_int<T: FromStr>(text: &str) -> Option<T> {
    // Integer parsing rejects numbers with a fraction or an exponent, which are
    // floats. So is `-0`, which integers cannot represent.
    if text == "-0" {
        return None;
    }
    text.parse().ok()
}

/// Representation of `NaN`.
const NAN: &str = "NaN";
/// Representation of positive infinity.
const INFINITY: &str = "+Infinity";
/// Representation of negative infinity.
const NEG_INFINITY: &str = "-Infinity";

fn decode_float(reader: &mut Reader<'_>) -> Result<f64, DecodeError> {
    const EXPECTED: DecodeErrorKind =
        DecodeErrorKind::Unexpected("a number or one of `NaN`, `+Infinity`, and `-Infinity`");
    let error = reader.error(EXPECTED);
    let value = match reader.peek()? {
        // Finite numbers beyond the range of `f64` are not numbers that floats
        // can represent.
        Kind::Number => {
            reader
                .number()?
                .parse()
                .ok()
                .filter(|value: &f64| value.is_finite())
        }
        Kind::String => {
            let text = reader.string()?;
            if text == NAN {
                Some(f64::NAN)
            } else if text == INFINITY || text == "Infinity" {
                Some(f64::INFINITY)
            } else if text == NEG_INFINITY {
                Some(f64::NEG_INFINITY)
            } else {
                None
            }
        }
        _ => None,
    };
    value.ok_or(error)
}

fn encode_non_finite(value: f64, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
    writer.str(if value.is_nan() {
        NAN
    } else if value.is_sign_positive() {
        INFINITY
    } else {
        NEG_INFINITY
    })
}

impl Decode<'_> for f64 {
    fn decode(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        decode_float(reader)
    }
}

impl Encode for f64 {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        if !self.is_finite() {
            return encode_non_finite(*self, writer);
        }
        // `Debug` writes the shortest decimal denoting the float, always with a
        // fraction or an exponent, so the number reads back as a float.
        writer.fmt(format_args!("{self:?}"))
    }
}

impl Decode<'_> for f32 {
    fn decode(reader: &mut Reader<'_>) -> Result<Self, DecodeError> {
        decode_float(reader).map(|value| value as f32)
    }
}

impl Encode for f32 {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        if !self.is_finite() {
            return encode_non_finite(f64::from(*self), writer);
        }
        writer.fmt(format_args!("{self:?}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::json;

    /// Encodes and decodes integers like the `sidex` crate, including the
    /// string representation of large 64-bit integers.
    #[test]
    fn test_integers() {
        let mut buffer = [0; 32];
        for (value, text) in [
            (0_u64, "0"),
            ((1 << 53) - 1, "9007199254740991"),
            (1 << 53, "\"9007199254740992\""),
            (u64::MAX, "\"18446744073709551615\""),
        ] {
            let len = json::to_slice(&value, &mut buffer).unwrap();
            assert_eq!(&buffer[..len], text.as_bytes());
            assert_eq!(json::from_str::<u64>(text).unwrap(), value);
        }
        assert_eq!(json::from_str::<i64>("\"-5\"").unwrap(), -5);
        assert_eq!(json::from_str::<i64>("\"+5\"").unwrap(), 5);
        assert_eq!(
            json::from_str::<i64>("-9007199254740993").unwrap(),
            -(1 << 53) - 1
        );
        for (text, valid) in [
            ("255", true),
            ("256", false),
            ("-1", false),
            ("1.0", false),
            ("\"1\"", false),
        ] {
            assert_eq!(json::from_str::<u8>(text).is_ok(), valid, "{text}");
        }
        assert!(json::from_str::<i64>("-0").is_err());
        assert!(json::from_str::<u64>("1e3").is_err());
    }

    /// Encodes floats so that they read back as floats and represents
    /// non-finite floats as strings.
    #[test]
    fn test_floats() {
        let mut buffer = [0; 32];
        for (value, text) in [
            (1.0, "1.0"),
            (0.1, "0.1"),
            (1e21, "1e21"),
            (f64::NAN, "\"NaN\""),
            (f64::NEG_INFINITY, "\"-Infinity\""),
        ] {
            let len = json::to_slice(&value, &mut buffer).unwrap();
            assert_eq!(&buffer[..len], text.as_bytes());
        }
        let len = json::to_slice(&0.1_f32, &mut buffer).unwrap();
        assert_eq!(&buffer[..len], b"0.1");
        assert_eq!(json::from_str::<f64>("1").unwrap(), 1.0);
        assert_eq!(
            json::from_str::<f64>("\"Infinity\"").unwrap(),
            f64::INFINITY
        );
        assert!(json::from_str::<f64>("1e400").is_err());
        assert!(json::from_str::<f64>("\"inf\"").is_err());
    }
}
