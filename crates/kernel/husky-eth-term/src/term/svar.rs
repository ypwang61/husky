mod index;
mod set;

pub use self::index::*;

use super::*;
use crate::fmt::symbol_name;
use thiserror::Error;

#[salsa::interned(db = EthTermDb, jar = EthTermJar, constructor = pub new_inner, override_debug)]
pub struct EthSymbolicVariable {
    pub toolchain: Toolchain,
    pub ty: EthTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    /// todo: improve this by adding TypeFamily
    pub index: EthTermSymbolIndex,
}

#[test]
fn term_symbol_size_works() {
    assert_eq!(
        std::mem::size_of::<EthSymbolicVariable>(),
        std::mem::size_of::<u32>()
    );
}

impl EthSymbolicVariable {
    #[inline(always)]
    pub fn from_dec(
        db: &::salsa::Db,
        declarative_term_symbol: DecSymbolicVariable,
    ) -> EthTermResult<Self> {
        let ty = declarative_term_symbol.ty(db)?;
        let ty = EthTerm::ty_from_dec(db, ty)?;
        Ok(Self::new_inner(
            db,
            declarative_term_symbol.toolchain(db),
            ty,
            EthTermSymbolIndex::from_dec(declarative_term_symbol.index(db)),
        ))
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermSymbolTypeErrorKind {
    #[error("signature term error")]
    SignatureTermError,
    #[error("sketch term error")]
    SketchTermError,
}
pub type TermSymbolTypeResult<T> = Result<T, TermSymbolTypeErrorKind>;

impl salsa::DebugWithDb for EthSymbolicVariable {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("EthSymbolicVariable(`{}`)", self.display(db)))
    }
}

impl salsa::DisplayWithDb for EthSymbolicVariable {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        symbol_name(*self, db).display_fmt_with_db(f, db)
    }
}

impl EthInstantiate for EthSymbolicVariable {
    type Output = EthTerm;

    fn instantiate(self, _db: &::salsa::Db, instantiation: &EthInstantiation) -> Self::Output {
        // it's assumed that all symbols will be replaced by its map
        // otherwise it's illegal
        instantiation.symbol_instantiation(self)
    }
}
