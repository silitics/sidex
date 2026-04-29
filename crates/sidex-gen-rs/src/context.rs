//! Code generation context.

use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;
use serde::Deserialize;
use serde::de::IntoDeserializer;
use sidex_attrs_rust::Visibility;
use sidex_attrs_rust::field_attrs as rust_field_attrs;
use sidex_gen::ir;

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
    pub unit: &'cx ir::Ir,
    pub bundle_idx: ir::BundleIdx,
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
    pub schema_idx: ir::SchemaIdx,
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
        let attrs = rust_field_attrs(field).map_err(|_| ()).unwrap();

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
        let attrs = rust_field_attrs(field).map_err(|_| ()).unwrap();

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
        let unit = self.bundle_ctx.unit;
        let bundle = &unit[instance.def.bundle];
        let schema = &unit[instance.def.schema];
        let def = &unit[instance.def];

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

    /// Resolves a Sidex type into the [`SerializeAs`]/[`DeserializeAs`]
    /// encoding that pairs with the Rust type returned by
    /// [`Self::resolve_type_old`].
    ///
    /// The encoding mirrors the structure of the Rust type but substitutes
    /// a Sidex-specific wire form (e.g. `AsU64`) at the leaves where the
    /// default Serde encoding doesn't roundtrip cleanly through JavaScript
    /// JSON. Container types like `Vec`, `Option`, and `HashMap` carry the
    /// encoding of their element type.
    ///
    /// [`SerializeAs`]: sidex_serde::SerializeAs
    /// [`DeserializeAs`]: sidex_serde::DeserializeAs
    /// Returns the concrete Rust path the user has configured for a
    /// container builtin (e.g. `indexmap::IndexMap` for `::core::builtins::Map`),
    /// falling back to `default_path` (e.g. `::std::collections::HashMap`)
    /// when no override is set.
    fn container_path(&self, qualified_path: &str, default_path: &str) -> TokenStream {
        let path = self
            .bundle_ctx
            .cfg
            .types
            .table
            .get(qualified_path)
            .map(String::as_str)
            .unwrap_or(default_path);
        syn::parse_str::<syn::TypePath>(path)
            .unwrap()
            .to_token_stream()
    }

    pub fn resolve_encoding(&self, def: &ir::Def, typ: &ir::Type) -> TokenStream {
        let resolved = self.bundle_ctx.unit.resolve_aliases(typ);
        match &resolved.kind {
            ir::TypeKind::TypeVar(var) => {
                let name = format_ident!("{}", def[var.idx].name.as_str());
                quote! { <#name as __sidex_serde::SidexType>::Encoding }
            }
            ir::TypeKind::Instance(instance) => {
                let unit = self.bundle_ctx.unit;
                let bundle = &unit[instance.def.bundle];
                let schema = &unit[instance.def.schema];
                let instance_def = &unit[instance.def];
                let qualified_path = format!(
                    "::{}::{}::{}",
                    bundle.metadata.name,
                    schema.name,
                    instance_def.name.as_str()
                );

                match qualified_path.as_str() {
                    "::core::builtins::i64" => return quote! { __sidex_serde::AsI64 },
                    "::core::builtins::u64" => return quote! { __sidex_serde::AsU64 },
                    "::core::builtins::f32" => return quote! { __sidex_serde::AsF32 },
                    "::core::builtins::f64" => return quote! { __sidex_serde::AsF64 },
                    "::core::builtins::bytes" => return quote! { __sidex_serde::AsBytes },
                    // For `Sequence` and `Map` we reuse the same concrete
                    // container the user configured for the value type
                    // (e.g. `indexmap::IndexMap` instead of `HashMap`) — the
                    // encoding only flips the leaf type parameters to their
                    // `SidexType::Encoding` equivalents. `sidex-serde` ships
                    // `SerializeAs`/`DeserializeAs`/`SidexType` impls for
                    // every container path it knows about.
                    "::core::builtins::Sequence" => {
                        let container = self.container_path(&qualified_path, "::std::vec::Vec");
                        let inner = self.resolve_encoding(def, &instance.subst[0]);
                        return quote! { #container<#inner> };
                    }
                    "::core::builtins::Map" => {
                        let container =
                            self.container_path(&qualified_path, "::std::collections::HashMap");
                        let key = self.resolve_encoding(def, &instance.subst[0]);
                        let value = self.resolve_encoding(def, &instance.subst[1]);
                        return quote! { #container<#key, #value> };
                    }
                    _ => {}
                }

                quote! { __sidex_serde::AsSelf }
            }
        }
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
                let unit = self.bundle_ctx.unit;
                let bundle = &unit[instance.def.bundle];
                let schema = &unit[instance.def.schema];
                let instance_def = &unit[instance.def];

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
                    if instance.def.bundle == self.bundle_ctx.bundle_idx {
                        let def_name = format_ident!("{}", &instance_def.name.as_str());
                        let prefix = if extra_super {
                            quote! { super::}
                        } else {
                            quote! {}
                        };
                        if instance.def.schema == self.schema_idx {
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

                // In plain-JSON lowering, the key type of `Map<K, V>` is
                // forced to `String` since JSON object keys are always strings —
                // the original key type may lower to something that doesn't
                // implement `Eq + Hash` (e.g. `serde_json::Value`).
                let force_string_keyed_map = matches!(
                    self.bundle_ctx.cfg.opaque_lowering,
                    crate::config::OpaqueLowering::Json,
                ) && qualified_path == "::core::builtins::Map";

                let subst = instance
                    .subst
                    .iter()
                    .enumerate()
                    .map(|(idx, t)| {
                        if force_string_keyed_map && idx == 0 {
                            quote! { ::std::string::String }
                        } else {
                            self.resolve_type_old(def, t, extra_super)
                        }
                    })
                    .collect::<Vec<_>>();

                quote! { #typ < #(#subst , )* > }
            }
        }
    }
}
