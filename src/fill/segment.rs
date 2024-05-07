use crate::x_segment::XSegment;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;

pub type SegmentFill = u8;

pub const NONE: SegmentFill = 0;

pub const SUBJ_TOP: SegmentFill = 0b0001;
pub const SUBJ_BOTTOM: SegmentFill = 0b0010;
pub const CLIP_TOP: SegmentFill = 0b0100;
pub const CLIP_BOTTOM: SegmentFill = 0b1000;

pub const SUBJ_BOTH: SegmentFill = SUBJ_TOP | SUBJ_BOTTOM;
pub const CLIP_BOTH: SegmentFill = CLIP_TOP | CLIP_BOTTOM;
pub const BOTH_TOP: SegmentFill = SUBJ_TOP | CLIP_TOP;
pub const BOTH_BOTTOM: SegmentFill = SUBJ_BOTTOM | CLIP_BOTTOM;

pub const ALL: SegmentFill = SUBJ_BOTH | CLIP_BOTH;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub(crate) seg: XSegment,
    pub(crate) count: ShapeCount,
    pub(crate) fill: SegmentFill,
}

impl Segment {

    #[inline(always)]
    pub(crate) fn new(edge: &ShapeEdge) -> Self {
        Self {
            seg: edge.x_segment,
            count: edge.count,
            fill: NONE,
        }
    }
}