use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAliasHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}