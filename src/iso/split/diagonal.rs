use crate::iso::segment::DgSegment;
use crate::iso::split::fragment::DgFragment;

pub(super) trait Diagonal {
    fn pos_b(&self) -> i32;
    fn neg_b(&self) -> i32;
}

impl Diagonal for DgFragment {
    #[inline(always)]
    fn pos_b(&self) -> i32 {
        self.y0.wrapping_sub(self.xx.min)
    }
    #[inline(always)]
    fn neg_b(&self) -> i32 {
        self.y0.wrapping_add(self.xx.min)
    }
}

impl Diagonal for DgSegment {
    #[inline(always)]
    fn pos_b(&self) -> i32 {
        self.y0.wrapping_sub(self.xx.min)
    }
    #[inline(always)]
    fn neg_b(&self) -> i32 {
        self.y0.wrapping_add(self.xx.min)
    }
}

