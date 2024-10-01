use crate::segm::segment::SegmentFill;
use crate::segm::shape_count::ShapeCount;

pub(crate) trait FillStrategy {
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill);
}

pub(super) struct EvenOddStrategy;

pub(super) struct NonZeroStrategy;

pub(super) struct PositiveStrategy;

pub(super) struct NegativeStrategy;


impl FillStrategy for EvenOddStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let new_count = sum_count.add(this);
        let fill: SegmentFill = (1 & new_count.subj as SegmentFill) // SUBJ_TOP
            | ((1 & sum_count.subj as SegmentFill) << 1) // SUBJ_BOTTOM
            | ((1 & new_count.clip as SegmentFill) << 2) // CLIP_TOP
            | ((1 & sum_count.clip as SegmentFill) << 3); // CLIP_BOTTOM

        (new_count, fill)
    }
}

impl FillStrategy for NonZeroStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let new_count = sum_count.add(this);
        let fill: SegmentFill = (new_count.subj != 0) as SegmentFill // SUBJ_TOP
            | ((sum_count.subj != 0) as SegmentFill) << 1 // SUBJ_BOTTOM
            | ((new_count.clip != 0) as SegmentFill) << 2 // CLIP_TOP
            | ((sum_count.clip != 0) as SegmentFill) << 3; // CLIP_BOTTOM

        (new_count, fill)
    }
}

impl FillStrategy for PositiveStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let new_count = sum_count.add(this);
        let fill: SegmentFill = (new_count.subj < 0) as SegmentFill // SUBJ_TOP
            | ((sum_count.subj < 0) as SegmentFill) << 1 // SUBJ_BOTTOM
            | ((new_count.clip < 0) as SegmentFill) << 2 // CLIP_TOP
            | ((sum_count.clip < 0) as SegmentFill) << 3; // CLIP_BOTTOM

        (new_count, fill)
    }
}

impl FillStrategy for NegativeStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let new_count = sum_count.add(this);
        let fill: SegmentFill = (new_count.subj > 0) as SegmentFill // SUBJ_TOP
            | ((sum_count.subj > 0) as SegmentFill) << 1 // SUBJ_BOTTOM
            | ((new_count.clip > 0) as SegmentFill) << 2 // CLIP_TOP
            | ((sum_count.clip > 0) as SegmentFill) << 3; // CLIP_BOTTOM

        (new_count, fill)
    }
}