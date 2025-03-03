//! Code generation context.

use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use serde::{Deserialize, de::IntoDeserializer};
use sidex_attrs_rust::{FieldAttrs, Visibility};
use sidex_gen::{attrs::TryFromAttrs, ir};

use crate::config::Config;

pub struct RustField {
    pub ident: syn::Ident,
    pub name: String,
    pub ty: syn::Type,
    pub vis: syn::Visibility,
    pub is_optional: bool,
    pub inner_ty: syn::Type,
}

pub struct RustVariant {
    pub ident: syn::Ident,
    pub name: String,
    pub ty: Option<syn::Type>,
}

pub struct RustTy {
    pub ident: syn::Ident,
    pub name: String,
}

pub struct Generics {
    generics: syn::Generics,
}

impl Generics {
    pub fn new(def: &ir::Def) -> Self {
        let mut generics = syn::Generics::default();
        for var in &def.vars {
            let ident = format_ident!("{}", var.name.as_str());
            let param = syn::GenericParam::Type(ident.into());
            generics.params.push(param);
        }
        Self { generics }
    }

    pub fn split_for_impl(
        &self,
    ) -> (
        syn::ImplGenerics,
        syn::TypeGenerics,
        Option<&syn::WhereClause>,
    ) {
        self.generics.split_for_impl()
    }
}

#[derive(Clone)]
pub struct BundleCtx<'cx> {
    pub cfg: &'cx Config,
    pub unit: &'cx ir::Unit,
    pub bundle: &'cx ir::Bundle,
}

impl<'cx> BundleCtx<'cx> {
    pub fn get_plugin_config<T: Default + for<'de> Deserialize<'de>>(&self, name: &str) -> T {
        self.cfg
            .plugin
            .get(name)
            .map(|plugin_cfg| T::deserialize(plugin_cfg.clone().into_deserializer()).unwrap())
            .unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct SchemaCtx<'cx> {
    pub bundle_ctx: BundleCtx<'cx>,
    pub schema: &'cx ir::Schema,
}

pub struct TypeInfo {
    pub ident: syn::Ident,
    pub vis: syn::Visibility,
    pub generics: TokenStream,
}

pub struct FieldInfo {
    pub name: syn::Ident,
    pub typ: syn::Type,
    pub vis: Visibility,
}

impl<'cx> SchemaCtx<'cx> {
    pub fn ty(&self, def: &ir::Def) -> RustTy {
        RustTy {
            ident: format_ident!("{}", def.name.as_str()),
            name: def.name.as_str().to_owned(),
        }
    }

    pub fn field(&self, def: &ir::Def, field: &ir::Field) -> RustField {
        let attrs = FieldAttrs::try_from_attrs(&field.attrs)
            .map_err(|_| ())
            .unwrap();

        let ident = format_ident!(
            "{}",
            attrs.name.as_deref().unwrap_or_else(|| field.name.as_str())
        );
        let name = field.name.as_str().to_owned();
        let mut inner_ty = self.resolve_type(def, &field.typ);

        for wrapper in attrs.wrappers {
            let wrapper = TokenStream::from_str(&wrapper.wrapper).unwrap();
            inner_ty = syn::parse2(quote! { #wrapper < #inner_ty > }).unwrap()
        }

        let ty = if field.is_optional {
            syn::parse2::<syn::Type>(quote! { ::core::option::Option< #inner_ty > }).unwrap()
        } else {
            inner_ty.clone()
        };

        let vis = syn::parse2(attrs.visibility.to_token_stream()).unwrap();

        RustField {
            ident,
            name,
            ty,
            vis,
            is_optional: field.is_optional,
            inner_ty,
        }
    }

    pub fn variant(&self, def: &ir::Def, variant: &ir::Variant) -> RustVariant {
        let ident = format_ident!("{}", variant.name.as_str());
        let name = variant.name.as_str().to_owned();
        let ty = variant.typ.as_ref().map(|typ| self.resolve_type(def, typ));
        RustVariant { ident, name, ty }
    }

    pub fn field_info(&self, def: &ir::Def, field: &ir::Field) -> FieldInfo {
        let attrs = FieldAttrs::try_from_attrs(&field.attrs)
            .map_err(|_| ())
            .unwrap();

        let name = format_ident!(
            "{}",
            attrs.name.as_deref().unwrap_or_else(|| field.name.as_str())
        );
        let mut typ = self.resolve_type_old(def, &field.typ, false);
        for wrapper in attrs.wrappers {
            let wrapper = TokenStream::from_str(&wrapper.wrapper).unwrap();
            typ = quote! { #wrapper < #typ > }
        }
        if field.is_optional {
            typ = quote! { ::std::option::Option< #typ > };
        }
        let vis = attrs.visibility.clone();
        FieldInfo {
            name,
            typ: syn::parse2::<syn::Type>(typ).unwrap(),
            vis,
        }
    }

    pub fn generics(&self, def: &ir::Def) -> Generics {
        Generics::new(def)
    }

    pub fn type_info(&self, def: &ir::Def) -> TypeInfo {
        let ident = format_ident!("{}", def.name.as_str());
        let vars = self.generic_type_vars(def);
        let generics = quote! { < #vars > };
        let vis = syn::parse_str::<syn::Visibility>("pub").unwrap();
        TypeInfo {
            ident,
            vis,
            generics,
        }
    }

    pub fn resolve_type_var(&self, def: &ir::Def, var: &ir::TypeVarType) -> syn::Ident {
        format_ident!("{}", def[var.idx].name.as_str())
    }

    pub fn generic_type_vars(&self, def: &ir::Def) -> TokenStream {
        if def.vars.is_empty() {
            quote! {}
        } else {
            let vars = def
                .vars
                .iter()
                .map(|var| format_ident!("{}", var.name.as_str()));

            quote! { #(#vars , )* }
        }
    }

    pub fn generic_type_vars_with_bounds(&self, def: &ir::Def, bounds: TokenStream) -> TokenStream {
        if def.vars.is_empty() {
            quote! {}
        } else {
            let vars = def
                .vars
                .iter()
                .map(|var| format_ident!("{}", var.name.as_str()));

            quote! { #(#vars: #bounds , )* }
        }
    }

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

    pub fn resolve_type(&self, def: &ir::Def, typ: &ir::Type) -> syn::Type {
        syn::parse2(self.resolve_type_old(def, typ, false)).unwrap()
    }

    pub fn resolve_type_old(
        &self,
        def: &ir::Def,
        typ: &ir::Type,
        extra_super: bool,
    ) -> TokenStream {
        // println!("Resolving type {:?}", typ);
        match &typ.kind {
            ir::TypeKind::TypeVar(var) => {
                let var = format_ident!("{}", def[var.idx].name.as_str());
                quote! { #var }
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
                    let rust_path = syn::parse_str::<syn::TypePath>(path).unwrap();
                    rust_path.to_token_stream()
                } else {
                    if instance.bundle == self.bundle_ctx.bundle.idx {
                        let def_name = format_ident!("{}", &instance_def.name.as_str());
                        let prefix = if extra_super {
                            quote! { super::}
                        } else {
                            quote! {}
                        };
                        if instance.schema == self.schema.idx {
                            quote! { #prefix #def_name }
                        } else {
                            let schema_name = format_ident!("{}", &schema.name);
                            quote! { #prefix super::#schema_name::#def_name }
                        }
                    } else {
                        let external_path = self
                            .bundle_ctx
                            .cfg
                            .external
                            .get(&bundle.metadata.name)
                            .unwrap();
                        let parsed = syn::parse_str::<TokenStream>(&external_path).unwrap();
                        let schema_name = format_ident!("{}", &schema.name);
                        let def_name = format_ident!("{}", &instance_def.name.as_str());
                        quote! { #parsed::#schema_name::#def_name }
                    }
                };

                let subst = instance
                    .subst
                    .iter()
                    .map(|typ| self.resolve_type_old(def, typ, extra_super))
                    .collect::<Vec<_>>();

                quote! { #typ < #(#subst , )* > }
            }
        }
    }
}
