mod associated_item;
mod decr;
mod impl_block;
mod module_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::decr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::{db::*, *};

pub trait HasHirDecl {
    type HirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum HirDecl {
    Submodule(SubmoduleHirDecl),
    ModuleItem(ModuleItemHirDecl),
    ImplBlock(ImplBlockHirDecl),
    AssociatedItem(AssociatedItemHirDecl),
    TypeVariant(TypeVariantHirDecl),
}

impl HirDecl {
    pub fn generic_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [EtherealGenericParameter] {
        match self {
            HirDecl::Submodule(_) => todo!(),
            HirDecl::ModuleItem(decl) => decl.generic_parameters(db),
            HirDecl::ImplBlock(decl) => decl.generic_parameters(db),
            HirDecl::AssociatedItem(decl) => decl.generic_parameters(db),
            HirDecl::TypeVariant(decl) => &[],
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> Option<HirExprRegion> {
        match self {
            HirDecl::Submodule(_) => None,
            HirDecl::ModuleItem(decl) => decl.hir_expr_region(db).into(),
            HirDecl::ImplBlock(decl) => decl.hir_expr_region(db).into(),
            HirDecl::AssociatedItem(decl) => decl.hir_expr_region(db).into(),
            HirDecl::TypeVariant(_decl) => todo!(),
        }
    }

    pub fn path(self, db: &dyn HirDeclDb) -> EntityPath {
        match self {
            HirDecl::Submodule(_) => todo!(),
            HirDecl::ModuleItem(decl) => decl.path(db).into(),
            HirDecl::ImplBlock(decl) => decl.path(db).into(),
            HirDecl::AssociatedItem(decl) => decl.path(db).into(),
            HirDecl::TypeVariant(decl) => decl.path(db).into(),
        }
    }
}
