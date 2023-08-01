use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnSynNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    template_parameter_decl_list: Option<Generics>,
    #[return_ref]
    parameter_decl_list: SelfParameterAndExplicitParameters<true>,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub eol_colon: EolToken,
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub self_parameter: Option<SelfParameterDeclPattern>,
    #[return_ref]
    pub parenate_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMethodFnSynDecl {}