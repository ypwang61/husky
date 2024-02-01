use crate::*;
use husky_declarative_signature::DeclarativeSignatureError;
use husky_ethereal_term::instantiation::error::EtherealTermInstantiationError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
pub enum EtherealSignatureError {
    #[error("term error")]
    TermError(EtherealTermError),
    #[error("term error")]
    TermInstantiationError(EtherealTermInstantiationError),
    #[error("DerivedFromDeclarative")]
    DerivedFromDeclarativeSignature(DeclarativeSignatureError),
    #[error("NoSuchItem")]
    NoSuchItemInTraitForTypeImplBlockEtherealSignatureBuilder {
        signature_builder: TraitForTypeImplBlockEtherealSignatureBuilder,
        ident: Ident,
    },
    #[error("EntityTreeError")]
    EntityTreeError,
    #[error("EntityTreeBundleError")]
    EntityTreeBundleError,
}

impl From<&EtherealSignatureError> for EtherealSignatureError {
    fn from(e: &EtherealSignatureError) -> Self {
        *e
    }
}

impl From<DeclarativeSignatureError> for EtherealSignatureError {
    fn from(e: DeclarativeSignatureError) -> Self {
        EtherealSignatureError::DerivedFromDeclarativeSignature(e)
    }
}

impl From<EtherealTermError> for EtherealSignatureError {
    fn from(e: EtherealTermError) -> Self {
        EtherealSignatureError::TermError(e)
    }
}

impl From<EtherealTermInstantiationError> for EtherealSignatureError {
    fn from(e: EtherealTermInstantiationError) -> Self {
        EtherealSignatureError::TermInstantiationError(e)
    }
}

pub type EtherealSignatureResult<T> = Result<T, EtherealSignatureError>;
pub type EtherealSignatureMaybeResult<T> = MaybeResult<T, EtherealSignatureError>;
