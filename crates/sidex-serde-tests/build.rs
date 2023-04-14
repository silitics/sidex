fn main() {
    sidex_build_rs::configure()
        .with_bundle("../../tests/json")
        .generate()
        .expect("Failed to generate code for Sidex bundles.");
}
