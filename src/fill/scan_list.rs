use i_float::point::IntPoint;
use crate::fill::count_segment::CountSegment;
use crate::fill::fill_segments::ScanFillStore;
use crate::split::shape_count::ShapeCount;
use crate::util::Int;
use crate::x_segment::XSegment;

pub(super) struct ScanFillList {
    prev_x: i32,
    buffer: Vec<CountSegment>,
}

impl ScanFillList {
    #[inline(always)]
    pub(super) fn new(count: usize) -> Self {
        Self { prev_x: i32::MIN, buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanFillStore for ScanFillList {
    #[inline(always)]
    fn insert(&mut self, segment: CountSegment) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, p: IntPoint) -> ShapeCount {
        self.clean(p.x);

        let mut i = 0;
        let mut best = CountSegment { count: ShapeCount { subj: 0, clip: 0 }, x_segment: XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO } };
        while i < self.buffer.len() {
            let item = &self.buffer[i];
            i += 1;

            if item.x_segment.is_under_point(p) {
                best = item.clone();
                break;
            }
        }

        while i < self.buffer.len() {
            let item = &self.buffer[i];
            if item.x_segment.is_under_point(p) && best.x_segment.is_under_segment(&item.x_segment) {
                best = item.clone();
            }
            i += 1;
        }

        best.count
    }
}

impl ScanFillList {
    #[inline(always)]
    fn clean(&mut self, x: i32) {
        if self.prev_x >= x || self.buffer.is_empty() {
            return;
        }

        self.prev_x = x;

        let mut i = 0;
        while i < self.buffer.len() - 1 {
            if unsafe { self.buffer.get_unchecked(i) }.x_segment.b.x <= x {
                self.buffer.swap_remove(i);
                continue;
            }

            i += 1;
        }

        if unsafe { self.buffer.get_unchecked(i) }.x_segment.b.x <= x {
            self.buffer.pop();
        }
    }
}