fn main() {
    sidex_build_rs::configure()
        .with_bundle(".")
        .generate()
        .expect("Failed to compiled Sidex bundles.");
}
