use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=SIDEX_GIT_VERSION");
    println!("cargo:rerun-if-changed=build.rs");

    let workspace_git = Path::new("../../.git");
    if workspace_git.exists() {
        println!("cargo:rerun-if-changed=../../.git/HEAD");
        println!("cargo:rerun-if-changed=../../.git/refs");
    }

    let version = std::env::var("SIDEX_GIT_VERSION")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(git_describe)
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

    println!("cargo:rustc-env=SIDEX_GIT_VERSION={version}");
}

fn git_describe() -> Option<String> {
    let output = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!v.is_empty()).then_some(v)
}
