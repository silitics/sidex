//! Serde's private `Content` enum.

/// Serde's private `Content` enum.
#[cfg(feature = "serde-private-content")]
pub type Content<'de> = serde::__private::de::Content<'de>;

/// Serde's private `ContentDeserializer`.
#[cfg(feature = "serde-private-content")]
pub type ContentDeserializer<'de, E> = serde::__private::de::ContentDeserializer<'de, E>;

/// Serde's private `ContentRefDeserializer`.
#[cfg(feature = "serde-private-content")]
pub type ContentRefDeserializer<'a, 'de, E> =
    serde::__private::de::ContentRefDeserializer<'a, 'de, E>;
