//! Representations of scalars shared by encoding and decoding.

use std::fmt;
use std::fmt::Write;

use base64::alphabet;
use base64::engine::DecodePaddingMode;
use base64::engine::GeneralPurpose;
use base64::engine::GeneralPurposeConfig;

/// Standard Base64 with padding, accepting input with or without padding.
pub(super) const BASE64: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

/// Representation of `NaN`.
pub(super) const NAN: &str = "NaN";
/// Representation of positive infinity.
pub(super) const INFINITY: &str = "Infinity";
/// Representation of negative infinity.
pub(super) const NEG_INFINITY: &str = "-Infinity";

/// Returns the representation of a non-finite float.
pub(super) fn non_finite(value: f64) -> &'static str {
    if value.is_nan() {
        NAN
    } else if value.is_sign_positive() {
        INFINITY
    } else {
        NEG_INFINITY
    }
}

/// Parses the representation of a non-finite float.
pub(super) fn parse_non_finite(text: &str) -> Option<f64> {
    match text {
        NAN => Some(f64::NAN),
        // Sidex 0.2 wrote `+Infinity`.
        INFINITY | "+Infinity" => Some(f64::INFINITY),
        NEG_INFINITY => Some(f64::NEG_INFINITY),
        _ => None,
    }
}

/// Formats `value` and passes the text to `f`.
///
/// Short texts, such as numbers and timestamps, are formatted on the stack.
pub(super) fn with_display<R>(value: impl fmt::Display, f: impl FnOnce(&str) -> R) -> R {
    let mut buffer = StackBuffer::default();
    match write!(buffer, "{value}") {
        Ok(()) => f(buffer.as_str()),
        Err(_) => f(&value.to_string()),
    }
}

/// Fixed-size buffer for formatting short texts.
struct StackBuffer {
    bytes: [u8; 64],
    len: usize,
}

impl Default for StackBuffer {
    fn default() -> Self {
        Self {
            bytes: [0; 64],
            len: 0,
        }
    }
}

impl StackBuffer {
    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.len]).expect("only whole `str`s are written")
    }
}

impl Write for StackBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len + s.len();
        let target = self.bytes.get_mut(self.len..end).ok_or(fmt::Error)?;
        target.copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}
