use std::{env, path::PathBuf, process};

fn main() {
    let mut output_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    output_path.push("generated");

    let bundle_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    sidex_build_rs::build(bundle_dir.as_ref(), &output_path);

    process::Command::new("rustfmt")
        .arg(output_path.join("mod.rs"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
