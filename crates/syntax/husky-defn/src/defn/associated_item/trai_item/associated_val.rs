use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedValNodeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub node_decl: TraitAssociatedValDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedValDefn {
    #[id]
    pub node_path: TraitItemPath,
    pub decl: TraitAssociatedValDecl,
    pub expr_region: ExprRegion,
}
