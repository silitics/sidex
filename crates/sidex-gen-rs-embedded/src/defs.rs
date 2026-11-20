//! Generation of the Rust types of definitions and their implementations of
//! `Decode`, `Encode`, and `MapKey`.

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use sidex_attrs_api::Stability;
use sidex_attrs_api::render_doc_prelude;
use sidex_attrs_api::render_group_prelude;
use sidex_attrs_api::stability_of_def;
use sidex_attrs_api::stability_of_field;
use sidex_attrs_api::stability_of_variant;
use sidex_attrs_json::JsonTaggedAttr;
use sidex_attrs_json::field_attrs as json_field_attrs;
use sidex_attrs_json::record_type_attrs;
use sidex_attrs_json::variant_attrs as json_variant_attrs;
use sidex_attrs_json::variant_type_attrs;
use sidex_attrs_rust::field_attrs as rust_field_attrs;
use sidex_gen::diagnostics::Diagnostic;
use sidex_gen::diagnostics::Result;
use sidex_gen::ir;

use crate::context::Ctx;
use crate::context::OpaqueRepr;
use crate::context::runtime;
use crate::context::rust_ident;

/// Generates the Rust type of a definition and its implementations.
pub(crate) fn generate(ctx: &Ctx<'_>, def_ref: ir::DefRef) -> Result<TokenStream> {
    let def = &ctx.unit[def_ref];
    let keyed = ctx.keyed_vars(def_ref.def);
    let generator = DefGenerator {
        ctx,
        def_ref,
        def,
        ident: rust_ident(def.name.as_str())?,
        borrows: ctx.def_borrows(def_ref),
        vars: def
            .vars
            .iter()
            .enumerate()
            .map(|(index, var)| Ok((rust_ident(var.name.as_str())?, keyed.contains(&index))))
            .collect::<Result<_>>()?,
    };
    match &def.kind {
        ir::DefKind::RecordType(record) => generator.record(record),
        ir::DefKind::VariantType(variant) => generator.variant(variant),
        ir::DefKind::WrapperType(wrapper) => generator.wrapper(wrapper),
        ir::DefKind::TypeAlias(alias) => generator.alias(alias),
        ir::DefKind::OpaqueType(_) => generator.opaque(),
    }
}

/// Generator for one definition.
struct DefGenerator<'g, 'cx> {
    ctx: &'g Ctx<'cx>,
    def_ref: ir::DefRef,
    def: &'g ir::Def,
    ident: Ident,
    /// Indicates whether the Rust type takes the lifetime `'a`.
    borrows: bool,
    /// Type variables and whether they are used as keys of maps.
    vars: Vec<(Ident, bool)>,
}

/// Field of a record.
struct Field {
    ident: Ident,
    docs: TokenStream,
    deprecated: TokenStream,
    ty: TokenStream,
    optional: bool,
    /// JSON name followed by its aliases.
    json_names: Vec<String>,
    inline: bool,
}

/// Case of a variant.
struct Case {
    ident: Ident,
    docs: TokenStream,
    deprecated: TokenStream,
    payload: Option<TokenStream>,
    json_name: String,
    content: Option<String>,
    fallback: bool,
}

impl DefGenerator<'_, '_> {
    fn record(&self, record: &ir::RecordTypeDef) -> Result<TokenStream> {
        let rt = runtime();
        let record_attrs = record_type_attrs(self.def)?;
        let fields = record
            .fields
            .iter()
            .map(|field| {
                let rust_attrs = rust_field_attrs(field)?;
                if !rust_attrs.wrappers.is_empty() {
                    return Err(Box::new(Diagnostic::error(format!(
                        "Field `{}` of `{}` uses `#[rust(box)]`, `#[rust(arc)]`, `#[rust(rc)]`, \
                         or `#[rust(wrap)]`, which the `rust-embedded` backend does not support.",
                        field.name.as_str(),
                        self.def.name.as_str()
                    ))));
                }
                let json_attrs = json_field_attrs(field)?;
                let stability = stability_of_field(field);
                Ok(Field {
                    ident: rust_ident(rust_attrs.name.as_deref().unwrap_or(field.name.as_str()))?,
                    docs: docs(&stability, "", field.docs.as_ref()),
                    deprecated: deprecated(&stability),
                    ty: self.resolve(&field.typ)?,
                    optional: field.is_optional,
                    json_names: std::iter::once(record_attrs.field_name(field, &json_attrs))
                        .chain(json_attrs.aliases)
                        .collect(),
                    inline: json_attrs.inline,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let field_definitions = fields.iter().map(|field| {
            let Field {
                ident,
                docs,
                deprecated,
                ty,
                ..
            } = field;
            let ty = if field.optional {
                quote! { ::core::option::Option<#ty> }
            } else {
                quote! { #ty }
            };
            quote! {
                #docs
                #deprecated
                pub #ident: #ty,
            }
        });

        let mut inline_decodes = Vec::new();
        let mut names = Vec::new();
        let mut construct = Vec::new();
        let mut encode_fields = Vec::new();
        for field in &fields {
            let ident = &field.ident;
            let ty = &field.ty;
            if field.inline {
                let var = format_ident!("__inline_{}", inline_decodes.len());
                inline_decodes.push(quote! {
                    let #var = <#ty as #rt::Decode<'a>>::decode(&mut reader.clone())?;
                });
                construct.push(quote! { #ident: #var, });
                encode_fields.push(quote! {
                    #rt::Encode::encode_fields(&self.#ident, object)?;
                });
                continue;
            }
            let index = names.len();
            let json_names = &field.json_names;
            names.push(quote! { &[#(#json_names),*] });
            let json_name = &field.json_names[0];
            if field.optional {
                construct.push(quote! { #ident: fields.optional(#index)?, });
                encode_fields.push(quote! {
                    if let ::core::option::Option::Some(value) = &self.#ident {
                        #rt::Encode::encode(value, object.field(#json_name)?)?;
                    }
                });
            } else {
                construct.push(quote! { #ident: fields.required(#index)?, });
                encode_fields.push(quote! {
                    #rt::Encode::encode(&self.#ident, object.field(#json_name)?)?;
                });
            }
        }

        let ident = &self.ident;
        let meta = self.meta()?;
        let decl_generics = self.decl_generics();
        let self_ty = self.self_ty();
        let decode_generics = self.decode_generics();
        let encode_generics = self.encode_generics();
        Ok(quote! {
            #meta
            pub struct #ident #decl_generics {
                #(#field_definitions)*
            }

            #[automatically_derived]
            impl #decode_generics #rt::Decode<'a> for #self_ty {
                const IS_RECORD: bool = true;

                fn decode(
                    reader: &mut #rt::json::Reader<'a>,
                ) -> ::core::result::Result<Self, #rt::DecodeError> {
                    #(#inline_decodes)*
                    let fields = reader.record(&[#(#names),*])?;
                    ::core::result::Result::Ok(Self { #(#construct)* })
                }
            }

            #[automatically_derived]
            impl #encode_generics #rt::Encode for #self_ty {
                fn encode(
                    &self,
                    writer: &mut #rt::json::Writer<'_>,
                ) -> ::core::result::Result<(), #rt::EncodeError> {
                    let mut object = writer.object()?;
                    #rt::Encode::encode_fields(self, &mut object)?;
                    object.end()
                }

                fn is_record(&self) -> bool {
                    true
                }

                fn encode_fields(
                    &self,
                    object: &mut #rt::json::ObjectWriter<'_, '_>,
                ) -> ::core::result::Result<(), #rt::EncodeError> {
                    #(#encode_fields)*
                    ::core::result::Result::Ok(())
                }
            }
        })
    }

    fn variant(&self, variant: &ir::VariantTypeDef) -> Result<TokenStream> {
        let rt = runtime();
        let variant_attrs = variant_type_attrs(self.def)?;
        let cases = variant
            .variants
            .iter()
            .map(|case| {
                let json_attrs = json_variant_attrs(case)?;
                let stability = stability_of_variant(case);
                Ok(Case {
                    ident: rust_ident(case.name.as_str())?,
                    docs: docs(&stability, "", case.docs.as_ref()),
                    deprecated: deprecated(&stability),
                    payload: case.typ.as_ref().map(|typ| self.resolve(typ)).transpose()?,
                    json_name: variant_attrs.variant_name(case, &json_attrs),
                    content: json_attrs.content,
                    fallback: json_attrs.fallback,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let case_definitions = cases.iter().map(|case| {
            let Case {
                ident,
                docs,
                deprecated,
                ..
            } = case;
            let payload = case.payload.as_ref().map(|ty| quote! { (#ty) });
            quote! {
                #docs
                #deprecated
                #ident #payload,
            }
        });

        let tag = &variant_attrs.tag;
        let (fallback, known): (Vec<_>, Vec<_>) = cases.iter().partition(|case| case.fallback);
        let fallback = fallback.first().copied();
        let decode = match variant_attrs.tagged {
            JsonTaggedAttr::Internally | JsonTaggedAttr::Adjacently => {
                decode_tagged(
                    &known,
                    fallback,
                    variant_attrs.tagged,
                    tag,
                    &variant_attrs.content,
                )
            }
            JsonTaggedAttr::Externally => decode_external(&known, fallback),
            JsonTaggedAttr::Implicitly => decode_implicit(&known, fallback),
        };
        let encode_arms = cases.iter().map(|case| {
            let ident = &case.ident;
            let name = &case.json_name;
            if case.payload.is_none() {
                let body = match variant_attrs.tagged {
                    JsonTaggedAttr::Internally | JsonTaggedAttr::Adjacently => {
                        quote! {
                            let mut object = writer.object()?;
                            object.field(#tag)?.str(#name)?;
                            object.end()
                        }
                    }
                    JsonTaggedAttr::Externally | JsonTaggedAttr::Implicitly => {
                        quote! { writer.str(#name) }
                    }
                };
                return quote! { Self::#ident => { #body } };
            }
            let content = case.content.as_ref().unwrap_or(&variant_attrs.content);
            let with_content = quote! {
                let mut object = writer.object()?;
                object.field(#tag)?.str(#name)?;
                #rt::Encode::encode(payload, object.field(#content)?)?;
                object.end()
            };
            let body = match variant_attrs.tagged {
                _ if case.fallback => quote! { #rt::Encode::encode(payload, writer) },
                JsonTaggedAttr::Internally if case.content.is_none() => {
                    quote! {
                        if #rt::Encode::is_record(payload) {
                            let mut object = writer.object()?;
                            object.field(#tag)?.str(#name)?;
                            #rt::Encode::encode_fields(payload, &mut object)?;
                            object.end()
                        } else {
                            #with_content
                        }
                    }
                }
                JsonTaggedAttr::Internally | JsonTaggedAttr::Adjacently => with_content,
                JsonTaggedAttr::Externally => {
                    quote! {
                        let mut object = writer.object()?;
                        #rt::Encode::encode(payload, object.field(#name)?)?;
                        object.end()
                    }
                }
                JsonTaggedAttr::Implicitly => quote! { #rt::Encode::encode(payload, writer) },
            };
            quote! { Self::#ident(payload) => { #body } }
        });

        // Empty enums have no values, so their encoding matches on the value.
        let scrutinee = if cases.is_empty() {
            quote! { *self }
        } else {
            quote! { self }
        };
        let ident = &self.ident;
        let meta = self.meta()?;
        let decl_generics = self.decl_generics();
        let self_ty = self.self_ty();
        let decode_generics = self.decode_generics();
        let encode_generics = self.encode_generics();
        Ok(quote! {
            #meta
            pub enum #ident #decl_generics {
                #(#case_definitions)*
            }

            #[automatically_derived]
            impl #decode_generics #rt::Decode<'a> for #self_ty {
                fn decode(
                    reader: &mut #rt::json::Reader<'a>,
                ) -> ::core::result::Result<Self, #rt::DecodeError> {
                    #decode
                }
            }

            #[automatically_derived]
            impl #encode_generics #rt::Encode for #self_ty {
                fn encode(
                    &self,
                    writer: &mut #rt::json::Writer<'_>,
                ) -> ::core::result::Result<(), #rt::EncodeError> {
                    match #scrutinee {
                        #(#encode_arms)*
                    }
                }
            }
        })
    }

    fn wrapper(&self, wrapper: &ir::WrapperTypeDef) -> Result<TokenStream> {
        let wrapped = self.resolve(&wrapper.wrapped)?;
        let map_key = self
            .ctx
            .implements_map_key(&wrapper.wrapped)?
            .then(|| self.map_key(&wrapped));
        self.newtype(&wrapped, map_key)
    }

    fn alias(&self, alias: &ir::TypeAliasDef) -> Result<TokenStream> {
        Ok(self.type_alias(&self.resolve(&alias.aliased)?))
    }

    fn opaque(&self) -> Result<TokenStream> {
        if let Some(rust_type) = self.ctx.parse_configured_type(self.def_ref)? {
            return Ok(self.type_alias(&quote! { #rust_type }));
        }
        if !self.vars.is_empty() {
            return Err(Box::new(
                Diagnostic::error(format!(
                    "Generic opaque type `{}` needs a Rust type.",
                    self.ctx.path(self.def_ref)
                ))
                .with_help(
                    "Add the type to the `types.table` setting of the `rust-embedded` backend.",
                ),
            ));
        }
        let repr = OpaqueRepr::of(self.def)?;
        let inner = repr.rust_type();
        let map_key = (repr == OpaqueRepr::Str).then(|| self.map_key(&inner));
        self.newtype(&inner, map_key)
    }

    /// Generates a type alias for the given type.
    fn type_alias(&self, target: &TokenStream) -> TokenStream {
        let docs = self.docs();
        let ident = &self.ident;
        let generics = self.type_args();
        quote! {
            #docs
            pub type #ident #generics = #target;
        }
    }

    /// Generates the implementation of `MapKey` for a newtype of `inner`.
    fn map_key(&self, inner: &TokenStream) -> TokenStream {
        let rt = runtime();
        let key_generics = self.key_generics();
        let self_ty = self.self_ty();
        quote! {
            #[automatically_derived]
            impl #key_generics #rt::MapKey<'a> for #self_ty {
                fn decode_key(
                    key: #rt::types::Str<'a>,
                ) -> ::core::result::Result<Self, #rt::DecodeErrorKind> {
                    <#inner as #rt::MapKey<'a>>::decode_key(key).map(Self)
                }

                fn encode_key(
                    &self,
                    writer: &mut #rt::json::Writer<'_>,
                ) -> ::core::result::Result<(), #rt::EncodeError> {
                    #rt::MapKey::encode_key(&self.0, writer)
                }
            }
        }
    }

    /// Generates a tuple struct wrapping a type with the same representation.
    fn newtype(&self, inner: &TokenStream, map_key: Option<TokenStream>) -> Result<TokenStream> {
        let rt = runtime();
        let ident = &self.ident;
        let meta = self.meta()?;
        let decl_generics = self.decl_generics();
        let self_ty = self.self_ty();
        let decode_generics = self.decode_generics();
        let encode_generics = self.encode_generics();
        Ok(quote! {
            #meta
            pub struct #ident #decl_generics(pub #inner);

            #[automatically_derived]
            impl #decode_generics #rt::Decode<'a> for #self_ty {
                fn decode(
                    reader: &mut #rt::json::Reader<'a>,
                ) -> ::core::result::Result<Self, #rt::DecodeError> {
                    <#inner as #rt::Decode<'a>>::decode(reader).map(Self)
                }
            }

            #[automatically_derived]
            impl #encode_generics #rt::Encode for #self_ty {
                fn encode(
                    &self,
                    writer: &mut #rt::json::Writer<'_>,
                ) -> ::core::result::Result<(), #rt::EncodeError> {
                    #rt::Encode::encode(&self.0, writer)
                }

                fn encode_fields(
                    &self,
                    object: &mut #rt::json::ObjectWriter<'_, '_>,
                ) -> ::core::result::Result<(), #rt::EncodeError> {
                    #rt::Encode::encode_fields(&self.0, object)
                }
            }

            #map_key
        })
    }

    fn resolve(&self, typ: &ir::Type) -> Result<TokenStream> {
        self.ctx.resolve(self.def.schema, self.def, typ)
    }

    /// Returns the documentation attribute of the definition.
    fn docs(&self) -> TokenStream {
        docs(
            &stability_of_def(self.def),
            &render_group_prelude(self.ctx.unit, self.def),
            self.def.docs.as_ref(),
        )
    }

    /// Returns the documentation, derives, and deprecation of the definition.
    fn meta(&self) -> Result<TokenStream> {
        let docs = self.docs();
        let mut derives = vec![quote! { Clone }, quote! { Copy }];
        for derive in &self.ctx.cfg.derive {
            if derive == "Clone" || derive == "Copy" {
                continue;
            }
            let path = syn::parse_str::<syn::Path>(derive).map_err(|error| {
                Diagnostic::error(format!("Invalid trait `{derive}` to derive: {error}."))
            })?;
            derives.push(quote! { #path });
        }
        let deprecated = deprecated(&stability_of_def(self.def));
        Ok(quote! {
            #docs
            #[derive(#(#derives),*)]
            #deprecated
        })
    }

    /// Generic parameters of the type's definition.
    ///
    /// Types that borrow require their type variables to be decodable, so that
    /// derived implementations get the bounds that sequences and maps need.
    fn decl_generics(&self) -> TokenStream {
        if self.borrows {
            let bounds = self
                .vars
                .iter()
                .map(|(var, keyed)| self.base_bounds(var, *keyed));
            quote! { <'a, #(#bounds),*> }
        } else {
            self.type_args()
        }
    }

    /// Generic arguments of the type.
    fn type_args(&self) -> TokenStream {
        let vars = self.vars.iter().map(|(var, _)| var);
        if self.borrows {
            quote! { <'a, #(#vars),*> }
        } else if self.vars.is_empty() {
            quote! {}
        } else {
            quote! { <#(#vars),*> }
        }
    }

    fn self_ty(&self) -> TokenStream {
        let ident = &self.ident;
        let args = self.type_args();
        quote! { #ident #args }
    }

    fn decode_generics(&self) -> TokenStream {
        let rt = runtime();
        let bounds = self.vars.iter().map(|(var, keyed)| {
            if self.borrows {
                self.base_bounds(var, *keyed)
            } else {
                quote! { #var: #rt::Decode<'a> }
            }
        });
        quote! { <'a, #(#bounds),*> }
    }

    fn encode_generics(&self) -> TokenStream {
        let rt = runtime();
        if self.borrows {
            let bounds = self.vars.iter().map(|(var, keyed)| {
                let base = self.base_bounds(var, *keyed);
                quote! { #base + #rt::Encode }
            });
            quote! { <'a, #(#bounds),*> }
        } else if self.vars.is_empty() {
            quote! {}
        } else {
            let bounds = self
                .vars
                .iter()
                .map(|(var, _)| quote! { #var: #rt::Encode });
            quote! { <#(#bounds),*> }
        }
    }

    fn key_generics(&self) -> TokenStream {
        let rt = runtime();
        let bounds = self.vars.iter().map(|(var, keyed)| {
            if self.borrows {
                let base = self.base_bounds(var, *keyed);
                quote! { #base + #rt::MapKey<'a> }
            } else {
                quote! { #var: #rt::MapKey<'a> }
            }
        });
        quote! { <'a, #(#bounds),*> }
    }

    /// Bounds of a type variable of a type that borrows.
    fn base_bounds(&self, var: &Ident, keyed: bool) -> TokenStream {
        let rt = runtime();
        let key = keyed.then(|| quote! { + #rt::MapKey<'a> });
        quote! { #var: #rt::Decode<'a> + ::core::clone::Clone #key }
    }
}

/// Generates the decoding of an internally or adjacently tagged variant with
/// the given known cases and fallback case.
fn decode_tagged(
    known: &[&Case],
    fallback: Option<&Case>,
    tagging: JsonTaggedAttr,
    tag: &str,
    content: &str,
) -> TokenStream {
    let rt = runtime();
    let names = known.iter().map(|case| &case.json_name);
    let arms = known.iter().enumerate().map(|(index, case)| {
        let ident = &case.ident;
        let Some(payload) = &case.payload else {
            return quote! { ::core::option::Option::Some(#index) => ::core::result::Result::Ok(Self::#ident), };
        };
        let content = case.content.as_deref().unwrap_or(content);
        let with_content = quote! {
            <#payload as #rt::Decode<'a>>::decode(&mut value.content(#content)?)?
        };
        let decode = if tagging == JsonTaggedAttr::Internally && case.content.is_none() {
            quote! {
                if <#payload as #rt::Decode<'a>>::IS_RECORD {
                    <#payload as #rt::Decode<'a>>::decode(&mut value.clone())?
                } else {
                    #with_content
                }
            }
        } else {
            with_content
        };
        quote! {
            ::core::option::Option::Some(#index) => ::core::result::Result::Ok(Self::#ident(#decode)),
        }
    });
    let (unknown, invalid) = unmatched(fallback);
    quote! {
        let value = reader.clone();
        let name = match reader.tag(#tag)? {
            ::core::result::Result::Ok(name) => name,
            #invalid
        };
        match #rt::json::case(&name, &[#(#names),*]) {
            #(#arms)*
            _ => #unknown,
        }
    }
}

/// Generates the decoding of an externally tagged variant with the given known
/// cases and fallback case.
fn decode_external(known: &[&Case], fallback: Option<&Case>) -> TokenStream {
    let rt = runtime();
    let names = known.iter().map(|case| &case.json_name);
    let arms = known.iter().enumerate().map(|(index, case)| {
        let ident = &case.ident;
        match &case.payload {
            Some(payload) => quote! {
                (::core::option::Option::Some(#index), ::core::option::Option::Some(mut payload)) => {
                    ::core::result::Result::Ok(Self::#ident(
                        <#payload as #rt::Decode<'a>>::decode(&mut payload)?,
                    ))
                }
                (::core::option::Option::Some(#index), ::core::option::Option::None) => {
                    ::core::result::Result::Err(value.error(#rt::DecodeErrorKind::Unexpected(
                        "a map holding the payload of the case",
                    )))
                }
            },
            None => quote! {
                (::core::option::Option::Some(#index), ::core::option::Option::None) => {
                    ::core::result::Result::Ok(Self::#ident)
                }
                (::core::option::Option::Some(#index), ::core::option::Option::Some(_)) => {
                    ::core::result::Result::Err(value.error(#rt::DecodeErrorKind::Unexpected(
                        "a string naming a case without payload",
                    )))
                }
            },
        }
    });
    let (unknown, invalid) = unmatched(fallback);
    quote! {
        let value = reader.clone();
        let (name, payload) = match reader.external()? {
            ::core::result::Result::Ok(#rt::json::External::Name(name)) => {
                (name, ::core::option::Option::None)
            }
            ::core::result::Result::Ok(#rt::json::External::Entry(name, payload)) => {
                (name, ::core::option::Option::Some(payload))
            }
            #invalid
        };
        match (#rt::json::case(&name, &[#(#names),*]), payload) {
            #(#arms)*
            _ => #unknown,
        }
    }
}

/// Generates the decoding of an implicitly tagged variant, which tries the
/// known cases in declaration order and the fallback case last.
fn decode_implicit(known: &[&Case], fallback: Option<&Case>) -> TokenStream {
    let rt = runtime();
    let attempts = known.iter().map(|case| {
        let ident = &case.ident;
        let name = &case.json_name;
        match &case.payload {
            Some(payload) => {
                quote! {
                    if let ::core::result::Result::Ok(payload) =
                        <#payload as #rt::Decode<'a>>::decode(&mut value.clone())
                    {
                        return ::core::result::Result::Ok(Self::#ident(payload));
                    }
                }
            }
            None => {
                quote! {
                    if value.clone().string().is_ok_and(|text| text == #name) {
                        return ::core::result::Result::Ok(Self::#ident);
                    }
                }
            }
        }
    });
    let otherwise = match fallback {
        Some(case) => decode_fallback(case),
        None => {
            quote! {
                ::core::result::Result::Err(value.error(#rt::DecodeErrorKind::NoMatchingCase))
            }
        }
    };
    quote! {
        let value = reader.clone();
        reader.skip()?;
        #(#attempts)*
        #otherwise
    }
}

/// Generates the results for values naming no case and for values that cannot
/// name a case, given as an error named `error`.
///
/// Both decode as the fallback case, if there is one.
fn unmatched(fallback: Option<&Case>) -> (TokenStream, TokenStream) {
    let rt = runtime();
    match fallback {
        Some(case) => {
            let fallback = decode_fallback(case);
            (
                fallback.clone(),
                quote! { ::core::result::Result::Err(_) => return #fallback, },
            )
        }
        None => {
            (
                quote! {
                    ::core::result::Result::Err(value.error(#rt::DecodeErrorKind::UnknownCase))
                },
                quote! {
                    ::core::result::Result::Err(error) => return ::core::result::Result::Err(error),
                },
            )
        }
    }
}

/// Generates the decoding of the entire value as the payload of a fallback
/// case.
fn decode_fallback(case: &Case) -> TokenStream {
    let rt = runtime();
    let ident = &case.ident;
    let payload = case
        .payload
        .as_ref()
        .expect("the JSON attribute check requires fallback cases to have payloads");
    quote! {
        ::core::result::Result::Ok(Self::#ident(
            <#payload as #rt::Decode<'a>>::decode(&mut value.clone())?,
        ))
    }
}

/// Returns the documentation attribute composed from the stability, group
/// membership, and docs, or nothing if there is no documentation.
fn docs(stability: &Stability, groups: &str, docs: Option<&ir::Docs>) -> TokenStream {
    let docs = format!(
        "{}{groups}{}",
        render_doc_prelude(stability),
        docs.map(|docs| docs.as_str()).unwrap_or_default()
    );
    if docs.is_empty() {
        return TokenStream::new();
    }
    quote! { #[doc = #docs] }
}

/// Returns the `#[deprecated]` attribute for the stability, if any.
fn deprecated(stability: &Stability) -> TokenStream {
    let Some(deprecated) = &stability.deprecated else {
        return TokenStream::new();
    };
    match (&deprecated.since, &deprecated.note) {
        (Some(since), Some(note)) => quote! { #[deprecated(since = #since, note = #note)] },
        (Some(since), None) => quote! { #[deprecated(since = #since)] },
        (None, Some(note)) => quote! { #[deprecated(note = #note)] },
        (None, None) => quote! { #[deprecated] },
    }
}
