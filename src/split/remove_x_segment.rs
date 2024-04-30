use i_float::point::IntPoint;
use crate::util::SwapRemoveIndex;
use crate::x_segment::XSegment;

pub(super) trait RemoveXSegment {
    fn remove_segment(&mut self, segment: &XSegment, scan_pos: IntPoint);
}

impl RemoveXSegment for Vec<XSegment> {
    fn remove_segment(&mut self, segment: &XSegment, scan_pos: IntPoint) {
        let mut j = 0;
        while j < self.len() {
            let seg = &self[j];
            if seg.b < scan_pos || segment == seg {
                self.swap_remove_index(j);
                continue;
            }

            j += 1;
        }
    }
}