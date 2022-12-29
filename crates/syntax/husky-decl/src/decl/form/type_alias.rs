use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAliasDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}