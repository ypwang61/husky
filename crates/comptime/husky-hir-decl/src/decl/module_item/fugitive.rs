mod r#fn;
mod gn;
mod ti;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ti::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum FugitiveHirDecl {
    Fn(FnHirDecl),
    Val(ValHirDecl),
    Gn(GnHirDecl),
    // todo: AliasType
}

impl HasHirDecl for FugitivePath {
    type HirDecl = FugitiveHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        todo!()
    }
}

impl FugitiveHirDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            FugitiveHirDecl::Fn(decl) => decl.template_parameters(db),
            FugitiveHirDecl::Val(_decl) => &[],
            FugitiveHirDecl::Gn(decl) => decl.template_parameters(db),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            FugitiveHirDecl::Fn(decl) => decl.hir_expr_region(db).into(),
            FugitiveHirDecl::Val(decl) => decl.hir_expr_region(db).into(),
            FugitiveHirDecl::Gn(decl) => decl.hir_expr_region(db).into(),
        }
    }

    pub fn path(self, db: &dyn HirDeclDb) -> FugitivePath {
        match self {
            FugitiveHirDecl::Fn(decl) => decl.path(db),
            FugitiveHirDecl::Val(decl) => decl.path(db),
            FugitiveHirDecl::Gn(decl) => decl.path(db),
        }
    }
}