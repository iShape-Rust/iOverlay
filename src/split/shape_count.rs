use crate::bool::fill::ShapeType;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShapeCount {
    pub(crate) subj: i32,
    pub(crate) clip: i32
}

impl ShapeCount {

    pub(super) fn is_empty(self) -> bool { self.subj < 0 && self.clip < 0 }

    pub(super) fn is_even(self) -> bool { self.subj % 2 == 0 && self.clip % 2 == 0 }

    pub(super) fn new(subj: i32, clip: i32) -> ShapeCount { ShapeCount { subj, clip } }

    pub(super) fn add(self, count: ShapeCount) -> ShapeCount {
        let subj = self.subj + count.subj;
        let clip = self.clip + count.clip;

        ShapeCount { subj, clip }
    }

    pub(super) fn increment(self, shape: ShapeType) -> ShapeCount {
        let subj = if ShapeType::SUBJECT.0 & shape.0 != 0 { 1 + self.subj } else { self.subj };
        let clip = if ShapeType::CLIP.0 & shape.0 != 0 { 1 + self.clip } else { self.clip };
        
        ShapeCount { subj, clip }
    }
}