use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_entity_syn_tree::HasSynNodePath;
use husky_syn_defn::HasSynDefn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EagerCallTracePathData {
    biological_parent_path: TracePath,
    callee_path: ItemPath,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EagerCallTraceData {
    path: TracePath,
    biological_parent: TraceId,
    callee_path: ItemPath,
}

impl TraceId {
    pub(crate) fn new_eager_call(
        biological_parent_path: TracePath,
        biological_parent: TraceId,
        callee_path: ItemPath,
        db: &::salsa::Db,
    ) -> Self {
        let path = TracePath::new(
            EagerCallTracePathData {
                biological_parent_path: biological_parent_path.into(),
                callee_path,
            },
            db,
        );
        TraceId::new(
            path,
            EagerCallTraceData {
                path,
                biological_parent: biological_parent.into(),
                callee_path,
            }
            .into(),
            db,
        )
    }
}

impl EagerCallTraceData {
    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        let callee_path = self.callee_path;
        TraceViewLines::new(
            callee_path.module_path(db),
            callee_path
                .syn_node_path(db)
                .decl_tokra_region_token_idx_range(db),
            VoidAssociatedTraceRegistry,
            db,
        )
    }

    pub(super) fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        self.callee_path
            .syn_defn(db)
            .expect("no syn error at trace time")
            .body_with_syn_expr_region(db)
            .is_some()
    }

    pub(super) fn subtraces(&self, trace: TraceId, db: &::salsa::Db) -> Vec<TraceId> {
        let biological_parent_path = self.path;
        let biological_parent = trace;
        TraceId::new_eager_stmts_from_syn_body_with_syn_expr_region(
            biological_parent_path,
            biological_parent,
            self.callee_path
                .syn_defn(db)
                .expect("no syn error at trace time")
                .body_with_syn_expr_region(db),
            db,
        )
    }
}
