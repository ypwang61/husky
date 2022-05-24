mod binding;
mod contract;
mod control;
mod entity;
mod error;
mod frame;
mod history;
mod instruction;
mod interpreter;
mod liason;
mod linkage;
mod loop_kind;
mod mode;
mod mutation;
mod signature;
mod snapshot;
mod stack;
mod ty;
mod value;

pub use binding::*;
pub use contract::*;
pub use control::{ControlSnapshot, VMControl};
pub use entity::*;
pub use error::*;
pub use frame::{FrameKind, LoopFrameData};
pub use history::{History, HistoryEntry};
pub use instruction::*;
pub use interpreter::{Interpreter, InterpreterQueryGroup};
pub use liason::*;
pub use linkage::Linkage;
pub use loop_kind::{BoundaryKind, LoopStep, VMLoopKind};
pub use mode::Mode;
pub use mutation::*;
pub use signature::*;
pub use snapshot::{StackSnapshot, StackValueSnapshot};
pub use stack::*;
pub use ty::*;
pub use value::*;

use error::*;
use std::sync::Arc;

pub fn eval_fast<'stack, 'eval: 'stack>(
    db: &'stack dyn InterpreterQueryGroup,
    opt_instrn_sheet: Option<&InstructionSheet>,
    maybe_linkage: Option<Linkage>,
    argument_iter: impl Iterator<Item = VMRuntimeResult<StackValue<'stack, 'eval>>>,
) -> EvalResult<'eval> {
    let mut interpreter = Interpreter::try_new(db, argument_iter)?;
    if let Some(linkage) = maybe_linkage {
        interpreter.eval_linkage(linkage)
    } else {
        interpreter.eval_instructions(opt_instrn_sheet.as_ref().unwrap(), Mode::Fast)
    }
}

pub fn exec_debug<'stack, 'eval: 'stack>(
    db: &'stack dyn InterpreterQueryGroup,
    sheet: &InstructionSheet,
    prestack: impl Into<VMStack<'stack, 'eval>>,
) -> Arc<History<'eval>> {
    let mut interpreter = Interpreter::from_prestack(db, prestack);
    interpreter.exec_all(sheet, Mode::TrackHistory);
    Arc::new(interpreter.history)
}

pub fn exec_loop_debug<'stack, 'eval: 'stack>(
    db: &'stack dyn InterpreterQueryGroup,
    loop_kind: VMLoopKind,
    sheet: &InstructionSheet,
    stack_snapshot: &StackSnapshot<'eval>,
) -> Vec<LoopFrameData<'eval>> {
    let mut interpreter = Interpreter::from_prestack(db, stack_snapshot);
    interpreter.exec_loop_tracking_frame(loop_kind, &sheet);
    interpreter.frames
}
