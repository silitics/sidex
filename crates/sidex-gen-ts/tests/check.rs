//! Generate, type-check, and execute structural checks against Rust JSON readers.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use serde::Serialize;
use sidex::Decode;
use sidex::formats::json;
use sidex::types::Bytes;
use sidex::types::Date;
use sidex::types::Map;
use sidex::types::Str;
use sidex::types::Timestamp;
use sidex::types::Uuid;
use sidex_core::ir;
use sidex_core::transformer::Transformer;
use sidex_gen::Generator;
use sidex_gen::Job;
use sidex_gen_ts::TsGenerator;
use sidex_gen_ts::config::Config;
use sidex_serde_tests::generated::data::*;

#[derive(Serialize)]
struct Document {
    typ: String,
    text: String,
    accepted: bool,
}

fn load(path: &Path) -> (ir::Ir, ir::BundleIdx) {
    let diagnostics = sidex_diagnostics::DiagnosticCtx::new();
    let result = diagnostics.exec(|| {
        let mut transformer = Transformer::new();
        let bundle = transformer
            .load_bundle_recursive(path)
            .expect("fixture loads");
        let unit = transformer.transform(bundle);
        (unit, bundle, transformer)
    });
    let report = diagnostics.report();
    report.eprint(&result.2.sources);
    assert!(!report.has_error(), "fixture has no diagnostics");
    (result.0, result.1)
}

fn generate(unit: &ir::Ir, bundle: ir::BundleIdx, out: &Path, config: &Config) {
    std::fs::create_dir_all(out).unwrap();
    let config = serde_json::to_value(config).unwrap();
    TsGenerator::new()
        .generate(Job {
            unit,
            bundle,
            output: out,
            config: &config,
        })
        .unwrap();
    for entry in std::fs::read_dir(out).unwrap() {
        let path = entry.unwrap().path();
        let source = std::fs::read_to_string(&path)
            .unwrap()
            .replace("@sidex/types", "../runtime/index")
            .replace("@sidex/validate", "../validate");
        std::fs::write(path, source).unwrap();
    }
}

fn tool(name: &str) -> Option<PathBuf> {
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

/// Default checks cover every definition and retain wire checks with custom mappings.
#[test]
fn generated_checks_are_always_present() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/check_fixture");
    let (unit, bundle) = load(&path);
    let out = tempfile::tempdir().unwrap();
    let mut config = Config::default();
    config
        .types
        .table
        .insert("::core::builtins::timestamp".into(), "string".into());
    config
        .external
        .insert("identity".into(), "../identity".into());
    generate(&unit, bundle, out.path(), &config);
    let source = std::fs::read_to_string(out.path().join("data.ts")).unwrap();
    assert!(source.contains("checker<Task>"));
    assert!(source.contains("value is Page<T>"));
    assert!(source.contains("builtin(\"timestamp\")"));
    assert!(source.contains("__bundle_identity.users.User.checker"));
    assert!(source.contains("[\"display.label\""));
    assert!(!source.contains("@sidex/validate"));
}

/// Runs all structural cases under Node, including Rust reader acceptance and paths.
#[test]
fn generated_checks_run_under_node() {
    if std::env::var_os("SIDEX_SKIP_NODE_TESTS").is_some() {
        return;
    }
    let (Some(node), Some(tsc)) = (tool("node"), tool("tsc")) else {
        eprintln!("Skipped: node/tsc unavailable");
        return;
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let work = tempfile::tempdir().unwrap();
    let (unit, bundle) = load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/check_fixture"));
    let mut config = Config::default();
    config.plugins.push("validate".into());
    config
        .external
        .insert("identity".into(), "../identity".into());
    config
        .types
        .table
        .insert("::core::builtins::timestamp".into(), "string".into());
    generate(&unit, bundle, &work.path().join("gen"), &config);
    let identity = unit
        .bundles
        .iter()
        .enumerate()
        .find(|(_, bundle)| bundle.metadata.name == "identity")
        .unwrap()
        .0;
    generate(
        &unit,
        ir::BundleIdx::from(identity),
        &work.path().join("identity"),
        &Config::default(),
    );
    let (json_unit, json_bundle) = load(&root.join("tests/json"));
    generate(
        &json_unit,
        json_bundle,
        &work.path().join("json"),
        &Config::default(),
    );

    std::fs::create_dir(work.path().join("runtime")).unwrap();
    for file in [
        "index.ts",
        "builtins.ts",
        "nominal.ts",
        "check.ts",
        "temporal.ts",
    ] {
        std::fs::copy(
            root.join("languages/typescript/packages/types/src")
                .join(file),
            work.path().join("runtime").join(file),
        )
        .unwrap();
    }
    std::fs::copy(
        root.join("languages/typescript/packages/validate/src/index.ts"),
        work.path().join("validate.ts"),
    )
    .unwrap();
    let docs = documents(&root);
    let driver = include_str!("check_driver.ts")
        .replace("DOCUMENTS", &serde_json::to_string(&docs).unwrap());
    std::fs::write(work.path().join("driver.ts"), driver).unwrap();
    std::fs::write(
        work.path().join("tsconfig.json"),
        r#"{
        "compilerOptions": { "target": "es2020", "module": "commonjs", "strict": true,
          "noUncheckedIndexedAccess": true, "outDir": "./out", "lib": ["es2020", "dom"] },
        "include": ["**/*.ts"]
    }"#,
    )
    .unwrap();
    let compiled = Command::new(tsc).current_dir(work.path()).output().unwrap();
    assert!(
        compiled.status.success(),
        "tsc: {}",
        String::from_utf8_lossy(&compiled.stdout)
    );
    let executed = Command::new(node)
        .arg(work.path().join("out/driver.js"))
        .output()
        .unwrap();
    assert!(
        executed.status.success(),
        "node: {}\n{}",
        String::from_utf8_lossy(&executed.stdout),
        String::from_utf8_lossy(&executed.stderr)
    );
    println!("{}", String::from_utf8_lossy(&executed.stdout));
}

/// Cases for new wire features, plus fixtures accepted by the generated Rust types.
fn documents(root: &Path) -> Vec<Document> {
    let mut docs = Vec::new();
    for (typ, accepted, texts) in [
        (
            "Task",
            true,
            vec![
                r#"{"title":"a","done":true}"#,
                r#"{"title":"","done":false,"note":null,"future":42}"#,
            ],
        ),
        (
            "Task",
            false,
            vec![r#"{"title":1,"done":"false","note":2}"#, r#"{}"#, r#"[]"#],
        ),
        ("Empty", true, vec![r#"{"unknown":1}"#]),
        (
            "Ids",
            true,
            vec![r#"["18446744073709551615","+42",9007199254740992]"#],
        ),
        ("Ids", false, vec![r#"["18446744073709551616",-1,"-0"]"#]),
        (
            "Tasks",
            true,
            vec![r#"{"items":[{"title":"x","done":false}],"next":null}"#],
        ),
        (
            "Tasks",
            false,
            vec![r#"{"items":[{},{"title":1,"done":null}],"next":{"items":false}}"#],
        ),
        (
            "Wrapped",
            true,
            vec![r#"{"items":[{"title":"x","done":true}]}"#],
        ),
        (
            "Inline",
            true,
            vec![r#"{"x":1,"y":2,"display.label":"here","optional-value":null,"future":true}"#],
        ),
        (
            "Inline",
            false,
            vec![r#"{"position":{"x":1,"y":2},"display.label":1}"#],
        ),
        (
            "Internal",
            true,
            vec![
                r#"{"kind":"no_value","extra":1}"#,
                r#"{"kind":"record","x":1,"y":2}"#,
                r#"{"kind":"parameter","title":"a","done":false}"#,
                r#"{"kind":"renamed","payload":{"x":1,"y":2}}"#,
            ],
        ),
        (
            "Internal",
            false,
            vec![
                r#"{"kind":"Record","x":1,"y":2}"#,
                r#"{"kind":"record","x":true}"#,
                r#"{"kind":"parameter"}"#,
                r#"{"kind":1}"#,
            ],
        ),
        (
            "Adjacent",
            true,
            vec![
                r#"{"kind":"Empty","value":1}"#,
                r#"{"kind":"Point","value":{"x":1,"y":2},"extra":1}"#,
            ],
        ),
        (
            "Adjacent",
            false,
            vec![
                r#"{"kind":"Point","x":1,"y":2}"#,
                r#"{"kind":"Point","value":null}"#,
            ],
        ),
        (
            "External",
            true,
            vec![
                r#""no-value""#,
                r#"{"point":{"x":1,"y":2,"extra":1}}"#,
                r#"{"list":{"items":[]}}"#,
            ],
        ),
        (
            "External",
            false,
            vec![
                r#"{"no-value":null}"#,
                r#""point""#,
                r#"{"point":{"x":1,"y":2},"extra":1}"#,
            ],
        ),
        (
            "Implicit",
            true,
            vec![r#""None""#, "127", r#"{"x":1,"y":2,"extra":true}"#],
        ),
        ("Implicit", false, vec!["128", "null", r#"{"x":true}"#]),
        (
            "Opaques",
            true,
            vec![r#"{"nullable":null,"container":[],"unknown":{"free":[1]},"primitive":true}"#],
        ),
        (
            "Opaques",
            false,
            vec![r#"{"nullable":42,"container":1,"unknown":null,"primitive":"1"}"#],
        ),
        (
            "OtherBundle",
            true,
            vec![
                r#"{"user":{"id":"67e55044-10b1-426f-9247-bb680e5fe0c8","value":{"title":"x","done":false}}}"#,
            ],
        ),
        (
            "OtherBundle",
            false,
            vec![r#"{"user":{"id":"invalid","value":{}}}"#],
        ),
        (
            "Forward",
            true,
            vec![r#"{"later":{"previous":{"later":{}}}}"#],
        ),
        ("Forward", false, vec![r#"{"later":{"previous":{}}}"#]),
        (
            "Maps",
            true,
            vec![
                r#"{"signed":{"+001":{"title":"x","done":true}},"unsigned":{"18446744073709551615":"x"},"wrapped":{"42":true},"identifiers":{}}"#,
            ],
        ),
        (
            "Maps",
            false,
            vec![
                r#"{"signed":{"128":{}},"unsigned":{"18446744073709551616":false},"wrapped":{"-0":1},"identifiers":{"bad":"bad"}}"#,
            ],
        ),
    ] {
        docs.extend(texts.into_iter().map(|text| {
            Document {
                typ: typ.into(),
                text: text.into(),
                accepted,
            }
        }));
    }
    scalar_documents(&mut docs);
    scalar_cases::<VariantInternallyTagged<Point>>(
        &mut docs,
        "VariantInternallyTaggedPoint",
        &[
            r#"{"tagFieldName":"G","x":1,"y":2}"#,
            r#"{"tagFieldName":"G","defaultContentField":{"x":1,"y":2}}"#,
            r#"{"tagFieldName":"H","x":{"x":1,"y":2}}"#,
        ],
    );
    for prefix in ["", "A", "AA", "AB", "AAA", "AAB", "AAAA", "AAAAA", "AAAAAA"] {
        for padding in 0..=4 {
            let text = serde_json::to_string(&format!("{prefix}{}", "=".repeat(padding))).unwrap();
            let accepted = json::from_str::<Bytes>(&text).is_ok();
            docs.push(Document {
                typ: "bytes".into(),
                text,
                accepted,
            });
        }
    }
    for year in [
        "0000", "9999", "-009999", "-000044", "+009999", "-000000", "+010000",
    ] {
        for day in ["-01-01", "-12-31", "0101"] {
            for time in [
                "T00Z",
                "T23:59:59.999999999Z",
                "T00:00+25:59:59",
                "T23:59:59-25:59:59",
            ] {
                let text = serde_json::to_string(&format!("{year}{day}{time}")).unwrap();
                let accepted = json::from_str::<Timestamp>(&text).is_ok();
                docs.push(Document {
                    typ: "timestamp".into(),
                    text,
                    accepted,
                });
            }
        }
    }
    for offset in [
        "+25:59:59.4",
        "+25:59:59.5",
        "+26",
        "-25:59",
        "+24",
        "+23:59:59.9",
        "+02:30:15.5",
    ] {
        let text = serde_json::to_string(&format!("2026-10-12T12:30{offset}")).unwrap();
        docs.push(Document {
            typ: "timestamp".into(),
            accepted: json::from_str::<Timestamp>(&text).is_ok(),
            text,
        });
    }
    rust_fixtures::<VariantInternallyTagged<i32>>(&mut docs, root, "VariantInternallyTagged");
    rust_fixtures::<VariantExternallyTagged<i32>>(&mut docs, root, "VariantExternallyTagged");
    rust_fixtures::<VariantAdjacentlyTagged<i32>>(&mut docs, root, "VariantAdjacentlyTagged");
    rust_fixtures::<VariantImplicitlyTagged<bool>>(&mut docs, root, "VariantImplicitlyTagged");
    rust_fixtures::<MaybeRef<u64>>(&mut docs, root, "MaybeRefInteger");
    rust_fixtures::<MaybeRef<Point>>(&mut docs, root, "MaybeRefPoint");
    rust_fixtures::<Any>(&mut docs, root, "Any");
    rust_fixtures::<JsRoundtrip>(&mut docs, root, "JsRoundtrip");
    rust_fixtures::<RecordFieldNamesDefault>(&mut docs, root, "RecordFieldNamesDefault");
    rust_fixtures::<RecordFieldNamesScreamingSnake>(
        &mut docs,
        root,
        "RecordFieldNamesScreamingSnake",
    );
    docs
}

fn rust_fixtures<T: Decode>(docs: &mut Vec<Document>, root: &Path, typ: &str) {
    for entry in std::fs::read_dir(root.join("tests/json/expected").join(typ)).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let accepted = json::from_str::<T>(&text).is_ok();
        assert!(accepted, "Rust fixture {typ}: {text}");
        docs.push(Document {
            typ: typ.into(),
            text,
            accepted,
        });
    }
}

fn scalar_cases<T: Decode>(docs: &mut Vec<Document>, typ: &str, cases: &[&str]) {
    for text in cases {
        docs.push(Document {
            typ: typ.into(),
            text: (*text).into(),
            accepted: json::from_str::<T>(text).is_ok(),
        });
    }
}

fn scalar_documents(docs: &mut Vec<Document>) {
    scalar_cases::<i8>(docs, "i8", &["-128", "127", "-129", "128", "\"1\"", "true"]);
    scalar_cases::<i16>(docs, "i16", &["-32768", "32767", "32768"]);
    scalar_cases::<i32>(docs, "i32", &["-2147483648", "2147483647", "2147483648"]);
    scalar_cases::<u8>(docs, "u8", &["0", "255", "-1", "256"]);
    scalar_cases::<u16>(docs, "u16", &["65535", "65536"]);
    scalar_cases::<u32>(docs, "u32", &["4294967295", "4294967296"]);
    scalar_cases::<i64>(
        docs,
        "i64",
        &[
            "9007199254740992",
            "\"+42\"",
            "\"-9223372036854775808\"",
            "\"9223372036854775807\"",
            "\"9223372036854775808\"",
            "\"-0\"",
            "\"1\\n\"",
        ],
    );
    scalar_cases::<u64>(
        docs,
        "u64",
        &[
            "9007199254740992",
            "\"+42\"",
            "\"18446744073709551615\"",
            "\"18446744073709551616\"",
            "\"-0\"",
            "\" 1\"",
        ],
    );
    scalar_cases::<usize>(docs, "idx", &["0", "-1", "9007199254740992"]);
    scalar_cases::<f32>(
        docs,
        "f32",
        &[
            "1e100",
            "\"NaN\"",
            "\"Infinity\"",
            "\"+Infinity\"",
            "\"-Infinity\"",
            "\"1\"",
        ],
    );
    scalar_cases::<f64>(docs, "f64", &["1", "\"NaN\"", "\"infinity\"", "null"]);
    scalar_cases::<Bytes>(
        docs,
        "bytes",
        &[
            "\"\"",
            "\"AA==\"",
            "\"AA\"",
            "\"AP8=\"",
            "\"AB==\"",
            "\"AAF=\"",
            "\"A\"",
            "\"AA=\"",
            "\"AA==\\n\"",
        ],
    );
    scalar_cases::<Uuid>(
        docs,
        "uuid",
        &[
            "\"67E55044-10B1-426F-9247-BB680E5FE0C8\"",
            "\"67e5504410b1426f9247bb680e5fe0c8\"",
            "\"67e55044-10b1-426f-9247-bb680e5fe0c8\\n\"",
        ],
    );
    scalar_cases::<Date>(
        docs,
        "date",
        &[
            "\"2024-02-29\"",
            "\"2026-02-29\"",
            "\"-000044-03-15\"",
            "\"+0099991231\"",
            "\"20261012\"",
            "\"2026-10-12[Europe/Paris]\"",
            "\"2026-10-12[u-ca=iso8601]\"",
            "\"2026-10-12[!u-ca=iso8601]\"",
            "\"2026-10-12T00:00Z\"",
        ],
    );
    scalar_cases::<Timestamp>(
        docs,
        "timestamp",
        &[
            "\"2026-10-12T10:30:00.123456789+02:00\"",
            "\"20261012t1030z\"",
            "\"2026-10-12 10Z\"",
            "\"2026-10-12T10:30:60Z\"",
            "\"2026-10-12T10:30:00+02:30:15.5[Europe/Paris]\"",
            "\"2026-02-30T00:00Z\"",
            "\"2026-10-12T24:00Z\"",
            "\"2026-10-12T10:30\"",
            "\"2026-10-12T10:30Z\\n\"",
        ],
    );
    scalar_cases::<Map<i8, Str>>(
        docs,
        "i8map",
        &[
            r#"{"-128":"a","+127":"b","0001":"c"}"#,
            r#"{"128":"a","-129":"b"}"#,
            r#"{"1.0":"a"}"#,
        ],
    );
}
