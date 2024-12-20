mod history;

use crate::*;
use husky_devsoul::helpers::DevsoulKiControlFlow;
use husky_hir_eager_expr::{HirEagerExprIdx, HirEagerStmtIdx};
use husky_vmir::{expr::VmirExprIdx, stmt::VmirStmtIdx, storage::IsVmirStorage};

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn eager_expr_trace_value(
        &self,
        biological_parent: Trace,
        hir_eager_expr_idx: Option<HirEagerExprIdx>,
        pedestal: Devsoul::Pedestal,
    ) -> Option<DevsoulVmControlFlowFrozen<Devsoul>> {
        let db = self.db();
        let history = self.trace_history(biological_parent, pedestal);
        let linket = history.linket();
        let linket_vmir_region =
            self.vmir_storage
                .linket_vmir_region(linket, db, self.runtime.comptime().linktime())?;
        Some(
            history
                .expr_record(linket_vmir_region[hir_eager_expr_idx?])?
                .control_flow()
                .clone(),
        )
    }

    pub fn eager_stmt_trace_value(
        &self,
        biological_parent: Trace,
        hir_eager_stmt_idx: Option<HirEagerStmtIdx>,
        pedestal: Devsoul::Pedestal,
    ) -> Option<DevsoulVmControlFlowFrozen<Devsoul>> {
        let db = self.db();
        let history = self.trace_history(biological_parent, pedestal);
        let linket = history.linket();
        let linket_vmir_region =
            self.vmir_storage
                .linket_vmir_region(linket, db, self.runtime.comptime().linktime())?;
        Some(
            history
                .stmt_record(linket_vmir_region[hir_eager_stmt_idx?])?
                .control_flow()
                .clone(),
        )
    }
}
