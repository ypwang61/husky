use husky_hir_defn::db::HirDefnDb;
use husky_linkage_path::db::LinkagePathDb;

pub trait RustTranspilationDb:
    salsa::DbWithJar<RustTranspilationJar> + HirDefnDb + LinkagePathDb
{
}

impl<Db> RustTranspilationDb for Db where
    Db: salsa::DbWithJar<RustTranspilationJar> + HirDefnDb + LinkagePathDb
{
}

#[salsa::jar(db = RustTranspilationDb)]
pub struct RustTranspilationJar(
    crate::defn::module_defn_rust_transpilation,
    crate::package::rust_transpilation_packages,
    crate::linkages::package_linkages_transpilation,
    crate::manifest::package_rust_manifest,
);
