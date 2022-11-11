use super::{try_parse_manifest, Manifest};

pub fn builtin_manifest() -> Manifest {
    try_parse_manifest(include_str!("../../../../lib/sidex/sidex.toml")).unwrap()
}
