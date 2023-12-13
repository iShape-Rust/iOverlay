use crate::dual_index::DualIndex;
use crate::index::EMPTY_INDEX;

pub (super) struct VersionedIndex {
    version: usize,
    index: DualIndex
}

impl VersionedIndex {
    pub (super) const EMPTY: Self = Self { version: usize::MAX, index: DualIndex::EMPTY };

    fn is_not_nil(&self) -> bool {
        self.index.major != EMPTY_INDEX && self.index.minor != EMPTY_INDEX
    }
}