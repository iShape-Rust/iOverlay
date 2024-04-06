use i_float::point::Point;
use crate::bind::segment::IdSegment;

pub(crate) trait ScanHoleStore {
    fn insert(&mut self, segment: IdSegment, stop: i32);

    fn find_under_and_nearest(&mut self, p: Point, stop: i32) -> usize;
}