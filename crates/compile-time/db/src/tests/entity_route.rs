use entity_route::EntityRouteKind;
use entity_route_query::EntitySource;
use word::RootIdentifier;

use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyLangCompileTime::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A {}

main:
    let a = 1
"#
        .into(),
    );

    let main_file = db.intern_file("haha/main.hsk".into());
    let pack = db.intern_entity_route(EntityRoute::package(
        main_file,
        db.intern_word("haha".into()).opt_custom().unwrap(),
    ));
    let subscope_table = db.subroute_table(pack).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 2);
    should_eq!(subscope_table.errors.len(), 0);
}

#[test]
fn no_error_many_files() {
    let mut db = HuskyLangCompileTime::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
mod husky_lord
struct A {}

main:
    let a = 1
"#
        .into(),
    );
    db.set_live_file_text(
        "haha/husky_lord.hsk".into(),
        r#"
struct B {}
"#
        .into(),
    );

    let main_file = db.intern_file("haha/main.hsk".into());
    let husky_lord_file = db.intern_file("haha/husky_lord.hsk".into());
    let package = db.intern_entity_route(EntityRoute::package(
        main_file,
        db.intern_word("haha".into()).opt_custom().unwrap(),
    ));
    let subroute_table = db.subroute_table(package).ok().unwrap();
    let husky_lord_route =
        db.make_subroute(package, db.intern_word("husky_lord").custom(), Vec::new());
    should_eq!(
        db.entity_source(husky_lord_route).unwrap(),
        EntitySource::Module {
            file: husky_lord_file
        }
    );
    should_eq!(subroute_table.entries.len(), 3);
    should_eq!(subroute_table.errors.len(), 0);
}

#[test]
fn error1() {
    let mut db = HuskyLangCompileTime::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
stru ct A {}

main:
    let a = 1
"#
        .into(),
    );

    let main_file = db.intern_file("haha/main.hsk".into());
    let pack = db.intern_entity_route(EntityRoute::package(
        main_file,
        db.intern_word("haha".into()).opt_custom().unwrap(),
    ));
    let subscope_table = db.subroute_table(pack).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 1);
    should_eq!(subscope_table.errors.len(), 1);
}

#[test]
fn datasets() {
    let db = HuskyLangCompileTime::default();
    let dataset_scope = db.intern_entity_route(EntityRoute {
        kind: EntityRouteKind::Root {
            ident: RootIdentifier::Datasets,
        },
        generic_arguments: vec![],
    });
    let synthetic_scope = db
        .child_route(
            dataset_scope,
            db.intern_word("synthetic").opt_custom().unwrap(),
            vec![],
        )
        .unwrap();
    let synthetic_trivial_scope = db
        .child_route(
            synthetic_scope,
            db.intern_word("trivial").opt_custom().unwrap(),
            vec![],
        )
        .unwrap();
    let _synthetic_trivial_real1d_scope = db
        .child_route(
            synthetic_trivial_scope,
            db.intern_word("real1d").opt_custom().unwrap(),
            vec![],
        )
        .unwrap();
}
