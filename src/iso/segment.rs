use crate::geom::line_range::LineRange;
use crate::segm::winding_count::ShapeCountBoolean;

#[derive(Debug, Clone, Copy)]
pub(super) struct VrSegment {
    pub(super) x: i32,
    pub(super) yy: LineRange,
    pub(super) count: ShapeCountBoolean
}

#[derive(Debug, Clone, Copy)]
pub(super) struct HzSegment {
    pub(super) y: i32,
    pub(super) xx: LineRange,
    pub(super) count: ShapeCountBoolean
}

#[derive(Debug, Clone, Copy)]
pub(super) struct DgSegment {
    pub(super) y0: i32,
    pub(super) xx: LineRange,
    pub(super) count: ShapeCountBoolean
}