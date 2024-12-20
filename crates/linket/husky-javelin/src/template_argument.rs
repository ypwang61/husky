pub mod constant;
pub mod ty;

use self::constant::*;
use self::ty::*;
use crate::instantiation::JavInstantiation;
use husky_hir_ty::HirTemplateArgument;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum JavTemplateArgument {
    Vacant,
    Type(JavType),
    Constant(JavelinConstant),
    Lifetime,
    Place,
}

impl JavTemplateArgument {
    pub(crate) fn from_hir_template_arguments(
        template_arguments: &[HirTemplateArgument],
        jav_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> JavTemplateArguments {
        template_arguments
            .iter()
            .map(|&template_argument| {
                JavTemplateArgument::from_hir(template_argument, jav_instantiation, db)
            })
            .collect()
    }

    pub(crate) fn from_hir(
        template_argument: HirTemplateArgument,
        instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match template_argument {
            HirTemplateArgument::Vacant => JavTemplateArgument::Vacant,
            HirTemplateArgument::Type(hir_ty) => {
                JavTemplateArgument::Type(JavType::from_hir(hir_ty, instantiation, db))
            }
            HirTemplateArgument::Constant(hir_constant) => JavTemplateArgument::Constant(
                JavelinConstant::from_hir(hir_constant, instantiation),
            ),
            HirTemplateArgument::Lifetime(_) => JavTemplateArgument::Lifetime,
            HirTemplateArgument::ContractedQuary(_) => JavTemplateArgument::Place,
        }
    }
}

pub type JavTemplateArguments = smallvec::SmallVec<[JavTemplateArgument; 2]>;
