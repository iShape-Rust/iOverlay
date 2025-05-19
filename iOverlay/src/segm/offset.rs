use crate::core::overlay::ShapeType;
use crate::segm::winding::WindingCount;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeCountOffset {
    pub(crate) subj: i32,
    pub(crate) bold: bool,
}

impl WindingCount for ShapeCountOffset {
    #[inline(always)]
    fn is_not_empty(&self) -> bool { self.subj != 0 }

    #[inline(always)]
    fn new(subj: i32, _: i32) -> Self {
        Self {subj, bold: true}
    }

    #[inline(always)]
    fn with_shape_type(shape_type: ShapeType) -> (Self, Self) {
        match shape_type {
            ShapeType::Subject => (Self {subj: 1, bold: true}, Self {subj: -1, bold: true}),
            ShapeType::Clip => (Self {subj: 0, bold: true}, Self {subj: 0, bold: true}),
        }
    }

    #[inline(always)]
    fn add(self, count: Self) -> Self {
        let subj = self.subj + count.subj;
        let bold = self.bold || count.bold;
        Self {subj, bold}
    }

    #[inline(always)]
    fn apply(&mut self, count: Self) {
        self.subj += count.subj;
        self.bold = self.bold || count.bold;
    }

    #[inline(always)]
    fn invert(self) -> Self {
        let subj = -self.subj;
        Self {subj, bold: self.bold}
    }
}