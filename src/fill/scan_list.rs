use i_float::point::IntPoint;
use crate::fill::count_segment::CountSegment;
use crate::fill::fill_segments::ScanFillStore;
use crate::split::shape_count::ShapeCount;
use crate::util::{Int, SwapRemoveIndex};
use crate::x_segment::XSegment;

pub(super) struct ScanFillList {
    buffer: Vec<CountSegment>,
}

impl ScanFillList {
    #[inline(always)]
    pub(super) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanFillStore for ScanFillList {
    #[inline(always)]
    fn insert(&mut self, segment: CountSegment) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, p: IntPoint) -> ShapeCount {
        let mut best = CountSegment {
            count: ShapeCount { subj: 0, clip: 0 },
            x_segment: XSegment { a: IntPoint { x: p.x, y: i32::MIN }, b: IntPoint { x: p.x + 1, y: i32::MIN } },
        };

        let mut i = 0;
        while i < self.buffer.len() {
            let item = &self.buffer[i];
            if item.x_segment.b.x <= p.x {
                self.buffer.swap_remove_index(i);
            } else {
                if item.x_segment.is_under_point(p) && best.x_segment.is_under_segment(&item.x_segment) {
                    best = item.clone();
                }

                i += 1
            }
        }

        best.count
    }
}