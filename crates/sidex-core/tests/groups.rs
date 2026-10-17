//! Tests for group membership of definitions.

use std::path::PathBuf;

use sidex_core::diagnostics::DiagnosticCtx;
use sidex_core::transformer::Transformer;

/// Membership extends from assigned definitions to all definitions of the
/// same bundle they use, including recursive types and request responses, but
/// not into dependencies.
#[test]
fn membership_extends_to_used_definitions() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/groups");
    let ctx = DiagnosticCtx::new();
    let ir = ctx.exec(|| {
        let mut transformer = Transformer::new();
        let bundle = transformer.load_bundle_recursive(&fixture).unwrap();
        transformer.transform(bundle)
    });
    assert_eq!(ctx.report().diagnostics().count(), 0);

    let mut groups: Vec<String> = ir
        .defs
        .iter()
        .filter(|def| !ir[ir.schemas[def.schema.idx()].bundle].is_internal)
        .map(|def| format!("{}: {}", def.name.as_str(), def.groups.join(", ")))
        .collect();
    groups.sort();
    assert_eq!(
        groups,
        [
            "Category: admin, device",
            "Device: admin",
            "DeviceId: admin, device",
            "Event: device",
            "EventKind: device",
            "GetCatalog: admin, device",
            "ListDevices: admin",
            "Page: admin",
            "Payload: device",
            "PublishEvents: device",
            "PublishEventsOutput: device",
            "Timestamp: ",
            "Unused: ",
        ]
    );
}
