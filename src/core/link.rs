use i_float::int::point::IntPoint;
use crate::core::fill_rule::FillRule;
use crate::core::filter::{ClipFilter, DifferenceFilter, StringClipInsideBoundaryExcludedFilter, StringClipInsideBoundaryIncludedFilter, FillerFilter, InclusionBooleanFilterStrategy, IntersectFilter, InverseDifferenceFilter, StringClipOutsideBoundaryExcludedFilter, StringClipOutsideBoundaryIncludedFilter, SubjectFilter, UnionFilter, XorFilter, InclusionStringFilterStrategy};
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::fill::solver::{FillSolver, FillStrategy};
use crate::geom::id_point::IdPoint;
use crate::segm::segment::{Segment, SegmentFill};
use crate::segm::shape_count::ShapeCount;
use crate::split::solver::SplitSegments;
use crate::string::clip::ClipRule;

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
        Self::build_boolean_all(segments, fill_rule, solver)
    }

    #[inline]
    pub(super) fn build_with_filler_filter(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        Self::build_boolean::<FillerFilter>(segments, fill_rule, solver)
    }

    #[inline]
    pub(super) fn build_with_overlay_filter(segments: Vec<Segment>, fill_rule: FillRule, overlay_rule: OverlayRule, solver: Solver) -> Vec<OverlayLink> {
        match overlay_rule {
            OverlayRule::Subject => Self::build_boolean::<SubjectFilter>(segments, fill_rule, solver),
            OverlayRule::Clip => Self::build_boolean::<ClipFilter>(segments, fill_rule, solver),
            OverlayRule::Intersect => Self::build_boolean::<IntersectFilter>(segments, fill_rule, solver),
            OverlayRule::Union => Self::build_boolean::<UnionFilter>(segments, fill_rule, solver),
            OverlayRule::Difference => Self::build_boolean::<DifferenceFilter>(segments, fill_rule, solver),
            OverlayRule::InverseDifference => Self::build_boolean::<InverseDifferenceFilter>(segments, fill_rule, solver),
            OverlayRule::Xor => Self::build_boolean::<XorFilter>(segments, fill_rule, solver),
        }
    }

    pub(crate) fn build_string_all(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        if segments.is_empty() { return vec![]; }
        let segments = segments.split_segments(solver);
        if segments.is_empty() { return vec![]; }
        let fills = Self::fill_string(&segments, fill_rule, solver);

        Self::build_all_links(&segments, &fills)
    }

    pub(crate) fn build_string_with_clip_rule(segments: Vec<Segment>, fill_rule: FillRule, clip_rule: ClipRule, solver: Solver) -> Vec<OverlayLink> {
        if segments.is_empty() { return vec![]; }
        let segments = segments.split_segments(solver);
        if segments.is_empty() { return vec![]; }
        let fills = Self::fill_string(&segments, fill_rule, solver);

        match clip_rule {
            ClipRule { invert: true, boundary_included: true } => Self::build_clip_string_links::<StringClipOutsideBoundaryIncludedFilter>(&segments, &fills),
            ClipRule { invert: true, boundary_included: false } => Self::build_clip_string_links::<StringClipOutsideBoundaryExcludedFilter>(&segments, &fills),
            ClipRule { invert: false, boundary_included: true } => Self::build_clip_string_links::<StringClipInsideBoundaryIncludedFilter>(&segments, &fills),
            ClipRule { invert: false, boundary_included: false } => Self::build_clip_string_links::<StringClipInsideBoundaryExcludedFilter>(&segments, &fills),
        }
    }

    fn fill_string(segments: &[Segment], fill_rule: FillRule, solver: Solver) -> Vec<SegmentFill> {
        let is_list = solver.is_list_fill(segments);
        match fill_rule {
            FillRule::EvenOdd => FillSolver::fill::<EvenOddStrategyString>(is_list, &segments),
            FillRule::NonZero => FillSolver::fill::<NonZeroStrategyString>(is_list, &segments),
            FillRule::Positive => FillSolver::fill::<PositiveStrategyString>(is_list, &segments),
            FillRule::Negative => FillSolver::fill::<NegativeStrategyString>(is_list, &segments),
        }
    }

    fn fill_boolean(segments: &[Segment], fill_rule: FillRule, solver: Solver) -> Vec<SegmentFill> {
        let is_list = solver.is_list_fill(segments);
        match fill_rule {
            FillRule::EvenOdd => FillSolver::fill::<EvenOddStrategy>(is_list, segments),
            FillRule::NonZero => FillSolver::fill::<NonZeroStrategy>(is_list, segments),
            FillRule::Positive => FillSolver::fill::<PositiveStrategy>(is_list, segments),
            FillRule::Negative => FillSolver::fill::<NegativeStrategy>(is_list, segments),
        }
    }

    fn build_boolean<F: InclusionBooleanFilterStrategy>(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        if segments.is_empty() { return vec![]; }
        let segments = segments.split_segments(solver);
        if segments.is_empty() { return vec![]; }
        let fills = Self::fill_boolean(&segments, fill_rule, solver);
        Self::build_boolean_links::<F>(&segments, &fills)
    }

    fn build_boolean_all(segments: Vec<Segment>, fill_rule: FillRule, solver: Solver) -> Vec<OverlayLink> {
        if segments.is_empty() { return vec![]; }
        let segments = segments.split_segments(solver);
        if segments.is_empty() { return vec![]; }
        let fills = Self::fill_boolean(&segments, fill_rule, solver);
        Self::build_all_links(&segments, &fills)
    }

    fn build_boolean_links<F: InclusionBooleanFilterStrategy>(segments: &[Segment], fills: &[SegmentFill]) -> Vec<OverlayLink> {
        let n = fills.iter().fold(0, |s, &fill| s + F::is_included(fill) as usize);

        let empty_id = IdPoint::new(0, IntPoint::ZERO);
        let empty_link = OverlayLink::new(empty_id, empty_id, 0);
        let mut links = vec![empty_link; n];

        let mut i = 0;
        for (j, &fill) in fills.iter().enumerate() {
            if !F::is_included(fill) {
                continue;
            }
            let (segment, link) = unsafe { (segments.get_unchecked(j), links.get_unchecked_mut(i)) };
            *link = OverlayLink::new(IdPoint::new(0, segment.x_segment.a), IdPoint::new(0, segment.x_segment.b), fill);

            i += 1;
        }

        links
    }

    fn build_clip_string_links<F: InclusionStringFilterStrategy>(segments: &[Segment], fills: &[SegmentFill]) -> Vec<OverlayLink> {
        let n = fills.iter().fold(0, |s, &fill| s + F::is_included(fill) as usize);

        let empty_id = IdPoint::new(0, IntPoint::ZERO);
        let empty_link = OverlayLink::new(empty_id, empty_id, 0);
        let mut links = vec![empty_link; n];

        let mut i = 0;
        for (j, &fill) in fills.iter().enumerate() {
            if !F::is_included(fill) {
                continue;
            }
            let (segment, link) = unsafe { (segments.get_unchecked(j), links.get_unchecked_mut(i)) };
            let s = segment.count.clip.signum();
            let dir_fill = (s + 1) as u8;

            *link = OverlayLink::new(IdPoint::new(0, segment.x_segment.a), IdPoint::new(0, segment.x_segment.b), dir_fill);

            i += 1;
        }

        links
    }

    fn build_all_links(segments: &[Segment], fills: &[SegmentFill]) -> Vec<OverlayLink> {
        let empty_id = IdPoint::new(0, IntPoint::ZERO);
        let empty_link = OverlayLink::new(empty_id, empty_id, 0);
        let mut links = vec![empty_link; fills.len()];

        let mut i = 0;
        for (j, &fill) in fills.iter().enumerate() {
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