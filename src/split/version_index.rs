use crate::space::dual_index::DualIndex;
use crate::index::EMPTY_INDEX;

#[derive(Copy, Clone)]
pub (super) struct VersionedIndex {
    pub (super) version: usize,
    pub (super) index: DualIndex
}

impl VersionedIndex {
    pub (super) const EMPTY: Self = Self { version: usize::MAX, index: DualIndex::EMPTY };

    pub (super) fn is_not_nil(&self) -> bool {
        self.index.major != EMPTY_INDEX && self.index.minor != EMPTY_INDEX
    }
}