#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeCount {
    pub(crate) subj: i32,
    pub(crate) clip: i32,
}

impl ShapeCount {

    #[inline(always)]
    pub(crate) fn is_empty(self) -> bool { self.subj == 0 && self.clip == 0 }

    #[inline(always)]
    pub(crate) fn is_not_empty(self) -> bool { self.subj != 0 || self.clip != 0 }

    #[inline(always)]
    pub fn new(subj: i32, clip: i32) -> ShapeCount { ShapeCount { subj, clip } }

    #[inline(always)]
    pub(crate) fn add(self, count: ShapeCount) -> ShapeCount {
        let subj = self.subj + count.subj;
        let clip = self.clip + count.clip;

        ShapeCount { subj, clip }
    }

    #[inline(always)]
    pub(crate) fn accumulate(&mut self, count: ShapeCount) {
        self.subj += count.subj;
        self.clip += count.clip;
    }

    #[inline(always)]
    pub(crate) fn invert(self) -> ShapeCount {
        ShapeCount { subj: -self.subj, clip: -self.clip }
    }
}