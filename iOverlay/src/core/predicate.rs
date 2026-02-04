use crate::build::sweep::FillHandler;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::{
    BOTH_BOTTOM, BOTH_TOP, CLIP_BOTH, CLIP_BOTTOM, CLIP_TOP, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP, Segment,
    SegmentFill,
};
use alloc::vec::Vec;
use core::ops::ControlFlow;
use i_float::int::point::IntPoint;
use i_key_sort::sort::two_keys::TwoKeysSort;

/// Collects segment endpoints and checks for coincidence between subject and clip.
///
/// Uses optimized algorithm: collect into separate Vecs, sort with `sort_by_two_keys`,
/// dedup, then binary search from shorter into longer array.
pub(crate) struct PointCoincidenceChecker {
    subj_points: Vec<IntPoint>,
    clip_points: Vec<IntPoint>,
}

impl PointCoincidenceChecker {
    /// Create a new checker with pre-allocated capacity.
    ///
    /// `capacity` is the number of segments; each segment contributes 2 endpoints.
    #[inline]
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            subj_points: Vec::with_capacity(capacity * 2),
            clip_points: Vec::with_capacity(capacity * 2),
        }
    }

    /// Add a segment's endpoints based on its count and fill.
    ///
    /// Uses fill to skip inner segments that can't contribute to boundary coincidence:
    /// - Segments entirely inside subject (SUBJ_BOTH, no clip contribution) with no
    ///   clip in the segment are skipped for clip collection
    /// - Similarly for clip-only interior segments
    #[inline]
    pub(crate) fn add_segment(&mut self, segment: &Segment<ShapeCountBoolean>, fill: SegmentFill) {
        let is_subj = segment.count.subj != 0;
        let is_clip = segment.count.clip != 0;

        // Skip inner segments optimization:
        // If segment is entirely inside one shape's interior (filled on both sides)
        // and has no contribution from the other shape, it's not on a boundary
        // where coincidence could occur.
        let subj_interior = (fill & SUBJ_BOTH) == SUBJ_BOTH;
        let clip_interior = (fill & CLIP_BOTH) == CLIP_BOTH;

        // Add to subj_points if:
        // - Segment belongs to subject AND
        // - Either it's not purely inside subject interior, OR clip is also present
        if is_subj && (!subj_interior || is_clip) {
            self.subj_points.push(segment.x_segment.a);
            self.subj_points.push(segment.x_segment.b);
        }

        // Add to clip_points if:
        // - Segment belongs to clip AND
        // - Either it's not purely inside clip interior, OR subject is also present
        if is_clip && (!clip_interior || is_subj) {
            self.clip_points.push(segment.x_segment.a);
            self.clip_points.push(segment.x_segment.b);
        }
    }

    /// Check if any subject point coincides with any clip point.
    ///
    /// Consumes self and returns true if coincidence found.
    #[inline]
    pub(crate) fn has_coincidence(mut self) -> bool {
        if self.subj_points.is_empty() || self.clip_points.is_empty() {
            return false;
        }

        // Sort using sort_by_two_keys (radix sort for integer keys)
        self.subj_points.sort_by_two_keys(false, |p| p.x, |p| p.y);
        self.clip_points.sort_by_two_keys(false, |p| p.x, |p| p.y);

        // Dedup (segment endpoints appear twice from adjacent segments)
        self.subj_points.dedup();
        self.clip_points.dedup();

        // Binary search from shorter into longer array
        let (shorter, longer) = if self.subj_points.len() <= self.clip_points.len() {
            (&self.subj_points, &self.clip_points)
        } else {
            (&self.clip_points, &self.subj_points)
        };

        shorter.iter().any(|p| longer.binary_search(p).is_ok())
    }
}

/// Handler that checks if subject and clip shapes intersect (share any point).
///
/// Returns `true` on the first segment where both shapes contribute fill,
/// indicating the geometries share at least one point (interior overlap or boundary contact).
/// This matches the DE-9IM definition of `intersects`.
///
/// This handler is designed for early-exit optimization - it breaks out of the sweep
/// loop as soon as an intersection is detected, avoiding processing of remaining segments.
///
/// Also collects endpoint information for point coincidence check in finalize.
pub(crate) struct IntersectsHandler {
    point_checker: PointCoincidenceChecker,
}

impl IntersectsHandler {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            point_checker: PointCoincidenceChecker::new(capacity),
        }
    }
}

impl FillHandler<ShapeCountBoolean> for IntersectsHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(
        &mut self,
        _index: usize,
        segment: &Segment<ShapeCountBoolean>,
        fill: SegmentFill,
    ) -> ControlFlow<bool> {
        // Shapes intersect if both contribute to any segment (interior overlap or boundary contact)
        let has_subj = (fill & SUBJ_BOTH) != 0;
        let has_clip = (fill & CLIP_BOTH) != 0;
        if has_subj && has_clip {
            ControlFlow::Break(true)
        } else {
            self.point_checker.add_segment(segment, fill);
            ControlFlow::Continue(())
        }
    }

    #[inline(always)]
    fn finalize(self) -> bool {
        self.point_checker.has_coincidence()
    }
}

/// Handler that checks if the interiors of subject and clip shapes overlap.
///
/// Returns `true` when both shapes have fill on the same side of a segment,
/// indicating their interiors share area. This is stricter than `intersects`
/// which also returns true for boundary-only contact.
///
/// Early-exits `true` on first interior overlap.
pub(crate) struct InteriorsIntersectHandler;

impl FillHandler<ShapeCountBoolean> for InteriorsIntersectHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(
        &mut self,
        _index: usize,
        _segment: &Segment<ShapeCountBoolean>,
        fill: SegmentFill,
    ) -> ControlFlow<bool> {
        // Interiors intersect if both shapes fill the same side
        if (fill & BOTH_TOP) == BOTH_TOP || (fill & BOTH_BOTTOM) == BOTH_BOTTOM {
            ControlFlow::Break(true)
        } else {
            ControlFlow::Continue(())
        }
    }

    #[inline(always)]
    fn finalize(self) -> bool {
        false
    }
}

/// Handler that checks if subject and clip shapes touch (boundaries intersect but interiors don't).
///
/// Returns `true` if boundaries contact without interior overlap.
/// Early-exits with `false` on first interior overlap since that definitively means
/// the shapes don't just touch.
///
/// Also collects endpoint information for point coincidence check in finalize.
pub(crate) struct TouchesHandler {
    has_boundary_contact: bool,
    point_checker: PointCoincidenceChecker,
}

impl TouchesHandler {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            has_boundary_contact: false,
            point_checker: PointCoincidenceChecker::new(capacity),
        }
    }
}

impl FillHandler<ShapeCountBoolean> for TouchesHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(
        &mut self,
        _index: usize,
        segment: &Segment<ShapeCountBoolean>,
        fill: SegmentFill,
    ) -> ControlFlow<bool> {
        // Interior overlap = not a touch (early exit false)
        if (fill & BOTH_TOP) == BOTH_TOP || (fill & BOTH_BOTTOM) == BOTH_BOTTOM {
            return ControlFlow::Break(false);
        }
        // Track boundary contact
        if (fill & SUBJ_BOTH) != 0 && (fill & CLIP_BOTH) != 0 {
            self.has_boundary_contact = true;
        }
        self.point_checker.add_segment(segment, fill);
        ControlFlow::Continue(())
    }

    #[inline(always)]
    fn finalize(self) -> bool {
        self.has_boundary_contact || self.point_checker.has_coincidence()
    }
}

/// Handler that checks if subject is completely within clip.
///
/// Returns `true` if everywhere the subject has fill, the clip also has fill
/// on the same side. Early-exits `false` on first violation.
pub(crate) struct WithinHandler {
    subj_present: bool,
}

impl WithinHandler {
    pub(crate) fn new() -> Self {
        Self { subj_present: false }
    }
}

impl FillHandler<ShapeCountBoolean> for WithinHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(
        &mut self,
        _index: usize,
        _segment: &Segment<ShapeCountBoolean>,
        fill: SegmentFill,
    ) -> ControlFlow<bool> {
        let subj_top = (fill & SUBJ_TOP) != 0;
        let subj_bot = (fill & SUBJ_BOTTOM) != 0;
        let clip_top = (fill & CLIP_TOP) != 0;
        let clip_bot = (fill & CLIP_BOTTOM) != 0;

        if subj_top || subj_bot {
            self.subj_present = true;
        }

        // Subject filled where clip isn't = not within
        if (subj_top && !clip_top) || (subj_bot && !clip_bot) {
            ControlFlow::Break(false)
        } else {
            ControlFlow::Continue(())
        }
    }

    #[inline(always)]
    fn finalize(self) -> bool {
        // Empty subject is not within anything
        self.subj_present
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::x_segment::XSegment;

    fn make_segment(ax: i32, ay: i32, bx: i32, by: i32, subj: i32, clip: i32) -> Segment<ShapeCountBoolean> {
        Segment {
            x_segment: XSegment {
                a: IntPoint::new(ax, ay),
                b: IntPoint::new(bx, by),
            },
            count: ShapeCountBoolean { subj, clip },
        }
    }

    #[test]
    fn test_point_coincidence_no_points() {
        let checker = PointCoincidenceChecker::new(10);
        assert!(!checker.has_coincidence());
    }

    #[test]
    fn test_point_coincidence_subj_only() {
        let mut checker = PointCoincidenceChecker::new(10);
        checker.add_segment(&make_segment(0, 0, 10, 0, 1, 0), SUBJ_TOP);
        assert!(!checker.has_coincidence());
    }

    #[test]
    fn test_point_coincidence_coincident_point() {
        let mut checker = PointCoincidenceChecker::new(10);
        // Subject segment with endpoint at (10, 10)
        checker.add_segment(&make_segment(0, 0, 10, 10, 1, 0), SUBJ_TOP);
        // Clip segment with endpoint at (10, 10)
        checker.add_segment(&make_segment(10, 10, 20, 20, 0, 1), CLIP_TOP);
        assert!(checker.has_coincidence());
    }

    #[test]
    fn test_point_coincidence_no_coincidence() {
        let mut checker = PointCoincidenceChecker::new(10);
        checker.add_segment(&make_segment(0, 0, 5, 5, 1, 0), SUBJ_TOP);
        checker.add_segment(&make_segment(10, 10, 20, 20, 0, 1), CLIP_TOP);
        assert!(!checker.has_coincidence());
    }

    #[test]
    fn test_point_coincidence_shared_segment() {
        let mut checker = PointCoincidenceChecker::new(10);
        // Segment belonging to both shapes
        checker.add_segment(&make_segment(0, 0, 10, 10, 1, 1), SUBJ_TOP | CLIP_BOTTOM);
        assert!(checker.has_coincidence());
    }

    #[test]
    fn test_point_coincidence_dedup_works() {
        let mut checker = PointCoincidenceChecker::new(10);
        // Two subject segments sharing endpoint (5, 5)
        checker.add_segment(&make_segment(0, 0, 5, 5, 1, 0), SUBJ_TOP);
        checker.add_segment(&make_segment(5, 5, 10, 10, 1, 0), SUBJ_TOP);
        // Clip at (5, 5)
        checker.add_segment(&make_segment(5, 5, 15, 15, 0, 1), CLIP_TOP);
        assert!(checker.has_coincidence());
    }

    #[test]
    fn test_intersects_handler_both_top() {
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = IntersectsHandler::new(10);
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_intersects_handler_both_bottom() {
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = IntersectsHandler::new(10);
        let fill = SUBJ_BOTTOM | CLIP_BOTTOM;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_intersects_handler_boundary_contact() {
        // Boundary contact (edge sharing) is still an intersection per DE-9IM
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = IntersectsHandler::new(10);
        let fill = SUBJ_TOP | CLIP_BOTTOM;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_intersects_handler_no_intersection() {
        // Only subject contributes - no intersection
        let seg = make_segment(0, 0, 10, 0, 1, 0);
        let mut handler = IntersectsHandler::new(10);
        let fill = SUBJ_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Continue(())));

        // Only clip contributes - no intersection
        let seg = make_segment(0, 0, 10, 0, 0, 1);
        let fill = CLIP_BOTTOM;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
    }

    #[test]
    fn test_intersects_handler_finalize_with_coincidence() {
        let mut handler = IntersectsHandler::new(10);
        // Add segments that don't trigger early exit but have point coincidence
        let seg1 = make_segment(0, 0, 10, 10, 1, 0);
        let seg2 = make_segment(10, 10, 20, 20, 0, 1);
        let _ = handler.handle(0, &seg1, SUBJ_TOP);
        let _ = handler.handle(1, &seg2, CLIP_TOP);
        assert!(handler.finalize());
    }

    #[test]
    fn test_interiors_intersect_handler_both_top() {
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = InteriorsIntersectHandler;
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_interiors_intersect_handler_both_bottom() {
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = InteriorsIntersectHandler;
        let fill = SUBJ_BOTTOM | CLIP_BOTTOM;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_interiors_intersect_handler_boundary_only() {
        // Boundary contact without interior overlap
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = InteriorsIntersectHandler;
        let fill = SUBJ_TOP | CLIP_BOTTOM;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(!handler.finalize());
    }

    #[test]
    fn test_touches_handler_boundary_only() {
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = TouchesHandler::new(10);
        let fill = SUBJ_TOP | CLIP_BOTTOM;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(handler.finalize()); // boundary contact, no interior overlap
    }

    #[test]
    fn test_touches_handler_interior_overlap() {
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = TouchesHandler::new(10);
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Break(false))); // early exit on interior overlap
    }

    #[test]
    fn test_touches_handler_no_contact() {
        let seg = make_segment(0, 0, 10, 0, 1, 0);
        let mut handler = TouchesHandler::new(10);
        let fill = SUBJ_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(!handler.finalize()); // no boundary contact, no interior overlap
    }

    #[test]
    fn test_touches_handler_point_coincidence() {
        let mut handler = TouchesHandler::new(10);
        // Add segments that don't touch via fill but have point coincidence
        let seg1 = make_segment(0, 0, 10, 10, 1, 0);
        let seg2 = make_segment(10, 10, 20, 20, 0, 1);
        let _ = handler.handle(0, &seg1, SUBJ_TOP);
        let _ = handler.handle(1, &seg2, CLIP_TOP);
        assert!(handler.finalize());
    }

    #[test]
    fn test_within_handler_subject_inside_clip() {
        let seg = make_segment(0, 0, 10, 0, 1, 1);
        let mut handler = WithinHandler::new();
        // Subject has top fill, clip also has top fill - subject is within
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(handler.finalize());
    }

    #[test]
    fn test_within_handler_subject_outside_clip() {
        let seg = make_segment(0, 0, 10, 0, 1, 0);
        let mut handler = WithinHandler::new();
        // Subject has top fill but clip doesn't - subject is outside
        let fill = SUBJ_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Break(false)));
    }

    #[test]
    fn test_within_handler_empty_subject() {
        let handler = WithinHandler::new();
        // Empty subject is not within anything
        assert!(!handler.finalize());
    }

    #[test]
    fn test_within_handler_clip_only() {
        let seg = make_segment(0, 0, 10, 0, 0, 1);
        let mut handler = WithinHandler::new();
        // Only clip contributes - ok, but need subject present
        let fill = CLIP_TOP;
        let result = handler.handle(0, &seg, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(!handler.finalize());
    }
}
