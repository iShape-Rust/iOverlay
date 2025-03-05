use crate::geom::line_range::LineRange;

#[derive(Debug, Clone)]
pub(super) struct VrFragment {
    pub(super) index: usize,
    pub(super) x: i32,
    pub(super) yy: LineRange,
}

#[derive(Debug, Clone)]
pub(super) struct HzFragment {
    pub(super) index: usize,
    pub(super) y: i32,
    pub(super) xx: LineRange,
}

#[derive(Debug, Clone)]
pub(super) struct DgFragment {
    pub(super) index: usize,
    pub(super) y0: i32,
    pub(super) xx: LineRange,
    pub(super) yy: LineRange,
}

impl DgFragment {
    #[inline(always)]
    pub(super) fn pos_b(&self) -> i32 {
        self.y0.wrapping_sub(self.xx.min)
    }
    #[inline(always)]
    pub(super) fn neg_b(&self) -> i32 {
        self.y0.wrapping_add(self.xx.min)
    }
}