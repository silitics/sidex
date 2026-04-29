fn main() {
    sidex_build_rs::configure()
        .with_bundle("../../lib/openapi")
        .generate()
        .expect("Failed to generate code for the OpenAPI bundle.");
}
