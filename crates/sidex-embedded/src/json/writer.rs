//! Writing JSON text.

use core::fmt;
use core::fmt::Write as _;

use crate::MapKey;
use crate::error::EncodeError;
use crate::types::Str;

/// Destination of JSON text.
///
/// Implement this trait to write directly to a peripheral or a network
/// connection.
pub trait Sink {
    /// Writes all bytes.
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError>;
}

/// Writer of compact JSON text to a [`Sink`].
///
/// Each method writes one value, or starts an object or an array.
pub struct Writer<'w> {
    sink: &'w mut dyn Sink,
}

impl<'w> Writer<'w> {
    /// Creates a writer.
    pub fn new(sink: &'w mut dyn Sink) -> Self {
        Self { sink }
    }

    /// Writes `null`.
    pub fn null(&mut self) -> Result<(), EncodeError> {
        self.sink.write(b"null")
    }

    /// Writes a boolean.
    pub fn bool(&mut self, value: bool) -> Result<(), EncodeError> {
        self.sink.write(if value { b"true" } else { b"false" })
    }

    /// Writes a string.
    pub fn str(&mut self, value: &str) -> Result<(), EncodeError> {
        self.sink.write(b"\"")?;
        let bytes = value.as_bytes();
        let mut start = 0;
        for (index, byte) in bytes.iter().enumerate() {
            let escape: &[u8] = match byte {
                b'"' => b"\\\"",
                b'\\' => b"\\\\",
                b'\n' => b"\\n",
                b'\r' => b"\\r",
                b'\t' => b"\\t",
                0x08 => b"\\b",
                0x0C => b"\\f",
                0x00..=0x1F => {
                    &[
                        b'\\',
                        b'u',
                        b'0',
                        b'0',
                        HEX[usize::from(byte >> 4)],
                        HEX[usize::from(byte & 0xF)],
                    ]
                }
                _ => continue,
            };
            self.sink.write(&bytes[start..index])?;
            self.sink.write(escape)?;
            start = index + 1;
        }
        self.sink.write(&bytes[start..])?;
        self.sink.write(b"\"")
    }

    /// Starts writing an object.
    pub fn object(&mut self) -> Result<ObjectWriter<'_, 'w>, EncodeError> {
        self.sink.write(b"{")?;
        Ok(ObjectWriter {
            writer: self,
            first: true,
        })
    }

    /// Starts writing an array.
    pub fn array(&mut self) -> Result<ArrayWriter<'_, 'w>, EncodeError> {
        self.sink.write(b"[")?;
        Ok(ArrayWriter {
            writer: self,
            first: true,
        })
    }

    /// Writes JSON string content whose escape sequences the reader has
    /// checked.
    pub(crate) fn escaped_str(&mut self, content: &str) -> Result<(), EncodeError> {
        self.sink.write(b"\"")?;
        self.sink.write(content.as_bytes())?;
        self.sink.write(b"\"")
    }

    /// Writes JSON text that the reader has checked.
    pub(crate) fn raw(&mut self, text: &str) -> Result<(), EncodeError> {
        self.sink.write(text.as_bytes())
    }

    /// Writes formatted text, which must be valid JSON.
    pub(crate) fn fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), EncodeError> {
        let mut adapter = FmtAdapter {
            sink: &mut *self.sink,
            error: None,
        };
        match adapter.write_fmt(args) {
            Ok(()) => Ok(()),
            Err(fmt::Error) => {
                Err(adapter
                    .error
                    .expect("only the sink fails to write formatted values"))
            }
        }
    }

    /// Writes the comma separating the items of an object or array, unless the
    /// next item is the first.
    fn separator(&mut self, first: &mut bool) -> Result<(), EncodeError> {
        if !core::mem::replace(first, false) {
            self.sink.write(b",")?;
        }
        Ok(())
    }

    /// Writes formatted text as a string, which must not need escaping.
    pub(crate) fn fmt_str(&mut self, args: fmt::Arguments<'_>) -> Result<(), EncodeError> {
        self.sink.write(b"\"")?;
        self.fmt(args)?;
        self.sink.write(b"\"")
    }
}

/// Object being written.
pub struct ObjectWriter<'s, 'w> {
    writer: &'s mut Writer<'w>,
    first: bool,
}

impl<'w> ObjectWriter<'_, 'w> {
    /// Writes the key of a field, whose value must be written next.
    pub fn field(&mut self, name: &str) -> Result<&mut Writer<'w>, EncodeError> {
        self.key(&Str::new(name))
    }

    /// Writes the key of a map entry, whose value must be written next.
    pub fn key<'a, K: MapKey<'a>>(&mut self, key: &K) -> Result<&mut Writer<'w>, EncodeError> {
        self.writer.separator(&mut self.first)?;
        key.encode_key(self.writer)?;
        self.writer.sink.write(b":")?;
        Ok(self.writer)
    }

    /// Finishes the object.
    pub fn end(self) -> Result<(), EncodeError> {
        self.writer.sink.write(b"}")
    }
}

/// Array being written.
pub struct ArrayWriter<'s, 'w> {
    writer: &'s mut Writer<'w>,
    first: bool,
}

impl<'w> ArrayWriter<'_, 'w> {
    /// Starts the next item, which must be written next.
    pub fn item(&mut self) -> Result<&mut Writer<'w>, EncodeError> {
        self.writer.separator(&mut self.first)?;
        Ok(self.writer)
    }

    /// Finishes the array.
    pub fn end(self) -> Result<(), EncodeError> {
        self.writer.sink.write(b"]")
    }
}

/// [`Sink`] writing to a byte slice.
pub(crate) struct SliceSink<'b> {
    pub(crate) buffer: &'b mut [u8],
    pub(crate) len: usize,
}

impl Sink for SliceSink<'_> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        let end = self.len + bytes.len();
        self.buffer
            .get_mut(self.len..end)
            .ok_or(EncodeError::BufferFull)?
            .copy_from_slice(bytes);
        self.len = end;
        Ok(())
    }
}

/// [`Sink`] counting the bytes written to it.
pub(crate) struct CountingSink(pub(crate) usize);

impl Sink for CountingSink {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.0 += bytes.len();
        Ok(())
    }
}

const HEX: &[u8; 16] = b"0123456789abcdef";

/// Adapter writing formatted text to a [`Sink`].
struct FmtAdapter<'s> {
    sink: &'s mut dyn Sink,
    error: Option<EncodeError>,
}

impl fmt::Write for FmtAdapter<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.sink.write(text.as_bytes()).map_err(|error| {
            self.error = Some(error);
            fmt::Error
        })
    }
}
