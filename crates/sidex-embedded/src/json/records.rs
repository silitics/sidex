//! Reading the JSON representation of records and variants.
//!
//! Generated code uses these helpers, which implement the parts of the JSON
//! representation that do not depend on a definition's types.

use super::Kind;
use super::Reader;
use crate::Decode;
use crate::error::DecodeError;
use crate::error::DecodeErrorKind;
use crate::types::Str;

/// Fields of a record read with [`Reader::record`].
///
/// The values of fields are decoded when they are taken from the record.
#[derive(Debug)]
pub struct Fields<'a, const N: usize> {
    /// Reader inside the object of the record.
    reader: Reader<'a>,
    /// Position of the object.
    offset: usize,
    names: &'static [&'static [&'static str]; N],
    /// Index of the name and position of the value of each field.
    locations: [Option<(usize, usize)>; N],
}

impl<'a, const N: usize> Fields<'a, N> {
    /// Decodes the value of a required field.
    pub fn required<T: Decode<'a>>(&self, field: usize) -> Result<T, DecodeError> {
        let Some((_, pos)) = self.locations[field] else {
            return Err(DecodeError::new(
                DecodeErrorKind::MissingField(self.names[field][0]),
                self.offset,
            ));
        };
        T::decode(&mut self.reader.at(pos))
    }

    /// Decodes the value of an optional field. `null` is absent, even if the
    /// field's type admits it.
    pub fn optional<T: Decode<'a>>(&self, field: usize) -> Result<Option<T>, DecodeError> {
        let Some((_, pos)) = self.locations[field] else {
            return Ok(None);
        };
        let mut reader = self.reader.at(pos);
        if reader.take_null()? {
            return Ok(None);
        }
        T::decode(&mut reader).map(Some)
    }
}

/// Value of an externally tagged variant.
#[derive(Debug)]
pub enum External<'a> {
    /// String naming a case without payload.
    Name(Str<'a>),
    /// Map with a single entry from the name of a case to its payload.
    Entry(Str<'a>, Reader<'a>),
}

impl<'a> Reader<'a> {
    /// Reads the object of a record whose fields have the given JSON names.
    ///
    /// Each field has a name and any number of aliases. If an object repeats a
    /// key, its last entry counts. Entries for the same field under different
    /// names are an error. Entries with other keys are ignored.
    pub fn record<const N: usize>(
        &mut self,
        names: &'static [&'static [&'static str]; N],
    ) -> Result<Fields<'a, N>, DecodeError> {
        let offset = self.value_start();
        let mut object = self.object()?;
        let reader = self.clone();
        let mut locations = [None; N];
        while let Some(key) = object.next_key(self)? {
            let found = names.iter().enumerate().find_map(|(field, keys)| {
                keys.iter()
                    .position(|name| key == *name)
                    .map(|name_index| (field, name_index))
            });
            if let Some((field, name_index)) = found {
                if locations[field].is_some_and(|(other, _)| other != name_index) {
                    return Err(self.error(DecodeErrorKind::DuplicateField(names[field][0])));
                }
                locations[field] = Some((name_index, self.value_start()));
            }
            self.skip()?;
        }
        Ok(Fields {
            reader,
            offset,
            names,
            locations,
        })
    }

    /// Reads the value of an internally or adjacently tagged variant and
    /// returns the name of its case.
    ///
    /// The inner result is an error if the value selects no case because it is
    /// not a map or its tag is absent or not a string. Such values may belong
    /// to a fallback case.
    pub fn tag(&mut self, tag: &'static str) -> Result<Result<Str<'a>, DecodeError>, DecodeError> {
        if self.peek()? != Kind::Object {
            let error = self.error(DecodeErrorKind::Unexpected("a map"));
            self.skip()?;
            return Ok(Err(error));
        }
        let mut name = Err(self.error(DecodeErrorKind::MissingField(tag)));
        let mut object = self.object()?;
        while let Some(key) = object.next_key(self)? {
            if key == tag {
                name = match self.peek()? {
                    Kind::String => Ok(self.string()?),
                    _ => {
                        let error = self.error(DecodeErrorKind::Unexpected("a string"));
                        self.skip()?;
                        Err(error)
                    }
                };
            } else {
                self.skip()?;
            }
        }
        Ok(name)
    }

    /// Returns a reader for the value of the given field of the object at the
    /// reader's position, without advancing the reader.
    ///
    /// The last entry of the field counts.
    pub fn content(&self, field: &'static str) -> Result<Reader<'a>, DecodeError> {
        let mut reader = self.clone();
        let mut object = reader.object()?;
        let mut value = None;
        while let Some(key) = object.next_key(&mut reader)? {
            if key == field {
                value = Some(reader.value_start());
            }
            reader.skip()?;
        }
        match value {
            Some(pos) => Ok(reader.at(pos)),
            None => Err(self.error(DecodeErrorKind::MissingField(field))),
        }
    }

    /// Reads the value of an externally tagged variant.
    ///
    /// The inner result is an error if the value is neither a string nor a map
    /// with a single entry. Such values may belong to a fallback case.
    pub fn external(&mut self) -> Result<Result<External<'a>, DecodeError>, DecodeError> {
        let unexpected = self.error(DecodeErrorKind::Unexpected(
            "a string or a map with a single entry",
        ));
        match self.peek()? {
            Kind::String => return Ok(Ok(External::Name(self.string()?))),
            Kind::Object => {}
            _ => {
                self.skip()?;
                return Ok(Err(unexpected));
            }
        }
        let mut object = self.object()?;
        let mut entry = None;
        let mut entries = 0;
        while let Some(key) = object.next_key(self)? {
            entry = Some((key, self.clone()));
            entries += 1;
            self.skip()?;
        }
        Ok(match entry {
            Some((key, payload)) if entries == 1 => Ok(External::Entry(key, payload)),
            _ => Err(unexpected),
        })
    }
}

/// Returns the index of the case with the given JSON name.
pub fn case(name: &Str<'_>, names: &[&str]) -> Option<usize> {
    names.iter().position(|candidate| name == candidate)
}
