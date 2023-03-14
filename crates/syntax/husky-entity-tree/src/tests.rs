pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_manifest::ManifestJar;
use husky_token::TokenJar;

use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ManifestJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
