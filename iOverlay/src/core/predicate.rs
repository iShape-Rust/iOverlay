use crate::build::sweep::FillHandler;
use crate::segm::segment::{
    BOTH_BOTTOM, BOTH_TOP, CLIP_BOTH, CLIP_BOTTOM, CLIP_TOP, SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP, SegmentFill,
};
use core::ops::ControlFlow;

/// Handler that checks if subject and clip shapes intersect (share any point).
///
/// Returns `true` on the first segment where both shapes contribute fill,
/// indicating the geometries share at least one point (interior overlap or boundary contact).
/// This matches the DE-9IM definition of `intersects`.
///
/// This handler is designed for early-exit optimization - it breaks out of the sweep
/// loop as soon as an intersection is detected, avoiding processing of remaining segments.
pub(crate) struct IntersectsHandler;

impl FillHandler for IntersectsHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(&mut self, _index: usize, fill: SegmentFill) -> ControlFlow<bool> {
        // Shapes intersect if both contribute to any segment (interior overlap or boundary contact)
        let has_subj = (fill & SUBJ_BOTH) != 0;
        let has_clip = (fill & CLIP_BOTH) != 0;
        if has_subj && has_clip {
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

/// Handler that checks if the interiors of subject and clip shapes overlap.
///
/// Returns `true` when both shapes have fill on the same side of a segment,
/// indicating their interiors share area. This is stricter than `intersects`
/// which also returns true for boundary-only contact.
///
/// Early-exits `true` on first interior overlap.
pub(crate) struct InteriorsIntersectHandler;

impl FillHandler for InteriorsIntersectHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(&mut self, _index: usize, fill: SegmentFill) -> ControlFlow<bool> {
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
/// Returns `true` if the shapes share boundary points but their interiors don't overlap.
/// Early-exits `false` on first interior overlap.
pub(crate) struct TouchesHandler {
    has_intersection: bool,
}

impl TouchesHandler {
    pub(crate) fn new() -> Self {
        Self {
            has_intersection: false,
        }
    }
}

impl FillHandler for TouchesHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(&mut self, _index: usize, fill: SegmentFill) -> ControlFlow<bool> {
        // Check interior overlap - if found, they don't just touch
        if (fill & BOTH_TOP) == BOTH_TOP || (fill & BOTH_BOTTOM) == BOTH_BOTTOM {
            return ControlFlow::Break(false);
        }
        // Check boundary contact (both shapes contribute but on opposite sides)
        if (fill & SUBJ_BOTH) != 0 && (fill & CLIP_BOTH) != 0 {
            self.has_intersection = true;
        }
        ControlFlow::Continue(())
    }

    #[inline(always)]
    fn finalize(self) -> bool {
        self.has_intersection
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

impl FillHandler for WithinHandler {
    type Output = bool;

    #[inline(always)]
    fn handle(&mut self, _index: usize, fill: SegmentFill) -> ControlFlow<bool> {
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

    #[test]
    fn test_intersects_handler_both_top() {
        let mut handler = IntersectsHandler;
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_intersects_handler_both_bottom() {
        let mut handler = IntersectsHandler;
        let fill = SUBJ_BOTTOM | CLIP_BOTTOM;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_intersects_handler_boundary_contact() {
        // Boundary contact (edge sharing) is still an intersection per DE-9IM
        let mut handler = IntersectsHandler;
        let fill = SUBJ_TOP | CLIP_BOTTOM;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_intersects_handler_no_intersection() {
        // Only subject contributes - no intersection
        let mut handler = IntersectsHandler;
        let fill = SUBJ_TOP;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Continue(())));

        // Only clip contributes - no intersection
        let fill = CLIP_BOTTOM;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
    }

    #[test]
    fn test_intersects_handler_finalize() {
        let handler = IntersectsHandler;
        assert!(!handler.finalize());
    }

    #[test]
    fn test_interiors_intersect_handler_both_top() {
        let mut handler = InteriorsIntersectHandler;
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_interiors_intersect_handler_both_bottom() {
        let mut handler = InteriorsIntersectHandler;
        let fill = SUBJ_BOTTOM | CLIP_BOTTOM;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Break(true)));
    }

    #[test]
    fn test_interiors_intersect_handler_boundary_only() {
        // Boundary contact without interior overlap
        let mut handler = InteriorsIntersectHandler;
        let fill = SUBJ_TOP | CLIP_BOTTOM;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(!handler.finalize());
    }

    #[test]
    fn test_touches_handler_boundary_only() {
        let mut handler = TouchesHandler::new();
        let fill = SUBJ_TOP | CLIP_BOTTOM;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(handler.finalize());
    }

    #[test]
    fn test_touches_handler_interior_overlap() {
        let mut handler = TouchesHandler::new();
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Break(false)));
    }

    #[test]
    fn test_touches_handler_no_contact() {
        let mut handler = TouchesHandler::new();
        let fill = SUBJ_TOP;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(!handler.finalize());
    }

    #[test]
    fn test_within_handler_subject_inside_clip() {
        let mut handler = WithinHandler::new();
        // Subject has top fill, clip also has top fill - subject is within
        let fill = SUBJ_TOP | CLIP_TOP;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(handler.finalize());
    }

    #[test]
    fn test_within_handler_subject_outside_clip() {
        let mut handler = WithinHandler::new();
        // Subject has top fill but clip doesn't - subject is outside
        let fill = SUBJ_TOP;
        let result = handler.handle(0, fill);
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
        let mut handler = WithinHandler::new();
        // Only clip contributes - ok, but need subject present
        let fill = CLIP_TOP;
        let result = handler.handle(0, fill);
        assert!(matches!(result, ControlFlow::Continue(())));
        assert!(!handler.finalize());
    }
}
