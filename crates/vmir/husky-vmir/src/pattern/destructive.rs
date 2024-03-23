use super::*;
use husky_control_flow_utils::require;
use husky_place::place::idx::PlaceIdx;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

/// takes ownership of the match src, destruct it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirDestructivePatternData<LinkageImpl: IsLinkageImpl> {
    Some,
    Todo(LinkageImpl),
}

pub type VmirDestructivePatternArena<LinkageImpl> = Arena<VmirDestructivePatternData<LinkageImpl>>;
pub type VmirDestructivePatternIdx<LinkageImpl> = ArenaIdx<VmirDestructivePatternData<LinkageImpl>>;
pub type VmirDestructivePatternIdxRange<LinkageImpl> =
    ArenaIdxRange<VmirDestructivePatternData<LinkageImpl>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VmirDestructivePattern<LinkageImpl: IsLinkageImpl> {
    Default(Option<PlaceIdx>) = 1,
    Other(VmirDestructivePatternIdx<LinkageImpl>),
}

#[test]
fn vmir_destructive_pattern_size_works() {
    use husky_linkage::linkage::Linkage;

    assert_eq!(
        std::mem::size_of::<VmirDestructivePattern<Linkage>>(),
        std::mem::size_of::<Option<VmirDestructivePattern<Linkage>>>(),
    )
}

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    pub(super) fn build_destructive_pattern(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> Option<VmirDestructivePatternIdx<Linktime::LinkageImpl>> {
        require!(hir_eager_pattern
            .entry(self.hir_eager_pattern_arena())
            .is_destructive());
        let pattern = self.build_destructive_pattern_aux(hir_eager_pattern);
        Some(self.alloc_destructive_pattern(pattern))
    }

    pub(super) fn build_destructive_pattern_aux(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> VmirDestructivePatternData<Linktime::LinkageImpl> {
        match *self.hir_eager_pattern_arena()[hir_eager_pattern].data() {
            HirEagerPatternData::Literal(_) => todo!(),
            HirEagerPatternData::Ident {
                symbol_modifier,
                ident,
            } => todo!(),
            HirEagerPatternData::Unit(_) => todo!(),
            HirEagerPatternData::Tuple { path, fields } => todo!(),
            HirEagerPatternData::Props { path, fields } => todo!(),
            HirEagerPatternData::OneOf { options } => todo!(),
            HirEagerPatternData::Binding { ident, src } => todo!(),
            HirEagerPatternData::Range { start, end } => todo!(),
            HirEagerPatternData::Some => VmirDestructivePatternData::Some,
        }
    }
}
