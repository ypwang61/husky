pub(crate) use husky_vfs::test_utils::*;

use crate::*;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::db(WordJar, VfsJar, TomlTokenJar, TomlAstJar, CorgiConfigAstJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl Database for DB {}

impl ParallelDatabase for DB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(DB {
            storage: self.storage.snapshot(),
        })
    }
}