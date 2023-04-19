mod parameter;
mod term;
mod trai_for_ty_impl;

pub(crate) use self::parameter::*;
pub(crate) use self::trai_for_ty_impl::*;

use self::term::*;
use crate::*;
use husky_declarative_signature::{
    HasDeclarativeSignatureTemplate, ImplicitParameterSignature, TypeDeclarativeSignatureTemplate,
};
use husky_decr::Decr;
use husky_entity_tree::TraitForTypeImplBlock;

pub(crate) trait HasTemplate: Copy {
    type Template;

    fn template(self, db: &dyn EtherealTermDb) -> TermResult<Self::Template>;
}