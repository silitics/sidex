//! Writing JSON values with Serde.

use std::cell::RefCell;
use std::fmt;

use serde::Serialize;
use serde::Serializer;
use serde::ser;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;

use crate::Encode;
use crate::EncodeError;
use crate::formats::json;
use crate::formats::view::ValueView;

/// Syntax that a [`SerializeView`] is written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Syntax {
    Json,
    /// TOML has no `null` and only signed 64-bit integers. Map entries whose
    /// value is `null` are omitted, `null` in arrays cannot be written, and
    /// larger integers are decimal strings.
    Toml,
}

/// Encodes `value` in the JSON representation and passes a view of the
/// result to `write`.
pub(crate) fn encode<T: Encode + ?Sized, R, E: fmt::Display>(
    value: &T,
    syntax: Syntax,
    mut write: impl FnMut(&SerializeView<'_, '_>) -> Result<R, E>,
) -> Result<R, EncodeError> {
    let mut out = None;
    let failure = RefCell::new(None);
    json::encode(value, &mut |view| {
        let view = SerializeView {
            view,
            syntax,
            failure: &failure,
        };
        // Serializers may replace the errors of nested values with their own.
        let result =
            write(&view).map_err(|error| failure.take().unwrap_or_else(|| EncodeError::new(error)));
        out = Some(result?);
        Ok(())
    })?;
    out.ok_or_else(|| EncodeError::new("encoding produced no value"))
}

/// View of a JSON value serialized with Serde.
pub(crate) struct SerializeView<'a, 'f> {
    view: ValueView<'a>,
    syntax: Syntax,
    /// First encoding error of the value or its nested values.
    failure: &'f RefCell<Option<EncodeError>>,
}

impl<'f> SerializeView<'_, 'f> {
    fn nested<'b>(&self, view: ValueView<'b>) -> SerializeView<'b, 'f> {
        SerializeView {
            view,
            syntax: self.syntax,
            failure: self.failure,
        }
    }

    /// Records an encoding error, unless an earlier one has been recorded.
    fn fail<E: ser::Error>(&self, error: EncodeError) -> E {
        let error_ = E::custom(&error);
        self.failure.borrow_mut().get_or_insert(error);
        error_
    }
}

impl Serialize for SerializeView<'_, '_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let toml = self.syntax == Syntax::Toml;
        match self.view {
            ValueView::Null => serializer.serialize_unit(),
            ValueView::Bool(value) => serializer.serialize_bool(value),
            ValueView::Int(value) => {
                if let Ok(value) = i64::try_from(value) {
                    serializer.serialize_i64(value)
                } else if toml {
                    serializer.collect_str(&value)
                } else if let Ok(value) = u64::try_from(value) {
                    serializer.serialize_u64(value)
                } else {
                    serializer.serialize_i128(value)
                }
            }
            ValueView::Float(value) => serializer.serialize_f64(value),
            ValueView::String(value) => serializer.serialize_str(value),
            ValueView::Seq(seq) => {
                let mut out = serializer.serialize_seq(seq.size_hint())?;
                let mut failure = None;
                let result = seq.each(&mut |item| {
                    if toml && matches!(item, ValueView::Null) {
                        return Err(EncodeError::new(
                            "TOML has no representation of `null` in arrays",
                        ));
                    }
                    out.serialize_element(&self.nested(item))
                        .map_err(|error| stash(&mut failure, error))
                });
                self.propagate(result, failure)?;
                out.end()
            }
            ValueView::Map(map) => {
                let size_hint = if toml { None } else { map.size_hint() };
                let mut out = serializer.serialize_map(size_hint)?;
                let mut failure = None;
                let result = map.each(&mut |key, item| {
                    if toml && matches!(item, ValueView::Null) {
                        return Ok(());
                    }
                    out.serialize_entry(key, &self.nested(item))
                        .map_err(|error| stash(&mut failure, error))
                });
                self.propagate(result, failure)?;
                out.end()
            }
        }
    }
}

impl SerializeView<'_, '_> {
    /// Returns the serializer's error or the encoding error, if any.
    fn propagate<E: ser::Error>(
        &self,
        result: Result<(), EncodeError>,
        failure: Option<E>,
    ) -> Result<(), E> {
        match (failure, result) {
            (Some(error), _) => Err(error),
            (None, Err(error)) => Err(self.fail(error)),
            (None, Ok(())) => Ok(()),
        }
    }
}

/// Keeps a serializer's error, so it can be returned unchanged after the
/// view's callback has been aborted.
fn stash<E>(failure: &mut Option<E>, error: E) -> EncodeError {
    *failure = Some(error);
    EncodeError::new("serialization failed")
}
