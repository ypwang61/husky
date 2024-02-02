use super::*;

#[salsa::tracked(db = DecTermDb, jar = DecTermJar)]
pub struct ListDecTerm {
    pub toolchain: Toolchain,
    #[return_ref]
    pub items: Vec<DecTerm>,
}

impl ListDecTerm {
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &SymbolDecTermNameMap,
    ) -> std::fmt::Result {
        f.write_str("[")?;
        let items = self.items(db);
        for (i, item) in items.iter().enumerate() {
            item.display_fmt_with_db_and_ctx(f, db, ctx)?;
            if i < items.len() {
                f.write_str(", ")?
            }
        }
        f.write_str("]")
    }
}