use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn function_signature(db: &dyn SignatureDb, decl: FunctionDecl) -> FunctionSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    FunctionSignature::new(
        db,
        ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        todo!(),
        todo!(),
        engine.finish(),
    )
}


#[salsa::tracked(jar = SignatureJar)]
pub struct FunctionSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
    #[return_ref]
    term_sheet: SignatureTermSheet,
}

impl FunctionSignature {}