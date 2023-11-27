use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{ItemPath, MajorItemPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_entity_syn_tree::HasSynNodePath;
use husky_print_utils::p;
use husky_vfs::SubmodulePath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = TraceDb)]
pub struct SubmoduleTracePathData {
    submodule_path: SubmodulePath,
}

impl TraceId {
    pub fn new_submodule(submodule_path: SubmodulePath, db: &::salsa::Db) -> Option<Self> {
        if !submodule_contains_val_item(db, submodule_path) {
            return None;
        }
        Some(TraceId::new(
            TracePath::new(
                TracePathData::Submodule(SubmoduleTracePathData { submodule_path }),
                db,
            ),
            SubmoduleTraceData { submodule_path }.into(),
            db,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = TraceDb)]
pub struct SubmoduleTraceData {
    submodule_path: SubmodulePath,
}

impl SubmoduleTraceData {
    pub(super) fn view_lines(self, db: &::salsa::Db) -> TraceViewLines {
        let submodule_path = self.submodule_path;
        let token_idx_range = submodule_path
            .syn_node_path(db)
            .decl_tokra_region_token_idx_range(db);
        TraceViewLines::new(
            submodule_path.module_path(db),
            token_idx_range,
            VoidAssociatedTraceRegistry,
            db,
        )
    }

    pub(super) fn have_subtraces(self) -> bool {
        true
    }

    pub(super) fn subtraces(self, db: &::salsa::Db) -> Vec<TraceId> {
        module_item_paths(db, self.submodule_path.inner())
            .iter()
            .filter_map(|&item_path| TraceId::from_item_path(item_path, db))
            .collect()
    }
}

#[salsa::tracked(jar = TraceJar)]
pub(super) fn submodule_contains_val_item(db: &::salsa::Db, submodule_path: SubmodulePath) -> bool {
    for &subitem_path in module_item_paths(db, *submodule_path) {
        match subitem_path {
            ItemPath::Submodule(_, subitem_submodule_path) => {
                if submodule_contains_val_item(db, subitem_submodule_path) {
                    return true;
                }
            }
            ItemPath::MajorItem(MajorItemPath::Fugitive(fugitive_path))
                if fugitive_path.fugitive_kind(db) == FugitiveKind::Val =>
            {
                return true
            }
            _ => (),
        }
    }
    false
}
