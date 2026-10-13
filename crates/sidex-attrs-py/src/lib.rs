//! Typed attribute schemas for the Python codegen target.
//!
//! These types are generated from
//! [`lib/py/schemas/attrs.sidex`](https://github.com/silitics/sidex/blob/main/crates/sidex-core/lib/py/schemas/attrs.sidex).
//! Codegen consumers read them off the IR's `typed_attrs["py"]` map via
//! [`extract`].

mod generated;

pub use generated::attrs::*;
use sidex::DecodeError;
use sidex::JsonValue;
use sidex_ir as ir;

/// Pull the typed `py` attributes off any IR node's `typed_attrs` map and
/// deserialize them into the typed struct `T`. Returns `Ok(None)` if the
/// node has no `py` attrs.
pub fn extract<T>(
    typed_attrs: &std::collections::HashMap<String, JsonValue>,
) -> Result<Option<T>, DecodeError>
where
    T: sidex::Decode,
{
    match typed_attrs.get("py") {
        Some(value) => sidex::formats::json::from_value(value).map(Some),
        None => Ok(None),
    }
}

/// Convenience: extract `OpaqueTypeAttrs` from a `Def`. Returns `None` if
/// the def has no `py` attrs.
pub fn opaque_type_attrs(def: &ir::Def) -> Result<Option<OpaqueTypeAttrs>, DecodeError> {
    extract(&def.typed_attrs)
}
