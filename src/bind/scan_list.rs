use i_float::point::IntPoint;
use crate::util::{Int, SwapRemoveIndex};
use crate::bind::segment::IdSegment;
use crate::bind::solver::ScanHoleStore;

pub(crate) struct ScanHoleList {
    buffer: Vec<IdSegment>,
}

impl ScanHoleList {
    #[inline(always)]
    pub(crate) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanHoleStore for ScanHoleList {
    #[inline(always)]
    fn insert(&mut self, segment: IdSegment, _stop: i32) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, p: IntPoint) -> usize {
        let mut i = 0;
        let mut j = usize::MAX;
        while i < self.buffer.len() {
            if self.buffer[i].x_segment.b.x <= p.x {
                self.buffer.swap_remove_index(i);
            } else {
                let segment = &self.buffer[i].x_segment;
                if segment.is_under_point(p) && (j == usize::MAX || unsafe { self.buffer.get_unchecked(j) }.x_segment.is_under_segment(segment)) {
                    j = i;
                }

                i += 1
            }
        }

        if j == usize::MAX {
            0
        } else {
            unsafe { self.buffer.get_unchecked(j) }.id
        }
    }
}