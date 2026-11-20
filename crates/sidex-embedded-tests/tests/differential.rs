use std::path::Path;

use sidex_core::transformer::Transformer;
use sidex_embedded_tests::generated as embedded;
use sidex_serde_tests::generated as standard;

/// Number of random values per type.
const SAMPLES: u64 = 200;

/// Generates random valid JSON values of each type and checks that the
/// embedded runtime and the `sidex` crate both decode them and encode them to
/// the same JSON.
macro_rules! compare {
    ($ir:expr, $path:literal, $standard:ty, $embedded:ty) => {
        let typ = sidex_fuzz::lookup_type($ir, $path).unwrap();
        for seed in 0..SAMPLES {
            let text = sidex_fuzz::sample($ir, &typ, seed).unwrap().to_string();
            let standard = sidex::formats::json::from_str::<$standard>(&text)
                .map(|value| sidex::formats::json::to_string(&value).unwrap());
            let embedded = sidex_embedded::json::from_str::<$embedded>(&text).map(|value| {
                let mut buffer = vec![0; sidex_embedded::json::encoded_len(&value).unwrap()];
                sidex_embedded::json::to_slice(&value, &mut buffer).unwrap();
                String::from_utf8(buffer).unwrap()
            });
            match (standard, embedded) {
                (Ok(standard), Ok(embedded)) => {
                    assert_eq!(
                        serde_json::from_str::<serde_json::Value>(&embedded).unwrap(),
                        serde_json::from_str::<serde_json::Value>(&standard).unwrap(),
                        "{}: {text}",
                        $path
                    )
                }
                (standard, embedded) => {
                    panic!(
                        "{}: {text}: `sidex` {} but the embedded runtime {}",
                        $path,
                        standard.map_or_else(
                            |error| format!("failed with {error}"),
                            |_| "succeeded".into()
                        ),
                        embedded.map_or_else(
                            |error| format!("failed with {error}"),
                            |_| "succeeded".into()
                        ),
                    )
                }
            }
        }
    };
}

#[test]
fn test_random_values() {
    let ir = load_bundle();
    let ir = &ir;
    compare!(
        ir,
        "json_test_bundle::data::Any",
        standard::data::Any,
        embedded::data::Any<'_>
    );
    compare!(
        ir,
        "json_test_bundle::data::FallbackInternallyTagged",
        standard::data::FallbackInternallyTagged,
        embedded::data::FallbackInternallyTagged<'_>
    );
    compare!(
        ir,
        "json_test_bundle::data::FallbackExternallyTagged",
        standard::data::FallbackExternallyTagged,
        embedded::data::FallbackExternallyTagged<'_>
    );
    compare!(
        ir,
        "json_test_bundle::data::FallbackAdjacentlyTagged",
        standard::data::FallbackAdjacentlyTagged,
        embedded::data::FallbackAdjacentlyTagged<'_>
    );
    compare!(
        ir,
        "json_test_bundle::data::FallbackImplicitlyTagged",
        standard::data::FallbackImplicitlyTagged,
        embedded::data::FallbackImplicitlyTagged<'_>
    );
    compare!(
        ir,
        "json_test_bundle::data::Enumeration",
        standard::data::Enumeration,
        embedded::data::Enumeration
    );
    compare!(
        ir,
        "json_test_bundle::data::RecordFieldNamesScreamingSnake",
        standard::data::RecordFieldNamesScreamingSnake,
        embedded::data::RecordFieldNamesScreamingSnake<'_>
    );
    compare!(
        ir,
        "json_test_bundle::data::JsRoundtrip",
        standard::data::JsRoundtrip,
        embedded::data::JsRoundtrip<'_>
    );
    compare!(
        ir,
        "json_test_bundle::data::TimestampFields",
        standard::data::TimestampFields,
        embedded::data::TimestampFields
    );
    compare!(
        ir,
        "json_test_bundle::data::SubscriptionId",
        standard::data::SubscriptionId,
        embedded::data::SubscriptionId
    );
    compare!(
        ir,
        "json_test_bundle::serialization::OptionalFields",
        standard::serialization::OptionalFields,
        embedded::serialization::OptionalFields<'_>
    );
    compare!(
        ir,
        "json_test_bundle::serialization::Internal",
        standard::serialization::Internal,
        embedded::serialization::Internal<'_>
    );
    compare!(
        ir,
        "json_test_bundle::serialization::Adjacent",
        standard::serialization::Adjacent,
        embedded::serialization::Adjacent<'_>
    );
    compare!(
        ir,
        "json_test_bundle::serialization::External",
        standard::serialization::External,
        embedded::serialization::External<'_>
    );
    compare!(
        ir,
        "json_test_bundle::serialization::Implicit",
        standard::serialization::Implicit,
        embedded::serialization::Implicit<'_>
    );
    compare!(
        ir,
        "json_test_bundle::serialization::Encodings",
        standard::serialization::Encodings,
        embedded::serialization::Encodings<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::Nested",
        standard::inline::Nested,
        embedded::inline::Nested<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::WrappedEnvelope",
        standard::inline::WrappedEnvelope,
        embedded::inline::WrappedEnvelope<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::CrossSchema",
        standard::inline::CrossSchema,
        embedded::inline::CrossSchema<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::RenamedEnvelope",
        standard::inline::RenamedEnvelope,
        embedded::inline::RenamedEnvelope<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::Internal",
        standard::inline::Internal,
        embedded::inline::Internal<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::Adjacent",
        standard::inline::Adjacent,
        embedded::inline::Adjacent<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::External",
        standard::inline::External,
        embedded::inline::External<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::Implicit",
        standard::inline::Implicit,
        embedded::inline::Implicit<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::TypedCase",
        standard::inline::TypedCase,
        embedded::inline::TypedCase<'_>
    );
    compare!(
        ir,
        "json_test_bundle::inline::NamedCase",
        standard::inline::NamedCase,
        embedded::inline::NamedCase<'_>
    );
    compare!(
        ir,
        "json_test_bundle::aliases::Event",
        standard::aliases::Event,
        embedded::aliases::Event<'_>
    );
    compare!(
        ir,
        "json_test_bundle::groups::SyncChanges",
        standard::groups::SyncChanges,
        embedded::groups::SyncChanges<'_>
    );
    compare!(
        ir,
        "json_test_bundle::builder::Defaults",
        standard::builder::Defaults,
        embedded::builder::Defaults<'_>
    );
}

fn load_bundle() -> sidex_core::ir::Ir {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/json");
    let ctx = sidex_core::diagnostics::DiagnosticCtx::new();
    ctx.exec(|| {
        let mut transformer = Transformer::new();
        let bundle = transformer.load_bundle_recursive(&path).unwrap();
        transformer.transform(bundle)
    })
}
