use crate::*;
#[cfg(test)]
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use vec_like::VecSet;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new)]
pub(crate) struct HirDefnDependencies {
    #[return_ref]
    pub(crate) item_paths_in_current_crate: VecSet<ItemPath>,
    #[return_ref]
    pub(crate) item_paths_in_other_local_crates: VecSet<ItemPath>,
}

pub(crate) struct HirDefnDependenciesBuilder<'a> {
    item_path: ItemPath,
    crate_path: CratePath,
    item_paths_in_current_crate: VecSet<ItemPath>,
    item_paths_in_other_local_crates: VecSet<ItemPath>,
    db: &'a dyn HirDefnDb,
}

impl<'a> HirDefnDependenciesBuilder<'a> {
    pub(crate) fn new(
        item_path: ItemPath,
        hir_expr_region: HirExprRegion,
        db: &'a dyn HirDefnDb,
    ) -> Self {
        let mut slf = Self {
            item_path,
            crate_path: item_path.crate_path(db),
            item_paths_in_current_crate: Default::default(),
            item_paths_in_other_local_crates: Default::default(),
            db,
        };
        slf.add_hir_expr_region(hir_expr_region);
        slf
    }

    fn add_hir_expr_region(&mut self, hir_expr_region: HirExprRegion) {
        match hir_expr_region {
            HirExprRegion::Eager(hir_eager_expr_region) => {
                self.add_hir_eager_expr_region(hir_eager_expr_region)
            }
            HirExprRegion::Lazy(hir_lazy_expr_region) => todo!(),
        }
    }

    fn add_hir_eager_expr_region(&mut self, hir_eager_expr_region: HirEagerExprRegion) {
        let db = self.db;
        let hir_eager_expr_arena = hir_eager_expr_region.hir_eager_expr_arena(db);
        for hir_eager_expr_data in hir_eager_expr_arena.iter() {
            match *hir_eager_expr_data {
                HirEagerExprData::Literal(_) => (),
                HirEagerExprData::PrincipalEntityPath(path) => match path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(path) => self.add_item_path(path),
                    PrincipalEntityPath::TypeVariant(path) => {
                        self.add_item_path(path.parent_ty_path(db))
                    }
                },
                HirEagerExprData::ConstSymbol(_) => (),
                HirEagerExprData::Variable(_) => (),
                HirEagerExprData::Binary { .. } => (),
                HirEagerExprData::Be { .. } => (),
                HirEagerExprData::Prefix { .. } => (),
                HirEagerExprData::Suffix { .. } => (),
                HirEagerExprData::TypeConstructorFnCall { path, .. } => self.add_item_path(path),
                HirEagerExprData::TypeVariantConstructorCall { path, .. } => {
                    self.add_item_path(path.parent_ty_path(db))
                }
                HirEagerExprData::FunctionFnCall { path, .. } => self.add_item_path(path),
                HirEagerExprData::AssociatedFunctionFnCall { path, .. } => self.add_item_path(path),
                HirEagerExprData::PropsStructField {
                    owner_hir_expr_idx,
                    ident,
                } => todo!(),
                HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx,
                    ident,
                } => todo!(),
                HirEagerExprData::MethodFnCall {
                    path,
                    ref template_arguments,
                    ..
                } => {
                    todo!();
                    self.add_item_path(path)
                }
                HirEagerExprData::NewTuple { .. } => (),
                HirEagerExprData::Index { .. } => todo!(),
                HirEagerExprData::NewList { .. } => (),
                HirEagerExprData::Block { .. } => (),
                HirEagerExprData::EmptyHtmlTag { .. } => (),
                HirEagerExprData::Todo => (),
                HirEagerExprData::Unreachable => (),
                HirEagerExprData::AssociatedFn {
                    associated_item_path,
                } => self.add_item_path(associated_item_path),
            }
        }
    }

    fn add_item_path(&mut self, item_path: impl Into<ItemPath>) {
        let item_path: ItemPath = item_path.into();
        if item_path == self.item_path {
            // no need to add self
            return;
        }
        let db = self.db;
        let crate_path = item_path.crate_path(db);
        if crate_path == self.crate_path {
            self.item_paths_in_current_crate.insert(item_path)
        } else {
            match crate_path.package_path(db).data(db) {
                PackagePathSource::Library => (),
                PackagePathSource::Registry { .. } => (),
                PackagePathSource::Git { .. } => (),
                PackagePathSource::Local { .. } => {
                    self.item_paths_in_other_local_crates.insert(item_path)
                }
            }
        }
    }

    fn finish(self) -> HirDefnDependencies {
        HirDefnDependencies::new(
            self.db,
            self.item_paths_in_current_crate,
            self.item_paths_in_other_local_crates,
        )
    }
}

#[cfg(test)]
pub(crate) fn module_hir_defn_dependencies(
    db: &DB,
    module_path: ModulePath,
) -> Vec<HirDefnDependencies> {
    module_item_paths(db, module_path)
        .iter()
        .filter_map(|path| path.hir_defn(db)?.dependencies(db))
        .collect()
}

#[test]
fn module_hir_defn_dependencies_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug(
        module_hir_defn_dependencies,
        &AstTestConfig::new("module_hir_defn_dependencies"),
    )
}