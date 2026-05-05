fn main() {
    sidex_build_rs::configure()
        .with_bundle("./bundle")
        .generate()
        .expect("Failed to generate validate test bundle.");
}
