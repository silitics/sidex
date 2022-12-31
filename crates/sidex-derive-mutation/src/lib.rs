use sidex_attrs::{AttrConvertExt, TryApplyAttr, TryFromAttr, TryFromAttrs};
use sidex_gen::rename;
use sidex_ir as ir;

pub struct Mutation {
    pub(crate) for_path: ir::Path,
}

impl TryFromAttr for Mutation {
    fn try_from_attr(attr: &ir::Attr) -> sidex_diagnostics::Result<Self> {
        let args = &attr.expect_list_with("mutation")?.args;
        let for_path = args[0].expect_list_with("for")?.args[0]
            .expect_path()?
            .clone();
        Ok(Self { for_path })
    }
}

#[derive(Debug)]
pub struct MutableWith {
    pub(crate) with_path: ir::Path,
}

impl TryFromAttr for MutableWith {
    fn try_from_attr(attr: &ir::Attr) -> sidex_diagnostics::Result<Self> {
        let with_path = attr.expect_list_with("mutable")?.args[0]
            .expect_list_with("with")?
            .args[0]
            .expect_path()?
            .clone();
        Ok(Self { with_path })
    }
}

pub fn is_settable(field: &ir::Field) -> bool {
    for attr in &field.attrs {
        if let Ok(path) = attr.expect_path() {
            if path.as_str() == "settable" {
                return true;
            }
        }
    }
    false
}

pub fn is_mutable(field: &ir::Field) -> bool {
    for attr in &field.attrs {
        if let Ok(path) = attr.expect_path() {
            if path.as_str() == "mutable" {
                return true;
            }
        }
    }
    false
}

pub fn derive_mutations(unit: &mut ir::Unit) {
    let mut replace = Vec::new();
    for bundle in unit.bundles.iter() {
        for schema in bundle.schemas.iter() {
            'outer: for (def_idx, definition) in schema.defs.iter().enumerate() {
                match &definition.kind {
                    ir::DefKind::DerivedType(_) => {
                        for attr in &definition.attrs {
                            if let Ok(mutation) = Mutation::try_from_attr(attr) {
                                let for_type = unit
                                    .resolve_path(
                                        ir::ItemRef::Schema(schema.schema_ref()),
                                        &mutation.for_path,
                                    )
                                    .unwrap();
                                let def_ref = if let ir::ItemRef::Def(def_ref) = for_type {
                                    def_ref
                                } else {
                                    continue 'outer;
                                };
                                let def = &unit[def_ref];
                                let mut variants = Vec::new();
                                match &def.kind {
                                    ir::DefKind::RecordType(typ) => {
                                        for field in &typ.fields {
                                            if !is_settable(field) && !is_mutable(field) {
                                                continue;
                                            }
                                            if is_settable(field) {
                                                let variant_name = rename::to_pascal_case(
                                                    &format!("set_{}", field.name.as_str()),
                                                );
                                                variants.push(
                                                    ir::Variant::new(ir::Identifier::new(
                                                        variant_name,
                                                    ))
                                                    .with_typ(Some(field.typ.clone()))
                                                    .with_docs(Some(ir::Docs::new(format!(
                                                        "Sets the value of the `{}` field.",
                                                        field.name.as_str()
                                                    )))),
                                                );
                                                if field.is_optional {
                                                    let variant_name = rename::to_pascal_case(
                                                        &format!("delete_{}", field.name.as_str()),
                                                    );
                                                    variants.push(
                                                        ir::Variant::new(ir::Identifier::new(
                                                            variant_name,
                                                        ))
                                                        .with_docs(Some(ir::Docs::new(format!(
                                                            "Deletes the value of the `{}` field.",
                                                            field.name.as_str()
                                                        )))),
                                                    );
                                                }
                                            }
                                            let field_typ_def_ref =
                                                unit.type_def_ref(&field.typ).unwrap();
                                            let field_typ_def = &unit[field_typ_def_ref.clone()];

                                            let mut mutable_with = None;
                                            for attr in &field_typ_def.attrs {
                                                if let Ok(x) = MutableWith::try_from_attr(attr) {
                                                    mutable_with = Some(x);
                                                    break;
                                                }
                                            }
                                            println!("{:?}", mutable_with);
                                            if let Some(mutable_with) = mutable_with {
                                                let mutation_item_ref = unit
                                                    .resolve_path(
                                                        ir::ItemRef::Schema(
                                                            field_typ_def_ref.schema,
                                                        ),
                                                        &mutable_with.with_path,
                                                    )
                                                    .unwrap();
                                                let mutation_def_ref =
                                                    if let ir::ItemRef::Def(def_ref) =
                                                        mutation_item_ref
                                                    {
                                                        def_ref
                                                    } else {
                                                        todo!()
                                                    };
                                                let mutation_typ =
                                                    ir::Type::new(ir::TypeKind::Instance({
                                                        let instance = ir::InstanceType::new(
                                                            mutation_def_ref.schema.bundle,
                                                            mutation_def_ref.schema.schema,
                                                            mutation_def_ref.def,
                                                        );
                                                        if let ir::TypeKind::Instance(
                                                            field_instance,
                                                        ) = &field.typ.kind
                                                        {
                                                            instance.with_subst(
                                                                field_instance.subst.clone(),
                                                            )
                                                        } else {
                                                            instance
                                                        }
                                                    }));
                                                if is_mutable(field) || is_settable(field) {
                                                    let variant_name = rename::to_pascal_case(
                                                        &format!("mutate_{}", field.name.as_str()),
                                                    );
                                                    variants.push(
                                                        ir::Variant::new(ir::Identifier::new(
                                                            variant_name,
                                                        ))
                                                        .with_typ(Some(mutation_typ)),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    _ => todo!(),
                                }
                                if !variants.is_empty() {
                                    replace.push((
                                        bundle.idx,
                                        schema.idx,
                                        ir::DefIdx::from(def_idx),
                                        ir::DefKind::VariantType(
                                            ir::VariantTypeDef::new().with_variants(variants),
                                        ),
                                    ));
                                }
                                //println!("Derived Type: {} for {:?}", definition.name.as_str(),);
                                continue 'outer;
                            }
                        }
                        // TODO: Generate mutation type and insert.
                    }
                    _ => {
                        // Do nothing for other kinds of definitions.
                    }
                }
            }
        }
    }
    for (bundle, schema, def, kind) in replace {
        unit[bundle][schema][def].set_kind(kind);
    }
}
