use crate::build::sweep::{FillHandler, SweepRunner};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::predicate::{InteriorsIntersectHandler, IntersectsHandler, TouchesHandler, WithinHandler};
use crate::core::solver::Solver;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::build::BuildSegments;
use crate::segm::segment::Segment;
use crate::split::solver::SplitSolver;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_shape::int::shape::{IntContour, IntShape};

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

    /// Adds a path to the overlay using an iterator, allowing for more flexible path input.
    /// This function is particularly useful when working with dynamically generated paths or
    /// when paths are not directly stored in a collection.
    /// - `iter`: An iterator over references to `IntPoint` that defines the path.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path_iter<I: Iterator<Item = IntPoint>>(&mut self, iter: I, shape_type: ShapeType) {
        self.segments.append_path_iter(iter, shape_type, false);
    }

    /// Adds a single path to the overlay as either subject or clip paths.
    /// - `contour`: An array of points that form a closed path.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_contour(&mut self, contour: &[IntPoint], shape_type: ShapeType) {
        self.segments
            .append_path_iter(contour.iter().copied(), shape_type, false);
    }

    /// Adds multiple paths to the overlay as either subject or clip paths.
    /// - `contours`: An array of `IntContour` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_contours(&mut self, contours: &[IntContour], shape_type: ShapeType) {
        for contour in contours.iter() {
            self.add_contour(contour, shape_type);
        }
    }

    /// Adds a single shape to the overlay as either a subject or clip shape.
    /// - `shape`: A reference to a `IntShape` instance to be added.
    /// - `shape_type`: Specifies the role of the added shape in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_shape(&mut self, shape: &IntShape, shape_type: ShapeType) {
        self.add_contours(shape, shape_type);
    }

    /// Adds multiple shapes to the overlay as either subject or clip shapes.
    /// - `shapes`: An array of `IntShape` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added shapes in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_shapes(&mut self, shapes: &[IntShape], shape_type: ShapeType) {
        for shape in shapes.iter() {
            self.add_contours(shape, shape_type);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.segments.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn square(x: i32, y: i32, size: i32) -> Vec<IntPoint> {
        vec![
            IntPoint::new(x, y),
            IntPoint::new(x, y + size),
            IntPoint::new(x + size, y + size),
            IntPoint::new(x + size, y),
        ]
    }

    #[test]
    fn test_add_contour_intersects() {
        let mut overlay = PredicateOverlay::new(16);
        overlay.add_contour(&square(0, 0, 10), ShapeType::Subject);
        overlay.add_contour(&square(5, 5, 10), ShapeType::Clip);
        assert!(overlay.intersects());
    }

    #[test]
    fn test_add_contour_disjoint() {
        let mut overlay = PredicateOverlay::new(16);
        overlay.add_contour(&square(0, 0, 10), ShapeType::Subject);
        overlay.add_contour(&square(20, 20, 10), ShapeType::Clip);
        assert!(!overlay.intersects());
    }

    #[test]
    fn test_add_contour_touches() {
        let mut overlay = PredicateOverlay::new(16);
        overlay.add_contour(&square(0, 0, 10), ShapeType::Subject);
        overlay.add_contour(&square(10, 0, 10), ShapeType::Clip);
        assert!(overlay.touches());

        overlay.clear();
        overlay.add_contour(&square(0, 0, 10), ShapeType::Subject);
        overlay.add_contour(&square(10, 0, 10), ShapeType::Clip);
        assert!(!overlay.interiors_intersect());
    }

    #[test]
    fn test_add_contour_within() {
        let mut overlay = PredicateOverlay::new(16);
        overlay.add_contour(&square(5, 5, 10), ShapeType::Subject);
        overlay.add_contour(&square(0, 0, 20), ShapeType::Clip);
        assert!(overlay.within());
    }

    #[test]
    fn test_add_contours() {
        let mut overlay = PredicateOverlay::new(16);
        let contours = vec![square(0, 0, 5), square(10, 10, 5)];
        overlay.add_contours(&contours, ShapeType::Subject);
        overlay.add_contour(&square(2, 2, 3), ShapeType::Clip);
        assert!(overlay.intersects());
    }

    #[test]
    fn test_add_shape() {
        let mut overlay = PredicateOverlay::new(16);
        let shape = vec![square(0, 0, 10)];
        overlay.add_shape(&shape, ShapeType::Subject);
        overlay.add_contour(&square(5, 5, 10), ShapeType::Clip);
        assert!(overlay.intersects());
    }

    #[test]
    fn test_add_shapes() {
        let mut overlay = PredicateOverlay::new(16);
        let shapes = vec![vec![square(0, 0, 5)], vec![square(20, 20, 5)]];
        overlay.add_shapes(&shapes, ShapeType::Subject);
        overlay.add_contour(&square(2, 2, 3), ShapeType::Clip);
        assert!(overlay.intersects());
    }

    #[test]
    fn test_add_path_iter() {
        let mut overlay = PredicateOverlay::new(16);
        let points = square(0, 0, 10);
        overlay.add_path_iter(points.into_iter(), ShapeType::Subject);
        overlay.add_contour(&square(5, 5, 10), ShapeType::Clip);
        assert!(overlay.intersects());
    }
}
