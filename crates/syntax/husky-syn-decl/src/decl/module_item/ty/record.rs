use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct RecordTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    template_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub syn_expr_region: SynExprRegion,
}

impl RecordTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct RecordTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl RecordTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: RecordTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, template_parameters, syn_expr_region))
    }
}