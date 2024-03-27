use i_float::point::Point;
use crate::fill::count_segment::CountSegment;
use crate::split::shape_count::ShapeCount;

pub(crate) trait ScanFillStore {
    fn insert(&mut self, segment: CountSegment, stop: i32);

    fn find_under_and_nearest(&mut self, p: Point, stop: i32) -> Option<ShapeCount>;
}
