mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum MajorItemHirDefn {
    Type(TypeHirDefn),
    Trait(TraitHirDefn),
    Fugitive(FugitiveHirDefn),
}

impl MajorItemHirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> MajorItemHirDecl {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorItemHirDefn::Fugitive(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            MajorItemHirDefn::Type(_) | MajorItemHirDefn::Trait(_) => None,
            MajorItemHirDefn::Fugitive(hir_defn) => hir_defn.hir_expr_region(db),
        }
    }
}

impl HasHirDefn for MajorItemPath {
    type HirDefn = MajorItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        Some(match self {
            MajorItemPath::Type(path) => path.hir_defn(db)?.into(),
            MajorItemPath::Fugitive(path) => path.hir_defn(db)?.into(),
            MajorItemPath::Trait(path) => path.hir_defn(db)?.into(),
        })
    }
}