//! Implementation of the transformation process from text sources to Sidex IR.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use sidex_syntax::{ast, parse, source::SourceStorage};
use thiserror::Error;

use crate::{
    builtins,
    bundle::{self, iter_schemas, BundleSource, Manifest},
    ir,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Manifest(#[from] bundle::ManifestLoadError),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone)]
struct LoadedBundle {
    idx: ir::BundleIdx,

    source: BundleSource,
    schemas: Vec<ParsedSchema>,

    schema_by_name: HashMap<String, ir::SchemaIdx>,
}

#[derive(Debug, Clone)]
pub struct ParsedSchema {
    idx: ir::SchemaIdx,

    name: String,
    docs: String,
    defs: Vec<ast::Def>,
    imports: Vec<ast::Import>,

    def_by_name: HashMap<String, ir::DefIdx>,
}

#[derive(Debug, Clone)]
pub struct Transformer {
    pub(crate) storage: SourceStorage,

    loaded: Vec<LoadedBundle>,

    bundle_by_name: HashMap<String, ir::BundleIdx>,
    bundle_by_path: HashMap<PathBuf, ir::BundleIdx>,
}

/// Splits a items of a schema into imports and definitions.
fn split_items(items: Vec<ast::Item>) -> (Vec<ast::Import>, Vec<ast::Def>) {
    let mut imports = Vec::new();
    let mut defs = Vec::new();
    for item in items {
        match item {
            ast::Item::Import(import) => imports.push(import),
            ast::Item::Def(def) => defs.push(def),
        }
    }
    (imports, defs)
}

const STD_BUNDLE: ir::BundleIdx = ir::BundleIdx(0);

/// Builds the definition table of a schema.
fn build_def_table(defs: &[ast::Def]) -> HashMap<String, ir::DefIdx> {
    defs.iter()
        .enumerate()
        .map(|(idx, def)| (def.name.as_str().to_owned(), ir::DefIdx(idx)))
        .collect()
}

#[derive(Debug, Clone)]
enum LookupEntry {
    Root,
    Bundle(ir::BundleIdx),
    Schema {
        bundle: ir::BundleIdx,
        schema: ir::SchemaIdx,
    },
    Def {
        bundle: ir::BundleIdx,
        schema: ir::SchemaIdx,
        def: ir::DefIdx,
    },
}

pub struct Resolver<'t, 'd> {
    transformer: &'t Transformer,
    bundle: ir::BundleIdx,
    schema: ir::SchemaIdx,

    dependencies: &'d HashMap<String, ir::BundleIdx>,

    table: HashMap<String, LookupEntry>,
}

impl<'t, 'd> Resolver<'t, 'd> {
    fn populate_defs(&mut self) {
        let schema = &self.transformer.loaded[self.bundle.0].schemas[self.schema.0];
        for (name, def_idx) in schema.def_by_name.iter() {
            if self.table.contains_key(name) {
                panic!("Duplicate name!");
            }
            self.table.insert(
                name.clone(),
                LookupEntry::Def {
                    bundle: self.bundle,
                    schema: self.schema,
                    def: *def_idx,
                },
            );
        }
    }

    fn populate_imports(&mut self) {
        let schema = &self.transformer.loaded[self.bundle.0].schemas[self.schema.0];
        for import in &schema.imports {
            let mut stack = vec![(LookupEntry::Bundle(self.bundle), &import.tree)];
            while let Some((root, tree)) = stack.pop() {
                match tree {
                    ast::ImportTree::Path(path) => {
                        let entry = self.resolve_segments(
                            if path.is_absolute {
                                LookupEntry::Root
                            } else {
                                root
                            },
                            &path.segments,
                        );
                        let name = path.segments.last().unwrap();
                        self.table.entry(name.as_str().to_owned()).or_insert(entry);
                    }
                    ast::ImportTree::Wildcard => {
                        match root {
                            LookupEntry::Bundle(bundle) => {
                                for (name, idx) in
                                    self.transformer.loaded[bundle.0].schema_by_name.iter()
                                {
                                    self.table.entry(name.to_owned()).or_insert_with(|| {
                                        LookupEntry::Schema {
                                            bundle,
                                            schema: *idx,
                                        }
                                    });
                                }
                            }
                            LookupEntry::Schema { bundle, schema } => {
                                for (name, idx) in self.transformer.loaded[bundle.0].schemas
                                    [schema.0]
                                    .def_by_name
                                    .iter()
                                {
                                    self.table.entry(name.to_owned()).or_insert_with(|| {
                                        LookupEntry::Def {
                                            bundle,
                                            schema,
                                            def: *idx,
                                        }
                                    });
                                }
                            }
                            LookupEntry::Root | LookupEntry::Def { .. } => {
                                panic!("Invalid import!")
                            }
                        }
                    }
                    ast::ImportTree::Group { path, trees } => {
                        let entry = self.resolve_segments(
                            if path.is_absolute {
                                LookupEntry::Root
                            } else {
                                root
                            },
                            &path.segments,
                        );
                        for tree in trees {
                            stack.push((entry.clone(), tree));
                        }
                    }
                }
            }
        }

        let std_bundle = &self.transformer.loaded[STD_BUNDLE.0];

        let builtins_schema =
            &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().0];

        for (name, def) in builtins_schema.def_by_name.iter() {
            self.table.entry(name.to_owned()).or_insert_with(|| {
                LookupEntry::Def {
                    bundle: STD_BUNDLE,
                    schema: builtins_schema.idx,
                    def: *def,
                }
            });
        }
    }

    fn resolve_segments(&self, root: LookupEntry, segments: &[ast::Identifier]) -> LookupEntry {
        if let Some((first, rest)) = segments.split_first() {
            let child = match root {
                LookupEntry::Root => {
                    LookupEntry::Bundle(*self.dependencies.get(first.as_str()).unwrap())
                }
                LookupEntry::Bundle(idx) => {
                    let schema = self.transformer.loaded[idx.0]
                        .schema_by_name
                        .get(first.as_str())
                        .unwrap();
                    LookupEntry::Schema {
                        bundle: idx,
                        schema: *schema,
                    }
                }
                LookupEntry::Schema { bundle, schema } => {
                    let def = self.transformer.loaded[bundle.0].schemas[schema.0]
                        .def_by_name
                        .get(first.as_str())
                        .unwrap();
                    LookupEntry::Def {
                        bundle,
                        schema,
                        def: *def,
                    }
                }
                LookupEntry::Def { .. } => {
                    panic!(
                        "Invalid lookup! {root:?} {segments:?} {}",
                        self.transformer.loaded[self.bundle.0].schemas[self.schema.0].name
                    )
                }
            };
            self.resolve_segments(child, rest)
        } else {
            root
        }
    }

    fn resolve_path(&self, path: &ast::Path) -> LookupEntry {
        let (root, tail) = if path.is_absolute {
            (LookupEntry::Root, path.segments.as_slice())
        } else {
            let first = path.segments.first().unwrap();
            let entry = self.table.get(first.as_str()).unwrap();
            (entry.clone(), &path.segments[1..])
        };
        self.resolve_segments(root, tail)
    }

    fn resolve_type_expr(&self, enclosing: &ast::Def, expr: &ast::TypeExpr) -> ir::Type {
        match expr {
            ast::TypeExpr::Instance(instance) => {
                if instance.path.segments.len() == 1 {
                    let name = instance.path.segments[0].as_str();
                    // May be a type variable.
                    if let Some(var_idx) = enclosing
                        .vars
                        .iter()
                        .position(|var| var.name.as_str() == name)
                    {
                        return ir::Type {
                            kind: ir::TypeKind::TypeVar(ir::TypeVarType {
                                idx: ir::TypeVarIdx(var_idx),
                            }),
                            span: None,
                        };
                    }
                }
                let entry = self.resolve_path(&instance.path);
                match entry {
                    LookupEntry::Def {
                        bundle,
                        schema,
                        def,
                    } => {
                        let type_def =
                            &self.transformer.loaded[bundle.0].schemas[schema.0].defs[def.0];
                        assert_eq!(type_def.vars.len(), instance.subst.len());

                        return ir::Type {
                            kind: ir::TypeKind::Instance(ir::InstanceType {
                                bundle,
                                schema,
                                def,
                                subst: instance
                                    .subst
                                    .iter()
                                    .map(|subst| self.resolve_type_expr(enclosing, subst))
                                    .collect(),
                            }),
                            span: None,
                        };
                    }
                    _ => panic!("Invalid path."),
                }
            }
            ast::TypeExpr::Sequence(sequence) => {
                let std_bundle = &self.transformer.loaded[STD_BUNDLE.0];
                let builtins_schema =
                    &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().0];
                let sequence_def = builtins_schema.def_by_name.get("Sequence").unwrap();

                ir::Type {
                    kind: ir::TypeKind::Instance(ir::InstanceType {
                        bundle: std_bundle.idx,
                        schema: builtins_schema.idx,
                        def: *sequence_def,
                        subst: vec![self.resolve_type_expr(enclosing, &sequence.element)],
                    }),
                    span: None,
                }
            }
            ast::TypeExpr::Map(map) => {
                let std_bundle = &self.transformer.loaded[STD_BUNDLE.0];
                let builtins_schema =
                    &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().0];
                let map_def = builtins_schema.def_by_name.get("Map").unwrap();

                ir::Type {
                    kind: ir::TypeKind::Instance(ir::InstanceType {
                        bundle: std_bundle.idx,
                        schema: builtins_schema.idx,
                        def: *map_def,
                        subst: vec![
                            self.resolve_type_expr(enclosing, &map.key),
                            self.resolve_type_expr(enclosing, &map.value),
                        ],
                    }),
                    span: None,
                }
            }
            ast::TypeExpr::Unit => {
                let std_bundle = &self.transformer.loaded[STD_BUNDLE.0];
                let builtins_schema =
                    &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().0];
                let unit_def = builtins_schema.def_by_name.get("unit").unwrap();

                ir::Type {
                    kind: ir::TypeKind::Instance(ir::InstanceType {
                        bundle: std_bundle.idx,
                        schema: builtins_schema.idx,
                        def: *unit_def,
                        subst: vec![],
                    }),
                    span: None,
                }
            }
        }
    }
}

fn transform_token_stream(stream: &ast::TokenStream) -> ir::TokenStream {
    let mut next_token = String::new();
    let mut ir_tokens = Vec::new();
    for token in stream.iter() {
        next_token.push_str(&token.to_string());
        if token.is_separated() {
            ir_tokens.push(ir::Token {
                token: next_token.clone(),
                span: None,
            });
            next_token.clear();
        }
    }
    ir::TokenStream(ir_tokens)
}

fn transform_attr(attr: &ast::Attr) -> ir::Attr {
    let kind = match &attr.kind {
        ast::AttrKind::Path(path) => ir::AttrKind::Path(ir::Path(path.to_string())),
        ast::AttrKind::List(list) => {
            ir::AttrKind::List(ir::AttrList {
                path: ir::Path(list.path.to_string()),
                elements: list.elements.iter().map(transform_attr).collect(),
            })
        }
        ast::AttrKind::Assign(assign) => {
            ir::AttrKind::Assign(ir::AttrAssign {
                path: ir::Path(assign.path.to_string()),
                value: Box::new(transform_attr(&assign.value)),
            })
        }
        ast::AttrKind::Tokens(tokens) => ir::AttrKind::Tokens(transform_token_stream(tokens)),
    };
    ir::Attr { kind, span: None }
}

fn transform_attrs(attrs: &[ast::Attr]) -> Vec<ir::Attr> {
    attrs.iter().map(transform_attr).collect()
}

impl Transformer {
    pub fn new() -> Self {
        let mut transformer = Self {
            storage: Default::default(),
            loaded: Default::default(),
            bundle_by_name: Default::default(),
            bundle_by_path: Default::default(),
        };
        let std_bundle = builtins::std_bundle(&mut transformer.storage);
        transformer.insert_bundle(std_bundle).unwrap();
        transformer
    }

    pub fn get_bundle_manifest(&self, idx: ir::BundleIdx) -> &Manifest {
        &self.loaded[idx.0].source.manifest
    }

    fn get_bundle_by_path(&self, path: &Path) -> Option<&LoadedBundle> {
        let canonical = path.canonicalize().unwrap();
        self.bundle_by_path
            .get(&canonical)
            .map(|idx| &self.loaded[idx.0])
    }

    pub(crate) fn insert_bundle(&mut self, source: BundleSource) -> Result<ir::BundleIdx, Error> {
        let idx = ir::BundleIdx(self.loaded.len());
        self.bundle_by_name
            .insert(source.manifest.metadata.name.clone(), idx);
        if let Some(path) = &source.path {
            self.bundle_by_path.insert(path.clone(), idx);
        }

        // Parse the bundle.
        let mut schemas = Vec::new();
        for (name, schema) in source.schemas.iter() {
            let schema = parse(&self.storage, &self.storage[*schema])
                .ok_or_else(|| Error::Other(format!("Error parsing schema {name:?}.")))?;
            let docs = schema.docs.to_string();
            let (imports, defs) = split_items(schema.items);
            let def_by_name = build_def_table(&defs);
            schemas.push(ParsedSchema {
                idx: ir::SchemaIdx(schemas.len()),
                name: name.clone(),
                docs,
                defs,
                imports,
                def_by_name,
            })
        }

        self.loaded.push(LoadedBundle {
            idx,
            source,
            schema_by_name: schemas
                .iter()
                .enumerate()
                .map(|(idx, schema)| (schema.name.clone(), ir::SchemaIdx(idx)))
                .collect(),
            schemas,
        });

        Ok(idx)
    }

    pub fn load_bundle_recursive(&mut self, path: &Path) -> Result<ir::BundleIdx, Error> {
        let idx = self.load_bundle(path)?;
        let mut stack = vec![idx];
        while let Some(idx) = stack.pop() {
            let dependencies = self.loaded[idx.0]
                .source
                .manifest
                .dependencies()
                .map(|(_, dependency)| dependency.clone())
                .collect::<Vec<_>>();
            for dependency in dependencies {
                let dependency_path = path.join(&dependency.path);
                stack.push(self.load_bundle(&dependency_path)?);
            }
        }
        Ok(idx)
    }

    pub fn load_bundle(&mut self, path: &Path) -> Result<ir::BundleIdx, Error> {
        let path = path.canonicalize()?;
        let manifest = bundle::try_load_manifest(&path)?;
        let name = &manifest.metadata.name;
        if let Some(idx) = self.bundle_by_name.get(name) {
            let loaded = &self.loaded[idx.0];
            if loaded.source.path.as_ref() != Some(&path) {
                let other_path = &loaded.source.path;
                Err(Error::Other(format!(
                    "There are two different bundles with name {name:?} ({path:?}, {other_path:?})."
                )))
            } else {
                Ok(*idx)
            }
        } else {
            let mut schemas = HashMap::new();
            for schema_path in iter_schemas(&path)? {
                let schema_path = schema_path?;
                let schema_name = schema_path
                    .file_stem()
                    .expect("Schema path should have a file stem.")
                    .to_string_lossy()
                    .into_owned();
                let source_id = self
                    .storage
                    .insert(std::fs::read_to_string(&schema_path)?, Some(schema_path));
                schemas.insert(schema_name, source_id);
            }
            self.insert_bundle(BundleSource {
                manifest,
                path: Some(path),
                schemas,
            })
        }
    }

    pub fn transform(&self) -> ir::Unit {
        ir::Unit {
            sources: Vec::new(),
            bundles: self
                .loaded
                .par_iter()
                .map(| bundle| {
                    let mut dependencies = Vec::new();

                    let mut dependency_table = HashMap::new();

                    for (name, dependency) in bundle.source.manifest.dependencies() {
                        let dependency_path =
                            bundle.source.path.as_ref().unwrap().join(&dependency.path);
                        let dependency_bundle = self.get_bundle_by_path(&dependency_path).unwrap();

                        dependencies.push(ir::Dependency {
                            name: name.to_owned(),
                            bundle: dependency_bundle.idx,
                        });
                        dependency_table.insert(name.to_owned(), dependency_bundle.idx);
                    }

                    // Add implicit dependency on `std`.
                    dependencies.push(ir::Dependency {
                        name: "std".to_owned(),
                        bundle: STD_BUNDLE,
                    });
                    dependency_table.insert("std".to_owned(), STD_BUNDLE);

                    ir::Bundle {
                        idx: bundle.idx,
                        metadata: bundle.source.manifest.metadata.clone(),
                        dependencies,
                        schemas: bundle
                            .schemas
                            .par_iter()
                            .map(| schema| {
                                // TODO: build lookup table for schema from imports
                                let mut resolver = Resolver {
                                    transformer: &self,
                                    bundle: bundle.idx,
                                    schema: schema.idx,
                                    dependencies: &dependency_table,
                                    table: Default::default(),
                                };
                                // Local definitions take precedence.
                                resolver.populate_defs();
                                resolver.populate_imports();

                                ir::Schema {
                                    idx: schema.idx,
                                    name: schema.name.clone(),
                                    docs: schema.docs.clone(),
                                    attrs: Default::default(),
                                    source: None,
                                    defs: schema
                                        .defs
                                        .iter()
                                        .map(|def| {
                                            let kind = match &def.kind {
                                                ast::DefKind::Alias(alias) => {
                                                    ir::DefKind::TypeAlias(ir::TypeAliasDef {
                                                        aliased: resolver.resolve_type_expr(
                                                            &def,
                                                            &alias.aliased,
                                                        ),
                                                    })
                                                }
                                                ast::DefKind::OpaqueType(_) => {
                                                    ir::DefKind::OpaqueType(ir::OpaqueTypeDef {})
                                                }
                                                ast::DefKind::RecordType(record) => {
                                                    ir::DefKind::RecordType(ir::RecordTypeDef {
                                                        fields: record
                                                            .fields
                                                            .iter()
                                                            .map(|field| {
                                                                ir::Field {
                                                                    name: field
                                                                        .name
                                                                        .as_str()
                                                                        .to_owned(),
                                                                    docs: field.docs.to_string(),
                                                                    attrs: transform_attrs(
                                                                        &field.attrs,
                                                                    ),
                                                                    typ: resolver
                                                                        .resolve_type_expr(
                                                                            def, &field.typ,
                                                                        ),
                                                                    is_optional: field.is_optional,
                                                                }
                                                            })
                                                            .collect(),
                                                    })
                                                }
                                                ast::DefKind::VariantType(variant) => {
                                                    ir::DefKind::VariantType(ir::VariantTypeDef {
                                                        variants: variant
                                                            .variants
                                                            .iter()
                                                            .map(|variant| {
                                                                ir::Variant {
                                                                    name: variant
                                                                        .name
                                                                        .as_str()
                                                                        .to_owned(),
                                                                    docs: variant.docs.to_string(),
                                                                    attrs: transform_attrs(
                                                                        &variant.attrs,
                                                                    ),
                                                                    typ: variant.typ.as_ref().map(
                                                                        |typ| {
                                                                            resolver
                                                                                .resolve_type_expr(
                                                                                    def, typ,
                                                                                )
                                                                        },
                                                                    ),
                                                                }
                                                            })
                                                            .collect(),
                                                    })
                                                }
                                                ast::DefKind::WrapperType(wrapped) => {
                                                    ir::DefKind::WrapperType(ir::WrapperTypeDef {
                                                        wrapped: resolver.resolve_type_expr(
                                                            &def,
                                                            &wrapped.wrapped,
                                                        ),
                                                    })
                                                }
                                                ast::DefKind::Service(service) => {
                                                    ir::DefKind::Service(ir::ServiceDef {
                                                        methods: service
                                                            .methods
                                                            .iter()
                                                            .map(|method| {
                                                                ir::Method {
                                                                    name: method
                                                                        .name
                                                                        .as_str()
                                                                        .to_owned(),
                                                                    docs: method.docs.to_string(),
                                                                    attrs: transform_attrs(
                                                                        &method.attrs,
                                                                    ),
                                                                    parameters: method
                                                                        .params
                                                                        .iter()
                                                                        .map(|param| {
                                                                            ir::MethodParam { name: param.name.as_str().to_owned(), typ: resolver.resolve_type_expr(def, &param.typ), is_optional: param.is_optional }
                                                                        })
                                                                        .collect(),
                                                                    returns: method.returns.as_ref().map(|typ| resolver.resolve_type_expr(def, typ)),
                                                                }
                                                            })
                                                            .collect(),
                                                    })
                                                }
                                            };
                                            ir::Def {
                                                name: def.name.as_str().to_owned(),
                                                docs: def.docs.to_string(),
                                                vars: def
                                                    .vars
                                                    .iter()
                                                    .map(|var| {
                                                        ir::TypeVar {
                                                            name: var.name.as_str().to_owned(),
                                                        }
                                                    })
                                                    .collect(),
                                                attrs: transform_attrs(&def.attrs),
                                                kind,
                                            }
                                        })
                                        .collect(),
                                }
                            })
                            .collect(),
                    }
                })
                .collect(),
        }
    }
}
