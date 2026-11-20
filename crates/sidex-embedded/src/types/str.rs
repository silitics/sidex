//! The `string` type.

use core::fmt;

use crate::Decode;
use crate::Encode;
use crate::MapKey;
use crate::error::BufferTooSmall;
use crate::error::DecodeError;
use crate::error::DecodeErrorKind;
use crate::error::EncodeError;
use crate::json::Reader;
use crate::json::Writer;

/// String, borrowed from the input or from the application.
///
/// Decoded strings borrow their text from the input. If the text contains
/// escape sequences, such as `\n`, they stay escaped until the string is read
/// with [`chars`](Self::chars) or [`unescape_into`](Self::unescape_into), so
/// decoding never copies. [`as_str`](Self::as_str) returns the text of strings
/// without escape sequences.
///
/// ```
/// use sidex_embedded::json;
/// use sidex_embedded::types::Str;
///
/// let plain: Str<'_> = json::from_str(r#""Hello""#).unwrap();
/// assert_eq!(plain.as_str(), Some("Hello"));
///
/// let escaped: Str<'_> = json::from_str(r#""Hello\nWorld""#).unwrap();
/// assert_eq!(escaped.as_str(), None);
/// assert_eq!(escaped, "Hello\nWorld");
///
/// let mut buffer = [0; 16];
/// assert_eq!(escaped.unescape_into(&mut buffer).unwrap(), "Hello\nWorld");
/// ```
#[derive(Clone, Copy)]
pub struct Str<'a> {
    text: &'a str,
    /// Indicates whether the text is JSON string content with escape
    /// sequences, which the reader has checked.
    escaped: bool,
}

impl<'a> Str<'a> {
    /// Creates a string with the given text.
    pub const fn new(text: &'a str) -> Self {
        Self {
            text,
            escaped: false,
        }
    }

    /// Creates a string from JSON string content with escape sequences.
    pub(crate) const fn escaped(text: &'a str) -> Self {
        Self {
            text,
            escaped: true,
        }
    }

    /// Returns the text if it contains no escape sequences.
    pub fn as_str(&self) -> Option<&'a str> {
        (!self.escaped).then_some(self.text)
    }

    /// Returns an iterator over the characters.
    pub fn chars(&self) -> Chars<'a> {
        Chars {
            rest: self.text,
            escaped: self.escaped,
        }
    }

    /// Length of the text in bytes, with escape sequences replaced.
    pub fn len(&self) -> usize {
        if self.escaped {
            self.chars().map(char::len_utf8).sum()
        } else {
            self.text.len()
        }
    }

    /// Indicates whether the string is empty.
    pub fn is_empty(&self) -> bool {
        // Escape sequences denote characters, so escaped text is never empty.
        self.text.is_empty()
    }

    /// Copies the text into `buffer`, replacing escape sequences, and returns
    /// the copy.
    pub fn unescape_into<'b>(&self, buffer: &'b mut [u8]) -> Result<&'b str, BufferTooSmall> {
        let mut len = 0;
        for char in self.chars() {
            let target = buffer
                .get_mut(len..len + char.len_utf8())
                .ok_or(BufferTooSmall)?;
            char.encode_utf8(target);
            len += char.len_utf8();
        }
        Ok(core::str::from_utf8(&buffer[..len]).expect("only whole characters are copied"))
    }

    /// Passes the text to `f`, unescaping it into a buffer of `N` bytes if it
    /// has escape sequences. Returns `None` if the unescaped text does not fit.
    pub(crate) fn with_text<const N: usize, R>(&self, f: impl FnOnce(&str) -> R) -> Option<R> {
        match self.as_str() {
            Some(text) => Some(f(text)),
            None => {
                let mut buffer = [0; N];
                self.unescape_into(&mut buffer).ok().map(f)
            }
        }
    }
}

impl Default for Str<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

impl<'a> From<&'a str> for Str<'a> {
    fn from(text: &'a str) -> Self {
        Self::new(text)
    }
}

impl PartialEq for Str<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self.as_str(), other.as_str()) {
            (Some(text), Some(other)) => text == other,
            _ => self.chars().eq(other.chars()),
        }
    }
}

impl Eq for Str<'_> {}

impl PartialEq<str> for Str<'_> {
    fn eq(&self, other: &str) -> bool {
        match self.as_str() {
            Some(text) => text == other,
            None => self.chars().eq(other.chars()),
        }
    }
}

impl PartialEq<&str> for Str<'_> {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl fmt::Display for Str<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Some(text) => f.write_str(text),
            None => {
                self.chars()
                    .try_for_each(|char| fmt::Write::write_char(f, char))
            }
        }
    }
}

impl fmt::Debug for Str<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"")?;
        for char in self.chars() {
            for escaped in char.escape_debug() {
                fmt::Write::write_char(f, escaped)?;
            }
        }
        f.write_str("\"")
    }
}

impl<'a> Decode<'a> for Str<'a> {
    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        reader.string()
    }
}

impl Encode for Str<'_> {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        if self.escaped {
            writer.escaped_str(self.text)
        } else {
            writer.str(self.text)
        }
    }
}

impl<'a> MapKey<'a> for Str<'a> {
    fn decode_key(key: Str<'a>) -> Result<Self, DecodeErrorKind> {
        Ok(key)
    }

    fn encode_key(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        self.encode(writer)
    }
}

/// Iterator over the characters of a [`Str`].
#[derive(Debug, Clone)]
pub struct Chars<'a> {
    rest: &'a str,
    escaped: bool,
}

impl Iterator for Chars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let mut chars = self.rest.chars();
        let char = chars.next()?;
        if self.escaped && char == '\\' {
            let (char, len) = unescape(&self.rest.as_bytes()[1..])
                .expect("the reader checks the escape sequences of strings");
            self.rest = &self.rest[1 + len..];
            return Some(char);
        }
        self.rest = chars.as_str();
        Some(char)
    }
}

/// Decodes the escape sequence following a backslash and returns its character
/// and length, or `None` if the sequence is invalid.
pub(crate) fn unescape(sequence: &[u8]) -> Option<(char, usize)> {
    let char = match sequence.first()? {
        b'"' => '"',
        b'\\' => '\\',
        b'/' => '/',
        b'b' => '\u{8}',
        b'f' => '\u{c}',
        b'n' => '\n',
        b'r' => '\r',
        b't' => '\t',
        b'u' => {
            let high = hex(sequence.get(1..5)?)?;
            if !(0xD800..0xDC00).contains(&high) {
                // Unpaired low surrogates are not characters.
                return Some((char::from_u32(high)?, 5));
            }
            if sequence.get(5..7)? != b"\\u" {
                return None;
            }
            let low = hex(sequence.get(7..11)?)?;
            if !(0xDC00..0xE000).contains(&low) {
                return None;
            }
            let code = 0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00);
            return Some((char::from_u32(code)?, 11));
        }
        _ => return None,
    };
    Some((char, 1))
}

/// Parses four hexadecimal digits.
fn hex(digits: &[u8]) -> Option<u32> {
    digits.iter().try_fold(0, |value, digit| {
        Some(value << 4 | char::from(*digit).to_digit(16)?)
    })
}

#[cfg(test)]
mod tests {
    use crate::json;
    use crate::types::Str;

    /// Unescapes all kinds of escape sequences and compares strings by their
    /// characters.
    #[test]
    fn test_escapes() {
        let text: Str<'_> = json::from_str(r#""a\"\\\/\b\f\n\r\té😀z""#).unwrap();
        let expected = "a\"\\/\u{8}\u{c}\n\r\té😀z";
        assert_eq!(text, expected);
        assert_eq!(text, Str::new(expected));
        assert_eq!(text.len(), expected.len());
        let mut buffer = [0; 32];
        assert_eq!(text.unescape_into(&mut buffer).unwrap(), expected);
        assert!(
            text.unescape_into(&mut buffer[..expected.len() - 1])
                .is_err()
        );
        assert_ne!(text, "a");
    }
}
