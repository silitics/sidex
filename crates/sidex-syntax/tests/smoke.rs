use sidex_diagnostics::DiagnosticCtx;
use sidex_ir as ir;
use sidex_syntax::ast;
use sidex_syntax::parse;

fn parse_str(src: &str) -> ast::Schema {
    let mut storage = ir::SourceStorage::new();
    let id = storage.insert(src.to_owned(), None);
    let ctx = DiagnosticCtx::new();
    let result = ctx.exec(|| parse(&storage[id]));
    let report = ctx.report();
    assert!(!report.has_error(), "diagnostics emitted while parsing");
    result.expect("parse failed")
}

#[test]
fn parses_simple_record() {
    let schema = parse_str(
        r#"
        /// A person.
        record Person {
            id?: idx,
            first_name: string,
        }
        "#,
    );
    assert_eq!(schema.items.len(), 1);
    let ast::Item::Def(def) = &schema.items[0] else {
        panic!("expected def");
    };
    assert_eq!(def.name.as_str(), "Person");
    assert_eq!(def.docs.iter().count(), 1);
    let ast::DefKind::RecordType(r) = &def.kind else {
        panic!("expected record");
    };
    assert_eq!(r.fields.len(), 2);
    assert!(r.fields[0].is_optional);
    assert!(!r.fields[1].is_optional);
}

#[test]
fn parses_attributes_on_field_and_record() {
    let schema = parse_str(
        r#"
        #[json(rename_all = "camelCase")]
        record Foo {
            #[json(rename = "x")]
            field_a: string,
        }
        "#,
    );
    let ast::Item::Def(def) = &schema.items[0] else {
        panic!("expected def");
    };
    assert_eq!(def.attrs.len(), 1, "record attrs");
    let ast::DefKind::RecordType(r) = &def.kind else {
        panic!()
    };
    assert_eq!(r.fields[0].attrs.len(), 1, "field attrs");
}

#[test]
fn parses_variants() {
    let schema = parse_str(
        r#"
        variant Either<L, R> {
            Left: L,
            Right: R,
            None,
        }
        "#,
    );
    let ast::Item::Def(def) = &schema.items[0] else {
        panic!()
    };
    assert_eq!(def.vars.len(), 2);
    let ast::DefKind::VariantType(v) = &def.kind else {
        panic!()
    };
    assert_eq!(v.variants.len(), 3);
    assert!(v.variants[0].typ.is_some());
    assert!(v.variants[2].typ.is_none());
}

#[test]
fn parses_imports() {
    let schema = parse_str(
        r#"
        import std::option::Option
        import std::result::{Result, Error}
        import std::value::*
        "#,
    );
    let imports: Vec<_> = schema
        .items
        .iter()
        .filter_map(|i| {
            match i {
                ast::Item::Import(im) => Some(im),
                _ => None,
            }
        })
        .collect();
    assert_eq!(imports.len(), 3);
}

#[test]
fn parses_alias_and_wrapper() {
    let schema = parse_str(
        r#"
        alias Bar: idx
        wrapper Baz: string
        opaque Qux
        "#,
    );
    assert_eq!(schema.items.len(), 3);
}

#[test]
fn parses_nested_generics() {
    let schema = parse_str(
        r#"
        record R {
            xs: Map<string, Box<Vec<int>>>,
        }
        "#,
    );
    let ast::Item::Def(def) = &schema.items[0] else {
        panic!()
    };
    let ast::DefKind::RecordType(r) = &def.kind else {
        panic!()
    };
    assert_eq!(r.fields.len(), 1);
}

#[test]
fn parses_real_ir_schema() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../lib/meta/schemas/ir.sidex"),
    )
    .unwrap();
    let schema = parse_str(&src);
    let names: Vec<_> = schema
        .items
        .iter()
        .filter_map(|i| {
            match i {
                ast::Item::Def(d) => Some(d.name.as_str().to_owned()),
                _ => None,
            }
        })
        .collect();
    assert!(names.contains(&"Attr".to_string()), "got: {:?}", names);
    assert!(names.contains(&"Schema".to_string()));
    assert!(names.contains(&"Token".to_string()));
}
