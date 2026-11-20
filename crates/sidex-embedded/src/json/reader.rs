//! Reading JSON text.

use crate::error::DecodeError;
use crate::error::DecodeErrorKind;
use crate::types::Str;
use crate::types::str::unescape;

/// Default maximum nesting depth of a [`Reader`].
pub const DEFAULT_MAX_DEPTH: usize = 32;

/// Kind of a JSON value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool,
    /// Number.
    Number,
    /// String.
    String,
    /// Array.
    Array,
    /// Object.
    Object,
}

/// Reader of JSON text held in a byte slice.
///
/// Each method reads one value, or a part of an object or array, and checks
/// that the part it reads is valid JSON. [`finish`](Self::finish) checks the
/// rest of the input. Readers are cheap to clone, so decoders read a value
/// repeatedly by cloning the reader first.
///
/// Readers limit the nesting depth of arrays and objects (see
/// [`with_max_depth`](Self::with_max_depth)), so that input cannot exhaust the
/// stack.
#[derive(Debug, Clone)]
pub struct Reader<'a> {
    input: &'a [u8],
    pos: usize,
    /// Remaining nesting depth.
    depth: usize,
    /// Indicates whether the input has already been decoded successfully as the
    /// values read from it.
    trusted: bool,
}

impl<'a> Reader<'a> {
    /// Creates a reader with [`DEFAULT_MAX_DEPTH`].
    pub const fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0,
            depth: DEFAULT_MAX_DEPTH,
            trusted: false,
        }
    }

    /// Sets the maximum nesting depth of arrays and objects.
    pub const fn with_max_depth(self, depth: usize) -> Self {
        Self { depth, ..self }
    }

    /// Creates a reader for input that has already been decoded successfully as
    /// the values read from it.
    ///
    /// Sequences and maps only skip their items when reading trusted input.
    /// The depth was checked when decoding the input, so it is not limited.
    pub(crate) const fn trusted(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0,
            depth: usize::MAX,
            trusted: true,
        }
    }

    pub(crate) fn is_trusted(&self) -> bool {
        self.trusted
    }

    /// Creates an error at the start of the next value.
    pub fn error(&self, kind: DecodeErrorKind) -> DecodeError {
        let mut reader = self.clone();
        reader.skip_whitespace();
        DecodeError::new(kind, reader.pos)
    }

    /// Returns the kind of the next value without reading it.
    pub fn peek(&mut self) -> Result<Kind, DecodeError> {
        self.skip_whitespace();
        Ok(match self.input.get(self.pos) {
            Some(b'n') => Kind::Null,
            Some(b't' | b'f') => Kind::Bool,
            Some(b'-' | b'0'..=b'9') => Kind::Number,
            Some(b'"') => Kind::String,
            Some(b'[') => Kind::Array,
            Some(b'{') => Kind::Object,
            _ => return Err(self.syntax_error()),
        })
    }

    /// Reads `null`.
    pub fn null(&mut self) -> Result<(), DecodeError> {
        self.expect(Kind::Null, "null")?;
        self.literal(b"null")
    }

    /// Reads `null` if it is the next value and indicates whether it was.
    pub fn take_null(&mut self) -> Result<bool, DecodeError> {
        if self.peek()? == Kind::Null {
            self.literal(b"null")?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Reads a boolean.
    pub fn bool(&mut self) -> Result<bool, DecodeError> {
        self.expect(Kind::Bool, "a boolean")?;
        self.lex_bool()
    }

    /// Reads a number and returns its text.
    pub fn number(&mut self) -> Result<&'a str, DecodeError> {
        self.expect(Kind::Number, "a number")?;
        self.lex_number()
    }

    /// Reads a string.
    pub fn string(&mut self) -> Result<Str<'a>, DecodeError> {
        self.string_as("a string")
    }

    /// Reads a string holding the text representation of a type, which
    /// `expected` describes for errors if the value is not a string.
    pub(crate) fn string_as(&mut self, expected: &'static str) -> Result<Str<'a>, DecodeError> {
        self.expect(Kind::String, expected)?;
        self.lex_string()
    }

    /// Starts reading an object.
    ///
    /// Read its entries with [`Object::next_key`].
    pub fn object(&mut self) -> Result<Object, DecodeError> {
        self.expect(Kind::Object, "a map")?;
        self.enter()?;
        Ok(Object { first: true })
    }

    /// Starts reading an array.
    ///
    /// Read its items with [`Array::next`].
    pub fn array(&mut self) -> Result<Array, DecodeError> {
        self.expect(Kind::Array, "a sequence")?;
        self.enter()?;
        Ok(Array { first: true })
    }

    /// Reads any value and returns its text.
    pub fn skip(&mut self) -> Result<&'a str, DecodeError> {
        let kind = self.peek()?;
        let start = self.pos;
        match kind {
            Kind::Null => self.literal(b"null")?,
            Kind::Bool => {
                self.lex_bool()?;
            }
            Kind::Number => {
                self.lex_number()?;
            }
            Kind::String => {
                self.lex_string()?;
            }
            Kind::Array => {
                self.enter()?;
                let mut array = Array { first: true };
                while array.next(self)? {
                    self.skip()?;
                }
            }
            Kind::Object => {
                self.enter()?;
                let mut object = Object { first: true };
                while object.next_key(self)?.is_some() {
                    self.skip()?;
                }
            }
        }
        Ok(self.text(start))
    }

    /// Checks that only whitespace follows.
    pub fn finish(mut self) -> Result<(), DecodeError> {
        self.skip_whitespace();
        if self.pos < self.input.len() {
            return Err(self.syntax_error());
        }
        Ok(())
    }

    /// Returns the input from `start` to the current position.
    pub(crate) fn input_from(&self, start: usize) -> &'a [u8] {
        &self.input[start..self.pos]
    }

    /// Returns the position of the next value.
    pub(crate) fn value_start(&mut self) -> usize {
        self.skip_whitespace();
        self.pos
    }

    /// Returns a reader at the given position, which must be the start of a
    /// value previously read by this reader or a clone of it.
    pub(crate) fn at(&self, pos: usize) -> Self {
        Self {
            pos,
            ..self.clone()
        }
    }

    /// Reads a boolean at the current position.
    fn lex_bool(&mut self) -> Result<bool, DecodeError> {
        if self.input[self.pos] == b't' {
            self.literal(b"true")?;
            Ok(true)
        } else {
            self.literal(b"false")?;
            Ok(false)
        }
    }

    /// Reads a number at the current position.
    fn lex_number(&mut self) -> Result<&'a str, DecodeError> {
        let start = self.pos;
        self.eat(b'-');
        if !self.eat(b'0') {
            self.digits()?;
        }
        if self.eat(b'.') {
            self.digits()?;
        }
        if self.eat(b'e') || self.eat(b'E') {
            let _ = self.eat(b'+') || self.eat(b'-');
            self.digits()?;
        }
        Ok(self.text(start))
    }

    /// Reads a string at the current position, which holds its opening quote.
    fn lex_string(&mut self) -> Result<Str<'a>, DecodeError> {
        let start = self.pos;
        self.pos += 1;
        let mut escaped = false;
        loop {
            match self.input.get(self.pos) {
                Some(b'"') => break,
                Some(b'\\') => {
                    let (_, len) =
                        unescape(&self.input[self.pos + 1..]).ok_or_else(|| self.syntax_error())?;
                    self.pos += 1 + len;
                    escaped = true;
                }
                Some(0x20..) => self.pos += 1,
                Some(_) | None => return Err(self.syntax_error()),
            }
        }
        let text = core::str::from_utf8(&self.input[start + 1..self.pos])
            .map_err(|_| DecodeError::new(DecodeErrorKind::Syntax, start))?;
        self.pos += 1;
        Ok(if escaped {
            Str::escaped(text)
        } else {
            Str::new(text)
        })
    }

    /// Checks that the next value has the given kind, or returns an error
    /// describing the expected value.
    fn expect(&mut self, kind: Kind, expected: &'static str) -> Result<(), DecodeError> {
        if self.peek()? != kind {
            return Err(self.error(DecodeErrorKind::Unexpected(expected)));
        }
        Ok(())
    }

    /// Consumes one level of the nesting depth.
    fn enter(&mut self) -> Result<(), DecodeError> {
        self.depth = self
            .depth
            .checked_sub(1)
            .ok_or_else(|| self.error(DecodeErrorKind::DepthLimitExceeded))?;
        self.pos += 1;
        Ok(())
    }

    fn literal(&mut self, literal: &[u8]) -> Result<(), DecodeError> {
        if !self.input[self.pos..].starts_with(literal) {
            return Err(self.syntax_error());
        }
        self.pos += literal.len();
        Ok(())
    }

    fn digits(&mut self) -> Result<(), DecodeError> {
        let start = self.pos;
        while self.input.get(self.pos).is_some_and(u8::is_ascii_digit) {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(self.syntax_error());
        }
        Ok(())
    }

    fn eat(&mut self, byte: u8) -> bool {
        let matches = self.input.get(self.pos) == Some(&byte);
        if matches {
            self.pos += 1;
        }
        matches
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.input.get(self.pos), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    /// Returns the text from `start` to the current position, which only
    /// contains complete values.
    fn text(&self, start: usize) -> &'a str {
        // Outside of strings, JSON only consists of ASCII characters, and the
        // contents of strings have been checked.
        core::str::from_utf8(&self.input[start..self.pos])
            .expect("JSON values read by the reader are valid UTF-8")
    }

    fn syntax_error(&self) -> DecodeError {
        DecodeError::new(DecodeErrorKind::Syntax, self.pos)
    }
}

/// Object being read.
///
/// After each key, read its value from the reader.
#[derive(Debug)]
pub struct Object {
    first: bool,
}

impl Object {
    /// Reads the key of the next entry, or returns `None` at the end of the
    /// object.
    pub fn next_key<'a>(
        &mut self,
        reader: &mut Reader<'a>,
    ) -> Result<Option<Str<'a>>, DecodeError> {
        reader.skip_whitespace();
        if reader.eat(b'}') {
            reader.depth += 1;
            return Ok(None);
        }
        if !self.first && !reader.eat(b',') {
            return Err(reader.syntax_error());
        }
        self.first = false;
        reader.skip_whitespace();
        if reader.input.get(reader.pos) != Some(&b'"') {
            return Err(reader.syntax_error());
        }
        let key = reader.lex_string()?;
        reader.skip_whitespace();
        if !reader.eat(b':') {
            return Err(reader.syntax_error());
        }
        Ok(Some(key))
    }
}

/// Array being read.
///
/// Whenever [`next`](Self::next) returns `true`, read the item from the reader.
#[derive(Debug)]
pub struct Array {
    first: bool,
}

impl Array {
    /// Advances to the next item, or returns `false` at the end of the array.
    pub fn next(&mut self, reader: &mut Reader<'_>) -> Result<bool, DecodeError> {
        reader.skip_whitespace();
        if reader.eat(b']') {
            reader.depth += 1;
            return Ok(false);
        }
        if !self.first && !reader.eat(b',') {
            return Err(reader.syntax_error());
        }
        self.first = false;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Skips each valid document entirely and returns its text.
    #[test]
    fn test_skip_valid() {
        for text in [
            "null",
            "true",
            "false",
            "0",
            "-0.5e+10",
            "12E3",
            r#""a\"b\\\/\b\f\n\r\t\u00e9\ud83d\ude00""#,
            "[]",
            "[1, [2, {}], {\"a\": [true]}]",
            " { \"a\" : 1 , \"b\" : \"\" } ",
        ] {
            let mut reader = Reader::new(text.as_bytes());
            assert_eq!(reader.skip().unwrap(), text.trim(), "{text}");
            reader.finish().unwrap();
        }
    }

    /// Rejects invalid documents with syntax errors.
    #[test]
    fn test_skip_invalid() {
        for text in [
            "",
            "nul",
            "01",
            "1.",
            ".5",
            "-",
            "1e",
            "+1",
            "\"a",
            "\"\\x\"",
            "\"\\u12\"",
            "\"\\ud800\"",
            "\"\\udc00\"",
            "\"\\ud800\\u0041\"",
            "\"\u{1}\"",
            "[1,]",
            "[,1]",
            "[1 2]",
            "{\"a\"}",
            "{\"a\":1,}",
            "{1:2}",
            "[}",
            "{]",
            "NaN",
        ] {
            let mut reader = Reader::new(text.as_bytes());
            let error = reader.skip().and_then(|_| reader.finish()).unwrap_err();
            assert_eq!(error.kind(), DecodeErrorKind::Syntax, "{text}");
        }
        let mut reader = Reader::new(b"1 2");
        reader.skip().unwrap();
        assert_eq!(reader.finish().unwrap_err().offset(), 2);
        assert!(Reader::new(b"\"\xff\"").skip().is_err());
    }

    /// Limits the nesting depth, including in skipped values.
    #[test]
    fn test_max_depth() {
        let mut reader = Reader::new(b"[[[]]]").with_max_depth(3);
        reader.skip().unwrap();
        let error = Reader::new(b"[{\"a\":[]}]")
            .with_max_depth(2)
            .skip()
            .unwrap_err();
        assert_eq!(error.kind(), DecodeErrorKind::DepthLimitExceeded);
        assert_eq!(error.offset(), 6);
    }
}
