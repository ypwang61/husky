use super::*;
use husky_token::{CurryToken, EolToken};

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct FnNodeDecl {
    #[id]
    pub node_path: FugitiveNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    explicit_parameter_decl_list: DeclExprResult<ExplicitParameterDeclList>,
    #[return_ref]
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: DeclExprResult<Option<ReturnTypeExpr>>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolToken>,
    pub expr_region: ExprRegion,
}

impl FnNodeDecl {}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_fn_node_decl(
        &self,
        node_path: FugitiveNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> FnNodeDecl {
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::False, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        let parameter_decl_list =
            ctx.try_parse_expected(OriginalDeclExprError::ExpectedParameterDeclList);
        let curry_token = ctx.try_parse_optional();
        let return_ty = if let Ok(Some(_)) = curry_token {
            ctx.try_parse_expected(OriginalDeclExprError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = ctx.try_parse_expected(OriginalDeclExprError::ExpectedEolColon);
        FnNodeDecl::new(
            self.db(),
            node_path,
            ast_idx,
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct FnDecl {
    #[id]
    pub path: FugitivePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub regular_parameters: RegularParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl FnDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: FugitivePath,
        node_decl: FnNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let explicit_parameter_decl_list = node_decl.explicit_parameter_decl_list(db).as_ref()?;
        let regular_parameters: RegularParameterDeclPatterns = explicit_parameter_decl_list
            .regular_parameters()
            .to_smallvec();
        let return_ty = *node_decl.return_ty(db).as_ref()?;
        let expr_region = node_decl.expr_region(db);
        Ok(FnDecl::new(
            db,
            path,
            implicit_parameters,
            regular_parameters,
            return_ty,
            expr_region,
        ))
    }
}