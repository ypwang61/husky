use super::default::*;
use super::*;
use husky_dec_signature::signature::HasDecTemplate;
use husky_entity_path::path::{
    assoc_item::ty_item::TypeItemPath,
    major_item::{form::MajorFormPath, trai::TraitPath, ty::TypePath, MajorItemPath},
    ItemPath,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VarianceRepr {
    base: Variance,
    dependency_exprs: Vec<VarianceExpr>,
    deps: Vec<VariancePath>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VarianceExpr {
    Atom(VariancePath),
    Compose(VariancePath, Vec<VarianceExpr>),
}

impl VarianceRepr {
    pub(crate) fn base(&self) -> Variance {
        self.base
    }

    pub(crate) fn deps(&self) -> &[VariancePath] {
        self.deps.as_ref()
    }
}

pub(crate) fn item_variance_reprs(
    db: &::salsa::Db,
    path: ItemPath,
) -> VarianceResultRef<&[VarianceRepr]> {
    let _dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    match path {
        ItemPath::Submodule(_, _) => todo!(),
        ItemPath::MajorItem(path) => match path {
            MajorItemPath::Type(path) => ty_template_parameter_variance_reprs(db, path),
            MajorItemPath::Trait(path) => trai_item_variance_reprs(db, path),
            MajorItemPath::Form(path) => form_item_variance_reprs(db, path),
        },
        ItemPath::AssocItem(_path) => todo!(),
        // match path {
        //     AssocItemPath::TypeItem(path) => ty_item_item_variance_reprs(db, path),
        //     AssocItemPath::TraitItem(_) => todo!(),
        //     AssocItemPath::TraitForTypeItem(_) => todo!(),
        // },
        ItemPath::TypeVariant(_, _) => todo!(),
        ItemPath::ImplBlock(_) => todo!(),
        ItemPath::Attr(_, _) => todo!(),
        ItemPath::Script(_, _) => todo!(),
    }
    .as_ref()
    .map(|t| t.as_ref())
}

#[salsa::tracked(jar = DecTypeJar, return_ref)]
pub(crate) fn ty_template_parameter_variance_reprs(
    db: &::salsa::Db,
    path: TypePath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let _dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let template_parameters = signature.template_parameters(db);
    let reprs = template_parameters
        .iter()
        .map(|template_parameter| VarianceRepr {
            base: template_parameter
                .annotated_variance()
                .unwrap_or(TYPE_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            deps: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // ad hoc: todo
        // match signature {
        //      TypeDecTemplate::Enum(_) => todo!(),
        //      TypeDecTemplate::PropsStruct(_) => todo!(),
        //      TypeDecTemplate::UnitStruct(_) => todo!(),
        //      TypeDecTemplate::TupleStruct(_) => todo!(),
        //      TypeDecTemplate::Record(_) => todo!(),
        //      TypeDecTemplate::Inductive(_) => todo!(),
        //      TypeDecTemplate::Structure(_) => todo!(),
        //      TypeDecTemplate::Foreign(_) => (),
        //      TypeDecTemplate::Union(_) => todo!(),
        // }
    }
    for (_repr, template_parameter) in std::iter::zip(reprs.iter(), template_parameters.iter()) {
        if let Some(_annotated_variance) = template_parameter.annotated_variance() {
            // verify the calculated is the same as the annotated
            // todo!()
        }
    }
    Ok(reprs)
}

#[salsa::tracked(jar = DecTypeJar, return_ref)]
pub(crate) fn trai_item_variance_reprs(
    db: &::salsa::Db,
    path: TraitPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let _dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    debug_assert!(matches!(
        signature
            .template_parameters(db)
            .last()
            .unwrap()
            .symbol()
            .index(db)
            .inner(),
        DecTermSymbolIndexImpl::SelfType
    ));
    let reprs = signature
        .template_parameters_without_self_ty(db)
        .iter()
        .map(|parameter| VarianceRepr {
            base: parameter
                .annotated_variance()
                .unwrap_or(TRAIT_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            deps: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // todo!()
    }
    Ok(reprs)
}

#[salsa::tracked(jar = DecTypeJar, return_ref)]
pub(crate) fn form_item_variance_reprs(
    db: &::salsa::Db,
    path: MajorFormPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let template_parameters = signature.template_parameters(db);
    let reprs = template_parameters
        .iter()
        .map(|parameter| VarianceRepr {
            base: parameter
                .annotated_variance()
                .unwrap_or(FORM_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            deps: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // todo!()
    }
    Ok(reprs)
}

#[salsa::tracked(jar = DecTypeJar, return_ref)]
pub(crate) fn ty_item_item_variance_reprs(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let template_parameters = signature.template_parameters(db);
    let reprs = template_parameters
        .iter()
        .map(|parameter| VarianceRepr {
            base: parameter
                .annotated_variance()
                .unwrap_or(TY_ITEM_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            deps: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // todo!()
    }
    Ok(reprs)
}
