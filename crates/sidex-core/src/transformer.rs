//! Transformer from Sidex text sources to SIR.

use hashbrown::HashMap;
use sidex_syntax::ast;

use crate::{ir, model::ModelSource};

struct Model {
    model_idx: ir::ModelIdx,
    schemas: HashMap<String, Schema>,
}

struct Schema {
    model_idx: ir::ModelIdx,
    schema_idx: ir::SchemaIdx,
    ast_schema: ast::Schema,
    resolution_table: HashMap<String, ResolvedPath>,
}

enum ResolvedPath {
    Model {
        model: ir::ModelIdx,
    },
    Schema {
        model: ir::ModelIdx,
        schema: ir::SchemaIdx,
    },
    Def {
        model: ir::ModelIdx,
        schema: ir::SchemaIdx,
        def: ir::DefIdx,
    },
}

pub struct Transformer {
    models: HashMap<String, Model>,
    unit: ir::Unit,
}

impl Transformer {
    pub(crate) fn add_model(&mut self, src: ModelSource) {
        todo!()
    }

    fn resolve_path(
        &self,
        model: ir::ModelIdx,
        schema: ir::SchemaIdx,
        path: &ast::Path,
    ) -> Result<ResolvedPath, ()> {
        todo!()
    }

    fn resolve_type(
        &self,
        model: ir::ModelIdx,
        schema: ir::SchemaIdx,
        scope: &HashMap<String, ir::TypeVarIdx>,
        expr: &ast::TypeExpr,
    ) -> Result<ir::Type, ()> {
        match expr {
            ast::TypeExpr::Instance(instance_type_expr) => {
                let path = &instance_type_expr.path;
                // Let's first check whether the path may be a type variable from the current scope.
                if path.segments.len() == 1 && !path.is_absolute {
                    let name = &path.segments[0];
                    if let Some(idx) = scope.get(name.as_str()) {
                        if instance_type_expr.subst.is_empty() {
                            return Ok(ir::Type::TypeVar(ir::TypeVarType { idx: *idx }));
                        } else {
                            todo!("Cannot instantiate type variable with substitutions! Generate error!")
                        }
                    }
                }
                // Try to resolve the path to the respective definition.
                let resolved = self.resolve_path(model, schema, path)?;
                match resolved {
                    ResolvedPath::Def { model, schema, def } => {
                        Ok(ir::Type::Instance(ir::InstanceType {
                            model,
                            schema,
                            def,
                            subst: instance_type_expr
                                .subst
                                .iter()
                                .map(|expr| self.resolve_type(model, schema, scope, expr))
                                .collect::<Result<Vec<_>, _>>()?,
                        }))
                    }
                    _ => todo!("The path did not resolve to a definition! Generate error."),
                }
            }
            ast::TypeExpr::Sequence(sequence_type_expr) => {
                let element =
                    self.resolve_type(model, schema, scope, &sequence_type_expr.element)?;
                Ok(ir::Type::Sequence(ir::SequenceType {
                    element: Box::new(element),
                }))
            }
            ast::TypeExpr::Map(map_type_expr) => {
                let key = self.resolve_type(model, schema, scope, &map_type_expr.key)?;
                let value = self.resolve_type(model, schema, scope, &map_type_expr.value)?;
                Ok(ir::Type::Map(ir::MapType {
                    key: Box::new(key),
                    value: Box::new(value),
                }))
            }
        }
    }

    pub fn transform(self) -> ir::Unit {
        /*
         * At this point, all the ASTs have been parsed and we already created `ir::Def`s
         * for all the definitions. It remains:
         *
         * 1️⃣ Populate the lookup tables with the imports.
         *
         * 2️⃣ Resolve the type expressions.
         */
        self.unit
    }
}
