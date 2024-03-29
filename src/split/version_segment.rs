use i_float::point::Point;
use crate::array::SwapRemoveIndex;
use crate::x_order::XOrder;
use crate::x_segment::XSegment;
use crate::split::version_index::VersionedIndex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VersionSegment {
    pub(super) index: VersionedIndex,
    pub(super) x_segment: XSegment,
}

pub(super) trait RemoveVersionSegment {
    fn remove_segment(&mut self, segment: &VersionSegment, scan_pos: Point);
}

impl RemoveVersionSegment for Vec<VersionSegment> {
    fn remove_segment(&mut self, segment: &VersionSegment, scan_pos: Point) {
        let mut j = 0;
        while j < self.len() {
            let seg = &self[j];
            if seg.x_segment.b.order_by_line_compare(scan_pos) || segment == seg {
                self.swap_remove_index(j);
                continue;
            }

            j += 1;
        }
    }
}