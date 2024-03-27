use crate::x_segment::XSegment;
use crate::split::shape_edge_cross::EdgeCross;
use crate::split::version_index::VersionedIndex;
use crate::split::version_segment::VersionSegment;

pub(super) struct CrossSegment {
    pub(super) index: VersionedIndex,
    pub(super) cross: EdgeCross
}

pub(super) trait ScanSplitStore {
    fn intersect(&mut self, this: XSegment) -> Option<CrossSegment>;

    fn insert(&mut self, segment: VersionSegment);

    fn clear(&mut self);
}