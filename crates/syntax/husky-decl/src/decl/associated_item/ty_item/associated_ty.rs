use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedTypeDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}