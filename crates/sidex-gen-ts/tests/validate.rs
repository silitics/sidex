//! Integration tests for the TypeScript `validate` plugin.
//!
//! Loads the validate fixture bundle, runs the TS generator with the
//! validate plugin enabled, and asserts the emitted code contains the
//! expected runtime calls.

use std::path::PathBuf;

use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen::Generator;
use sidex_gen::Job;
use sidex_gen_ts::TsGenerator;

fn load_fixture() -> (ir::Ir, ir::BundleIdx) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/validate_fixture");
    let ctx = sidex_diagnostics::DiagnosticCtx::new();
    ctx.exec(|| {
        let mut transformer = Transformer::new();
        let bundle = transformer
            .load_bundle_recursive(&path)
            .expect("fixture bundle loads");
        let unit = transformer.transform(bundle);
        (unit, bundle)
    })
}

fn render() -> String {
    let (unit, bundle) = load_fixture();
    let out_dir = tempfile::tempdir().expect("tempdir");
    let job = Job {
        unit: &unit,
        bundle,
        output: out_dir.path(),
        config: &serde_json::json!({ "plugins": ["validate"] }),
    };
    TsGenerator::new()
        .generate(job)
        .expect("ts generation succeeds");
    std::fs::read_to_string(out_dir.path().join("data.ts")).expect("data.ts present")
}

#[test]
fn record_with_string_field_emits_size_helpers() {
    let out = render();
    assert!(
        out.contains("__validate.minInclusive"),
        "expected minInclusive call, got:\n{out}"
    );
    assert!(
        out.contains("__validate.maxInclusive"),
        "expected maxInclusive call"
    );
    assert!(
        out.contains("__validate.charCount"),
        "expected charCount accessor"
    );
    assert!(
        out.contains("\"min_size\""),
        "expected min_size code, got snippet"
    );
}

#[test]
fn message_and_code_overrides_thread_through() {
    let out = render();
    assert!(
        out.contains("\"Email length is out of range.\""),
        "expected user message"
    );
    assert!(out.contains("\"format:order\""), "expected user code");
}

#[test]
fn regex_rule_emits_runtime_call() {
    let out = render();
    assert!(
        out.contains("__validate.regexMatch"),
        "expected regexMatch call"
    );
}

#[test]
fn wrapper_with_rules_emits_try_new() {
    let out = render();
    assert!(
        out.contains("export const Slug"),
        "expected Slug exported const"
    );
    assert!(out.contains("tryNew"), "expected Slug.tryNew, got:\n{out}");
}

#[test]
fn optional_field_validates_only_when_present() {
    let out = render();
    assert!(
        out.contains("__instance.nickname !== undefined"),
        "expected optional-field guard, got:\n{out}"
    );
}

#[test]
fn validators_are_under_namespaced_const() {
    let out = render();
    // `User` and `Slug` and `CodeOnly` all show up as exported consts in
    // the validators module; the per-field map for User is keyed by
    // `email`, `age`, `nickname`.
    assert!(out.contains("export const User"), "expected User const");
    assert!(out.contains("email:"), "expected email field validator");
    assert!(out.contains("age:"), "expected age field validator");
    assert!(
        out.contains("nickname:"),
        "expected nickname field validator"
    );
}

/// End-to-end: generate the fixture, ship it through `tsc` against an
/// inlined copy of the runtime, then run the compiled JavaScript with
/// `node` to assert validators behave the same way the Rust ones do.
///
/// Skipped when `node`/`tsc` aren't available in PATH (e.g. CI without
/// the Node toolchain).
#[test]
fn generated_code_runs_under_node() {
    if std::env::var_os("SIDEX_SKIP_NODE_TESTS").is_some() {
        eprintln!("Skipped: SIDEX_SKIP_NODE_TESTS is set");
        return;
    }
    let Some(node_path) = which("node") else {
        eprintln!("Skipped: node not in PATH");
        return;
    };
    let Some(tsc_path) = which("tsc") else {
        eprintln!("Skipped: tsc not in PATH");
        return;
    };

    let (unit, bundle) = load_fixture();
    let work = tempfile::tempdir().expect("tempdir");
    let gen_dir = work.path().join("gen");
    std::fs::create_dir_all(&gen_dir).unwrap();

    let job = Job {
        unit: &unit,
        bundle,
        output: &gen_dir,
        config: &serde_json::json!({ "plugins": ["validate"] }),
    };
    TsGenerator::new()
        .generate(job)
        .expect("ts generation succeeds");

    // The generated code imports from `@sidex/validate` and `@sidex/types`.
    // Stand in local copies so the test doesn't need a published/installed
    // copy of the runtime packages.
    let validate_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../languages/typescript/packages/validate/src/index.ts");
    let validate_dest = work.path().join("sidex_validate.ts");
    std::fs::copy(&validate_src, &validate_dest).expect("copy validate runtime");

    // Minimal `@sidex/types` shim — only `Nominal` and the builtins used
    // by the fixture are needed.
    std::fs::write(
        work.path().join("sidex_types.ts"),
        r#"
            declare const SIDEX_PATH_SYMBOL: unique symbol;
            type N<P extends string> = { [SIDEX_PATH_SYMBOL]: { [key in P]: null } };
            export type Nominal<T, P extends string> = T & N<P>;
            export namespace builtins {
                export type String = string;
                export type Bytes = Uint8Array;
                export type I8 = number;
                export type I16 = number;
                export type I32 = number;
                export type I64 = number;
                export type U8 = number;
                export type U16 = number;
                export type U32 = number;
                export type U64 = number;
                export type Idx = number;
                export type F32 = number;
                export type F64 = number;
                export type Bool = boolean;
                export type Unit = null;
                export type Sequence<T> = T[];
                export type ObjectMap<K extends string, V> = Record<K, V>;
            }
        "#,
    )
    .unwrap();

    // Rewrite generated import paths to point at the local runtime stubs.
    let data_path = gen_dir.join("data.ts");
    let data = std::fs::read_to_string(&data_path).expect("data.ts");
    let data = data
        .replace("@sidex/validate", "../sidex_validate")
        .replace("@sidex/types", "../sidex_types");
    std::fs::write(&data_path, data).unwrap();

    // A driver script that runs each validator on inputs and prints a JSON
    // summary of (path, code) tuples — easy to diff against the Rust
    // runtime test.
    let driver_path = work.path().join("driver.ts");
    std::fs::write(
        &driver_path,
        r#"
import * as validators from "./gen/data";

const reports: Record<string, string[][]> = {};

function record(name: string, report: { errors: readonly { path: string; code: string }[] }) {
    reports[name] = report.errors.map((e) => [e.path, e.code]);
}

record("user_valid", validators.User.validate({
    email: "test@example.com",
    age: 30,
    nickname: "rusty",
}));
record("user_empty_email", validators.User.validate({
    email: "",
    age: 30,
    nickname: undefined,
} as any));
record("user_age_oor", validators.User.validate({
    email: "x",
    age: 200,
    nickname: undefined,
} as any));
record("user_optional_empty", validators.User.validate({
    email: "x",
    age: 1,
    nickname: "",
}));
const slugBad = validators.Slug.tryNew("");
if (slugBad.ok) {
    reports["slug_bad_try_new"] = [];
} else {
    reports["slug_bad_try_new"] = slugBad.report.errors.map((e) => [e.path, e.code]);
}
const slugOk = validators.Slug.tryNew("hello");
if (slugOk.ok) {
    reports["slug_ok_try_new"] = [["ok", "ok"]];
} else {
    reports["slug_ok_try_new"] = slugOk.report.errors.map((e) => [e.path, e.code]);
}
record("code_format_override", validators.CodeOnly.validate({ code: "lower" } as any));

// Per-field validators: each field is itself an object with `.validate`,
// leaving room for other plugins to attach sibling methods later.
record("field_email_too_short", validators.User.fields.email.validate(""));
record("field_email_ok", validators.User.fields.email.validate("user@example.com"));
record("field_age_oor", validators.User.fields.age.validate(200));

console.log(JSON.stringify(reports));
"#,
    )
    .unwrap();

    // tsconfig that compiles both the runtime copy and the driver.
    std::fs::write(
        work.path().join("tsconfig.json"),
        r#"{
            "compilerOptions": {
                "target": "es2020",
                "module": "commonjs",
                "esModuleInterop": true,
                "strict": true,
                "outDir": "./out",
                "skipLibCheck": true,
                "lib": ["es2020", "dom"]
            },
            "include": ["driver.ts", "sidex_validate.ts", "sidex_types.ts", "gen/**/*.ts"]
        }"#,
    )
    .unwrap();

    let tsc_out = std::process::Command::new(&tsc_path)
        .current_dir(work.path())
        .output()
        .expect("invoke tsc");
    if !tsc_out.status.success() {
        panic!(
            "tsc failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&tsc_out.stdout),
            String::from_utf8_lossy(&tsc_out.stderr),
        );
    }

    let node_out = std::process::Command::new(&node_path)
        .arg(work.path().join("out/driver.js"))
        .output()
        .expect("invoke node");
    if !node_out.status.success() {
        panic!(
            "node failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&node_out.stdout),
            String::from_utf8_lossy(&node_out.stderr),
        );
    }
    let stdout = String::from_utf8_lossy(&node_out.stdout);
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("driver output isn't JSON: {e}\n--\n{stdout}"));

    let codes_for = |key: &str| -> Vec<(String, String)> {
        parsed[key]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|p| {
                let arr = p.as_array().unwrap();
                (
                    arr[0].as_str().unwrap_or("").to_owned(),
                    arr[1].as_str().unwrap_or("").to_owned(),
                )
            })
            .collect()
    };

    assert_eq!(codes_for("user_valid"), Vec::<(String, String)>::new());
    assert!(
        codes_for("user_empty_email")
            .iter()
            .any(|(p, c)| p == "/email" && c == "min_size"),
        "expected /email min_size, got {:?}",
        codes_for("user_empty_email"),
    );
    assert!(
        codes_for("user_age_oor")
            .iter()
            .any(|(p, c)| p == "/age" && c == "max"),
        "expected /age max, got {:?}",
        codes_for("user_age_oor"),
    );
    assert!(
        codes_for("user_optional_empty")
            .iter()
            .any(|(p, c)| p == "/nickname" && c == "min_size"),
        "expected /nickname min_size, got {:?}",
        codes_for("user_optional_empty"),
    );
    assert!(
        codes_for("slug_bad_try_new")
            .iter()
            .any(|(_, c)| c == "min_size"),
        "expected slug tryNew to fail with min_size, got {:?}",
        codes_for("slug_bad_try_new"),
    );
    assert_eq!(
        codes_for("slug_ok_try_new"),
        vec![("ok".to_owned(), "ok".to_owned())]
    );
    assert!(
        codes_for("code_format_override")
            .iter()
            .any(|(_, c)| c == "format:order"),
        "expected format:order code override, got {:?}",
        codes_for("code_format_override"),
    );

    // Per-field validators: standalone `User.fields.<name>.validate(value)`.
    // The path on errors is rooted at `/` (not `/email`) since the caller
    // passes the value directly, not a nested object.
    assert!(
        codes_for("field_email_too_short")
            .iter()
            .any(|(p, c)| p == "/" && c == "min_size"),
        "expected per-field email min_size, got {:?}",
        codes_for("field_email_too_short"),
    );
    assert_eq!(codes_for("field_email_ok"), Vec::<(String, String)>::new());
    assert!(
        codes_for("field_age_oor").iter().any(|(_, c)| c == "max"),
        "expected per-field age max, got {:?}",
        codes_for("field_age_oor"),
    );
}

fn which(bin: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let cand = dir.join(bin);
        if cand.is_file() {
            return Some(cand);
        }
    }
    None
}
