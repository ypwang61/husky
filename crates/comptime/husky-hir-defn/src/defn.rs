mod associated_item;
mod impl_block;
mod module_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_hir_ty::{
    template_parameter::{HirTemplateParameter, HirTemplateParameters},
    *,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum HirDefn {
    Submodule(SubmoduleHirDefn),
    ModuleItem(ModuleItemHirDefn),
    TypeVariant(TypeVariantHirDefn),
    ImplBlock(ImplBlockHirDecl),
    AssociatedItem(AssociatedItemHirDefn),
}

impl HirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> HirDecl {
        match self {
            HirDefn::Submodule(hir_defn) => HirDecl::Submodule(hir_defn.hir_decl()),
            HirDefn::ModuleItem(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::TypeVariant(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::ImplBlock(hir_decl) => hir_decl.into(),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn HirDefnDb) -> &'a [HirTemplateParameter] {
        self.hir_decl(db).template_parameters(db)
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            HirDefn::Submodule(_) => None,
            HirDefn::ModuleItem(hir_defn) => hir_defn.hir_expr_region(db),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.hir_expr_region(db),
            HirDefn::TypeVariant(_defn) => None,
            HirDefn::ImplBlock(_) => None,
        }
    }
}

impl HirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> Option<ItemPath> {
        todo!()
        // match self {
        //     HirDefn::Type(hir_defn) => Some(hir_defn.path(db).into()),
        //     HirDefn::Trait(hir_defn) => Some(hir_defn.path(db).into()),
        //     HirDefn::Fugitive(hir_defn) => Some(hir_defn.path(db).into()),
        //     HirDefn::AssociatedItem(hir_defn) => hir_defn.path(db).map(|path| path.into()),
        //     HirDefn::Variant(hir_defn) => Some(hir_defn.path(db).into()),
        //     HirDefn::ImplBlock(_) => None,
        // }
    }
}

pub trait HasHirDefn: Copy {
    type HirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn>;
}

impl HasHirDefn for ItemPath {
    type HirDefn = HirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        Some(match self {
            ItemPath::Submodule(path) => path.hir_defn(db)?.into(),
            ItemPath::ModuleItem(path) => path.hir_defn(db)?.into(),
            ItemPath::ImplBlock(path) => path.hir_defn(db)?.into(),
            ItemPath::AssociatedItem(path) => path.hir_defn(db)?.into(),
            ItemPath::TypeVariant(_) => todo!(),
        })
    }
}

#[salsa::tracked(jar = HirDefnJar, return_ref)]
pub(crate) fn module_hir_defns(db: &dyn HirDefnDb, module_path: ModulePath) -> Vec<HirDefn> {
    todo!()
    // module_item_paths(db, module_path)
    //     .as_ref()?
    //     .iter()
    //     .copied()
    //     .filter_map(|path| path.hir_defn(db).ok())
    //     .collect()
}

#[test]
fn module_defns_works() {
    // use tests::*;

    // DB::default()
    //     .ast_expect_test_debug_with_db("module_hir_defns", |db, module_path: ModulePath| {
    //         module_path.defns(db)
    //     });
}