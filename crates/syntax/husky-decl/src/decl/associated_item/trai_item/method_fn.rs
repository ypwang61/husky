use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<Generics>,
    #[return_ref]
    parameter_decl_list: SelfParameterAndExplicitParameters<true>,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub eol_colon: EolToken,
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub self_parameter: Option<SelfParameterDeclPattern>,
    #[return_ref]
    pub parenic_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub expr_region: ExprRegion,
}

impl TraitMethodFnDecl {}
