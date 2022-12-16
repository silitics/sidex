//! Deserialization helpers.

use std::cmp;

mod content;

pub mod tagged;

/// Limit for the size hints.
const SIZE_HINT_LIMIT: usize = 4096;

/// Sanitizes a size hint to prevent DoS attacks caused by excessive allocation.
#[inline]
fn sanitize_size_hint(hint: Option<usize>) -> usize {
    cmp::min(hint.unwrap_or(0), SIZE_HINT_LIMIT)
}
