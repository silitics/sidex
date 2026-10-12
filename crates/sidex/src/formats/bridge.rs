//! Writing freeform values with Serde.

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

/// Encodes `value` in the JSON representation and passes a view of the
/// result to `write`.
pub(crate) fn encode<T: Encode + ?Sized, R, E: fmt::Display>(
    value: &T,
    mut write: impl FnMut(&SerializeView<'_>) -> Result<R, E>,
) -> Result<R, EncodeError> {
    let mut out = None;
    json::encode(value, &mut |view| {
        out = Some(write(&SerializeView(view)).map_err(EncodeError::new)?);
        Ok(())
    })?;
    out.ok_or_else(|| EncodeError::new("encoding produced no value"))
}

/// View of a freeform value serialized with Serde.
pub(crate) struct SerializeView<'a>(ValueView<'a>);

impl Serialize for SerializeView<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            ValueView::Null => serializer.serialize_unit(),
            ValueView::Bool(value) => serializer.serialize_bool(value),
            ValueView::Int(value) => {
                if let Ok(value) = i64::try_from(value) {
                    serializer.serialize_i64(value)
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
                    out.serialize_element(&SerializeView(item))
                        .map_err(|error| stash(&mut failure, error))
                });
                propagate(result, failure)?;
                out.end()
            }
            ValueView::Map(map) => {
                let mut out = serializer.serialize_map(map.size_hint())?;
                let mut failure = None;
                let result = map.each(&mut |key, item| {
                    out.serialize_entry(key, &SerializeView(item))
                        .map_err(|error| stash(&mut failure, error))
                });
                propagate(result, failure)?;
                out.end()
            }
        }
    }
}

/// Keeps a serializer's error, so it can be returned unchanged after the
/// view's callback has been aborted.
fn stash<E>(failure: &mut Option<E>, error: E) -> EncodeError {
    *failure = Some(error);
    EncodeError::new("serialization failed")
}

/// Returns the serializer's error or the encoding error, if any.
fn propagate<E: ser::Error>(result: Result<(), EncodeError>, failure: Option<E>) -> Result<(), E> {
    match (failure, result) {
        (Some(error), _) => Err(error),
        (None, Err(error)) => Err(E::custom(error)),
        (None, Ok(())) => Ok(()),
    }
}
