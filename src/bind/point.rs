use i_float::point::IntPoint;
use crate::id_point::IdPoint;

pub(crate) trait PathPoint: Copy {
    fn id(&self) -> usize;
    fn point(&self) -> IntPoint;
    fn is_not_exclusion(&self, path_id: usize) -> bool;
}

impl PathPoint for IdPoint {
    #[inline(always)]
    fn id(&self) -> usize {
        self.id
    }

    #[inline(always)]
    fn point(&self) -> IntPoint {
        self.point
    }

    #[inline(always)]
    fn is_not_exclusion(&self, _path_id: usize) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExclusionPathPoint {
    pub(crate) id: usize,
    pub(crate) point: IntPoint,
    pub(crate) exclusion_path: usize,
}

impl PathPoint for ExclusionPathPoint {
    #[inline(always)]
    fn id(&self) -> usize {
        self.id
    }

    #[inline(always)]
    fn point(&self) -> IntPoint {
        self.point
    }

    #[inline(always)]
    fn is_not_exclusion(&self, path_id: usize) -> bool {
        path_id != self.exclusion_path
    }
}

