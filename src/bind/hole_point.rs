use i_float::point::IntPoint;
use crate::id_point::IdPoint;

pub(crate) trait HolePoint {
    fn hole_id(&self) -> usize;
    fn point(&self) -> IntPoint;
    fn filter(&self, path_id: usize) -> bool;
}

impl HolePoint for IdPoint {
    #[inline(always)]
    fn hole_id(&self) -> usize {
        self.id
    }

    #[inline(always)]
    fn point(&self) -> IntPoint {
        self.point
    }

    #[inline(always)]
    fn filter(&self, _path_id: usize) -> bool {
        true
    }
}