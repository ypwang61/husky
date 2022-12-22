use crate::*;
use husky_word::WordJar;
use salsa::DebugWithDb;

#[salsa::db(WordJar, VfsJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn entity_path_debug_works() {
    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    expect_test::expect![[r#"
        ModuleItemPath {
            [salsa id]: 14,
            module: ModulePath(
                Id {
                    value: 5,
                },
            ),
            ident: Identifier(
                Word(
                    Id {
                        value: 5,
                    },
                ),
            ),
        }
    "#]]
    .assert_debug_eq(&entity_path_menu.i32().debug(&db));
}