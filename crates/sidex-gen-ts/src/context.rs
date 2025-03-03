use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use sidex_attrs_json::types::JsonType;
use sidex_gen::ir;

use crate::config::Config;

/// A TypeScript type expression.
pub struct TypeExpr(pub(crate) TokenStream);

impl TypeExpr {
    pub fn any() -> Self {
        Self(quote! { any })
    }

    pub fn number() -> Self {
        Self(quote! { number })
    }

    pub fn string() -> Self {
        Self(quote! { string })
    }

    pub fn boolean() -> Self {
        Self(quote! { boolean })
    }

    pub fn null() -> Self {
        Self(quote! { null })
    }

    pub fn string_literal(literal: &str) -> Self {
        Self(quote! { #literal })
    }

    pub fn number_literal(literal: f64) -> Self {
        Self(quote! { #literal })
    }

    pub fn union<'t, I: IntoIterator<Item = Self>>(types: I) -> Self {
        let types = types.into_iter();
        Self(quote! { ( #(#types)|* ) })
    }

    pub fn intersection<'t, I: IntoIterator<Item = Self>>(types: I) -> Self {
        let types = types.into_iter();
        Self(quote! { ( #(#types)&* ) })
    }

    pub fn tuple<'t, I: IntoIterator<Item = &'t Self>>(elements: I) -> Self {
        let elements = elements.into_iter();
        Self(quote! { [ #(#elements),* ] })
    }

    pub fn array(&self) -> Self {
        let inner = &self.0;
        Self(quote! { #inner [] })
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

impl ToTokens for TypeExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
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
                let var = format_ident!("{}", def[var.idx].name.as_str());
                TypeExpr(quote! { #var })
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

                let typ = if let Some(path) = self.bundle_ctx.cfg.types.table.get(&qualified_path) {
                    path.parse().unwrap()
                } else {
                    if instance.bundle == self.bundle_ctx.bundle.idx {
                        let def_name = format_ident!("{}", &instance_def.name.as_str());
                        if instance.schema == self.schema.idx {
                            quote! { #def_name }
                        } else {
                            let schema_name = format_ident!("__schema_{}", &schema.name);
                            quote! { #schema_name . #def_name }
                        }
                    } else {
                        let bundle_name = format_ident!("__bundle_{}", &bundle.metadata.name);
                        let schema_name = format_ident!("{}", schema.name);
                        let def_name = format_ident!("{}", &instance_def.name.as_str());

                        quote! { #bundle_name . #schema_name . #def_name }
                    }
                };

                if instance_def.vars.is_empty() {
                    TypeExpr(typ)
                } else {
                    let subst = instance
                        .subst
                        .iter()
                        .map(|typ| self.resolve_type(def, typ))
                        .collect::<Vec<_>>();

                    TypeExpr(quote! { #typ < #(#subst),* > })
                }
            }
        }
    }
}
