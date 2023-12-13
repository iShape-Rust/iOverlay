use crate::fill::shape_type::ShapeType;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShapeCount {
    pub(crate) subj: i32,
    pub(crate) clip: i32
}

impl ShapeCount {

    pub(super) fn is_empty(self) -> bool { self.subj == 0 && self.clip == 0 }

    pub(crate) fn new(subj: i32, clip: i32) -> ShapeCount { ShapeCount { subj, clip } }

    pub(super) fn add(self, count: ShapeCount) -> ShapeCount {
        let subj = self.subj + count.subj;
        let clip = self.clip + count.clip;

        ShapeCount { subj, clip }
    }
}