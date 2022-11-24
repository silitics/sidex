//! Deserialization helpers.

use std::cmp;

/// Sanitizes a size hint to prevent DoS attacks caused by excessive allocation.
#[inline]
fn sanitize_size_hint(hint: Option<usize>) -> usize {
    cmp::min(hint.unwrap_or(0), 4096)
}

mod content;

pub mod tagged;
