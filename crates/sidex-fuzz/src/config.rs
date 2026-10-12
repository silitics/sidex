//! Knobs for bounding generated values.

use sidex_attrs_json::types::JsonType;
use sidex_attrs_json::types::JsonUnionType;

/// Bounds on the size and depth of generated structures.
///
/// All limits are soft caps on draws, not hard truncations: the [`Source`]'s
/// length-bias keeps typical structures small and cuts off at these maxima.
///
/// [`Source`]: crate::source::Source
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum recursion depth for nested types.
    ///
    /// When the depth limit is reached, the walker prefers terminating choices:
    /// unit variants in sums, and leaf JSON types under `any`.
    pub max_depth: usize,
    /// Maximum number of elements in a generated list, map, or object.
    pub max_collection_len: usize,
    /// Maximum number of characters in a generated string or map key.
    pub max_string_len: usize,
    /// JSON-shape fallback for unbound type variables.
    ///
    /// When the walker hits a generic type whose parameters were not
    /// substituted (e.g. `Variant<T>` invoked without picking a `T`), it draws
    /// a value from this union. Default: `string` — the most permissive scalar
    /// shape that round-trips through every target's primitive `string` type.
    pub unbound_var_default: JsonUnionType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_depth: 6,
            max_collection_len: 8,
            max_string_len: 16,
            unbound_var_default: JsonUnionType {
                types: std::iter::once(JsonType::String).collect(),
            },
        }
    }
}
