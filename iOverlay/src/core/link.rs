use crate::core::overlay_rule::OverlayRule;
use crate::geom::id_point::IdPoint;
use crate::segm::segment::SegmentFill;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OverlayLink {
    pub(crate) a: IdPoint,
    pub(crate) b: IdPoint,
    pub(crate) fill: SegmentFill,
}

impl OverlayLink {
    // #[inline(always)]
    // pub(crate) fn empty() -> OverlayLink {
    //     OverlayLink {
    //         a: IdPoint::new(0, IntPoint::EMPTY),
    //         b: IdPoint::new(0, IntPoint::EMPTY),
    //         fill: NONE,
    //     }
    // }

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

pub(crate) trait OverlayLinkFilter {
    fn filter_by_overlay(&self, fill_rule: OverlayRule) -> Vec<bool>;
}

/*
pub(crate) struct OverlayLinkBuilder;

impl OverlayLinkBuilder {
    #[inline]
    pub(crate) fn build_without_filter(
        segments: &[Segment<ShapeCountBoolean>],
        fill_rule: FillRule,
        solver: &Solver,
    ) -> Vec<OverlayLink> {
        Self::build_boolean_all(segments, fill_rule, solver)
    }

    #[inline]
    pub(super) fn build_with_filler_filter(
        segments: &[Segment<ShapeCountBoolean>],
        fill_rule: FillRule,
        solver: &Solver,
    ) -> Vec<OverlayLink> {
        Self::build_boolean::<FillerFilter>(segments, fill_rule, solver)
    }

    #[inline]
    pub(super) fn build_with_overlay_filter(
        segments: &[Segment<ShapeCountBoolean>],
        fill_rule: FillRule,
        overlay_rule: OverlayRule,
        solver: &Solver,
    ) -> Vec<OverlayLink> {
        match overlay_rule {
            OverlayRule::Subject => {
                Self::build_boolean::<SubjectFilter>(segments, fill_rule, solver)
            }
            OverlayRule::Clip => Self::build_boolean::<ClipFilter>(segments, fill_rule, solver),
            OverlayRule::Intersect => {
                Self::build_boolean::<IntersectFilter>(segments, fill_rule, solver)
            }
            OverlayRule::Union => Self::build_boolean::<UnionFilter>(segments, fill_rule, solver),
            OverlayRule::Difference => {
                Self::build_boolean::<DifferenceFilter>(segments, fill_rule, solver)
            }
            OverlayRule::InverseDifference => {
                Self::build_boolean::<InverseDifferenceFilter>(segments, fill_rule, solver)
            }
            OverlayRule::Xor => Self::build_boolean::<XorFilter>(segments, fill_rule, solver),
        }
    }

    pub(crate) fn build_string_all(
        segments: &mut [Segment<ShapeCountString>],
        fill_rule: FillRule,
        solver: &Solver,
    ) -> Vec<OverlayLink> {
        let fills = Self::fill_string(segments, fill_rule, solver);
        Self::build_all_links(segments, &fills)
    }

    pub(crate) fn build_string_with_clip_rule(
        segments: &[Segment<ShapeCountString>],
        fill_rule: FillRule,
        clip_rule: ClipRule,
        solver: &Solver,
    ) -> Vec<OverlayLink> {
        let fills = Self::fill_string(segments, fill_rule, solver);

        match clip_rule {
            ClipRule {
                invert: true,
                boundary_included: true,
            } => Self::build_links::<StringClipOutsideBoundaryIncludedFilter, ShapeCountString>(
                segments, &fills,
            ),
            ClipRule {
                invert: true,
                boundary_included: false,
            } => Self::build_links::<StringClipOutsideBoundaryExcludedFilter, ShapeCountString>(
                segments, &fills,
            ),
            ClipRule {
                invert: false,
                boundary_included: true,
            } => Self::build_links::<StringClipInsideBoundaryIncludedFilter, ShapeCountString>(
                segments, &fills,
            ),
            ClipRule {
                invert: false,
                boundary_included: false,
            } => Self::build_links::<StringClipInsideBoundaryExcludedFilter, ShapeCountString>(
                segments, &fills,
            ),
        }
    }

    fn fill_string(
        segments: &[Segment<ShapeCountString>],
        fill_rule: FillRule,
        solver: &Solver,
    ) -> Vec<SegmentFill> {
        let is_list = solver.is_list_fill(segments);
        match fill_rule {
            FillRule::EvenOdd => {
                GraphBuilder::fill::<EvenOddStrategyString, ShapeCountString>(is_list, segments)
            }
            FillRule::NonZero => {
                GraphBuilder::fill::<NonZeroStrategyString, ShapeCountString>(is_list, segments)
            }
            FillRule::Positive => {
                GraphBuilder::fill::<PositiveStrategyString, ShapeCountString>(is_list, segments)
            }
            FillRule::Negative => {
                GraphBuilder::fill::<NegativeStrategyString, ShapeCountString>(is_list, segments)
            }
        }
    }

    fn fill_boolean(
        segments: &[Segment<ShapeCountBoolean>],
        fill_rule: FillRule,
        solver: &Solver,
    ) -> Vec<SegmentFill> {
        let is_list = solver.is_list_fill(segments);
        match fill_rule {
            FillRule::EvenOdd => {
                GraphBuilder::fill::<EvenOddStrategy, ShapeCountBoolean>(is_list, segments)
            }
            FillRule::NonZero => {
                GraphBuilder::fill::<NonZeroStrategy, ShapeCountBoolean>(is_list, segments)
            }
            FillRule::Positive => {
                GraphBuilder::fill::<PositiveStrategy, ShapeCountBoolean>(is_list, segments)
            }
            FillRule::Negative => {
                GraphBuilder::fill::<NegativeStrategy, ShapeCountBoolean>(is_list, segments)
            }
        }
    }

    #[inline]
    fn build_boolean<F: InclusionFilterStrategy>(
        segments: &[Segment<ShapeCountBoolean>],
        fill_rule: FillRule,
        solver: &Solver,
    ) -> Vec<OverlayLink> {
        let fills = Self::fill_boolean(segments, fill_rule, solver);
        Self::build_links::<F, ShapeCountBoolean>(segments, &fills)
    }

    fn build_boolean_all(
        segments: &[Segment<ShapeCountBoolean>],
        fill_rule: FillRule,
        solver: &Solver,
    ) -> Vec<OverlayLink> {
        let fills = Self::fill_boolean(segments, fill_rule, solver);
        Self::build_all_links(segments, &fills)
    }

    fn build_links<F: InclusionFilterStrategy, C: Send>(
        segments: &[Segment<C>],
        fills: &[SegmentFill],
    ) -> Vec<OverlayLink> {
        let n = fills
            .iter()
            .fold(0, |s, &fill| s + F::is_included(fill) as usize);

        let empty_id = IdPoint::new(0, IntPoint::ZERO);
        let empty_link = OverlayLink::new(empty_id, empty_id, 0);
        let mut links = vec![empty_link; n];

        let mut i = 0;
        for (j, &fill) in fills.iter().enumerate() {
            if !F::is_included(fill) {
                continue;
            }
            let (segment, link) =
                unsafe { (segments.get_unchecked(j), links.get_unchecked_mut(i)) };
            *link = OverlayLink::new(
                IdPoint::new(0, segment.x_segment.a),
                IdPoint::new(0, segment.x_segment.b),
                fill,
            );

            i += 1;
        }

        links
    }

    pub(crate) fn build_all_links<C: Send>(
        segments: &[Segment<C>],
        fills: &[SegmentFill],
    ) -> Vec<OverlayLink> {
        let empty_id = IdPoint::new(0, IntPoint::ZERO);
        let empty_link = OverlayLink::new(empty_id, empty_id, 0);
        let mut links = vec![empty_link; fills.len()];

        for (i, &fill) in fills.iter().enumerate() {
            let (segment, link) =
                unsafe { (segments.get_unchecked(i), links.get_unchecked_mut(i)) };
            *link = OverlayLink::new(
                IdPoint::new(0, segment.x_segment.a),
                IdPoint::new(0, segment.x_segment.b),
                fill,
            );
        }

        links
    }
}

 */