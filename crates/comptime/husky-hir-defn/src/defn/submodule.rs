use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
pub struct SubmoduleHirDefn {
    hir_decl: SubmoduleHirDecl,
}

impl SubmoduleHirDefn {
    pub fn hir_decl(self) -> SubmoduleHirDecl {
        self.hir_decl
    }
}

impl HasHirDefn for ModulePath {
    type HirDefn = SubmoduleHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        Some(SubmoduleHirDefn {
            hir_decl: self.hir_decl(db)?,
        })
    }
}