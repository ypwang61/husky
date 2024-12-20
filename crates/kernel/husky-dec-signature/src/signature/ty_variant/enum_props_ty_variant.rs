use super::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_syn_decl::decl::ty_variant::props_ty_variant::TypePropsVariantSynDecl;

#[salsa::interned]
pub struct EnumPropsVariantDecTemplate {
    pub parent_ty_template: EnumDecTemplate,
    pub field_tys: SmallVec<[EnumPropsVariantFieldDecTemplate; 4]>,
    pub return_ty: DecTerm,
    pub instance_constructor_ty: DecRitchie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnumPropsVariantFieldDecTemplate {
    ident: Ident,
    ty: DecTerm,
}

impl EnumPropsVariantDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        parent_ty_template: EnumDecTemplate,
        decl: TypePropsVariantSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(EnumPropsVariantFieldDecTemplate {
                    ident: field.ident(),
                    ty: dec_term_region.expr_term(field.ty()).map_err(|_| {
                        DecSignatureError::FieldTypeDecTermError(i.try_into().unwrap())
                    })?,
                })
            })
            .collect::<DecSignatureResult<SmallVec<_>>>()?;
        // todo: GADT can override return_ty
        let return_ty = parent_ty_template.self_ty(db);
        let instance_constructor_ty = DecRitchie::new(
            db,
            RitchieItemKind::Fn.into(),
            fields
                .iter()
                .copied()
                .map(|field: EnumPropsVariantFieldDecTemplate| {
                    DeclarativeRitchieSimpleParameter::new(Contract::Move, field.ty).into()
                })
                .collect(),
            return_ty,
        );
        Ok(Self::new(
            db,
            parent_ty_template,
            fields,
            return_ty,
            instance_constructor_ty,
        ))
    }
}
