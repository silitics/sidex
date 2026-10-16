//! Tests constructors and defaults in generated Rust records.

use sidex::formats::json;
use sidex_serde_tests::generated::builder::Defaults;
use sidex_serde_tests::generated::builder::Empty;
use sidex_serde_tests::generated::builder::OptionalFields;
use sidex_serde_tests::generated::builder::Required;

/// Empty records and records with only defaulted fields implement `Default`
/// with the same field values and JSON representation as `new()`.
#[test]
fn default_matches_zero_argument_constructor() {
    assert_eq!(
        json::to_string(&Empty::default()).unwrap(),
        json::to_string(&Empty::new()).unwrap(),
    );

    let defaults = Defaults::default();
    assert!(!defaults.enabled);
    assert!(defaults.items.is_empty());
    assert!(defaults.nested.value.is_none());
    assert_eq!(
        json::to_string(&defaults).unwrap(),
        json::to_string(&Defaults::new()).unwrap(),
    );
}

/// An optional generic field does not require its type argument to implement
/// `Default`, while a record with a required field still takes that argument.
#[test]
fn optional_generic_default_needs_no_default_bound() {
    let optional = OptionalFields::<Required>::default();
    assert!(optional.value.is_none());
    assert!(OptionalFields::<Required>::new().value.is_none());

    let required = Required::new(42);
    assert_eq!(required.value, 42);
    assert!(required.note.is_none());
}
