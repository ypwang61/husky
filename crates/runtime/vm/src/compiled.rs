use crate::*;

#[derive(Clone, Copy)]
pub struct CompiledRoutine {
    pub call: for<'stack, 'eval> fn(
        &mut [StackValue<'stack, 'eval>],
    ) -> VMResult<StackValue<'stack, 'eval>>,
}

impl std::fmt::Debug for CompiledRoutine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("Compiled(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for CompiledRoutine {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for CompiledRoutine {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for CompiledRoutine {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MembVarAccessCompiled {}
