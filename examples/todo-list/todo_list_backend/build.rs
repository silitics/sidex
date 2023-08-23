fn main() {
    sidex_build_rs::configure()
        .with_bundle("../todo_list_data")
        .with_bundle("../todo_list_api")
        .generate()
        .expect("Failed to generate code for Sidex bundles.");
}
