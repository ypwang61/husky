use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitNodeDecl {
    #[id]
    pub node_path: TraitNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl HasNodeDecl for TraitNodePath {
    type NodeDecl = TraitNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        trai_node_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn trai_node_decl(db: &dyn DeclDb, node_path: TraitNodePath) -> TraitNodeDecl {
    let parser = DeclParser::new(db, node_path.module_path(db));
    parser.parse_trai_node_decl(node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_node_decl(&self, node_path: TraitNodePath) -> TraitNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_trai_decl_aux(ast_idx, node_path, token_group_idx, saved_stream_state),
            _ => unreachable!(),
        }
    }

    fn parse_trai_decl_aux(
        &self,
        ast_idx: AstIdx,
        id: TraitNodePath,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitNodeDecl {
        let mut parser = self.expr_parser(id, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.try_parse_optional();
        TraitNodeDecl::new(self.db(), id, ast_idx, implicit_parameters, parser.finish())
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct TraitDecl {
    #[id]
    pub path: TraitPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}

impl TraitDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitPath,
        node_decl: TraitNodeDecl,
    ) -> DeclResult<TraitDecl> {
        let ast_idx = node_decl.ast_idx(db);
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(TraitDecl::new(
            db,
            path,
            ast_idx,
            implicit_parameters,
            expr_region,
        ))
    }
}

impl HasDecl for TraitPath {
    type Decl = TraitDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        trai_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn trai_decl(db: &dyn DeclDb, path: TraitPath) -> DeclResult<TraitDecl> {
    let node_decl = path.node_path(db).node_decl(db);
    TraitDecl::from_node_decl(db, path, node_decl)
}