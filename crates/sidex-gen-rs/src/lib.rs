use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sidex_gen::ir;

pub mod config;

pub struct Generator {
    unit: ir::Unit,
}

impl Generator {
    pub fn generate_type(
        &self,
        model_idx: ir::BundleIdx,
        schema_idx: ir::SchemaIdx,
        def_idx: ir::DefIdx,
        typ: &ir::Type,
    ) -> TokenStream {
        match typ {
            ir::Type::TypeVar(_) => todo!(),
            ir::Type::Instance(_) => todo!(),
            // ir::Type::Sequence(typ) => {
            //     let element_ty = self.generate_type(model_idx, schema_idx, def_idx,
            // &typ.element);     quote!(::std::vec::Vec< #element_ty >)
            // }
            // ir::Type::Map(_) => todo!(),
        }
    }

    pub fn generate_schema(&self) {}

    pub fn generate_model(&self, model_idx: ir::BundleIdx) -> TokenStream {
        let model = &self.unit[model_idx];

        let mut rs_modules = Vec::new();
        for schema in &model.schemas {
            let rs_mod_name = format_ident!("{}", schema.name);
            rs_modules.push(quote! {
                mod #rs_mod_name {
                    // #(#rs_items)*
                }
            });
        }

        quote! {
            #(#rs_modules)*
        }
    }
}

// use std::ops::Deref;

// use proc_macro2::TokenStream;
// use quote::{format_ident, quote};
// use sidex_gen::hir;

// pub struct Generator {}

// pub fn generate_code(unit: &hir::Unit, model: hir::ModelId) -> TokenStream {
//     let model = unit.get_model(model);
//     let mut rs_modules = Vec::new();
//     for (name, module) in model.modules() {
//         let module = unit.get_module(module);
//         let rs_mod_name = format_ident!("{}", name);
//         let rs_items = module
//             .defs
//             .iter()
//             .filter_map(|(name, def)| {
//                 let def = unit.get_def(*def);
//                 let ident = format_ident!("{}", name.deref());
//                 match def.kind {
//                     hir::DefKind::StructDef(_) => Some(quote! {
//                         pub struct #ident {}
//                     }),
//                     hir::DefKind::EnumDef(_) => Some(quote! {
//                         pub enum #ident {}
//                     }),
//                     hir::DefKind::OpaqueDef(_) => Some(quote! {
//                         type #ident = ();
//                     }),
//                     hir::DefKind::AliasDef(_) => Some(quote! {
//                         type #ident = ();
//                     }),
//                     hir::DefKind::FunDef(_) => Some(quote! {
//                         type #ident = fn() -> ();
//                     }),
//                     _ => todo!(),
//                 }
//             })
//             .collect::<Vec<_>>();
//         rs_modules.push(quote! {
//             mod #rs_mod_name {
//                 #(#rs_items)*
//             }
//         });
//     }

//     quote! {
//         #(#rs_modules)*
//     }
// }
