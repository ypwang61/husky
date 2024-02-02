use crate::*;

#[salsa::tracked(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct StructureTypeDecTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
}

impl StructureTypeDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: StructureTypeSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        );
        Ok(Self::new(db, template_parameters))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct StructureTypeDecSignature {}