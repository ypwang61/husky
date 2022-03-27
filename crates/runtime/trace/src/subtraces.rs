use semantics_eager::EagerOpnKind;

use crate::*;

impl<'eval> Trace<'eval> {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        match self.kind {
            TraceKind::Main(_)
            | TraceKind::FeatureStmt(_)
            | TraceKind::FeatureBranch(_)
            | TraceKind::Input(_)
            | TraceKind::StrictDeclStmt { .. }
            | TraceKind::ImprStmt { .. }
            | TraceKind::LoopFrame { .. }
            | TraceKind::CallHead { .. } => None,
            TraceKind::FeatureExpr(ref expr) => match expr.kind {
                FeatureExprKind::PrimitiveLiteral(_)
                | FeatureExprKind::PrimitiveBinaryOpr { .. }
                | FeatureExprKind::Variable { .. } => None,
                FeatureExprKind::FuncCall { .. } | FeatureExprKind::ProcCall { .. } => {
                    Some(SubtracesContainerClass::Call)
                }
                FeatureExprKind::StructMembVarAccess { .. } => todo!(),
                FeatureExprKind::EnumLiteral { .. } => todo!(),
                FeatureExprKind::MembFuncCall {
                    memb_ident,
                    ref opds,
                    ref instruction_sheet,
                    ref stmts,
                    ..
                } => todo!(),
                FeatureExprKind::MembProcCall {
                    memb_ident,
                    ref opds,
                    ref instruction_sheet,
                    ref stmts,
                    ..
                } => todo!(),
                FeatureExprKind::MembPattCall {
                    memb_ident,
                    ref opds,
                    ref instruction_sheet,
                    ref stmts,
                } => todo!(),
                FeatureExprKind::ScopedFeature { .. } => todo!(),
                FeatureExprKind::ClassCall { ty, ref opds, .. } => todo!(),
                FeatureExprKind::RecordMembAccess {
                    ref this,
                    memb_ident,
                    ..
                } => todo!(),
                FeatureExprKind::This { ref repr } => todo!(),
            },
            TraceKind::EagerExpr { ref expr, .. } => match expr.kind {
                EagerExprKind::Variable(_)
                | EagerExprKind::Scope { .. }
                | EagerExprKind::PrimitiveLiteral(_) => None,
                EagerExprKind::Opn {
                    ref opn_kind,
                    ref opds,
                    ..
                } => match opn_kind {
                    EagerOpnKind::MembVarAccess { .. } | EagerOpnKind::ElementAccess => None,
                    EagerOpnKind::Binary { .. }
                    | EagerOpnKind::Prefix { .. }
                    | EagerOpnKind::Suffix { .. } => {
                        if opds[0].ty.is_builtin() {
                            None
                        } else {
                            Some(SubtracesContainerClass::Call)
                        }
                    }
                    EagerOpnKind::RoutineCall { .. } | EagerOpnKind::MembRoutineCall { .. } => {
                        Some(SubtracesContainerClass::Call)
                    }
                    EagerOpnKind::PatternCall => panic!(),
                    EagerOpnKind::TypeCall { .. } => todo!(),
                },
                EagerExprKind::Lambda(_, _) => todo!(),
                EagerExprKind::Bracketed(_) => panic!(),
                EagerExprKind::This => todo!(),
            },
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}
