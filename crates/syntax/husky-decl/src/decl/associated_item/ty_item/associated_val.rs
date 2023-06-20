use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedValNodeDecl {
    #[id]
    pub id: TypeItemNodePath,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedValDecl {
    #[id]
    pub path: TypeItemPath,
    pub expr_region: ExprRegion,
}

impl TypeAssociatedValDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeItemPath,
        node_decl: TypeAssociatedValNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
