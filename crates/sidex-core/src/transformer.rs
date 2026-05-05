//! Implementation of the transformation process from text sources to Sidex IR.

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use sidex_syntax::ast;
use sidex_syntax::parse;
use sidex_syntax::tokens;
use thiserror::Error;

use crate::builtins;
use crate::bundle::BundleSource;
use crate::bundle::Manifest;
use crate::bundle::iter_schemas;
use crate::bundle::{self};
use crate::ir::{self};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Manifest(#[from] bundle::ManifestLoadError),
    #[error("{0}")]
    Other(String),
}

/// A loaded bundle along with its parsed schemas. The transformer keeps these
/// around so that name resolution and IR assembly can happen lazily.
#[derive(Debug, Clone)]
struct LoadedBundle {
    /// The transformer-local bundle index. This is the index in
    /// `Transformer::loaded` and matches the eventual `BundleIdx` in the
    /// produced IR (the IR's bundles are inserted in load order).
    idx: ir::BundleIdx,
    source: BundleSource,
    schemas: Vec<ParsedSchema>,
    schema_by_name: HashMap<String, LocalSchemaIdx>,
    /// Whether this bundle is auto-loaded by the compiler (the standard
    /// library or a plugin attribute schema). Propagated to [`ir::Bundle::is_internal`].
    is_internal: bool,
}

/// Local schema index inside a bundle. We keep a separate type alias to
/// avoid confusing it with the global `ir::SchemaIdx` produced at
/// transformation time.
type LocalSchemaIdx = usize;

/// Local definition index inside a parsed schema, again distinct from the
/// global `ir::DefIdx`.
type LocalDefIdx = usize;

#[derive(Debug, Clone)]
pub struct ParsedSchema {
    /// Local index within the bundle (its position in `LoadedBundle::schemas`).
    idx: LocalSchemaIdx,
    name: String,
    docs: String,
    defs: Vec<ast::Def>,
    imports: Vec<ast::Import>,
    def_by_name: HashMap<String, LocalDefIdx>,
}

impl ParsedSchema {
    /// The parsed `import` directives.
    pub fn imports(&self) -> &[ast::Import] {
        &self.imports
    }

    /// The parsed top-level definitions.
    pub fn defs(&self) -> &[ast::Def] {
        &self.defs
    }

    /// The schema's name (file stem of the schema source).
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Transformer {
    /// Source storage for all loaded schemas. The transformer accumulates
    /// sources here during loading; `transform` clones the list into the
    /// produced IR.
    pub sources: Vec<ir::Source>,
    loaded: Vec<LoadedBundle>,
    bundle_by_name: HashMap<String, ir::BundleIdx>,
    bundle_by_path: HashMap<PathBuf, ir::BundleIdx>,
}

const STD_BUNDLE: ir::BundleIdx = ir::STD_BUNDLE_IDX;

/// Splits the items of a schema into imports and definitions.
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

/// Builds the local definition table of a schema.
fn build_def_table(defs: &[ast::Def]) -> HashMap<String, LocalDefIdx> {
    defs.iter()
        .enumerate()
        .map(|(idx, def)| (def.name.as_str().to_owned(), idx))
        .collect()
}

/// Maps a (bundle, local-schema, local-def) triple to a global `ir::DefIdx`
/// allocated during transformation.
type DefIdxMap = HashMap<(ir::BundleIdx, LocalSchemaIdx, LocalDefIdx), ir::DefIdx>;

/// Maps a (bundle, local-schema) pair to a global `ir::SchemaIdx`.
type SchemaIdxMap = HashMap<(ir::BundleIdx, LocalSchemaIdx), ir::SchemaIdx>;

/// What a name in a schema's import / def table resolves to.
#[derive(Debug, Clone)]
enum LookupEntry {
    Root,
    Bundle(ir::BundleIdx),
    Schema {
        bundle: ir::BundleIdx,
        schema: LocalSchemaIdx,
    },
    Def {
        bundle: ir::BundleIdx,
        schema: LocalSchemaIdx,
        def: LocalDefIdx,
    },
}

struct Resolver<'t, 'm> {
    transformer: &'t Transformer,
    bundle: ir::BundleIdx,
    schema: LocalSchemaIdx,
    dependencies: &'m HashMap<String, ir::BundleIdx>,
    def_idx_map: &'m DefIdxMap,
    schema_idx_map: &'m SchemaIdxMap,
    table: HashMap<String, LookupEntry>,
}

impl<'t, 'm> Resolver<'t, 'm> {
    fn populate_defs(&mut self) {
        let schema = &self.transformer.loaded[self.bundle.idx()].schemas[self.schema];
        for (name, &local) in &schema.def_by_name {
            self.table.insert(
                name.clone(),
                LookupEntry::Def {
                    bundle: self.bundle,
                    schema: self.schema,
                    def: local,
                },
            );
        }
    }

    fn populate_imports(&mut self) {
        let schema = &self.transformer.loaded[self.bundle.idx()].schemas[self.schema];
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
                                    &self.transformer.loaded[bundle.idx()].schema_by_name
                                {
                                    self.table
                                        .entry(name.clone())
                                        .or_insert(LookupEntry::Schema {
                                            bundle,
                                            schema: *idx,
                                        });
                                }
                            }
                            LookupEntry::Schema { bundle, schema } => {
                                for (name, idx) in &self.transformer.loaded[bundle.idx()].schemas
                                    [schema]
                                    .def_by_name
                                {
                                    self.table.entry(name.clone()).or_insert(LookupEntry::Def {
                                        bundle,
                                        schema,
                                        def: *idx,
                                    });
                                }
                            }
                            LookupEntry::Root | LookupEntry::Def { .. } => {
                                panic!("Invalid import target.")
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

        // Implicitly bring builtins into scope. core::attrs is *not*
        // implicit — user schemas that use the meta-attrs vocabulary
        // (`#[attrs(...)]`, `TypeRef`, etc.) bring it in with an explicit
        // `import ::core::attrs::*`.
        let std_bundle = &self.transformer.loaded[STD_BUNDLE.idx()];
        let builtins_local = *std_bundle.schema_by_name.get("builtins").unwrap();
        for (name, &def) in &std_bundle.schemas[builtins_local].def_by_name {
            self.table.entry(name.clone()).or_insert(LookupEntry::Def {
                bundle: STD_BUNDLE,
                schema: builtins_local,
                def,
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
                        .unwrap_or_else(|| {
                            panic!(
                                "Unable to find schema {} in bundle {}.",
                                first.as_str(),
                                self.transformer.loaded[idx.idx()]
                                    .source
                                    .manifest
                                    .metadata
                                    .name
                            )
                        });
                    LookupEntry::Schema {
                        bundle: idx,
                        schema: *schema,
                    }
                }
                LookupEntry::Schema { bundle, schema } => {
                    let def = self.transformer.loaded[bundle.idx()].schemas[schema]
                        .def_by_name
                        .get(first.as_str())
                        .unwrap_or_else(|| panic!("Unable to find definition {}.", first.as_str()));
                    LookupEntry::Def {
                        bundle,
                        schema,
                        def: *def,
                    }
                }
                LookupEntry::Def { .. } => {
                    panic!("Invalid lookup chain ending at a definition.")
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
            let entry = self
                .table
                .get(first.as_str())
                .unwrap_or_else(|| panic!("Unable to resolve `{}`.", path));
            (entry.clone(), &path.segments[1..])
        };
        self.resolve_segments(root, tail)
    }

    /// Translate a local (bundle, schema, def) triple into a fully-resolved
    /// `ir::DefRef` using the global allocation maps.
    fn def_ref(
        &self,
        bundle: ir::BundleIdx,
        schema: LocalSchemaIdx,
        def: LocalDefIdx,
    ) -> ir::DefRef {
        let schema_global = *self
            .schema_idx_map
            .get(&(bundle, schema))
            .expect("schema should have been allocated globally");
        let def_global = *self
            .def_idx_map
            .get(&(bundle, schema, def))
            .expect("def should have been allocated globally");
        ir::DefRef::new(bundle, schema_global, def_global)
    }

    fn builtin_def_ref(&self, name: &str) -> ir::DefRef {
        let std_bundle = &self.transformer.loaded[STD_BUNDLE.idx()];
        let builtins_local = *std_bundle.schema_by_name.get("builtins").unwrap();
        let local_def = *std_bundle.schemas[builtins_local]
            .def_by_name
            .get(name)
            .unwrap_or_else(|| panic!("Builtin `{name}` not found."));
        self.def_ref(STD_BUNDLE, builtins_local, local_def)
    }

    fn resolve_type_expr(&self, enclosing: &ast::Def, expr: &ast::TypeExpr) -> ir::Type {
        match expr {
            ast::TypeExpr::Instance(instance) => {
                if instance.path.segments.len() == 1 {
                    let name = instance.path.segments[0].as_str();
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
                        let def_ref = self.def_ref(bundle, schema, def);
                        ir::Type::new(ir::TypeKind::Instance(
                            ir::InstanceType::new(def_ref).with_subst(
                                instance
                                    .subst
                                    .iter()
                                    .map(|subst| self.resolve_type_expr(enclosing, subst))
                                    .collect(),
                            ),
                        ))
                    }
                    _ => panic!("Path does not resolve to a definition."),
                }
            }
            ast::TypeExpr::Sequence(sequence) => {
                let def_ref = self.builtin_def_ref("Sequence");
                ir::Type::new(ir::TypeKind::Instance(
                    ir::InstanceType::new(def_ref)
                        .with_subst(vec![self.resolve_type_expr(enclosing, &sequence.element)]),
                ))
            }
            ast::TypeExpr::Map(map) => {
                let def_ref = self.builtin_def_ref("Map");
                ir::Type::new(ir::TypeKind::Instance(
                    ir::InstanceType::new(def_ref).with_subst(vec![
                        self.resolve_type_expr(enclosing, &map.key),
                        self.resolve_type_expr(enclosing, &map.value),
                    ]),
                ))
            }
            ast::TypeExpr::Unit => {
                let def_ref = self.builtin_def_ref("unit");
                ir::Type::new(ir::TypeKind::Instance(ir::InstanceType::new(def_ref)))
            }
        }
    }
}

/// Convert an AST attribute value (`Attr` on the rhs of an Assign) into a
/// typed `ir::AttrValue`. Best-effort: anything that doesn't fit one of the
/// `AttrValue` cases is dropped.
fn ast_attr_to_value(attr: &ast::Attr) -> Option<ir::AttrValue> {
    match &attr.kind {
        ast::AttrKind::Path(path) => Some(ir::AttrValue::Path(path.to_string())),
        ast::AttrKind::Tokens(tokens) if tokens.len() == 1 => {
            match &tokens[0].kind {
                tokens::TokenKind::Literal(lit) => {
                    match lit {
                        tokens::Literal::String(s) => {
                            Some(ir::AttrValue::String(s.as_ref().clone()))
                        }
                        tokens::Literal::Numeric { .. } => {
                            Some(ir::AttrValue::Number(tokens[0].to_string()))
                        }
                        tokens::Literal::Boolean(b) => Some(ir::AttrValue::Bool(*b)),
                    }
                }
                tokens::TokenKind::Identifier(s) => Some(ir::AttrValue::Path(s.to_string())),
                _ => None,
            }
        }
        ast::AttrKind::Tokens(tokens) => {
            // A `{ ... }`-form value: balanced brace group captured verbatim.
            // Inner text is reconstructed from significant tokens via Display
            // (whitespace-collapsed but lex-equivalent); the span covers the
            // first-to-last inner token in the originating source.
            tokens_brace_group_value(tokens).map(ir::AttrValue::Tokens)
        }
        _ => None,
    }
}

/// Recognize a balanced `{ ... }` token group and lift it into a
/// [`ir::TokensValue`]. Returns `None` when the stream isn't brace-delimited.
fn tokens_brace_group_value(stream: &ast::TokenStream) -> Option<ir::TokensValue> {
    if stream.len() < 2 {
        return None;
    }
    let is_open_brace = matches!(
        &stream[0].kind,
        tokens::TokenKind::Delimiter(tokens::DelimiterSymbol::Open(tokens::DelimiterKind::Brace))
    );
    let is_close_brace = matches!(
        &stream[stream.len() - 1].kind,
        tokens::TokenKind::Delimiter(tokens::DelimiterSymbol::Close(tokens::DelimiterKind::Brace))
    );
    if !(is_open_brace && is_close_brace) {
        return None;
    }
    let inner = &stream[1..stream.len() - 1];
    let span = inner
        .first()
        .zip(inner.last())
        .map(|(first, last)| ir::Span::new(first.span().src, first.span().start, last.span().end));
    // Reconstruct text from significant tokens. This is whitespace-collapsed
    // but lex-equivalent — the consuming plugin re-lexes the body anyway.
    let mut text = String::new();
    for token in inner {
        use std::fmt::Write as _;
        write!(text, "{}", token).expect("write to String never fails");
        if token.is_separated() {
            text.push(' ');
        }
    }
    let mut value = ir::TokensValue::new(text.trim().to_owned());
    if let Some(span) = span {
        value.span = Some(span);
    }
    Some(value)
}

fn transform_attr(attr: &ast::Attr) -> Option<ir::Attr> {
    let kind = match &attr.kind {
        ast::AttrKind::Path(path) => ir::AttrKind::Path(path.to_string()),
        ast::AttrKind::List(list) => {
            ir::AttrKind::List(ir::AttrList {
                path: list.path.to_string(),
                args: list.elements.iter().filter_map(transform_attr).collect(),
            })
        }
        ast::AttrKind::Assign(assign) => {
            let value = ast_attr_to_value(&assign.value)?;
            ir::AttrKind::Assign(ir::AttrAssign {
                path: assign.path.to_string(),
                value,
            })
        }
        // Token streams as positional list args: brace groups lift into
        // `AttrKind::Value(AttrValue::Tokens(...))` so the typed-attrs
        // parser can bind them positionally to a `core::attrs::Tokens`-
        // typed schema field. Single-token literals (the legacy
        // `derive("Clone")`-style positional) lift the same way for
        // forward compatibility, even though no current plugin reads
        // them positionally.
        ast::AttrKind::Tokens(stream) => {
            if let Some(value) = tokens_brace_group_value(stream) {
                ir::AttrKind::Value(ir::AttrValue::Tokens(value))
            } else if let Some(value) = single_literal_token_value(stream) {
                ir::AttrKind::Value(value)
            } else {
                return None;
            }
        }
    };
    Some(ir::Attr::new(kind))
}

/// Recognize a single-token literal stream (`Clone`, `42`, `"text"`) and
/// extract the same `AttrValue` shape `ast_attr_to_value` produces.
/// Used when a positional list arg is a bare literal — currently rare,
/// but keeps the lowering symmetric so callers can write either named
/// (`derive = "Clone"`) or positional (`derive("Clone")`) forms.
fn single_literal_token_value(stream: &ast::TokenStream) -> Option<ir::AttrValue> {
    if stream.len() != 1 {
        return None;
    }
    match &stream[0].kind {
        tokens::TokenKind::Literal(lit) => {
            match lit {
                tokens::Literal::String(s) => Some(ir::AttrValue::String(s.as_ref().clone())),
                tokens::Literal::Numeric { .. } => {
                    Some(ir::AttrValue::Number(stream[0].to_string()))
                }
                tokens::Literal::Boolean(b) => Some(ir::AttrValue::Bool(*b)),
            }
        }
        tokens::TokenKind::Identifier(s) => Some(ir::AttrValue::Path(s.to_string())),
        _ => None,
    }
}

fn transform_attrs(attrs: &[ast::Attr]) -> Vec<ir::Attr> {
    attrs.iter().filter_map(transform_attr).collect()
}

fn docs_or_none(text: &str) -> Option<ir::Docs> {
    if text.is_empty() {
        None
    } else {
        Some(ir::Docs::new(text.to_owned()))
    }
}

impl Default for Transformer {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformer {
    pub fn new() -> Self {
        let mut transformer = Self {
            sources: Vec::new(),
            loaded: Vec::new(),
            bundle_by_name: HashMap::new(),
            bundle_by_path: HashMap::new(),
        };
        let std_bundle = builtins::std_bundle(&mut transformer);
        transformer.insert_internal_bundle(std_bundle).unwrap();
        for plugin_attrs in builtins::plugin_attrs_bundles(&mut transformer) {
            transformer.insert_internal_bundle(plugin_attrs).unwrap();
        }
        transformer
    }

    /// Append a source to the transformer's storage and return its index.
    pub fn insert_source(&mut self, text: String, origin: Option<String>) -> ir::SourceIdx {
        let idx = ir::SourceIdx::from(self.sources.len());
        self.sources
            .push(ir::Source::new().with_text(Some(text)).with_origin(origin));
        idx
    }

    pub fn get_bundle_manifest(&self, idx: ir::BundleIdx) -> &Manifest {
        &self.loaded[idx.idx()].source.manifest
    }

    pub fn iter_user_schemas(&self, bundle: ir::BundleIdx) -> impl Iterator<Item = &ParsedSchema> {
        self.loaded[bundle.idx()].schemas.iter()
    }

    /// The source index of the schema with `name` in `bundle`.
    pub fn schema_source_idx_by_name(
        &self,
        bundle: ir::BundleIdx,
        name: &str,
    ) -> Option<ir::SourceIdx> {
        let loaded = self.loaded.get(bundle.idx())?;
        loaded.source.schemas.get(name).copied()
    }

    fn get_bundle_by_path(&self, path: &Path) -> Option<&LoadedBundle> {
        let canonical = path.canonicalize().unwrap();
        self.bundle_by_path
            .get(&canonical)
            .map(|idx| &self.loaded[idx.idx()])
    }

    pub(crate) fn insert_bundle(&mut self, source: BundleSource) -> Result<ir::BundleIdx, Error> {
        self.insert_bundle_inner(source, false)
    }

    /// Insert an auto-loaded bundle (the standard library or a plugin
    /// attribute schema). The resulting `ir::Bundle` is flagged
    /// [`is_internal`](ir::Bundle::is_internal) so code generators can skip it.
    fn insert_internal_bundle(&mut self, source: BundleSource) -> Result<ir::BundleIdx, Error> {
        self.insert_bundle_inner(source, true)
    }

    fn insert_bundle_inner(
        &mut self,
        source: BundleSource,
        is_internal: bool,
    ) -> Result<ir::BundleIdx, Error> {
        let idx = ir::BundleIdx::from(self.loaded.len());
        self.bundle_by_name
            .insert(source.manifest.metadata.name.clone(), idx);
        if let Some(path) = &source.path {
            self.bundle_by_path.insert(path.clone(), idx);
        }

        let mut schemas = Vec::new();
        for (name, schema_src) in &source.schemas {
            let schema = parse(*schema_src, &self.sources[schema_src.idx()])
                .ok_or_else(|| Error::Other(format!("Error parsing schema {name:?}.")))?;
            let docs = schema.docs.to_string();
            let (imports, defs) = split_items(schema.items);
            let def_by_name = build_def_table(&defs);
            schemas.push(ParsedSchema {
                idx: schemas.len(),
                name: name.clone(),
                docs,
                defs,
                imports,
                def_by_name,
            });
        }

        self.loaded.push(LoadedBundle {
            idx,
            source,
            schema_by_name: schemas
                .iter()
                .enumerate()
                .map(|(idx, schema)| (schema.name.clone(), idx))
                .collect(),
            schemas,
            is_internal,
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
        if let Some(idx) = self.bundle_by_name.get(name).copied() {
            let loaded = &mut self.loaded[idx.idx()];
            match &loaded.source.path {
                Some(existing) if existing == &path => Ok(idx),
                Some(other_path) => {
                    Err(Error::Other(format!(
                        "There are two different bundles with name {name:?} ({path:?}, {other_path:?})."
                    )))
                }
                // The bundle was auto-loaded (e.g. a plugin attribute schema)
                // and the user is now loading it from disk. Adopt the path so
                // dependency resolution works, and stop treating it as
                // internal — the user has it on their dependency graph.
                None => {
                    loaded.source.path = Some(path.clone());
                    loaded.is_internal = false;
                    self.bundle_by_path.insert(path, idx);
                    Ok(idx)
                }
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
                let source_id = self.insert_source(
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

    /// Build the IR from the loaded bundles, with `root` as the assembled-around bundle.
    pub fn transform(&self, root: ir::BundleIdx) -> ir::Ir {
        let mut ir = ir::Ir::new(root).with_sources(self.sources.clone());

        // Phase 1: allocate bundles in load order so that BundleIdx values
        // match the transformer's internal indices.
        for loaded in &self.loaded {
            let metadata = loaded.source.manifest.metadata.clone();
            ir.bundles
                .push(ir::Bundle::new(metadata).with_is_internal(loaded.is_internal));
        }

        // Phase 2: allocate schemas globally and record the mapping.
        let mut schema_idx_map: SchemaIdxMap = HashMap::new();
        for loaded in &self.loaded {
            for parsed in &loaded.schemas {
                let global = ir::SchemaIdx::from(ir.schemas.len());
                let source = loaded.source.schemas.get(&parsed.name).copied();
                let schema = ir::Schema::new(loaded.idx, parsed.name.clone())
                    .with_docs(docs_or_none(&parsed.docs))
                    .with_source(source);
                ir.schemas.push(schema);
                ir.bundles[loaded.idx.idx()].schemas.push(global);
                schema_idx_map.insert((loaded.idx, parsed.idx), global);
            }
        }

        // Phase 3: allocate def stubs (placeholder bodies). This lets the
        // resolver translate any (bundle, schema, def-name) reference into
        // a global DefIdx without worrying about declaration order.
        let mut def_idx_map: DefIdxMap = HashMap::new();
        for loaded in &self.loaded {
            for parsed in &loaded.schemas {
                let schema_idx = schema_idx_map[&(loaded.idx, parsed.idx)];
                for (local_def_idx, ast_def) in parsed.defs.iter().enumerate() {
                    let global = ir::DefIdx::from(ir.defs.len());
                    let stub = ir::Def::new(
                        schema_idx,
                        ir::Ident::new(ast_def.name.as_str().to_owned()),
                        // Placeholder kind; replaced in phase 4.
                        ir::DefKind::OpaqueType(ir::OpaqueTypeDef::new()),
                    );
                    ir.defs.push(stub);
                    ir.schemas[schema_idx.idx()].defs.push(global);
                    def_idx_map.insert((loaded.idx, parsed.idx, local_def_idx), global);
                }
            }
        }

        // Phase 4: fill in def bodies with fully-resolved type references.
        for loaded in &self.loaded {
            // Build the dependency table for this bundle.
            let mut dependencies: HashMap<String, ir::BundleIdx> = HashMap::new();
            if let Some(path) = &loaded.source.path {
                for (name, dep) in loaded.source.manifest.dependencies() {
                    let dep_path = path.join(&dep.path);
                    let dep_bundle = self
                        .get_bundle_by_path(&dep_path)
                        .expect("Dependency should be loaded.");
                    dependencies.insert(name.to_string(), dep_bundle.idx);
                }
            }
            // Make the standard library implicitly available under its
            // declared bundle name (currently "core") so user schemas can
            // write `import core::attrs::*` etc. We also keep the legacy
            // "std" alias for backwards compatibility with existing schemas.
            let std_bundle_name = self.loaded[STD_BUNDLE.idx()]
                .source
                .manifest
                .metadata
                .name
                .clone();
            dependencies.insert(std_bundle_name, STD_BUNDLE);
            dependencies.insert("std".to_owned(), STD_BUNDLE);

            for parsed in &loaded.schemas {
                let mut resolver = Resolver {
                    transformer: self,
                    bundle: loaded.idx,
                    schema: parsed.idx,
                    dependencies: &dependencies,
                    def_idx_map: &def_idx_map,
                    schema_idx_map: &schema_idx_map,
                    table: HashMap::new(),
                };
                resolver.populate_defs();
                resolver.populate_imports();

                for (local_def_idx, ast_def) in parsed.defs.iter().enumerate() {
                    let global_def = def_idx_map[&(loaded.idx, parsed.idx, local_def_idx)];

                    let kind =
                        match &ast_def.kind {
                            ast::DefKind::Alias(alias) => {
                                ir::DefKind::TypeAlias(ir::TypeAliasDef::new(
                                    resolver.resolve_type_expr(ast_def, &alias.aliased),
                                ))
                            }
                            ast::DefKind::OpaqueType(_) => {
                                ir::DefKind::OpaqueType(ir::OpaqueTypeDef::new())
                            }
                            ast::DefKind::RecordType(record) => {
                                ir::DefKind::RecordType(
                                    ir::RecordTypeDef::new().with_fields(
                                        record
                                            .fields
                                            .iter()
                                            .map(|field| {
                                                ir::Field::new(
                                                    ir::Ident::new(field.name.as_str().to_owned()),
                                                    resolver.resolve_type_expr(ast_def, &field.typ),
                                                )
                                                .with_docs(docs_or_none(&field.docs.to_string()))
                                                .with_attrs(transform_attrs(&field.attrs))
                                                .with_is_optional(field.is_optional)
                                            })
                                            .collect(),
                                    ),
                                )
                            }
                            ast::DefKind::VariantType(variant) => {
                                ir::DefKind::VariantType(
                                    ir::VariantTypeDef::new().with_variants(
                                        variant
                                            .variants
                                            .iter()
                                            .map(|var| {
                                                ir::Variant::new(ir::Ident::new(
                                                    var.name.as_str().to_owned(),
                                                ))
                                                .with_docs(docs_or_none(&var.docs.to_string()))
                                                .with_attrs(transform_attrs(&var.attrs))
                                                .with_typ(var.typ.as_ref().map(|t| {
                                                    resolver.resolve_type_expr(ast_def, t)
                                                }))
                                            })
                                            .collect(),
                                    ),
                                )
                            }
                            ast::DefKind::WrapperType(wrap) => {
                                ir::DefKind::WrapperType(ir::WrapperTypeDef::new(
                                    resolver.resolve_type_expr(ast_def, &wrap.wrapped),
                                ))
                            }
                        };

                    let def = &mut ir.defs[global_def.idx()];
                    def.kind = kind;
                    def.vars = ast_def
                        .vars
                        .iter()
                        .map(|v| ir::TypeVar::new(ir::Ident::new(v.name.as_str().to_owned())))
                        .collect();
                    def.attrs = transform_attrs(&ast_def.attrs);
                    def.docs = docs_or_none(&ast_def.docs.to_string());
                }
            }
        }

        // Phase 5: discover plugin attribute schemas and parse typed attrs.
        let registry = crate::attrs_meta::PluginRegistry::build(&ir);
        crate::attrs_parser::populate_typed_attrs(&mut ir, &registry);

        ir
    }
}
