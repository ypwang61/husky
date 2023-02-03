use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct FeatureDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
    pub expr_region: ExprRegion,
}
