use crate::segm::segment::Segment;
use i_key_sort::sort::key_sort::KeySort;

pub(crate) trait ShapeSegmentsSort {
    fn sort_by_ab(&mut self, parallel: bool);
}

impl<C: Send + Sync + Copy> ShapeSegmentsSort for [Segment<C>] {
    #[inline]
    fn sort_by_ab(&mut self, parallel: bool) {
        self.sort_by_two_keys_then_by(
            parallel,
            |s| s.x_segment.a.x,
            |s| s.x_segment.a.y,
            |s0, s1| s0.x_segment.b.cmp(&s1.x_segment.b)
        )
    }
}