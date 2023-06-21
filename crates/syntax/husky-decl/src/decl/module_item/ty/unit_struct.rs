use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitStructTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl UnitStructTypeNodeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
}

impl UnitStructTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: UnitStructTypeNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}