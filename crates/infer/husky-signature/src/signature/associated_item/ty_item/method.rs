use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_method_signature(
    db: &dyn SignatureDb,
    decl: TypeMethodDecl,
) -> SignatureResult<TypeMethodSignature> {
    let impl_block = decl.associated_item(db).impl_block(db);
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();

    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        signature_term_region,
        term_menu,
    );

    let parameters =
        RegularParameterSignatures::from_decl(decl.parameters(db), signature_term_region)?;
    let return_ty = match decl.return_ty(db) {
        Ok(return_ty) => match signature_term_region.expr_term(return_ty.expr()) {
            Ok(return_ty) => return_ty,
            Err(_) => todo!(),
        },
        Err(_) => todo!(), //  Err(SignatureTermAbortion::ExprError),
    };
    Ok(TypeMethodSignature::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct TypeMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: RegularParameterSignatures,
    pub return_ty: Term,
}
