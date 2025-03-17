use i_float::int::point::IntPoint;
use crate::fill::count_segment::CountSegment;
use crate::fill::solver::ScanFillStore;
use crate::segm::winding_count::WindingCount;
use crate::util::log::Int;

pub(super) struct ScanFillList<C> {
    buffer: Vec<CountSegment<C>>,
    min_x: i32,
}

impl<C: WindingCount> ScanFillList<C> {
    #[inline(always)]
    pub(super) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()), min_x: i32::MAX }
    }

    #[inline(always)]
    fn clear(&mut self, x: i32) {
        if self.min_x > x {
            return
        }
        let mut new_min_x = i32::MAX;
        self.buffer.retain(|s| {
            let keep = s.x_segment.b.x > x;
            if keep {
                new_min_x = new_min_x.min(s.x_segment.b.x);
            }
            keep
        });
        self.min_x = new_min_x;
    }
}

impl<C: WindingCount> ScanFillStore<C> for ScanFillList<C> {
    #[inline(always)]
    fn insert(&mut self, segment: CountSegment<C>) {
        self.min_x = self.min_x.min(segment.x_segment.b.x);
        if let Err(index) = self.buffer.binary_search(&segment) {
            self.buffer.insert(index, segment)
        }
    }

    #[inline(always)]
    fn find_under_and_nearest(&mut self, p: IntPoint) -> C {
        self.clear(p.x);
        if let Err(index) = self.buffer.binary_search_by(|s| s.x_segment.is_under_point_order(p)) {
            if index == 0 {
                C::new(0, 0)
            } else {
                unsafe { self.buffer.get_unchecked(index - 1) }.count
            }
        } else {
            debug_assert!(false, "This condition should never occur");
            C::new(0, 0)
        }
    }
}