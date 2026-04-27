use sidex_attrs_json::types::JsonType;
use sidex_codegen::{Code, ToCode, quote};
use sidex_gen::ir;

use crate::config::Config;

/// A TypeScript type expression. Wraps a [`Code`] with combinators for the
/// common operations (union, intersection, tuple, array indexing).
#[derive(Clone)]
pub struct TypeExpr(pub Code);

impl TypeExpr {
    pub fn any() -> Self {
        Self(Code::from("any"))
    }

    pub fn number() -> Self {
        Self(Code::from("number"))
    }

    pub fn string() -> Self {
        Self(Code::from("string"))
    }

    pub fn boolean() -> Self {
        Self(Code::from("boolean"))
    }

    pub fn null() -> Self {
        Self(Code::from("null"))
    }

    pub fn string_literal(literal: &str) -> Self {
        Self(Code::from(format!("\"{}\"", literal.replace('\\', "\\\\").replace('"', "\\\""))))
    }

    pub fn number_literal(literal: f64) -> Self {
        Self(Code::from(literal.to_string()))
    }

    pub fn union<I: IntoIterator<Item = Self>>(types: I) -> Self {
        let parts: Vec<Code> = types.into_iter().map(|t| t.0).collect();
        Self(quote!("(@(@parts) | +)"))
    }

    pub fn intersection<I: IntoIterator<Item = Self>>(types: I) -> Self {
        let parts: Vec<Code> = types.into_iter().map(|t| t.0).collect();
        Self(quote!("(@(@parts) & +)"))
    }

    pub fn tuple<'t, I: IntoIterator<Item = &'t Self>>(elements: I) -> Self {
        let parts: Vec<Code> = elements.into_iter().map(|t| t.0.clone()).collect();
        Self(quote!("[@(@parts), +]"))
    }

    pub fn array(&self) -> Self {
        let inner = self.0.clone();
        Self(quote!("@inner[]"))
    }
}

impl From<&JsonType> for TypeExpr {
    fn from(value: &JsonType) -> Self {
        match value {
            JsonType::Number => Self::number(),
            JsonType::Boolean => Self::boolean(),
            JsonType::String => Self::string(),
            JsonType::Null => Self::null(),
            JsonType::Object => Self::any(),
            JsonType::Array => Self::any().array(),
            JsonType::Any => Self::any(),
        }
    }
}

impl ToCode for TypeExpr {
    fn to_code(&self) -> Code {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct BundleCtx<'cx> {
    pub cfg: &'cx Config,
    pub unit: &'cx ir::Unit,
    pub bundle: &'cx ir::Bundle,
}

#[derive(Clone)]
pub struct SchemaCtx<'cx> {
    pub bundle_ctx: BundleCtx<'cx>,
    pub schema: &'cx ir::Schema,
}

impl<'cx> SchemaCtx<'cx> {
    pub fn fully_qualified_type_name(&self, instance: &ir::InstanceType) -> String {
        let bundle = &self.bundle_ctx.unit[instance.bundle];
        let schema = &bundle[instance.schema];
        let def = &schema[instance.def];

        format!(
            "::{}::{}::{}",
            bundle.metadata.name,
            schema.name,
            def.name.as_str()
        )
    }

    pub fn resolve_type(&self, def: &ir::Def, typ: &ir::Type) -> TypeExpr {
        match &typ.kind {
            ir::TypeKind::TypeVar(var) => {
                let name = def[var.idx].name.as_str().to_owned();
                TypeExpr(Code::from(name))
            }
            ir::TypeKind::Instance(instance) => {
                let bundle = &self.bundle_ctx.unit[instance.bundle];
                let schema = &bundle[instance.schema];
                let instance_def = &schema[instance.def];

                let qualified_path = format!(
                    "::{}::{}::{}",
                    bundle.metadata.name,
                    schema.name,
                    instance_def.name.as_str()
                );

                let base = if let Some(path) = self.bundle_ctx.cfg.types.table.get(&qualified_path) {
                    Code::from(path.as_str())
                } else if instance.bundle == self.bundle_ctx.bundle.idx {
                    if instance.schema == self.schema.idx {
                        Code::from(instance_def.name.as_str())
                    } else {
                        Code::from(format!("__schema_{}.{}", schema.name, instance_def.name.as_str()))
                    }
                } else {
                    Code::from(format!(
                        "__bundle_{}.{}.{}",
                        bundle.metadata.name,
                        schema.name,
                        instance_def.name.as_str()
                    ))
                };

                if instance_def.vars.is_empty() {
                    TypeExpr(base)
                } else {
                    let subst: Vec<Code> = instance
                        .subst
                        .iter()
                        .map(|t| self.resolve_type(def, t).0)
                        .collect();
                    TypeExpr(quote!("@base<@(@subst), +>"))
                }
            }
        }
    }
}
