use i_float::point::IntPoint;
use crate::bind::hole_point::HolePoint;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct ExclusionIdPoint {
    pub(crate) id: usize,
    pub(crate) exclusion: usize,
    pub(crate) point: IntPoint,
}

impl HolePoint for ExclusionIdPoint {
    #[inline(always)]
    fn hole_id(&self) -> usize {
        self.id
    }

    #[inline(always)]
    fn point(&self) -> IntPoint {
        self.point
    }

    #[inline(always)]
    fn filter(&self, path_id: usize) -> bool {
        path_id != self.exclusion
    }
}