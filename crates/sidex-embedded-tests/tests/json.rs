use std::fs;
use std::path::Path;

use sidex_embedded::DecodeErrorKind;
use sidex_embedded::json;
use sidex_embedded::types::Str;
use sidex_embedded_tests::generated::aliases::Thread;
use sidex_embedded_tests::generated::data::*;
use sidex_embedded_tests::generated::groups::Change;
use sidex_embedded_tests::generated::groups::Page;
use sidex_embedded_tests::generated::inline::Envelope;
use sidex_embedded_tests::generated::inline::Internal;
use sidex_embedded_tests::generated::serialization::OptionalFields;

/// Decodes each fixture of a type and checks that encoding the value yields
/// the fixture again.
macro_rules! test_fixtures {
    ($ty:ty, $name:literal) => {
        let dir = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/json/expected"
        ));
        for entry in fs::read_dir(dir.join($name)).unwrap() {
            let path = entry.unwrap().path();
            let text = fs::read(&path).unwrap();
            let decoded: $ty = json::from_slice(&text)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            assert_eq!(parse(&encode(&decoded)), parse(&text), "{}", path.display());
        }
    };
}

/// Every fixture of the shared JSON test bundle round-trips.
#[test]
fn test_fixtures() {
    test_fixtures!(VariantInternallyTagged<'_, i32>, "VariantInternallyTagged");
    test_fixtures!(VariantExternallyTagged<'_, i32>, "VariantExternallyTagged");
    test_fixtures!(VariantAdjacentlyTagged<'_, i32>, "VariantAdjacentlyTagged");
    test_fixtures!(VariantImplicitlyTagged<'_, bool>, "VariantImplicitlyTagged");
    test_fixtures!(MaybeRef<'_, u64>, "MaybeRefInteger");
    test_fixtures!(MaybeRef<'_, Point>, "MaybeRefPoint");
    test_fixtures!(Any<'_>, "Any");
    test_fixtures!(FallbackInternallyTagged<'_>, "FallbackInternallyTagged");
    test_fixtures!(FallbackExternallyTagged<'_>, "FallbackExternallyTagged");
    test_fixtures!(FallbackAdjacentlyTagged<'_>, "FallbackAdjacentlyTagged");
    test_fixtures!(FallbackImplicitlyTagged<'_>, "FallbackImplicitlyTagged");
    test_fixtures!(RecordFieldNamesDefault<'_>, "RecordFieldNamesDefault");
    test_fixtures!(
        RecordFieldNamesScreamingSnake<'_>,
        "RecordFieldNamesScreamingSnake"
    );
    test_fixtures!(JsRoundtrip<'_>, "JsRoundtrip");
}

/// Known cases take precedence over fallback cases, known cases with invalid
/// payloads do not fall back, and values that select no case of an implicitly
/// tagged variant fail to decode, like with the `sidex` crate.
#[test]
fn test_variant_selection() {
    assert!(matches!(
        json::from_str(r#"{"kind":"B","content":42}"#).unwrap(),
        FallbackInternallyTagged::B(42)
    ));
    assert!(matches!(
        json::from_str("42").unwrap(),
        FallbackImplicitlyTagged::Number(42)
    ));
    let error = json::from_str::<FallbackInternallyTagged<'_>>(r#"{"kind":"B","content":"x"}"#)
        .unwrap_err();
    assert_eq!(
        error.kind(),
        DecodeErrorKind::Unexpected("a 64-bit unsigned integer")
    );
    assert_eq!(error.offset(), 22);
    // The payload of a fallback case is the entire value.
    assert!(json::from_str::<FallbackExternallyTagged<'_>>(r#"{"C":"x"}"#).is_err());
    let FallbackExternallyTagged::Other(name) = json::from_str(r#""C""#).unwrap() else {
        panic!("expected the fallback case");
    };
    assert_eq!(name, "C");

    let error = json::from_str::<MaybeRef<'_, u64>>(r#" {"$ref": 1}"#).unwrap_err();
    assert_eq!(error.kind(), DecodeErrorKind::NoMatchingCase);
    assert_eq!(error.offset(), 1);
}

/// Records ignore unknown fields, treat `null` as absent for optional fields,
/// accept aliases, and report missing and duplicate fields.
#[test]
fn test_records() {
    let thread: Thread<'_> =
        json::from_str(r#"{"chatId": "a", "unknown": [1, {}], "title": null}"#).unwrap();
    assert_eq!(thread.thread_id, "a");
    assert!(thread.display_name.is_none());
    assert_eq!(encode(&thread), br#"{"threadId":"a"}"#);

    let thread: Thread<'_> =
        json::from_str(r#"{"threadId": "a", "oldTitle": "x", "oldTitle": "y"}"#).unwrap();
    assert_eq!(thread.display_name.unwrap(), "y");

    let error = json::from_str::<Thread<'_>>(r#"{"threadId": "a", "chatId": "b"}"#).unwrap_err();
    assert_eq!(error.kind(), DecodeErrorKind::DuplicateField("threadId"));
    let error = json::from_str::<Thread<'_>>(r#" {"title": "x"}"#).unwrap_err();
    assert_eq!(error.kind(), DecodeErrorKind::MissingField("threadId"));
    assert_eq!(error.offset(), 1);

    // `null` is a value of required fields whose type admits it.
    let fields: OptionalFields<'_> = json::from_str(
        r#"{"requiredNull": null, "requiredJson": null, "values": [null], "entries": {}, "optionalJson": null}"#,
    )
    .unwrap();
    assert!(fields.optional_json.is_none());
    assert_eq!(fields.required_json.as_text(), Some("null"));
}

/// Inline fields read their fields from the enclosing map, also as the merged
/// payload of an internally tagged variant.
#[test]
fn test_inline_fields() {
    let text = br#"{"tag":"Envelope","envelopeId":"e","value":"v","extra.note":"n"}"#;
    let Internal::Envelope(envelope) = json::from_slice(text).unwrap() else {
        panic!("expected the `Envelope` case");
    };
    assert_eq!(envelope.payload.value, "v");
    assert_eq!(envelope.extras.extra_note.unwrap(), "n");
    assert_eq!(parse(&encode(&Internal::Envelope(envelope))), parse(text));

    let envelope = Envelope {
        envelope_id: Str::new("e"),
        ..envelope
    };
    assert_eq!(
        encode(&envelope),
        br#"{"envelopeId":"e","value":"v","extra.note":"n"}"#
    );
}

/// Generic records hold sequences of their type argument, which decode on
/// access.
#[test]
fn test_generic_sequences() {
    let text = br#"{"items": [{"tag": "Added"}, {"tag": "Removed", "content": "x\ny"}]}"#;
    let page: Page<'_, Change<'_>> = json::from_slice(text).unwrap();
    let mut changes = page.items.iter();
    assert!(matches!(changes.next(), Some(Change::Added(_))));
    let Some(Change::Removed(id)) = changes.next() else {
        panic!("expected the `Removed` case");
    };
    assert_eq!(id.0, "x\ny");
    assert!(changes.next().is_none());
    assert_eq!(
        format!("{page:?}"),
        r#"Page { items: [Added(Item), Removed(ItemId("x\ny"))] }"#
    );
}

fn encode<T: sidex_embedded::Encode>(value: &T) -> Vec<u8> {
    let mut buffer = vec![0; json::encoded_len(value).unwrap()];
    json::to_slice(value, &mut buffer).unwrap();
    buffer
}

fn parse(text: &[u8]) -> serde_json::Value {
    serde_json::from_slice(text).unwrap()
}
