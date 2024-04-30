use crate::split::cross_solver::CrossResult;
use crate::x_segment::XSegment;

pub(super) struct CrossSegment {
    pub(super) other: XSegment,
    pub(super) cross: CrossResult
}

pub(super) trait ScanSplitStore {
    fn intersect_and_remove_other(&mut self, this: XSegment) -> Option<CrossSegment>;

    fn insert(&mut self, segment: XSegment);

    fn clear(&mut self);
}