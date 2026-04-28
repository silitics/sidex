//! Lints over parsed schemas.
//!
//! Lints surface as [`sidex_diagnostics::Diagnostic`]s emitted into the
//! active diagnostic context. They are non-fatal — typically `Warning`
//! severity — and do not stop further compilation.

use std::collections::HashMap;
use std::collections::HashSet;

use sidex_diagnostics::Diagnostic;
use sidex_diagnostics::Label;
use sidex_ir as ir;
use sidex_syntax::ast;

use crate::transformer::ParsedSchema;
use crate::transformer::Transformer;

/// A single unused-import finding.
#[derive(Debug, Clone)]
pub struct UnusedImport {
    /// The name of the schema that contains the unused import. Schema names
    /// are unique within a bundle, so this is sufficient to identify the
    /// schema for fix-up purposes.
    pub schema: String,
    /// The imported name (last segment of the path).
    pub name: String,
    /// Span of the imported name in the source.
    pub span: ir::Span,
    /// The full rendered import path — matches what the formatter would
    /// emit. Used by `sidex check --fix` to filter the import out.
    pub rendered_path: String,
}

/// Collect all unused-import findings for `bundle` without emitting them.
pub fn collect_unused_imports(
    transformer: &Transformer,
    bundle: ir::BundleIdx,
) -> Vec<UnusedImport> {
    let mut out = Vec::new();
    for schema in transformer.iter_user_schemas(bundle) {
        collect_for_schema(schema, &mut out);
    }
    out
}

/// Emit `unused-import` warnings for every schema in `bundle`.
pub fn lint_unused_imports(transformer: &Transformer, bundle: ir::BundleIdx) {
    for finding in collect_unused_imports(transformer, bundle) {
        Diagnostic::warning(format!("Unused import `{}`.", finding.name))
            .with_span(Some(finding.span.clone()))
            .with_label(Label::new(finding.span, "unused import"))
            .with_help("Remove the import or run `sidex check --fix`.")
            .emit();
    }
}

/// Group findings by schema. The returned map keys are schema indices and
/// the values are the rendered import paths of every unused import in that
/// schema — exactly what [`sidex_fmt::FormatOptions::excluded_imports`]
/// consumes.
pub fn unused_imports_by_schema(findings: &[UnusedImport]) -> HashMap<String, HashSet<String>> {
    let mut out: HashMap<String, HashSet<String>> = HashMap::new();
    for f in findings {
        out.entry(f.schema.clone())
            .or_default()
            .insert(f.rendered_path.clone());
    }
    out
}

fn collect_for_schema(schema: &ParsedSchema, out: &mut Vec<UnusedImport>) {
    let mut imported: Vec<(&ast::Identifier, String)> = Vec::new();
    for import in schema.imports() {
        walk_import_leaves(&import.tree, "", &mut |last, full_path| {
            imported.push((last, full_path));
        });
    }
    if imported.is_empty() {
        return;
    }

    let mut referenced: HashSet<String> = HashSet::new();
    for def in schema.defs() {
        let var_names: HashSet<&str> = def.vars.iter().map(|v| v.name.as_str()).collect();
        let mut paths: Vec<&ast::Path> = Vec::new();
        collect_paths_in_def(def, &mut paths);
        for p in paths {
            if p.is_absolute || p.segments.is_empty() {
                continue;
            }
            let head = p.segments[0].as_str();
            if p.segments.len() == 1 && var_names.contains(head) {
                continue;
            }
            referenced.insert(head.to_owned());
        }
    }

    for (ident, rendered_path) in imported {
        if !referenced.contains(ident.as_str()) {
            out.push(UnusedImport {
                schema: schema.name().to_owned(),
                name: ident.as_str().to_owned(),
                span: ident.span().clone(),
                rendered_path,
            });
        }
    }
}

/// Walk an import tree, invoking `visit` once per leaf path with the leaf's
/// last identifier and the full rendered path string.
fn walk_import_leaves<'a, F: FnMut(&'a ast::Identifier, String)>(
    tree: &'a ast::ImportTree,
    prefix: &str,
    visit: &mut F,
) {
    match tree {
        ast::ImportTree::Path(p) => {
            let local = render_path_text(p);
            let full = compose(prefix, &local);
            if let Some(last) = p.segments.last() {
                visit(last, full);
            }
        }
        ast::ImportTree::Wildcard => {}
        ast::ImportTree::Group { path, trees } => {
            let local = render_path_text(path);
            let new_prefix = compose(prefix, &local);
            for t in trees {
                walk_import_leaves(t, &new_prefix, visit);
            }
        }
    }
}

fn render_path_text(p: &ast::Path) -> String {
    let mut s = String::new();
    if p.is_absolute {
        s.push_str("::");
    }
    for (i, seg) in p.segments.iter().enumerate() {
        if i > 0 {
            s.push_str("::");
        }
        s.push_str(seg.as_str());
    }
    s
}

fn compose(prefix: &str, local: &str) -> String {
    if prefix.is_empty() {
        local.to_owned()
    } else if local.is_empty() {
        prefix.to_owned()
    } else {
        format!("{}::{}", prefix, local)
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
