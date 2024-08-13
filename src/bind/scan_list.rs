use i_float::point::IntPoint;
use crate::util::{Int, SwapRemoveIndex};
use crate::bind::segment::IdSegment;
use crate::bind::solver::ScanHoleStore;
use crate::x_segment::XSegment;

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
        let mut best = IdSegment {
            id: 0,
            x_segment: XSegment { a: IntPoint { x: p.x, y: i32::MIN }, b: IntPoint { x: p.x + 1, y: i32::MIN } },
        };

        while i < self.buffer.len() {
            let id_segment = &self.buffer[i];
            if id_segment.x_segment.b.x <= p.x {
                self.buffer.swap_remove_index(i);
            } else {
                if id_segment.x_segment.is_under_point(p) && best.x_segment.is_under_segment(&id_segment.x_segment) {
                    best = id_segment.clone();
                }

                i += 1
            }
        }

        best.id
    }
}