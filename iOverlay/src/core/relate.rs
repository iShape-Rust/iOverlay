use crate::build::sweep::{FillHandler, SweepRunner};
use crate::core::fill_rule::FillRule;
use crate::core::predicate::{InteriorsIntersectHandler, IntersectsHandler, TouchesHandler, WithinHandler};
use crate::core::solver::Solver;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use crate::split::solver::SplitSolver;
use alloc::vec::Vec;

/// Overlay structure optimized for spatial predicate evaluation.
///
/// `PredicateOverlay` provides efficient spatial relationship testing between
/// two polygon sets without computing full boolean operation results. It is
/// designed for cases where you only need to know *whether* shapes intersect,
/// not *what* the intersection looks like.
///
/// # Example
///
/// ```ignore
/// use i_overlay::core::relate::PredicateOverlay;
/// use i_overlay::core::overlay::ShapeType;
/// use i_overlay::segm::build::BuildSegments;
///
/// let mut overlay = PredicateOverlay::new(16);
/// // Add subject and clip segments...
/// let intersects = overlay.intersects();
/// ```
///
/// For float coordinates, prefer using [`FloatPredicateOverlay`](crate::float::relate::FloatPredicateOverlay)
/// or the [`FloatRelate`](crate::float::relate::FloatRelate) trait.
pub struct PredicateOverlay {
    /// Solver configuration for segment operations.
    pub solver: Solver,
    /// Fill rule for determining polygon interiors.
    pub fill_rule: FillRule,
    pub(crate) segments: Vec<Segment<ShapeCountBoolean>>,
    pub(crate) split_solver: SplitSolver,
    sweep_runner: SweepRunner<ShapeCountBoolean>,
}

impl PredicateOverlay {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            solver: Default::default(),
            fill_rule: FillRule::EvenOdd,
            segments: Vec::with_capacity(capacity),
            split_solver: SplitSolver::new(),
            sweep_runner: SweepRunner::new(),
        }
    }

    fn evaluate<H: FillHandler<Output = bool>>(&mut self, handler: H) -> bool {
        if self.segments.is_empty() {
            return false;
        }
        self.split_solver.split_segments(&mut self.segments, &self.solver);
        if self.segments.is_empty() {
            return false;
        }
        self.sweep_runner
            .run_with_fill_rule(self.fill_rule, &self.solver, &self.segments, handler)
    }

    /// Returns `true` if the subject and clip shapes intersect (share any point).
    #[inline]
    pub fn intersects(&mut self) -> bool {
        self.evaluate(IntersectsHandler)
    }

    /// Returns `true` if the interiors of subject and clip shapes overlap.
    ///
    /// Unlike `intersects()`, this returns `false` for shapes that only share
    /// boundary points (edges or vertices) without interior overlap.
    #[inline]
    pub fn interiors_intersect(&mut self) -> bool {
        self.evaluate(InteriorsIntersectHandler)
    }

    /// Returns `true` if subject and clip shapes touch (boundaries intersect but interiors don't).
    ///
    /// This returns `true` when shapes share boundary points (edges or vertices)
    /// but their interiors don't overlap.
    #[inline]
    pub fn touches(&mut self) -> bool {
        self.evaluate(TouchesHandler::new())
    }

    /// Returns `true` if subject is completely within clip.
    ///
    /// Subject is within clip if everywhere the subject has fill, the clip
    /// also has fill on the same side.
    #[inline]
    pub fn within(&mut self) -> bool {
        self.evaluate(WithinHandler::new())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.segments.clear();
    }
}
