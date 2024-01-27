use crate::geom::x_segment::XSegment;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;

pub type SegmentFill = u8;

pub const NONE: SegmentFill = 0;

pub const SUBJECT_TOP: SegmentFill = 0b0001;
pub const SUBJECT_BOTTOM: SegmentFill = 0b0010;
pub const CLIP_TOP: SegmentFill = 0b0100;
pub const CLIP_BOTTOM: SegmentFill = 0b1000;

pub const SUBJECT_BOTH: SegmentFill = SUBJECT_TOP | SUBJECT_BOTTOM;
pub const CLIP_BOTH: SegmentFill = CLIP_TOP | CLIP_BOTTOM;
pub const BOTH_TOP: SegmentFill = SUBJECT_TOP | CLIP_TOP;
pub const BOTH_BOTTOM: SegmentFill = SUBJECT_BOTTOM | CLIP_BOTTOM;

pub const ALL: SegmentFill = SUBJECT_BOTH | CLIP_BOTH;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub(crate) seg: XSegment,
    pub(crate) count: ShapeCount,
    pub(crate) fill: SegmentFill,
}

impl Segment {
    pub(crate) fn new(edge: &ShapeEdge) -> Self {
        Self {
            seg: XSegment::with_edge(edge),
            count: edge.count,
            fill: NONE,
        }
    }

}