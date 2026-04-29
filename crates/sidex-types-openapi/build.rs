fn main() {
    // The bundle and its `json_schema` dep both live inside this crate
    // (under `lib/`) so a downstream consumer that vendors only this crate
    // still has the schemas available at build time.
    sidex_build_rs::configure()
        .with_bundle("lib/openapi")
        .generate()
        .expect("Failed to generate code for the OpenAPI bundle.");
}
