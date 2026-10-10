use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn c_json_runtime_builds_and_passes_tests() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("crate must live under workspace/crates");
    let out_dir = workspace_dir.join("target").join("sidex-gen-c-tests");
    fs::create_dir_all(&out_dir).expect("failed to create C test output directory");

    let exe = out_dir.join(format!("runtime_json_test{}", env::consts::EXE_SUFFIX));
    let cc = env::var_os("CC").unwrap_or_else(|| "cc".into());

    let runtime_dir = manifest_dir.join("runtime");
    let test_src = manifest_dir.join("tests").join("runtime_json.c");
    let sources = [
        runtime_dir.join("sidex_types.c"),
        runtime_dir.join("sidex_json_reader.c"),
        runtime_dir.join("sidex_json_value.c"),
        runtime_dir.join("sidex_json_writer.c"),
        test_src,
    ];

    let compile = Command::new(&cc)
        .arg("-std=c99")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .arg("-I")
        .arg(&runtime_dir)
        .args(sources)
        .arg("-o")
        .arg(&exe)
        .output()
        .expect("failed to run C compiler");

    assert!(
        compile.status.success(),
        "C runtime test compilation failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        compile.status.code(),
        String::from_utf8_lossy(&compile.stdout),
        String::from_utf8_lossy(&compile.stderr)
    );

    let run = Command::new(&exe)
        .output()
        .expect("failed to run compiled C runtime test");

    assert!(
        run.status.success(),
        "C runtime test failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        run.status.code(),
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
}
