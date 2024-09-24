use crate::runtime::IsVmRuntime;
use crate::vm::{Vm, VmMode};
use crate::*;
use history::VmHistory;
use husky_linket_impl::linket_impl::LinketImplThawedValue;
use husky_linktime::helpers::LinktimeThawedValue;
use husky_vmir::stmt::{VmirStmtIdx, VmirStmtIdxRange};

pub fn eval_linket_on_arguments<LinketImpl, VmRuntime: IsVmRuntime<LinketImpl>>(
    linket: Linket,
    arguments: Vec<LinketImpl::Value>,
    mode: VmMode,
    db: &::salsa::Db,
    runtime: &VmRuntime,
    vmir_storage: &impl IsVmirStorage<LinketImpl>,
) -> Option<(
    LinketImplVmControlFlowThawed<LinketImpl>,
    VmHistory<LinketImpl>,
)>
where
    LinketImpl: IsLinketImpl,
{
    let vmir_region = vmir_storage.linket_vmir_region(linket, db, runtime.linktime())?;
    let mut vm = vm::Vm::new_fresh(
        linket,
        arguments,
        mode,
        &vmir_region,
        db,
        runtime,
        vmir_storage,
    );
    let cf = vmir_region.root_expr().eval(None, &mut vm);
    let history = vm.to_history();
    Some((cf, history))
}

impl<'a, LinketImpl, Runtime, VmirStorage> EvalVmir<'a, LinketImpl>
    for Vm<'a, LinketImpl, Runtime, VmirStorage>
where
    LinketImpl: IsLinketImpl,
    Runtime: IsVmRuntime<LinketImpl>,
    VmirStorage: IsVmirStorage<LinketImpl>,
{
    fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    fn vmir_region(&self) -> &'a VmirRegion<LinketImpl> {
        self.vmir_region
    }

    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => {
                // todo: do something?
                // set something to indicate the the expr starts to eval (children included)
                f(self)
            }
        }
    }

    fn eval_expr_itself(
        &mut self,
        expr: VmirExprIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => self.record_expr(expr, f),
        }
    }

    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => {
                // todo: do something?
                f(self)
            }
        }
    }

    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => self.record_stmt(stmt, f),
        }
    }

    fn access_place(
        &mut self,
        place_idx: PlaceIdx,
        qual: LinQual,
    ) -> LinketImplThawedValue<LinketImpl> {
        match qual {
            LinQual::Ref => todo!(),
            LinQual::RefMut => todo!(),
            LinQual::Transient => todo!(),
        }
    }

    fn init_place(&mut self, place_idx: PlaceIdx, value: LinketImplThawedValue<LinketImpl>) {
        self.place_thawed_values[place_idx.index()] = value
    }

    fn eval_val(
        &self,
        major_form_path: husky_entity_path::path::major_item::form::MajorFormPath,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        self.runtime.eval_val(major_form_path)
    }
}