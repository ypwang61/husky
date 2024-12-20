use super::*;

#[salsa::interned]
pub struct TraitAssocTypeHirDecl {
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}
