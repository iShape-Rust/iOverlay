use i_float::point::IntPoint;
use crate::fill::count_segment::CountSegment;
use crate::fill::scan_store::ScanFillStore;
use crate::split::shape_count::ShapeCount;
use crate::util::{Int, SwapRemoveIndex};

pub(crate) struct ScanFillList {
    buffer: Vec<CountSegment>,
}

impl ScanFillList {
    #[inline(always)]
    pub(crate) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanFillStore for ScanFillList {
    #[inline(always)]
    fn insert(&mut self, segment: CountSegment, _stop: i32) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, p: IntPoint) -> Option<ShapeCount> {
        let mut i = 0;
        let mut j = usize::MAX;
        while i < self.buffer.len() {
            if self.buffer[i].x_segment.b.x <= p.x {
                self.buffer.swap_remove_index(i);
            } else {
                let segment = &self.buffer[i].x_segment;
                if segment.is_under_point(p) && (j == usize::MAX || unsafe { &self.buffer.get_unchecked(j) }.x_segment.is_under_segment(segment)) {
                    j = i;
                }

                i += 1
            }
        }

        if j == usize::MAX {
            None
        } else {
            Some(unsafe { self.buffer.get_unchecked(j) }.count)
        }
    }
}