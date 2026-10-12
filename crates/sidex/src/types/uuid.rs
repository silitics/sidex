//! The `uuid` built-in type.

use std::fmt;
use std::str::FromStr;

use super::ParseError;

/// Universally unique identifier.
///
/// The text representation is the hyphenated form, for instance,
/// `67e55044-10b1-426f-9247-bb680e5fe0c8`. Formatting uses lowercase digits and
/// parsing accepts either case.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid([u8; 16]);

impl_scalar!(Uuid, "uuid", Uuid, uuid);

impl Uuid {
    /// The UUID with all bits set to zero.
    pub const NIL: Uuid = Uuid([0; 16]);

    /// Creates a UUID from its bytes in big-endian order.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Returns the bytes of the UUID in big-endian order.
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = || ParseError::new("UUID");
        let text = s.as_bytes();
        if text.len() != 36 || [8, 13, 18, 23].iter().any(|&index| text[index] != b'-') {
            return Err(error());
        }
        let mut digits = text.iter().filter(|&&c| c != b'-');
        let mut bytes = [0; 16];
        for byte in &mut bytes {
            let (Some(high), Some(low)) = (digits.next(), digits.next()) else {
                return Err(error());
            };
            *byte = hex_value(*high).ok_or_else(error)? << 4 | hex_value(*low).ok_or_else(error)?;
        }
        Ok(Self(bytes))
    }
}

#[cfg(feature = "uuid-1")]
impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.into_bytes())
    }
}

#[cfg(feature = "uuid-1")]
impl From<Uuid> for uuid::Uuid {
    fn from(value: Uuid) -> Self {
        uuid::Uuid::from_bytes(value.0)
    }
}

fn hex_value(digit: u8) -> Option<u8> {
    char::from(digit).to_digit(16).map(|value| value as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parsing accepts either case and formatting produces lowercase.
    #[test]
    fn text_roundtrip() {
        let uuid: Uuid = "67E55044-10b1-426f-9247-BB680E5FE0C8".parse().unwrap();
        assert_eq!(uuid.to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
        for invalid in [
            "67e5504410b1426f9247bb680e5fe0c8",
            "67e55044-10b1-426f-9247-bb680e5fe0c",
            "67e55044+10b1-426f-9247-bb680e5fe0c8",
            "67e55044-10b1-426f-9247-bb680e5fe0cg",
            "67e55044-10b1-426f-92-47bb680e5fe0c8",
        ] {
            assert!(invalid.parse::<Uuid>().is_err(), "{invalid}");
        }
    }
}
