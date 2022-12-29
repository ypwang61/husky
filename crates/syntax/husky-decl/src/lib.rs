#![feature(trait_upcasting)]
mod collector;
mod db;
mod decl;
mod error;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use decl::*;
pub use error::*;
use husky_entity_tree::EntityTreeResult;
pub use sheet::*;

use collector::*;
use husky_ast::AstIdx;
use husky_entity_path::*;
use husky_expr::*;
use husky_vfs::{ModulePath, VfsResult};

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    decl_sheet,
    // type
    EnumTypeDecl,
    StructTypeDecl,
    RecordTypeDecl,
    InductiveTypeDecl,
    StructureTypeDecl,
    AlienTypeDecl,
    UnionTypeDecl,
    // form
    ConstantDecl,
    FeatureDecl,
    FunctionDecl,
    MorphismDecl,
    TypeAliasDecl,
    // trait
    TraitDecl,
    // type item
    TypeAssociatedFunctionDecl,
    TypeMethodDecl,
    TypeAssociatedTypeDecl,
    TypeAssociatedValueDecl,
    TypeMemoDecl,
    // trait item
    TraitAssociatedFunctionDecl,
    TraitMethodDecl,
    TraitAssociatedTypeDecl,
    TraitAssociatedValueDecl,
);

#[salsa::tracked(jar = DeclJar, return_ref)]
fn decl_sheet(db: &dyn DeclDb, module_path: ModulePath) -> EntityTreeResult<DeclSheet> {
    Ok(DeclCollector::new(db, module_path)?.collect_all())
}