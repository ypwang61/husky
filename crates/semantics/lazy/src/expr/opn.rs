use std::sync::Arc;

use infer_signature::{MembAccessKind, TySignature};
use scope::{RangedScope, ScopePtr};
use syntax_types::*;
use vm::PureBinaryOpr;
use word::CustomIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyOpnKind {
    Binary {
        opr: PureBinaryOpr,
        this: ScopePtr,
    },
    Prefix(PrefixOpr),
    RoutineCall(RangedScope),
    StructCall(RangedScope),
    ClassCall(RangedScope),
    PatternCall,
    MembAccess {
        memb_ident: CustomIdentifier,
        memb_access_kind: MembAccessKind,
    },
    MembCall {
        memb_ident: CustomIdentifier,
    },
    ElementAccess,
}
