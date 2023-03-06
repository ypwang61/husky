use husky_ty::{
    form_path_ty_unchecked, trai_path_ty_unchecked, ty_constructor_path_ty_unchecked,
    ty_ontology_path_ty_unchecked,
};
use salsa::assert_eq_with_db;

use crate::*;

#[inline(always)]
pub fn term_entity_path_ty(db: &dyn TermDb, path: TermEntityPath) -> TypeResult<Term> {
    match path {
        TermEntityPath::Form(_) => todo!(),
        TermEntityPath::Trait(_) => todo!(),
        TermEntityPath::TypeOntology(_) => todo!(),
        TermEntityPath::TypeConstructor(_) => todo!(),
    }
}

#[inline(always)]
pub fn entity_path_ty(
    db: &dyn TermDb,
    disambiguation: TypePathDisambiguation,
    path: EntityPath,
) -> TypeResult<Term> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => match disambiguation {
                TypePathDisambiguation::Ontology => ty_ontology_path_ty_unchecked(db, path),
                TypePathDisambiguation::Constructor => ty_constructor_path_ty_unchecked(db, path),
            },
            ModuleItemPath::Trait(path) => trai_path_ty_unchecked(db, path),
            ModuleItemPath::Form(path) => form_path_ty_unchecked(db, path),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

#[test]
fn entity_path_path_term_ty_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    let term_menu = db.term_menu(toolchain).unwrap();
    let invariant_ty0_to_trai_ty: Term = term_menu.ex_inv_ty0_to_trai_ty().into();
    let ex_co_lifetime_to_ex_co_ty0_to_ty0: Term =
        term_menu.ex_co_lifetime_to_ex_co_ty0_to_ty0().into();
    let ex_co_lifetime_to_ex_ct_ty0_to_ty0: Term =
        term_menu.ex_co_lifetime_to_ex_ct_ty0_to_ty0().into();
    let trai_ty = term_menu.trai_ty();
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.bool_ty_path().into(),
        ),
        Ok(term_menu.ty0().into())
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_add().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_add_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_and().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_and_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_or().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_or_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_xor().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_bit_xor_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_div().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_div_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_mul().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_mul_assign().into(),
        ),
        Ok(invariant_ty0_to_trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_neg().into(),
        ),
        Ok(trai_ty)
    );
    assert_eq_with_db!(
        db,
        entity_path_ty(
            &db,
            TypePathDisambiguation::Ontology,
            entity_path_menu.core_ops_not().into(),
        ),
        Ok(trai_ty)
    );
    // assert_eq_with_db!(
    //     db,
    //     entity_path_ty(&db, entity_path_menu.ref_ty_path().into()),
    //     Ok(covariant_lifetime_to_covariant_ty0_to_ty0)
    // );
}

pub fn ty_path_ty(
    db: &dyn TermDb,
    path: TypePath,
    disambiguation: TypePathDisambiguation,
) -> TypeResult<Term> {
    match disambiguation {
        TypePathDisambiguation::Ontology => ty_ontology_path_ty_unchecked(db, path),
        TypePathDisambiguation::Constructor => ty_constructor_path_ty_unchecked(db, path),
    }
}

pub(crate) fn function_entity_ty(
    db: &dyn TermDb,
    variances: &[Variance],
    signature: FunctionSignature,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    todo!()
}

pub(crate) fn feature_entity_ty(
    db: &dyn TermDb,
    signature: FeatureSignature,
    term_menu: &TermMenu,
) -> TypeResult<Term> {
    todo!()
}

fn curry_from_implicit_parameter_tys(
    db: &dyn TermDb,
    term_curry_kind: CurryKind,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    mut term: Term,
) -> Term {
    todo!()
}