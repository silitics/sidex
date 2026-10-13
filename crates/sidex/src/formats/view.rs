//! Borrowed views of values for writing.

use crate::JsonValue;
use crate::encode::EncodeError;
use crate::types::Map;
use crate::types::Str;

/// Borrowed view of a [`JsonValue`] whose sequences and maps compute their items on
/// demand.
///
/// The JSON representation produces views, which formats write without
/// building a [`JsonValue`] first.
#[derive(Clone, Copy)]
pub(crate) enum ValueView<'a> {
    /// Null.
    Null,
    /// Boolean.
    Bool(bool),
    /// Integer.
    Int(i128),
    /// Floating-point number.
    Float(f64),
    /// String.
    String(&'a str),
    /// Sequence of values.
    Seq(&'a dyn ValueSeqView),
    /// Map from strings to values.
    Map(&'a dyn ValueMapView),
}

/// Sequence of a [`ValueView`].
pub(crate) trait ValueSeqView {
    /// Number of items, if known in advance.
    fn size_hint(&self) -> Option<usize>;

    /// Calls `f` for each item in order.
    fn each(
        &self,
        f: &mut dyn FnMut(ValueView<'_>) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError>;
}

/// Map of a [`ValueView`].
pub(crate) trait ValueMapView {
    /// Number of entries, if known in advance.
    fn size_hint(&self) -> Option<usize>;

    /// Calls `f` for each entry in order.
    fn each(
        &self,
        f: &mut dyn FnMut(&str, ValueView<'_>) -> Result<(), EncodeError>,
    ) -> Result<(), EncodeError>;
}

impl JsonValue {
    /// Builds a value from a [`ValueView`].
    pub(crate) fn from_view(view: ValueView<'_>) -> Result<Self, EncodeError> {
        Ok(match view {
            ValueView::Null => JsonValue::Null,
            ValueView::Bool(value) => JsonValue::Bool(value),
            ValueView::Int(value) => JsonValue::Int(value),
            ValueView::Float(value) => JsonValue::Float(value),
            ValueView::String(value) => JsonValue::String(value.into()),
            ValueView::Seq(seq) => {
                let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                seq.each(&mut |item| {
                    items.push(JsonValue::from_view(item)?);
                    Ok(())
                })?;
                JsonValue::Seq(items.into())
            }
            ValueView::Map(map) => {
                let mut entries = Map::with_capacity(map.size_hint().unwrap_or(0));
                map.each(&mut |key, item| {
                    entries.insert(Str::from(key), JsonValue::from_view(item)?);
                    Ok(())
                })?;
                JsonValue::Map(entries)
            }
        })
    }
}
