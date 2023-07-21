use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct TypeAliasNodeDefn {
    #[id]
    pub node_path: FugitiveNodePath,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct TypeAliasDefn {
    #[id]
    pub path: FugitivePath,
    pub expr_region: ExprRegion,
}
