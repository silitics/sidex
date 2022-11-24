//! Implementation of the transformation process from text sources to Sidex IR.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use sidex_syntax::{
    ast, parse,
    tokens::{self},
};
use thiserror::Error;

use crate::{
    builtins,
    bundle::{self, iter_schemas, BundleSource, Manifest},
    ir::{self, TokenKind},
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
    pub storage: ir::SourceStorage,

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

const STD_BUNDLE: ir::BundleIdx = ir::STD_BUNDLE_IDX;

/// Builds the definition table of a schema.
fn build_def_table(defs: &[ast::Def]) -> HashMap<String, ir::DefIdx> {
    defs.iter()
        .enumerate()
        .map(|(idx, def)| (def.name.as_str().to_owned(), ir::DefIdx::from(idx)))
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
        let schema = &self.transformer.loaded[self.bundle.idx()].schemas[self.schema.idx()];
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
        let schema = &self.transformer.loaded[self.bundle.idx()].schemas[self.schema.idx()];
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
                                    self.transformer.loaded[bundle.idx()].schema_by_name.iter()
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
                                for (name, idx) in self.transformer.loaded[bundle.idx()].schemas
                                    [schema.idx()]
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

        let std_bundle = &self.transformer.loaded[STD_BUNDLE.idx()];

        let builtins_schema =
            &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().idx()];

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
                    let schema = self.transformer.loaded[idx.idx()]
                        .schema_by_name
                        .get(first.as_str())
                        .unwrap();
                    LookupEntry::Schema {
                        bundle: idx,
                        schema: *schema,
                    }
                }
                LookupEntry::Schema { bundle, schema } => {
                    let def = self.transformer.loaded[bundle.idx()].schemas[schema.idx()]
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
                        self.transformer.loaded[self.bundle.idx()].schemas[self.schema.idx()].name
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
                        return ir::Type::new(ir::TypeKind::TypeVar(ir::TypeVarType::new(
                            ir::TypeVarIdx::from(var_idx),
                        )));
                    }
                }
                let entry = self.resolve_path(&instance.path);
                match entry {
                    LookupEntry::Def {
                        bundle,
                        schema,
                        def,
                    } => {
                        let type_def = &self.transformer.loaded[bundle.idx()].schemas[schema.idx()]
                            .defs[def.idx()];
                        assert_eq!(type_def.vars.len(), instance.subst.len());

                        return ir::Type::new(ir::TypeKind::Instance(
                            ir::InstanceType::new(bundle, schema, def).with_subst(
                                instance
                                    .subst
                                    .iter()
                                    .map(|subst| self.resolve_type_expr(enclosing, subst))
                                    .collect(),
                            ),
                        ));
                    }
                    _ => panic!("Invalid path."),
                }
            }
            ast::TypeExpr::Sequence(sequence) => {
                let std_bundle = &self.transformer.loaded[STD_BUNDLE.idx()];
                let builtins_schema =
                    &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().idx()];
                let sequence_def = builtins_schema.def_by_name.get("Sequence").unwrap();

                ir::Type::new(ir::TypeKind::Instance(
                    ir::InstanceType::new(std_bundle.idx, builtins_schema.idx, *sequence_def)
                        .with_subst(vec![self.resolve_type_expr(enclosing, &sequence.element)]),
                ))
            }
            ast::TypeExpr::Map(map) => {
                let std_bundle = &self.transformer.loaded[STD_BUNDLE.idx()];
                let builtins_schema =
                    &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().idx()];
                let map_def = builtins_schema.def_by_name.get("Map").unwrap();

                ir::Type::new(ir::TypeKind::Instance(
                    ir::InstanceType::new(std_bundle.idx, builtins_schema.idx, *map_def)
                        .with_subst(vec![
                            self.resolve_type_expr(enclosing, &map.key),
                            self.resolve_type_expr(enclosing, &map.value),
                        ]),
                ))
            }
            ast::TypeExpr::Unit => {
                let std_bundle = &self.transformer.loaded[STD_BUNDLE.idx()];
                let builtins_schema =
                    &std_bundle.schemas[std_bundle.schema_by_name.get("builtins").unwrap().idx()];
                let unit_def = builtins_schema.def_by_name.get("unit").unwrap();

                ir::Type::new(ir::TypeKind::Instance(ir::InstanceType::new(
                    std_bundle.idx,
                    builtins_schema.idx,
                    *unit_def,
                )))
            }
        }
    }
}

fn transform_token_stream(stream: &ast::TokenStream) -> ir::TokenStream {
    let mut composed = String::new();
    let mut ir_tokens = Vec::new();
    let mut start = None;
    for token in stream.iter() {
        if start.is_none() {
            start = Some(token.start())
        }
        let kind = match &token.kind {
            tokens::TokenKind::Delimiter(delimiter) => TokenKind::Delimiter(delimiter.to_string()),
            tokens::TokenKind::Punctuation(punctuation) => {
                composed.push_str(&punctuation.to_string());
                if punctuation.is_composed {
                    continue;
                }
                TokenKind::Punctuation(composed.clone())
            }
            tokens::TokenKind::Keyword(keyword) => {
                // Keywords do not bear any special meaning in attributes.
                TokenKind::Identifier(keyword.to_string())
            }
            tokens::TokenKind::Literal(literal) => {
                match literal {
                    tokens::Literal::Numeric { .. } => {
                        TokenKind::Literal(ir::Literal::Number(token.to_string()))
                    }
                    tokens::Literal::String(string) => {
                        TokenKind::Literal(ir::Literal::String(string.as_ref().clone()))
                    }
                    tokens::Literal::Boolean(boolean) => {
                        TokenKind::Literal(ir::Literal::Bool(*boolean))
                    }
                }
            }
            tokens::TokenKind::Identifier(_) => TokenKind::Identifier(token.to_string()),
            tokens::TokenKind::Comment { .. } | tokens::TokenKind::Doc { .. } => {
                // Strip from IR.
                start = None;
                continue;
            }
        };
        composed.clear();
        ir_tokens.push(ir::Token::new(kind).with_span(Some(ir::Span::new(
            ir::SourceIdx::from(0),
            start.unwrap(),
            token.end(),
        ))));
    }
    ir_tokens.into()
}

fn transform_attr(attr: &ast::Attr) -> ir::Attr {
    let kind = match &attr.kind {
        ast::AttrKind::Path(path) => ir::AttrKind::Path(ir::Path::from(path.to_string())),
        ast::AttrKind::List(list) => {
            ir::AttrKind::List(ir::AttrList {
                path: ir::Path::from(list.path.to_string()),
                args: list.elements.iter().map(transform_attr).collect(),
            })
        }
        ast::AttrKind::Assign(assign) => {
            ir::AttrKind::Assign(ir::AttrAssign {
                path: ir::Path::from(assign.path.to_string()),
                value: Box::new(transform_attr(&assign.value)),
            })
        }
        ast::AttrKind::Tokens(tokens) => ir::AttrKind::Tokens(transform_token_stream(tokens)),
    };
    ir::Attr::new(kind)
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
        &self.loaded[idx.idx()].source.manifest
    }

    fn get_bundle_by_path(&self, path: &Path) -> Option<&LoadedBundle> {
        let canonical = path.canonicalize().unwrap();
        self.bundle_by_path
            .get(&canonical)
            .map(|idx| &self.loaded[idx.idx()])
    }

    pub(crate) fn insert_bundle(&mut self, source: BundleSource) -> Result<ir::BundleIdx, Error> {
        let idx = ir::BundleIdx::from(self.loaded.len());
        self.bundle_by_name
            .insert(source.manifest.metadata.name.clone(), idx);
        if let Some(path) = &source.path {
            self.bundle_by_path.insert(path.clone(), idx);
        }

        // Parse the bundle.
        let mut schemas = Vec::new();
        for (name, schema) in source.schemas.iter() {
            let schema = parse(&self.storage[*schema])
                .ok_or_else(|| Error::Other(format!("Error parsing schema {name:?}.")))?;
            let docs = schema.docs.to_string();
            let (imports, defs) = split_items(schema.items);
            let def_by_name = build_def_table(&defs);
            schemas.push(ParsedSchema {
                idx: ir::SchemaIdx::from(schemas.len()),
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
                .map(|(idx, schema)| (schema.name.clone(), ir::SchemaIdx::from(idx)))
                .collect(),
            schemas,
        });

        Ok(idx)
    }

    pub fn load_bundle_recursive(&mut self, path: &Path) -> Result<ir::BundleIdx, Error> {
        let idx = self.load_bundle(path)?;
        let mut stack = vec![idx];
        while let Some(idx) = stack.pop() {
            let dependencies = self.loaded[idx.idx()]
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
            let loaded = &self.loaded[idx.idx()];
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
                let source_id = self.storage.insert(
                    std::fs::read_to_string(&schema_path)?,
                    Some(schema_path.to_string_lossy().into_owned()),
                );
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
        ir::Unit::new().with_bundles(
            self.loaded
                .par_iter()
                .map(|bundle| {
                    let mut dependencies = Vec::new();

                    let mut dependency_table = HashMap::new();

                    for (name, dependency) in bundle.source.manifest.dependencies() {
                        let dependency_path =
                            bundle.source.path.as_ref().unwrap().join(&dependency.path);
                        let dependency_bundle = self.get_bundle_by_path(&dependency_path).unwrap();

                        dependencies
                            .push(ir::Dependency::new(name.to_owned(), dependency_bundle.idx));
                        dependency_table.insert(name.to_owned(), dependency_bundle.idx);
                    }

                    // Add implicit dependency on `std`.
                    dependencies.push(ir::Dependency::new("std".to_owned(), STD_BUNDLE));
                    dependency_table.insert("std".to_owned(), STD_BUNDLE);

                    ir::Bundle::new(bundle.idx, bundle.source.manifest.metadata.clone())
                        .with_dependencies(dependencies)
                        .with_schemas(
                            bundle
                                .schemas
                                .par_iter()
                                .map(|schema| {
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

                                    ir::Schema::new(
                                        schema.idx,
                                        schema.name.clone(),
                                    ).with_docs(Some(ir::Docs::new(schema.docs.clone())))
                                    .with_defs(schema
                                        .defs
                                        .iter()
                                        .map(|def| {
                                            let kind = match &def.kind {
                                                ast::DefKind::Alias(alias) => {
                                                    ir::DefKind::TypeAlias(ir::TypeAliasDef::new(
                                                         resolver.resolve_type_expr(
                                                            &def,
                                                            &alias.aliased,
                                                        ),
                                                    ))
                                                }
                                                ast::DefKind::OpaqueType(_) => {
                                                    ir::DefKind::OpaqueType(ir::OpaqueTypeDef::new())
                                                }
                                                ast::DefKind::RecordType(record) => {
                                                    ir::DefKind::RecordType(ir::RecordTypeDef::new().with_fields(
                                                        record
                                                            .fields
                                                            .iter()
                                                            .map(|field| {
                                                                ir::Field::new(
                                                                    ir::Identifier::new(field
                                                                        .name
                                                                        .as_str()
                                                                        .to_owned()),
                                                                    resolver
                                                                    .resolve_type_expr(
                                                                        def, &field.typ,
                                                                    )
                                                                    ).with_docs(Some(ir::Docs::new(field.docs.to_string()))).with_attrs(transform_attrs(
                                                                        &field.attrs,
                                                                    )).with_is_optional(field.is_optional)
                                                                }
                                                            )
                                                            .collect(),
                                                    ))
                                                }
                                                ast::DefKind::VariantType(variant) => {
                                                    ir::DefKind::VariantType(ir::VariantTypeDef::new().with_variants(
                                                         variant
                                                            .variants
                                                            .iter()
                                                            .map(|variant| {
                                                                ir::Variant::new( ir::Identifier::new(variant
                                                                        .name
                                                                        .as_str()
                                                                        .to_owned()),
                                                                    ).with_docs(Some(ir::Docs::new(variant.docs.to_string()))).with_attrs(transform_attrs(
                                                                        &variant.attrs,
                                                                    )).with_typ(variant.typ.as_ref().map(
                                                                        |typ| {
                                                                            resolver
                                                                                .resolve_type_expr(
                                                                                    def, typ,
                                                                                )
                                                                        },
                                                                    ))
                                                            })
                                                            .collect(),
                                                    ))
                                                }
                                                ast::DefKind::WrapperType(wrapped) => {
                                                    ir::DefKind::WrapperType(ir::WrapperTypeDef::new(
                                                         resolver.resolve_type_expr(
                                                            &def,
                                                            &wrapped.wrapped,
                                                        ),
                                                    ))
                                                }
                                                ast::DefKind::Service(service) => {
                                                    ir::DefKind::Service(ir::ServiceDef::new().with_methods(service
                                                            .methods
                                                            .iter()
                                                            .map(|method| {
                                                                ir::Method::new(ir::Identifier::new(method
                                                                        .name
                                                                        .as_str()
                                                                        .to_owned()),
                                                                    ).with_docs(Some(ir::Docs::new(method.docs.to_string()))).with_attrs(transform_attrs(
                                                                        &method.attrs,
                                                                    )).with_parameters(method
                                                                        .params
                                                                        .iter()
                                                                        .map(|param| {
                                                                            ir::MethodParam::new(ir::Identifier::new(param.name.as_str().to_owned()), resolver.resolve_type_expr(def, &param.typ) ).with_is_optional(param.is_optional)
                                                                        })
                                                                        .collect()).with_returns(method.returns.as_ref().map(|typ| resolver.resolve_type_expr(def, typ)))
                                                            })
                                                            .collect(),
                                                    ))
                                                }
                                            };
                                            ir::Def::new( ir::Identifier::new(def.name.as_str().to_owned()), kind).with_docs(Some(ir::Docs::new(def.docs.to_string()))).with_vars(def
                                                    .vars
                                                    .iter()
                                                    .map(|var| {
                                                        ir::TypeVar::new(ir::Identifier::new(var.name.as_str().to_owned()))
                                                    })
                                                    .collect()).with_attrs(transform_attrs(&def.attrs))
                                        })
                                        .collect(),)
                                })
                                .collect(),
                        )
                })
                .collect(),
        )
    }
}
