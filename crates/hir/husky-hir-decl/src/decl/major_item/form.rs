pub mod compterm;
pub mod function_ritchie;
pub mod r#static;
pub mod ty_alias;
pub mod val;

use self::compterm::*;
pub use self::function_ritchie::*;
use self::r#static::*;
pub use self::ty_alias::*;
pub use self::val::*;

use super::*;
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_syn_decl::decl::FormSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorFormHirDecl {
    Ritchie(MajorFunctionRitchieHirDecl),
    Val(MajorValHirDecl),
    Const(MajorComptermHirDecl),
    Static(MajorStaticHirDecl),
    TypeAlias(MajorTypeAliasHirDecl),
}

impl MajorFormHirDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            MajorFormHirDecl::Ritchie(decl) => Some(decl.template_parameters(db)),
            MajorFormHirDecl::Val(_decl) => None,
            MajorFormHirDecl::TypeAlias(_) => todo!(),
            MajorFormHirDecl::Const(_decl) => None,
            MajorFormHirDecl::Static(_) => None,
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            MajorFormHirDecl::Ritchie(decl) => decl.hir_expr_region(db).into(),
            MajorFormHirDecl::Val(decl) => decl.hir_eager_expr_region(db).into(),
            MajorFormHirDecl::TypeAlias(decl) => decl.hir_eager_expr_region(db).into(),
            MajorFormHirDecl::Const(_) => todo!(),
            MajorFormHirDecl::Static(_) => todo!(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            MajorFormHirDecl::Ritchie(decl) => decl.path(db),
            MajorFormHirDecl::Val(decl) => decl.path(db),
            MajorFormHirDecl::TypeAlias(decl) => decl.path(db),
            MajorFormHirDecl::Const(_) => todo!(),
            MajorFormHirDecl::Static(_) => todo!(),
        }
    }
}

impl HasHirDecl for MajorFormPath {
    type HirDecl = MajorFormHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        major_form_hir_decl(db, self)
    }
}

#[salsa::tracked]
fn major_form_hir_decl(db: &::salsa::Db, path: MajorFormPath) -> Option<MajorFormHirDecl> {
    match path.syn_decl(db).expect("no errors for hir stage") {
        FormSynDecl::Ritchie(syn_decl) => {
            Some(MajorFunctionRitchieHirDecl::from_syn(path, syn_decl, db).into())
        }
        FormSynDecl::Val(syn_decl) => Some(MajorValHirDecl::from_syn(path, syn_decl, db).into()),
        FormSynDecl::TypeAlias(_) => None,
        FormSynDecl::Const(syn_decl) => {
            Some(MajorComptermHirDecl::from_syn(path, syn_decl, db).into())
        }
        FormSynDecl::Static(syn_decl) => {
            Some(MajorStaticHirDecl::from_syn(path, syn_decl, db).into())
        }
    }
}
