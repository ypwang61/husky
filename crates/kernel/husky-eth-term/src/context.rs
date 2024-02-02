mod entry;

use crate::*;
use entry::*;

use vec_like::VecMap;

#[derive(Default)]
pub(crate) struct TermShowContext {
    entries: VecMap<TermSymbolShowEntry>,
}

impl TermShowContext {
    pub(crate) fn fmt_symbol(
        &mut self,
        db: &::salsa::Db,
        symbol: SymbolEthTerm,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(entry) = self.entries.get_entry(symbol) {
            entry.show(db, f)
        } else {
            let new_entry = self.new_external_entry(db, symbol, None);
            new_entry.show(db, f)?;
            self.entries.insert_new(new_entry).unwrap();
            Ok(())
        }
    }

    pub(crate) fn fmt_variable(
        &mut self,
        _db: &::salsa::Db,
        _rune: RuneEthTerm,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_str("variable_ad_hoc_fmt")
    }

    pub(crate) fn fmt_with_symbol(
        &mut self,
        db: &::salsa::Db,
        symbol: SymbolEthTerm,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        self.enter_block(db, symbol);
        f(self)?;
        self.exit_block(symbol);
        Ok(())
    }

    pub(crate) fn fmt_with_variable(
        &mut self,
        _db: &::salsa::Db,
        _variable: RuneEthTerm,
        f: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        // ad hoc
        f(self)
    }
}