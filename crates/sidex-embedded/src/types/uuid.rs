//! The `uuid` type.

use core::fmt;
use core::str::FromStr;

use crate::types::ParseError;

/// Universally unique identifier.
///
/// The text representation is hyphenated and lowercase, for instance,
/// `67e55044-10b1-426f-9247-bb680e5fe0c8`. Parsing also accepts uppercase
/// digits.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Uuid([u8; 16]);

impl Uuid {
    /// The nil UUID, whose bits are all zero.
    pub const NIL: Uuid = Uuid([0; 16]);

    /// Creates a UUID from its bytes.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Bytes of the UUID.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, byte) in self.0.iter().enumerate() {
            if matches!(index, 4 | 6 | 8 | 10) {
                f.write_str("-")?;
            }
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl FromStr for Uuid {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, ParseError> {
        const ERROR: ParseError = ParseError::new("UUID");
        let text = text.as_bytes();
        if text.len() != 36 || [8, 13, 18, 23].iter().any(|index| text[*index] != b'-') {
            return Err(ERROR);
        }
        let mut digits = text
            .iter()
            .filter(|char| **char != b'-')
            .map(|char| char::from(*char).to_digit(16));
        let mut bytes = [0; 16];
        for byte in &mut bytes {
            let high = digits.next().flatten().ok_or(ERROR)?;
            let low = digits.next().flatten().ok_or(ERROR)?;
            *byte = (high << 4 | low) as u8;
        }
        Ok(Self(bytes))
    }
}

impl_text!(Uuid, "a UUID", 36);

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::Uuid;

    /// Parses hyphenated UUIDs in either case and writes them in lowercase.
    #[test]
    fn test_text() {
        let uuid: Uuid = "123E4567-e89b-12d3-a456-426614174000".parse().unwrap();
        assert_eq!(uuid.as_bytes()[..2], [0x12, 0x3e]);
        assert_eq!(uuid.to_string(), "123e4567-e89b-12d3-a456-426614174000");
        for text in [
            "123e4567e89b12d3a456426614174000",
            "123e4567-e89b-12d3-a456-42661417400",
            "123e4567-e89b-12d3-a456-42661417400g",
            "123e4567+e89b-12d3-a456-426614174000",
        ] {
            assert!(text.parse::<Uuid>().is_err(), "{text}");
        }
    }
}
