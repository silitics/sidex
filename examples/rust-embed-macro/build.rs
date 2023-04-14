fn main() {
    sidex_build_rs::configure()
        .with_bundle("../todo-list/data")
        .generate()
        .expect("Failed to generate code for Sidex bundles.");
}
