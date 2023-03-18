pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    ExprJar,
    DeclJar,
    TermPreludeJar,
    RawTermJar,
    SignatureJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn module_signatures(db: &DB, module_path: ModulePath) -> Vec<SignatureResult<Signature>> {
    let Ok(decl_sheet) = db.decl_sheet(module_path) else {
        return vec![]
    };
    decl_sheet
        .decls()
        .iter()
        .filter_map(|decl| {
            decl.1
                .as_ref()
                .ok()
                .copied()
                .map(|decl| signature_from_decl(db, decl))
        })
        .collect()
}

#[test]
fn module_signatures_works() {
    DB::default().ast_expect_test_debug_with_db("signature", module_signatures)
}

#[test]
fn menu_ty_signatures_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.i16_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.i32_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.i64_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.u8_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.u16_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.u32_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.u64_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.f32_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.f64_ty_path()).unwrap())
        .is_ok());
    assert!(db
        .ty_signature_from_decl(db.ty_decl(entity_path_menu.trai_ty_path()).unwrap())
        .is_ok());
    // todo: uncomment
    // assert!(db
    //     .ty_signature(db.ty_decl(entity_path_menu.ref_ty_path()).unwrap())
    //     .is_ok());
}
