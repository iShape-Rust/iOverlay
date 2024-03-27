use crate::index::EMPTY_INDEX;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DualIndex {
    pub major: usize,
    pub minor: usize
}

impl DualIndex {
    pub const EMPTY: Self = Self { major: usize::MAX, minor: usize::MAX };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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