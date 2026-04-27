//! Lints over parsed schemas.
//!
//! Lints surface as [`sidex_diagnostics::Diagnostic`]s emitted into the
//! active diagnostic context. They are non-fatal — typically `Warning`
//! severity — and do not stop further compilation.

use std::collections::HashSet;

use sidex_diagnostics::{Diagnostic, Label};
use sidex_ir as ir;
use sidex_syntax::ast;

use crate::transformer::Transformer;

/// Emit `unused-import` warnings for every schema in `bundle`.
///
/// An import is considered unused when its imported name (the last segment
/// of a path import, or each name inside a brace group) does not appear as
/// the head of any path used in the schema's definitions. Wildcard imports
/// (`path::*`) are skipped — we cannot tell which names they bring in.
///
/// Type variables of the enclosing definition are not counted as references
/// to imports.
pub fn lint_unused_imports(transformer: &Transformer, bundle: ir::BundleIdx) {
    for schema in transformer.iter_user_schemas(bundle) {
        lint_schema(schema);
    }
}

fn lint_schema(schema: &crate::transformer::ParsedSchema) {
    let mut imported: Vec<&ast::Identifier> = Vec::new();
    for import in schema.imports() {
        collect_imported_names(&import.tree, &mut imported);
    }
    if imported.is_empty() {
        return;
    }

    let mut referenced: HashSet<String> = HashSet::new();
    for def in schema.defs() {
        let var_names: HashSet<&str> =
            def.vars.iter().map(|v| v.name.as_str()).collect();
        let mut paths: Vec<&ast::Path> = Vec::new();
        collect_paths_in_def(def, &mut paths);
        for p in paths {
            if p.is_absolute || p.segments.is_empty() {
                continue;
            }
            let head = p.segments[0].as_str();
            // A bare single-segment path may be a type variable of the
            // enclosing def; skip those.
            if p.segments.len() == 1 && var_names.contains(head) {
                continue;
            }
            referenced.insert(head.to_owned());
        }
    }

    for ident in imported {
        if !referenced.contains(ident.as_str()) {
            let span = ident.span().clone();
            Diagnostic::warning(format!("Unused import `{}`.", ident.as_str()))
                .with_span(Some(span.clone()))
                .with_label(Label::new(span, "unused import"))
                .with_help("Remove the import or `sidex check --fix` to clean up.")
                .emit();
        }
    }
}

fn collect_imported_names<'a>(
    tree: &'a ast::ImportTree,
    out: &mut Vec<&'a ast::Identifier>,
) {
    match tree {
        ast::ImportTree::Path(p) => {
            if let Some(last) = p.segments.last() {
                out.push(last);
            }
        }
        ast::ImportTree::Wildcard => {}
        ast::ImportTree::Group { trees, .. } => {
            for t in trees {
                collect_imported_names(t, out);
            }
        }
    }
}

fn collect_paths_in_def<'a>(def: &'a ast::Def, out: &mut Vec<&'a ast::Path>) {
    match &def.kind {
        ast::DefKind::Alias(a) => collect_paths_in_type(&a.aliased, out),
        ast::DefKind::OpaqueType(_) => {}
        ast::DefKind::RecordType(r) => {
            for field in &r.fields {
                collect_paths_in_type(&field.typ, out);
            }
        }
        ast::DefKind::VariantType(v) => {
            for variant in &v.variants {
                if let Some(t) = &variant.typ {
                    collect_paths_in_type(t, out);
                }
            }
        }
        ast::DefKind::WrapperType(w) => collect_paths_in_type(&w.wrapped, out),
    }
}

fn collect_paths_in_type<'a>(typ: &'a ast::TypeExpr, out: &mut Vec<&'a ast::Path>) {
    match typ {
        ast::TypeExpr::Instance(i) => {
            out.push(&i.path);
            for sub in &i.subst {
                collect_paths_in_type(sub, out);
            }
        }
        ast::TypeExpr::Sequence(s) => collect_paths_in_type(&s.element, out),
        ast::TypeExpr::Map(m) => {
            collect_paths_in_type(&m.key, out);
            collect_paths_in_type(&m.value, out);
        }
        ast::TypeExpr::Unit => {}
    }
}
