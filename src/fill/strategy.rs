use crate::segm::segment::SegmentFill;
use crate::segm::shape_count::ShapeCount;

pub(crate) trait FillStrategy {
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill);
}

pub(super) struct EvenOddStrategy;

pub(super) struct NonZeroStrategy;

pub(super) struct PositiveStrategy;

pub(super) struct NegativeStrategy;

pub(super) struct EvenOddStrategyString;

pub(super) struct NonZeroStrategyString;

pub(super) struct PositiveStrategyString;

pub(super) struct NegativeStrategyString;


impl FillStrategy for EvenOddStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let new_count = sum_count.add(this);
        let fill: SegmentFill = (1 & new_count.subj) as SegmentFill // SUBJ_TOP
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

impl FillStrategy for EvenOddStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = sum_count.subj + this.subj;
        let new_count = ShapeCount { subj, clip: 1 };
        let clip = (this.clip != 0) as u8;
        let fill: SegmentFill = (1 & subj) as SegmentFill // SUBJ_TOP
            | ((1 & sum_count.subj as SegmentFill) << 1) // SUBJ_BOTTOM
            | clip << 2 // CLIP_TOP
            | clip << 3; // CLIP_BOTTOM

        (new_count, fill)
    }
}

impl FillStrategy for NonZeroStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = sum_count.subj + this.subj;
        let new_count = ShapeCount { subj, clip: 1 };
        let clip = (this.clip != 0) as u8;
        let fill: SegmentFill = (new_count.subj != 0) as SegmentFill // SUBJ_TOP
            | ((sum_count.subj != 0) as SegmentFill) << 1 // SUBJ_BOTTOM
            | clip << 2 // CLIP_TOP
            | clip << 3; // CLIP_BOTTOM

        (new_count, fill)
    }
}

impl FillStrategy for PositiveStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = sum_count.subj + this.subj;
        let new_count = ShapeCount { subj, clip: 1 };
        let clip = (this.clip != 0) as u8;
        let fill: SegmentFill = (new_count.subj < 0) as SegmentFill // SUBJ_TOP
            | ((sum_count.subj < 0) as SegmentFill) << 1 // SUBJ_BOTTOM
            | clip << 2 // CLIP_TOP
            | clip << 3; // CLIP_BOTTOM

        (new_count, fill)
    }
}

impl FillStrategy for NegativeStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, sum_count: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = sum_count.subj + this.subj;
        let new_count = ShapeCount { subj, clip: 1 };
        let clip = (this.clip != 0) as u8;
        let fill: SegmentFill = (new_count.subj > 0) as SegmentFill // SUBJ_TOP
            | ((sum_count.subj > 0) as SegmentFill) << 1 // SUBJ_BOTTOM
            | clip << 2 // CLIP_TOP
            | clip << 3; // CLIP_BOTTOM

        (new_count, fill)
    }
}