use std::num::NonZeroU32;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceId {
    value: NonZeroU32,
}

impl TraceId {
    pub fn new(raw: NonZeroU32) -> Self {
        Self { value: raw }
    }

    pub fn value(&self) -> NonZeroU32 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceKind {
    Submodule,
    ValItem,
    LazyCall,
    LazyCallInput,
    LazyExpr,
    LazyPatternExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerPatternExpr,
    EagerStmt,
    EagerCallInput,
}
