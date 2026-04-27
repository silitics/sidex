use sidex_fmt::format;

#[test]
fn formats_simple_record() {
    let input = r#"
record Person{
id?:idx,
first_name:string,
}
"#;
    let out = format(input).unwrap();
    insta::assert_snapshot!("simple_record", out);
}

#[test]
fn formats_attrs_short() {
    let input = r#"
#[json(rename_all = "camelCase")]
record Foo {
    #[json(rename="x")]
    a: string,
}
"#;
    let out = format(input).unwrap();
    insta::assert_snapshot!("attrs_short", out);
}

#[test]
fn formats_attrs_long_breaks() {
    // Long attr arg list — should break.
    let input = r#"
#[rust(derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ExtraTrait))]
record VeryLong {
    a: int,
}
"#;
    let out = format(input).unwrap();
    insta::assert_snapshot!("attrs_long_breaks", out);
}

#[test]
fn sorts_imports_and_groups_by_bundle() {
    let input = r#"
import b::B
import std::option::Option
import a::A
import std::result::{Result, Error}
"#;
    let out = format(input).unwrap();
    insta::assert_snapshot!("sorted_imports", out);
}

#[test]
fn sub_groups_external_by_bundle() {
    // External imports from two different bundles (`std`, `other`) plus
    // internal imports. External should be split into per-bundle subgroups.
    let input = r#"
import std::option::Option
import ::other::Foo
import schema::Bar
import std::result::Result
import ::other::{Baz, Quux}
"#;
    let opts = sidex_fmt::FormatOptions {
        external_bundles: vec!["std".to_owned(), "other".to_owned()],
        ..Default::default()
    };
    let out = sidex_fmt::format_with(input, &opts).unwrap();
    insta::assert_snapshot!("subgrouped_imports", out);
}

#[test]
fn flattens_nested_import_groups() {
    let input = r#"
import ::ec_pdm::{programs::*, strings::*}
import ::ec_pdm::strings::{NameStr, PathStr, MarkdownStr}
"#;
    let opts = sidex_fmt::FormatOptions {
        external_bundles: vec!["std".to_owned(), "ec_pdm".to_owned()],
        ..Default::default()
    };
    let out = sidex_fmt::format_with(input, &opts).unwrap();
    insta::assert_snapshot!("flattened_imports", out);
}

#[test]
fn preserves_comments() {
    let input = r#"
// schema-level comment
//! schema doc

// before record
/// doc
record Foo {
    /// field doc
    // before field
    a: int,
}

// trailing
"#;
    let out = format(input).unwrap();
    insta::assert_snapshot!("preserves_comments", out);
}

#[test]
fn formats_variants() {
    let input = r#"
variant Either<L, R> {
None,
Left:L,
Right:R,
}
"#;
    let out = format(input).unwrap();
    insta::assert_snapshot!("variants", out);
}

#[test]
fn formats_alias_wrapper_opaque() {
    let input = r#"
alias Foo : idx
wrapper Bar:string
opaque Baz
"#;
    let out = format(input).unwrap();
    insta::assert_snapshot!("alias_wrapper_opaque", out);
}

#[test]
fn idempotent_on_real_ir_schema() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../lib/meta/schemas/ir.sidex");
    let src = std::fs::read_to_string(&path).unwrap();
    let once = format(&src).unwrap();
    let twice = format(&once).unwrap();
    assert_eq!(once, twice, "format(format(x)) should equal format(x)");
}

/// Walks every `.sidex` schema in `lib/` and asserts the formatter:
///   1) succeeds without diagnostic errors,
///   2) re-parses,
///   3) is idempotent (`format(format(x)) == format(x)`).
#[test]
fn idempotent_on_lib_schemas() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../lib");
    assert_idempotent_under(&root);
}

/// Idempotency over the sibling `sidex-private-tests` repo, when present.
/// Skipped silently when the directory is not on disk.
#[test]
fn idempotent_on_sidex_private_tests() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../sidex-private-tests/vendored");
    if !root.exists() {
        return;
    }
    assert_idempotent_under(&root);
}

fn assert_idempotent_under(root: &std::path::Path) {
    let mut count = 0;
    for entry in walkdir(root) {
        if entry.extension().and_then(|s| s.to_str()) != Some("sidex") {
            continue;
        }
        let src = std::fs::read_to_string(&entry).unwrap();
        let once = format(&src).unwrap_or_else(|e| {
            panic!("first format failed for {}: {:?}", entry.display(), e)
        });
        let twice = format(&once).unwrap_or_else(|e| {
            panic!("second format failed for {}: {:?}", entry.display(), e)
        });
        assert_eq!(
            once,
            twice,
            "format is not idempotent for {}",
            entry.display()
        );
        count += 1;
    }
    assert!(count > 0, "no .sidex files found under {}", root.display());
}

fn walkdir(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(p) = stack.pop() {
        if p.is_dir() {
            for entry in std::fs::read_dir(&p).unwrap().flatten() {
                stack.push(entry.path());
            }
        } else if p.is_file() {
            out.push(p);
        }
    }
    out
}
