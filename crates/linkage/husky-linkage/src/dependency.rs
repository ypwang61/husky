use super::*;
use crate::linkage::{Linkage, LinkagePathData};

#[salsa::tracked(jar = LinkageJar)]
fn linkage_path_dependencies(db: &dyn LinkageDb, linkage_path: Linkage) {
    match linkage_path.data(db) {
        LinkagePathData::Coersion {} => todo!(),
        LinkagePathData::Item {
            path,
            template_arguments,
        } => todo!(),
        LinkagePathData::Field => todo!(),
        LinkagePathData::Index => todo!(),
        LinkagePathData::Method => todo!(),
    }
}