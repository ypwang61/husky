use crate::*;
use husky_expr::ExprSheet;

#[salsa::tracked(jar = DeclJar)]
pub struct TraitAssociatedFunctionDecl {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
