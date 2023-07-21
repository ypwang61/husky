use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeNodeDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

impl TypeAssociatedTypeNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        Default::default()
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedTypeDecl {
    #[id]
    pub path: TypeItemPath,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParser<'a> {}
