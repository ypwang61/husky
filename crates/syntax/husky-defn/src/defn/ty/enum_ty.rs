use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct EnumTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}