//! Type elaboration transforming ASTs to HIR.

use std::{ffi::OsStr, path::PathBuf, sync::Arc};

use hashbrown::{HashMap, HashSet};

use crate::source::SourceStorage;
use crate::{
    hir::{self, DefKind},
    manifest::{load_manifest, Manifest},
    syntax::{
        ast::{self, Docs, TypeExpr},
        spans::Span,
    },
};

struct Model {
    _manifest: Manifest,
    modules: HashMap<String, ast::Module>,
}

fn resolve_type_expr<D>(
    unit: &hir::Unit<D>,
    builtins: &HashMap<Arc<str>, hir::DefId>,
    model_id: hir::ModelId,
    module_id: hir::ModuleId,
    expr: &ast::TypeExpr,
) -> hir::Ty {
    match expr {
        TypeExpr::Sequence { element } => hir::Ty::Sequence {
            element: Box::new(resolve_type_expr(
                unit,
                builtins,
                model_id,
                module_id,
                element.value(),
            )),
        },
        TypeExpr::Map { key, value } => hir::Ty::Map {
            key: Box::new(resolve_type_expr(
                unit,
                builtins,
                model_id,
                module_id,
                key.value(),
            )),
            value: Box::new(resolve_type_expr(
                unit,
                builtins,
                model_id,
                module_id,
                value.value(),
            )),
        },
        TypeExpr::Named { path } => {
            let mut remaining = path.value.segments.as_slice();
            let path_model = if path.value().is_root {
                let name = remaining[0].as_str();
                let id = unit.model_names.get(name).unwrap();
                remaining = &remaining[1..];
                *id
            } else {
                model_id
            };
            let path_module = if remaining.len() > 1 {
                let name = remaining[0].as_str();
                let id = unit.get_model(path_model).modules.get(name).unwrap();
                remaining = &remaining[1..];
                *id
            } else {
                module_id
            };
            let module = unit.get_module(path_module);
            let name = remaining[0].as_str();
            let id = *builtins.get(name).unwrap_or_else(|| {
                module
                    .defs
                    .get(name)
                    .expect(&format!("Unable to resolve {}.", name))
            });
            hir::Ty::Defined { id }
        }
    }
}

pub fn load(manifest_path: PathBuf) -> eyre::Result<(hir::Unit<hir::Def>, hir::ModelId)> {
    let mut models = HashMap::new();

    let root_manifest = load_manifest(&manifest_path)?;

    let mut manifest_paths = vec![manifest_path.clone()];

    let mut seen_paths = HashSet::new();
    seen_paths.insert(manifest_path);

    while let Some(manifest_path) = manifest_paths.pop() {
        let manifest = load_manifest(&manifest_path)?;
        let name = manifest.metadata.name.clone();
        let mut modules = HashMap::new();
        let modules_path = manifest_path.parent().unwrap().join("modules");
        for entry in std::fs::read_dir(&modules_path)? {
            let path = entry?.path();
            if path.extension() == Some(OsStr::new("sidex")) {
                let mut storage = SourceStorage::new();
                let src = std::fs::read_to_string(&path)?;
                let id = storage.insert(
                    src,
                    Some(path.strip_prefix(&modules_path).unwrap().to_owned()),
                );
                let module = crate::syntax::parser::parse(&storage, &storage[id]).unwrap();
                modules.insert(
                    path.file_stem().unwrap().to_string_lossy().into_owned(),
                    module,
                );
            }
        }
        for dependency in manifest.requires.values() {
            let dependency_manifest = manifest_path
                .parent()
                .unwrap()
                .join(&dependency.path)
                .join("sidex.toml");
            if seen_paths.insert(dependency_manifest.clone()) {
                manifest_paths.push(dependency_manifest);
            }
        }

        models.insert(
            name,
            Model {
                _manifest: manifest,
                modules,
            },
        );
    }

    // Lower the ASTs to Sidex IR.

    #[derive(Debug, Clone)]
    struct PartialDef {
        model: hir::ModelId,
        module: hir::ModuleId,
        item: ast::Node<ast::Item>,
    }

    let mut unit = hir::Unit::<PartialDef>::default();

    let mut builtins = HashMap::new();
    let builtin_model = unit.builtin_model();
    let builtin_module = unit.builtin_module();
    for builtin in [
        "string", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool",
        "unit",
    ] {
        let name: Arc<str> = builtin.into();
        let id = unit.create_def_with(builtin_module, name.clone(), |_| PartialDef {
            model: builtin_model,
            module: builtin_module,
            item: ast::Node::new(
                ast::Item {
                    docs: Default::default(),
                    name: ast::Identifier(ast::Node::new(
                        Arc::new(builtin.to_owned()),
                        Span::dummy(),
                    )),
                    kind: ast::ItemKind::OpaqueDef,
                    annotations: Default::default(),
                },
                Span::dummy(),
            ),
        });
        builtins.insert(name, id);
    }

    for (model_name, model) in models.into_iter() {
        let model_id = unit.create_model(model_name.as_str().into());
        for (_module_name, module) in model.modules {
            let module_id = unit.create_module(model_id, model_name.as_str().into());
            for item in module.items {
                unit.create_def_with(
                    module_id,
                    item.value().name.0.value().as_str().into(),
                    |_| PartialDef {
                        model: model_id,
                        module: module_id,
                        item,
                    },
                );
            }
        }
    }

    let unit = unit.map_defs(|unit, def| {
        let resolve_type_expr =
            |expr| resolve_type_expr(unit, &builtins, def.model, def.module, expr);

        let strip_docs = |docs: &Docs| Arc::new(docs.to_string());

        let item = def.item.value.clone();
        let kind = match &def.item.value().kind {
            ast::ItemKind::OpaqueDef => DefKind::OpaqueDef(hir::OpaqueDef {}),
            ast::ItemKind::AliasDef(ast::AliasDef { aliased }) => {
                DefKind::AliasDef(hir::AliasDef {
                    aliased: resolve_type_expr(aliased.value()),
                })
            }
            ast::ItemKind::StructDef(ast::StructDef { fields }) => {
                DefKind::StructDef(hir::StructDef {
                    fields: fields
                        .iter()
                        .map(|field| hir::FieldDef {
                            name: field.value.name.clone(),
                            ty: resolve_type_expr(&field.value.typ.value),
                            is_optional: field.value.is_optional,
                            docs: strip_docs(&field.value.docs),
                        })
                        .collect(),
                })
            }
            ast::ItemKind::EnumDef(ast::EnumDef { variants }) => DefKind::EnumDef(hir::EnumDef {
                variants: variants
                    .iter()
                    .map(|variant| hir::VariantDef {
                        name: variant.value.name.clone(),
                        ty: match &variant.value().typ {
                            Some(typ) => Some(resolve_type_expr(&typ.value)),
                            None => None,
                        },
                        docs: strip_docs(&variant.value.docs),
                    })
                    .collect(),
            }),
            ast::ItemKind::FunDef(ast::FunDef { arguments, returns }) => {
                DefKind::FunDef(hir::FunDef {
                    arguments: arguments
                        .iter()
                        .map(|argument| hir::ArgumentDef {
                            name: argument.value.name.clone(),
                            ty: resolve_type_expr(&argument.value.typ.value),
                            is_optional: argument.value.is_optional,
                        })
                        .collect(),
                    returns: returns.as_ref().map(|expr| resolve_type_expr(expr.value())),
                })
            }
        };
        hir::Def {
            name: item.name.clone(),
            kind,
            docs: strip_docs(&item.docs),
            model_id: def.model,
            module_id: def.module,
        }
    });

    let root_id = *unit
        .model_names
        .get(root_manifest.metadata.name.as_str())
        .unwrap();

    Ok((unit, root_id))
}
