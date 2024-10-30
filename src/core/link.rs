use i_float::int::point::IntPoint;
use crate::core::fill_rule::FillRule;
use crate::core::filter::{ClipFilter, DifferenceFilter, FillerFilter, InclusionFilterStrategy, IntersectFilter, InverseDifferenceFilter, NoneFilter, SubjectFilter, UnionFilter, XorFilter};
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::fill::solver::{FillSolver, FillStrategy};
use crate::geom::id_point::IdPoint;
use crate::segm::segment::{Segment, SegmentFill};
use crate::segm::shape_count::ShapeCount;
use crate::split::solver::SplitSegments;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OverlayLink {
    pub(crate) a: IdPoint,
    pub(crate) b: IdPoint,
    pub(crate) fill: SegmentFill,
}

impl OverlayLink {
    #[inline(always)]
    pub(crate) fn new(a: IdPoint, b: IdPoint, fill: SegmentFill) -> OverlayLink {
        OverlayLink { a, b, fill }
    }

    #[inline(always)]
    pub(crate) fn other(&self, node_id: usize) -> IdPoint {
        if self.a.id == node_id { self.b } else { self.a }
    }

    #[inline(always)]
    pub(crate) fn is_direct(&self) -> bool {
        self.a.point < self.b.point
    }
}

pub(crate) struct OverlayLinkBuilder;

impl OverlayLinkBuilder {
    #[inline]
    pub(crate) fn build_without_filter(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        Self::build_base::<NoneFilter>(segments, fill_rule, solver)
    }

    #[inline]
    pub(super) fn build_with_filler_filter(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        Self::build_base::<FillerFilter>(segments, fill_rule, solver)
    }

    #[inline]
    pub(super) fn build_with_overlay_filter(segments: Vec<Segment>, fill_rule: FillRule, overlay_rule: OverlayRule, solver: Solver) -> Vec<OverlayLink> {
        match overlay_rule {
            OverlayRule::Subject => Self::build_base::<SubjectFilter>(segments, fill_rule, solver),
            OverlayRule::Clip => Self::build_base::<ClipFilter>(segments, fill_rule, solver),
            OverlayRule::Intersect => Self::build_base::<IntersectFilter>(segments, fill_rule, solver),
            OverlayRule::Union => Self::build_base::<UnionFilter>(segments, fill_rule, solver),
            OverlayRule::Difference => Self::build_base::<DifferenceFilter>(segments, fill_rule, solver),
            OverlayRule::InverseDifference => Self::build_base::<InverseDifferenceFilter>(segments, fill_rule, solver),
            OverlayRule::Xor => Self::build_base::<XorFilter>(segments, fill_rule, solver),
        }
    }

    pub(crate) fn build_string(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        if segments.is_empty() {
            return vec![];
        }

        let segments = segments.split_segments(solver);

        let is_list = solver.is_list_fill(&segments);

        let fills = match fill_rule {
            FillRule::EvenOdd => FillSolver::fill::<EvenOddStrategyString>(is_list, &segments),
            FillRule::NonZero => FillSolver::fill::<NonZeroStrategyString>(is_list, &segments),
            FillRule::Positive => FillSolver::fill::<PositiveStrategyString>(is_list, &segments),
            FillRule::Negative => FillSolver::fill::<NegativeStrategyString>(is_list, &segments),
        };

        Self::build_links::<NoneFilter>(&segments, &fills)
    }

    fn build_base<F: InclusionFilterStrategy>(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        if segments.is_empty() {
            return vec![];
        }

        let segments = segments.split_segments(solver);

        let is_list = solver.is_list_fill(&segments);

        let fills = match fill_rule {
            FillRule::EvenOdd => FillSolver::fill::<EvenOddStrategy>(is_list, &segments),
            FillRule::NonZero => FillSolver::fill::<NonZeroStrategy>(is_list, &segments),
            FillRule::Positive => FillSolver::fill::<PositiveStrategy>(is_list, &segments),
            FillRule::Negative => FillSolver::fill::<NegativeStrategy>(is_list, &segments),
        };

        Self::build_links::<F>(&segments, &fills)
    }

    fn build_links<F: InclusionFilterStrategy>(segments: &[Segment], fills: &[SegmentFill]) -> Vec<OverlayLink> {
        let n = fills.iter().fold(0, |s, &fill| s + F::is_included(fill) as usize);

        let empty_id = IdPoint::new(0, IntPoint::ZERO);
        let empty_link = OverlayLink::new(empty_id, empty_id, 0);
        let mut links = vec![empty_link; n];

        let mut i = 0;
        for (j, &fill) in fills.into_iter().enumerate() {
            if !F::is_included(fill) {
                continue;
            }
            let (segment, link) = unsafe { (segments.get_unchecked(j), links.get_unchecked_mut(i)) };
            *link = OverlayLink::new(IdPoint::new(0, segment.x_segment.a), IdPoint::new(0, segment.x_segment.b), fill);

            i += 1;
        }

        links
    }
}

struct EvenOddStrategy;
struct NonZeroStrategy;
struct PositiveStrategy;
struct NegativeStrategy;

impl FillStrategy for EvenOddStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = 1 & top.subj as SegmentFill;
        let subj_bot = 1 & bot.subj as SegmentFill;
        let clip_top = 1 & top.clip as SegmentFill;
        let clip_bot = 1 & bot.clip as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}

impl FillStrategy for NonZeroStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = (top.subj != 0) as SegmentFill;
        let subj_bot = (bot.subj != 0) as SegmentFill;
        let clip_top = (top.clip != 0) as SegmentFill;
        let clip_bot = (bot.clip != 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}

impl FillStrategy for PositiveStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = (top.subj < 0) as SegmentFill;
        let subj_bot = (bot.subj < 0) as SegmentFill;
        let clip_top = (top.clip < 0) as SegmentFill;
        let clip_bot = (bot.clip < 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}

impl FillStrategy for NegativeStrategy {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let top = bot.add(this);
        let subj_top = (top.subj > 0) as SegmentFill;
        let subj_bot = (bot.subj > 0) as SegmentFill;
        let clip_top = (top.clip > 0) as SegmentFill;
        let clip_bot = (bot.clip > 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (clip_top << 2) | (clip_bot << 3);

        (top, fill)
    }
}

pub(crate) struct EvenOddStrategyString;
pub(crate) struct NonZeroStrategyString;
pub(crate) struct PositiveStrategyString;
pub(crate) struct NegativeStrategyString;

impl FillStrategy for EvenOddStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = 1 & top.subj as SegmentFill;
        let subj_bot = 1 & bot.subj as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}

impl FillStrategy for NonZeroStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = (top.subj != 0) as SegmentFill;
        let subj_bot = (bot.subj != 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}

impl FillStrategy for PositiveStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = (top.subj < 0) as SegmentFill;
        let subj_bot = (bot.subj < 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}

impl FillStrategy for NegativeStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = (top.subj > 0) as SegmentFill;
        let subj_bot = (bot.subj > 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}