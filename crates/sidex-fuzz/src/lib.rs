#![doc = include_str!("../README.md")]

mod config;
mod source;
mod walker;

pub use config::Config;
use sidex_diagnostics::Result;
use sidex_ir as ir;
pub use source::Source;
pub use walker::generate;

/// Generate a JSON value satisfying `typ` from a fresh source seeded with
/// `seed`, using the default [`Config`].
///
/// Convenience wrapper around [`generate`] for the common case of "give me one
/// sample with this seed". The same `(ir, typ, seed)` tuple always produces
/// the same value.
pub fn sample(ir: &ir::Ir, typ: &ir::Type, seed: u64) -> Result<serde_json::Value> {
    let mut source = Source::from_seed(seed);
    generate(ir, typ, &mut source, &Config::default())
}

/// Resolve a fully-qualified type path of the form `bundle::schema::Name` to
/// an [`ir::Type`] with no substitutions for its type parameters.
///
/// Returns `None` if no definition matches the path. Generic types come back
/// with type variables left unsubstituted, which the walker will reject with a
/// clear error — pass concrete substitutions via the returned type if needed.
pub fn lookup_type(ir: &ir::Ir, path: &str) -> Option<ir::Type> {
    let mut parts = path.splitn(3, "::");
    let bundle_name = parts.next()?;
    let schema_name = parts.next()?;
    let def_name = parts.next()?;
    for (bundle_idx, bundle) in ir.bundles.iter().enumerate() {
        if bundle.metadata.name != bundle_name {
            continue;
        }
        let bundle_idx = ir::BundleIdx::from(bundle_idx);
        for (schema_idx, schema) in ir.schemas_of(bundle_idx) {
            if schema.name != schema_name {
                continue;
            }
            for (def_idx, def) in ir.defs_of(schema_idx) {
                if def.name.as_str() == def_name {
                    return Some(ir::Type::new(ir::TypeKind::Instance(
                        ir::InstanceType::new(ir::DefRef::new(bundle_idx, schema_idx, def_idx)),
                    )));
                }
            }
        }
    }
    None
}
