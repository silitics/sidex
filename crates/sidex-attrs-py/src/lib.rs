//! Typed attribute schemas for the Python codegen target.
//!
//! These types are generated from
//! [`lib/py/schemas/attrs.sidex`](https://github.com/silitics/sidex/blob/main/lib/py/schemas/attrs.sidex).
//! Codegen consumers read them off the IR's `typed_attrs["py"]` map via
//! [`extract`].

mod generated;

pub use generated::attrs::*;
use serde_json::Value;
use sidex_ir as ir;

/// Pull the typed `py` attributes off any IR node's `typed_attrs` map and
/// deserialize them into the typed struct `T`. Returns `Ok(None)` if the
/// node has no `py` attrs.
pub fn extract<T>(
    typed_attrs: &std::collections::HashMap<String, Value>,
) -> serde_json::Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    match typed_attrs.get("py") {
        Some(value) => serde_json::from_value(value.clone()).map(Some),
        None => Ok(None),
    }
}

/// Convenience: extract `OpaqueTypeAttrs` from a `Def`. Returns `None` if
/// the def has no `py` attrs.
pub fn opaque_type_attrs(def: &ir::Def) -> serde_json::Result<Option<OpaqueTypeAttrs>> {
    extract(&def.typed_attrs)
}
