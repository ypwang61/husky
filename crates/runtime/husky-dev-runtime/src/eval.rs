use crate::*;

use husky_entity_path::TypeVariantIndex;
use husky_hir_opr::binary::HirBinaryOpr;
use husky_opr::{BinaryClosedOpr, BinaryComparisonOpr};
use husky_task::{
    dev_ascension::{dev_eval_context, with_runtime_and_base_point, IsDevAscension},
    helpers::{TaskError, TaskValue},
    IsTask,
};
use husky_task_interface::{val_control_flow::ValControlFlow, IsLinkageImpl};
use husky_task_interface::{val_repr::ValArgumentReprInterface, value::IsValue};
use husky_term_prelude::literal::TermLiteral;
use husky_val::{ValOpn, ValPatternData};
use husky_val_repr::repr::{ValArgumentRepr, ValDomainRepr, ValRepr};

impl<Task: IsTask> DevRuntime<Task> {
    pub fn eval_val_repr_at_pedestal(
        &self,
        val_repr: ValRepr,
        pedestal: TaskDevPedestal<Task>,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        with_runtime_and_base_point::<TaskDevAscension<Task>, _, _>(self, pedestal, || {
            self.eval_val_repr(val_repr)
        })
    }

    pub fn eval_val_domain_repr_at_pedestal(
        &self,
        val_domain_repr: ValDomainRepr,
        pedestal: TaskDevPedestal<Task>,
    ) -> ValControlFlow<(), Infallible, TaskError<Task>> {
        use husky_task_interface::value::IsValue;
        match val_domain_repr {
            ValDomainRepr::Omni => ValControlFlow::Continue(()),
            ValDomainRepr::ConditionSatisfied(condition_val_repr) => {
                match self.eval_val_repr_at_pedestal(condition_val_repr, pedestal) {
                    ValControlFlow::Continue(value) => match value.to_bool() {
                        true => ValControlFlow::Continue(()),
                        false => ValControlFlow::Undefined,
                    },
                    ValControlFlow::LoopContinue => todo!(),
                    ValControlFlow::LoopExit(_) => todo!(),
                    ValControlFlow::Return(_) => todo!(),
                    ValControlFlow::Undefined => ValControlFlow::Undefined,
                    ValControlFlow::Err(_) => todo!(),
                }
            }
            ValDomainRepr::ConditionNotSatisfied(condition_val_repr) => {
                match self.eval_val_repr_at_pedestal(condition_val_repr, pedestal) {
                    ValControlFlow::Continue(value) => match value.to_bool() {
                        true => ValControlFlow::Undefined,
                        false => ValControlFlow::Continue(()),
                    },
                    ValControlFlow::LoopContinue => todo!(),
                    ValControlFlow::LoopExit(_) => todo!(),
                    ValControlFlow::Return(_) => todo!(),
                    ValControlFlow::Undefined => ValControlFlow::Undefined,
                    ValControlFlow::Err(_) => todo!(),
                }
            }
            ValDomainRepr::StmtNotReturned(stmt_val_repr) => {
                match self.eval_val_repr_at_pedestal(stmt_val_repr, pedestal) {
                    ValControlFlow::Continue(_) => ValControlFlow::Continue(()),
                    ValControlFlow::LoopContinue => todo!(),
                    ValControlFlow::LoopExit(_) => todo!(),
                    ValControlFlow::Return(_) | ValControlFlow::Undefined => {
                        ValControlFlow::Undefined
                    }
                    ValControlFlow::Err(_) => todo!(),
                }
            }
            ValDomainRepr::ExprNotReturned(_) => todo!(),
        }
    }

    fn eval_val_repr(
        &self,
        val_repr: ValRepr,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        // todo: consider domain
        let db = self.db();
        let result = match val_repr.opn(db) {
            ValOpn::Return => todo!(),
            ValOpn::Require => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let ValArgumentRepr::Ordinary(condition) = arguments[0] else {
                    unreachable!()
                };
                if self.eval_val_repr(condition)?.to_bool() {
                    ValControlFlow::Continue(().into())
                } else {
                    let ValArgumentRepr::Ordinary(default) = arguments[1] else {
                        unreachable!()
                    };
                    ValControlFlow::Return(self.eval_val_repr(default)?)
                }
            }
            ValOpn::Assert => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 1);
                let ValArgumentRepr::Ordinary(condition) = arguments[0] else {
                    unreachable!()
                };
                if !self.eval_val_repr(condition)?.to_bool() {
                    todo!()
                }
                ValControlFlow::Continue(().into())
            }
            ValOpn::Literal(lit) => {
                // ad hoc
                let db = self.db();
                let value: TaskValue<Task> = match lit {
                    TermLiteral::Unit(_) => ().into(),
                    TermLiteral::Bool(b) => b.into(),
                    TermLiteral::I8(i) => i.into(),
                    TermLiteral::I16(i) => i.into(),
                    TermLiteral::I32(i) => i.into(),
                    TermLiteral::I64(lit) => lit.value(db).into(),
                    TermLiteral::I128(lit) => lit.value(db).into(),
                    TermLiteral::ISize(lit) => (lit.value(db) as isize).into(),
                    TermLiteral::U8(i) => i.into(),
                    TermLiteral::U16(i) => i.into(),
                    TermLiteral::U32(i) => i.into(),
                    TermLiteral::U64(lit) => lit.value(db).into(),
                    TermLiteral::U128(lit) => lit.value(db).into(),
                    TermLiteral::USize(lit) => (lit.value(db) as usize).into(),
                    TermLiteral::R8(_) => todo!(),
                    TermLiteral::R16(_) => todo!(),
                    TermLiteral::R32(_) => todo!(),
                    TermLiteral::R64(_) => todo!(),
                    TermLiteral::R128(_) => todo!(),
                    TermLiteral::RSize(_) => todo!(),
                    TermLiteral::Nat(_) => todo!(),
                    TermLiteral::F32(lit) => lit.value(db).into_inner().into(),
                    TermLiteral::F64(lit) => lit.value(db).into_inner().into(),
                    TermLiteral::String(_) => todo!(),
                    TermLiteral::StaticLifetime => todo!(),
                };
                ValControlFlow::Continue(value)
            }
            ValOpn::ValItemLazilyDefined(_path) => {
                let expansion = val_repr.expansion(db).unwrap();
                self.eval_root_stmts(expansion.root_hir_lazy_stmt_val_reprs(db))
            }
            ValOpn::Linkage(linkage) => {
                let linkage_impl = self.comptime.linkage_impl(linkage);
                let control_flow = linkage_impl.eval(
                    val_repr.into(),
                    dev_eval_context::<Task::DevAscension>(),
                    unsafe {
                        std::mem::transmute::<_, &[ValArgumentReprInterface]>(
                            val_repr.arguments(db) as &[ValArgumentRepr],
                        )
                    },
                );
                control_flow
            }
            ValOpn::FunctionGn(_) => todo!(),
            ValOpn::Prefix(_) => todo!(),
            ValOpn::Suffix(_) => todo!(),
            ValOpn::Binary(opr) => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let ValArgumentRepr::Ordinary(lopd) = arguments[0] else {
                    unreachable!()
                };
                let ValArgumentRepr::Ordinary(ropd) = arguments[1] else {
                    unreachable!()
                };
                match opr {
                    HirBinaryOpr::Closed(opr) => {
                        let lopd = self.eval_val_repr(lopd)?;
                        let ropd = self.eval_val_repr(ropd)?;
                        ValControlFlow::Continue(
                            match opr {
                                BinaryClosedOpr::Add => lopd + ropd,
                                BinaryClosedOpr::BitAnd => lopd & ropd,
                                BinaryClosedOpr::BitOr => lopd | ropd,
                                BinaryClosedOpr::BitXor => lopd ^ ropd,
                                BinaryClosedOpr::Div => lopd / ropd,
                                BinaryClosedOpr::Mul => lopd * ropd,
                                BinaryClosedOpr::RemEuclid => todo!(),
                                BinaryClosedOpr::Power => todo!(),
                                BinaryClosedOpr::Sub => lopd - ropd,
                            }
                            .into(),
                        )
                    }
                    HirBinaryOpr::Shift(_) => todo!(),
                    HirBinaryOpr::Assign => todo!(),
                    HirBinaryOpr::AssignClosed(_) => todo!(),
                    HirBinaryOpr::AssignShift(_) => todo!(),
                    HirBinaryOpr::Comparison(opr) => {
                        let lopd = self.eval_val_repr(lopd)?;
                        let ropd = self.eval_val_repr(ropd)?;
                        ValControlFlow::Continue(
                            match opr {
                                BinaryComparisonOpr::Eq => lopd == ropd,
                                BinaryComparisonOpr::Neq => lopd != ropd,
                                BinaryComparisonOpr::Geq => lopd >= ropd,
                                BinaryComparisonOpr::Greater => lopd > ropd,
                                BinaryComparisonOpr::Leq => lopd <= ropd,
                                BinaryComparisonOpr::Less => lopd < ropd,
                            }
                            .into(),
                        )
                    }
                    HirBinaryOpr::ShortCircuitLogic(_) => todo!(),
                }
            }
            ValOpn::EvalDiscarded => todo!(),
            ValOpn::Branches => {
                for val_argument_repr in val_repr.arguments(db) {
                    let ValArgumentRepr::Branch {
                        condition,
                        ref stmts,
                    } = *val_argument_repr
                    else {
                        unreachable!()
                    };
                    if let Some(condition) = condition {
                        if !self.eval_val_repr(condition)?.to_bool() {
                            continue;
                        }
                    }
                    return self.eval_stmts(stmts);
                }
                ValControlFlow::Continue(().into())
            }
            ValOpn::TypeVariant(path) => match path.index(db) {
                TypeVariantIndex::U8(index_raw) => {
                    let presenter = self
                        .comptime
                        .linkage_impl(Linkage::new_enum_u8_presenter(path.parent_ty_path(db), db))
                        .enum_u8_value_presenter();
                    ValControlFlow::Continue(TaskValue::<Task>::from_enum_u8(index_raw, presenter))
                }
            },
            ValOpn::Be { pattern_data } => {
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 1);
                let ValArgumentRepr::Ordinary(src) = arguments[0] else {
                    unreachable!()
                };
                let src = self.eval_val_repr(src)?;
                ValControlFlow::Continue(
                    match pattern_data {
                        ValPatternData::None => src.is_none(),
                        ValPatternData::Some => src.is_some(),
                    }
                    .into(),
                )
            }
            ValOpn::Unwrap {} => {
                use husky_print_utils::p;
                let pedestal =
                    <TaskDevAscension<Task> as IsDevAscension>::dev_eval_context_local_key()
                        .get()
                        .expect("`DEV_EVAL_CONTEXT` not set")
                        .pedestal();
                p!(pedestal);
                p!(val_repr.source(db).debug_info(db));
                todo!()
            }
            ValOpn::Index => {
                // ad hoc
                let arguments: &[_] = val_repr.arguments(db);
                debug_assert_eq!(arguments.len(), 2);
                let ValArgumentRepr::Ordinary(owner) = arguments[0] else {
                    unreachable!()
                };
                let owner = self.eval_val_repr(owner)?;
                let ValArgumentRepr::Ordinary(index) = arguments[1] else {
                    unreachable!()
                };
                let index = self.eval_val_repr(index)?.to_usize();
                ValControlFlow::Continue(owner.index(index))
            }
        };
        result
    }

    fn eval_root_stmts(
        &self,
        stmt_val_reprs: &[ValRepr],
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        match self.eval_stmts(stmt_val_reprs) {
            ValControlFlow::Continue(value) | ValControlFlow::Return(value) => {
                ValControlFlow::Continue(value)
            }
            ValControlFlow::LoopContinue => unreachable!(),
            ValControlFlow::LoopExit(_) => unreachable!(),
            ValControlFlow::Undefined => unreachable!(),
            ValControlFlow::Err(e) => ValControlFlow::Err(e),
        }
    }

    fn eval_stmts(
        &self,
        stmt_val_reprs: &[ValRepr],
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        for &stmt_val_repr in &stmt_val_reprs[..stmt_val_reprs.len() - 1] {
            let _: () = self.eval_val_repr(stmt_val_repr)?.into();
        }
        self.eval_val_repr(*stmt_val_reprs.last().unwrap())
    }

    fn eval_val_argument(
        &self,
        val_argument_repr: &ValArgumentRepr,
    ) -> ValControlFlow<TaskValue<Task>, TaskValue<Task>, TaskError<Task>> {
        match *val_argument_repr {
            ValArgumentRepr::Ordinary(val_repr) => self.eval_val_repr(val_repr),
            ValArgumentRepr::Keyed(_) => todo!(),
            ValArgumentRepr::Variadic(_) => todo!(),
            ValArgumentRepr::Branch {
                condition: _,
                stmts: _,
            } => todo!(),
            ValArgumentRepr::RuntimeConstants(_) => todo!(),
        }
    }
}

#[test]
fn val_repr_eval_works() {
    use husky_entity_kind::FugitiveKind;
    use husky_entity_path::*;
    use husky_entity_syn_tree::helpers::paths::module_item_paths;
    use husky_ml_task::MlTask;
    use husky_ml_task_interface::InputId;
    use husky_path_utils::dev_paths::*;

    let dev_paths = HuskyLangDevPaths::new();
    let runtime = DevRuntime::new(
        MlTask::<()>::new(),
        dev_paths.dev_root().join("examples/mnist-classifier"),
        None,
    )
    .unwrap();
    let db = runtime.db();
    let DevComptimeTarget::SingleCrate(crate_path) = runtime.comptime_target() else {
        unreachable!()
    };
    for &item_path in module_item_paths(db, crate_path.root_module_path(db)) {
        let ItemPath::MajorItem(MajorItemPath::Fugitive(fugitive_path)) = item_path else {
            continue;
        };
        if fugitive_path.fugitive_kind(db) != FugitiveKind::Val {
            continue;
        }
        let val_repr = ValRepr::new_val_item(fugitive_path, db);
        runtime.eval_val_repr_at_pedestal(val_repr, InputId::from_index(0).into());
    }
}
