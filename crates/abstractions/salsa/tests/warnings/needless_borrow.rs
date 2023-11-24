trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: salsa::DbWithJar<Jar> {}

#[salsa::jar(db = Db)]
struct Jar(TokenTree);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Token {}

impl salsa::DebugWithDb<dyn Db + '_> for Token {
    fn fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        unreachable!()
    }
}

#[salsa::tracked(db = Db, jar = Jar)]
struct TokenTree {
    #[return_ref]
    tokens: Vec<Token>,
}
