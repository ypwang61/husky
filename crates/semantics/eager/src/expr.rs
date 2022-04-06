mod gen_instructions;
mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
pub use opn::*;
pub(crate) use parser::EagerExprParser;

use entity_route::EntityRoutePtr;
use text::TextRange;
use vm::{CompiledRoutine, EagerContract, InstructionId, InstructionSource, PrimitiveValue};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub ty: EntityRoutePtr,
    pub kind: EagerExprKind,
    pub instruction_id: InstructionId,
    pub contract: EagerContract,
}

impl InstructionSource for EagerExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerExprKind {
    Variable(CustomIdentifier),
    This,
    Scope {
        scope: EntityRoutePtr,
        compiled: Option<CompiledRoutine>,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(Arc<EagerExpr>),
    Opn {
        opn_kind: EagerOpnKind,
        compiled: Option<CompiledRoutine>,
        opds: Vec<Arc<EagerExpr>>,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<EntityRoutePtr>)>,
        Box<EagerExpr>,
    ),
}
