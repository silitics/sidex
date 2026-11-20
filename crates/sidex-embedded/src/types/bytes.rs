//! The `bytes` type and its Base64 representation.

use core::fmt;

use crate::Decode;
use crate::Encode;
use crate::error::BufferTooSmall;
use crate::error::DecodeError;
use crate::error::DecodeErrorKind;
use crate::error::EncodeError;
use crate::json::Reader;
use crate::json::Writer;
use crate::types::Str;
use crate::types::str::Chars;

/// Bytes, borrowed from the input or from the application.
///
/// The JSON representation is a Base64 string with padding, also accepted
/// without padding. Decoded bytes keep the string and decode it on access.
///
/// ```
/// use sidex_embedded::json;
/// use sidex_embedded::types::Bytes;
///
/// let bytes: Bytes<'_> = json::from_str(r#""aGVsbG8=""#).unwrap();
/// let mut buffer = [0; 8];
/// assert_eq!(bytes.decode_into(&mut buffer).unwrap(), b"hello");
/// assert_eq!(bytes, Bytes::new(b"hello"));
/// ```
#[derive(Clone, Copy)]
pub struct Bytes<'a> {
    repr: Repr<'a>,
}

#[derive(Clone, Copy)]
enum Repr<'a> {
    /// Base64 string that has been checked, with the number of bytes it
    /// encodes.
    Base64(Str<'a>, usize),
    Slice(&'a [u8]),
}

impl<'a> Bytes<'a> {
    /// Creates bytes from a slice.
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            repr: Repr::Slice(bytes),
        }
    }

    /// Number of bytes.
    pub const fn len(&self) -> usize {
        match self.repr {
            Repr::Base64(_, len) => len,
            Repr::Slice(bytes) => bytes.len(),
        }
    }

    /// Indicates whether there are no bytes.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the bytes.
    pub fn iter(&self) -> Iter<'a> {
        Iter {
            repr: match self.repr {
                Repr::Base64(text, _) => {
                    IterRepr::Base64 {
                        chars: text.chars(),
                        bits: 0,
                        bit_count: 0,
                    }
                }
                Repr::Slice(bytes) => IterRepr::Slice(bytes.iter()),
            },
        }
    }

    /// Copies the bytes into `buffer` and returns the copy.
    pub fn decode_into<'b>(&self, buffer: &'b mut [u8]) -> Result<&'b [u8], BufferTooSmall> {
        let target = buffer.get_mut(..self.len()).ok_or(BufferTooSmall)?;
        for (target, byte) in target.iter_mut().zip(self.iter()) {
            *target = byte;
        }
        Ok(target)
    }
}

impl Default for Bytes<'_> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self::new(bytes)
    }
}

impl PartialEq for Bytes<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl Eq for Bytes<'_> {}

impl fmt::Debug for Bytes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a> Decode<'a> for Bytes<'a> {
    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        const EXPECTED: &str = "a Base64 string";
        let error = reader.error(DecodeErrorKind::Unexpected(EXPECTED));
        let text = reader.string_as(EXPECTED)?;
        let len = decoded_len(text.chars()).ok_or(error)?;
        Ok(Self {
            repr: Repr::Base64(text, len),
        })
    }
}

impl Encode for Bytes<'_> {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut text = [0; 64];
        let mut len = 0;
        let mut bytes = self.iter();
        writer.raw("\"")?;
        loop {
            let group = [bytes.next(), bytes.next(), bytes.next()];
            let Some(first) = group[0] else {
                break;
            };
            let value = u32::from(first) << 16
                | u32::from(group[1].unwrap_or(0)) << 8
                | u32::from(group[2].unwrap_or(0));
            let chars = 1 + group.iter().flatten().count();
            for index in 0..4 {
                text[len + index] = if index < chars {
                    ALPHABET[(value >> (18 - 6 * index) & 0x3F) as usize]
                } else {
                    b'='
                };
            }
            len += 4;
            if len == text.len() {
                writer.raw(core::str::from_utf8(&text).expect("Base64 is ASCII"))?;
                len = 0;
            }
        }
        writer.raw(core::str::from_utf8(&text[..len]).expect("Base64 is ASCII"))?;
        writer.raw("\"")
    }
}

/// Iterator over [`Bytes`].
pub struct Iter<'a> {
    repr: IterRepr<'a>,
}

enum IterRepr<'a> {
    Base64 {
        chars: Chars<'a>,
        /// Decoded bits that have not been returned yet.
        bits: u32,
        /// Number of decoded bits that have not been returned yet.
        bit_count: u32,
    },
    Slice(core::slice::Iter<'a, u8>),
}

impl Iterator for Iter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match &mut self.repr {
            IterRepr::Base64 {
                chars,
                bits,
                bit_count,
            } => {
                while *bit_count < 8 {
                    *bits = *bits << 6 | digit_value(chars.next()?)?;
                    *bit_count += 6;
                }
                *bit_count -= 8;
                let byte = (*bits >> *bit_count) as u8;
                *bits &= (1 << *bit_count) - 1;
                Some(byte)
            }
            IterRepr::Slice(bytes) => bytes.next().copied(),
        }
    }
}

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Returns the value of a Base64 digit, or `None` for padding and other
/// characters.
fn digit_value(char: char) -> Option<u32> {
    Some(match char {
        'A'..='Z' => char as u32 - 'A' as u32,
        'a'..='z' => char as u32 - 'a' as u32 + 26,
        '0'..='9' => char as u32 - '0' as u32 + 52,
        '+' => 62,
        '/' => 63,
        _ => return None,
    })
}

/// Checks standard Base64 text with or without padding and returns the number
/// of bytes it encodes.
///
/// Like the `base64` crate, the bits that the last character does not use must
/// be zero.
fn decoded_len(chars: impl Iterator<Item = char>) -> Option<usize> {
    let (mut digits, mut padding, mut last_digit) = (0, 0, 0);
    for char in chars {
        match digit_value(char) {
            Some(value) if padding == 0 => {
                digits += 1;
                last_digit = value;
            }
            None if char == '=' => padding += 1,
            _ => return None,
        }
    }
    let (unused_bits, trailing_bytes) = match digits % 4 {
        0 => (0, 0),
        2 => (4, 1),
        3 => (2, 2),
        _ => return None,
    };
    let expected_padding = (4 - digits % 4) % 4;
    if (padding != 0 && padding != expected_padding) || last_digit & ((1 << unused_bits) - 1) != 0 {
        return None;
    }
    Some(digits / 4 * 3 + trailing_bytes)
}

#[cfg(test)]
mod tests {
    use crate::json;
    use crate::types::Bytes;

    /// Encodes with padding, decodes with and without it, and rejects invalid
    /// Base64.
    #[test]
    fn test_base64() {
        let mut buffer = [0; 256];
        for (bytes, text) in [
            (&b""[..], "\"\""),
            (b"f", "\"Zg==\""),
            (b"fo", "\"Zm8=\""),
            (b"foo", "\"Zm9v\""),
            (b"foob", "\"Zm9vYg==\""),
        ] {
            let len = json::to_slice(&Bytes::new(bytes), &mut buffer).unwrap();
            assert_eq!(&buffer[..len], text.as_bytes());
            let decoded: Bytes<'_> = json::from_str(text).unwrap();
            assert_eq!(decoded, Bytes::new(bytes), "{text}");
        }
        let long = [0xAB; 100];
        let len = json::to_slice(&Bytes::new(&long), &mut buffer).unwrap();
        let text = core::str::from_utf8(&buffer[..len]).unwrap();
        assert_eq!(
            json::from_str::<Bytes<'_>>(text).unwrap(),
            Bytes::new(&long)
        );
        let decoded: Bytes<'_> = json::from_str("\"Zm8\"").unwrap();
        assert_eq!(decoded, Bytes::new(b"fo"));
        let decoded: Bytes<'_> = json::from_str("\"Zm\\u0038=\"").unwrap();
        assert_eq!(decoded, Bytes::new(b"fo"));
        for text in [
            "\"Z\"", "\"Zm9=\"", "\"Zh==\"", "\"Zg=\"", "\"Zg=a\"", "\"Z===\"", "\"Zm8*\"", "1",
        ] {
            assert!(json::from_str::<Bytes<'_>>(text).is_err(), "{text}");
        }
    }
}
