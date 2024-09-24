use husky_linket::linket::Linket;
use husky_linket_impl::linket_impl::{IsLinketImpl, LinketImplFrozenValue};
use husky_vmir::stmt::VmirStmtIdx;
use vec_like::{ordered_vec_map::OrderedVecPairMap, SmallVecPairMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VmSnapshotKey {}

pub struct VmSnapshot<LinketImpl: IsLinketImpl> {
    linket: Linket,
    place_frozen_values: Vec<LinketImplFrozenValue<LinketImpl>>,
}

impl<LinketImpl: IsLinketImpl> VmSnapshot<LinketImpl> {
    pub fn linket(&self) -> Linket {
        self.linket
    }

    pub fn place_frozen_values(&self) -> &[LinketImplFrozenValue<LinketImpl>] {
        &self.place_frozen_values
    }
}

pub(crate) type VmSnapshotsData<LinketImpl: IsLinketImpl> = SmallVecPairMap<
    VmirStmtIdx<LinketImpl>,
    OrderedVecPairMap<VmSnapshotKey, VmSnapshot<LinketImpl>>,
    4,
>;

pub struct VmSnapshots<LinketImpl: IsLinketImpl> {
    data: VmSnapshotsData<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> VmSnapshots<LinketImpl> {
    pub fn new(data: VmSnapshotsData<LinketImpl>) -> Self {
        Self { data }
    }
}