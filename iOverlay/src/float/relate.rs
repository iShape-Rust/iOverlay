use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::relate::PredicateOverlay;
use crate::core::solver::Solver;
use crate::segm::build::BuildSegments;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::source::resource::ShapeResource;

/// Float-coordinate wrapper for spatial predicate evaluation.
///
/// `FloatPredicateOverlay` handles conversion from floating-point coordinates to
/// the internal integer representation, then delegates to [`PredicateOverlay`](crate::core::relate::PredicateOverlay)
/// for efficient predicate evaluation.
///
/// # Example
///
/// ```
/// use i_overlay::float::relate::FloatPredicateOverlay;
///
/// let subject = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
/// let clip = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];
///
/// let mut overlay = FloatPredicateOverlay::with_subj_and_clip(&subject, &clip);
/// assert!(overlay.intersects());
/// ```
///
/// For a more ergonomic API, see the [`FloatRelate`] trait which provides
/// methods directly on shape types.
pub struct FloatPredicateOverlay<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(crate) overlay: PredicateOverlay,
    pub(crate) adapter: FloatPointAdapter<P, T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatPredicateOverlay<P, T> {
    /// Creates a new predicate overlay with a pre-configured adapter.
    ///
    /// Use this when you need fixed-scale precision via `FloatPointAdapter::with_scale()`.
    ///
    /// # Arguments
    /// * `adapter` - A `FloatPointAdapter` instance for coordinate conversion.
    /// * `capacity` - Initial capacity for storing segments.
    #[inline]
    pub fn with_adapter(adapter: FloatPointAdapter<P, T>, capacity: usize) -> Self {
        Self {
            overlay: PredicateOverlay::new(capacity),
            adapter,
        }
    }

    /// Creates a new predicate overlay with a pre-configured adapter, fill rule, and solver.
    ///
    /// Use this when you need fixed-scale precision with custom overlay settings.
    ///
    /// # Arguments
    /// * `adapter` - A `FloatPointAdapter` instance for coordinate conversion.
    /// * `fill_rule` - Fill rule to determine filled areas.
    /// * `solver` - Type of solver to use.
    /// * `capacity` - Initial capacity for storing segments.
    #[inline]
    pub fn with_adapter_custom(
        adapter: FloatPointAdapter<P, T>,
        fill_rule: FillRule,
        solver: Solver,
        capacity: usize,
    ) -> Self {
        let mut overlay = PredicateOverlay::new(capacity);
        overlay.fill_rule = fill_rule;
        overlay.solver = solver;
        Self { overlay, adapter }
    }

    /// Creates a new predicate overlay from subject and clip shapes.
    pub fn with_subj_and_clip<R0, R1>(subj: &R0, clip: &R1) -> Self
    where
        R0: ShapeResource<P, T> + ?Sized,
        R1: ShapeResource<P, T> + ?Sized,
    {
        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        let mut result = Self {
            overlay: PredicateOverlay::new(subj_capacity + clip_capacity),
            adapter,
        };
        result.add_source(subj, ShapeType::Subject);
        result.add_source(clip, ShapeType::Clip);
        result
    }

    /// Creates a new predicate overlay with custom solver and fill rule.
    pub fn with_subj_and_clip_custom<R0, R1>(
        subj: &R0,
        clip: &R1,
        fill_rule: FillRule,
        solver: Solver,
    ) -> Self
    where
        R0: ShapeResource<P, T> + ?Sized,
        R1: ShapeResource<P, T> + ?Sized,
    {
        let iter = subj.iter_paths().chain(clip.iter_paths()).flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let subj_capacity = subj.iter_paths().fold(0, |s, c| s + c.len());
        let clip_capacity = clip.iter_paths().fold(0, |s, c| s + c.len());

        let mut overlay = PredicateOverlay::new(subj_capacity + clip_capacity);
        overlay.fill_rule = fill_rule;
        overlay.solver = solver;

        let mut result = Self { overlay, adapter };
        result.add_source(subj, ShapeType::Subject);
        result.add_source(clip, ShapeType::Clip);
        result
    }

    /// Adds a shape resource as subject or clip.
    ///
    /// # Arguments
    /// * `resource` - A `ShapeResource` specifying the geometry to add.
    /// * `shape_type` - Whether to add as `Subject` or `Clip`.
    pub fn add_source<R: ShapeResource<P, T> + ?Sized>(&mut self, resource: &R, shape_type: ShapeType) {
        for contour in resource.iter_paths() {
            self.overlay.segments.append_path_iter(
                contour.iter().map(|p| self.adapter.float_to_int(p)),
                shape_type,
                false,
            );
        }
    }

    /// Clears segments for reuse with new geometry.
    #[inline]
    pub fn clear(&mut self) {
        self.overlay.clear();
    }

    /// Returns `true` if the subject and clip shapes intersect.
    ///
    /// Uses early-exit optimization - returns immediately when the first
    /// intersection is found.
    #[inline]
    pub fn intersects(&mut self) -> bool {
        self.overlay.intersects()
    }

    /// Returns `true` if the interiors of subject and clip shapes overlap.
    #[inline]
    pub fn interiors_intersect(&mut self) -> bool {
        self.overlay.interiors_intersect()
    }

    /// Returns `true` if subject and clip shapes touch (boundaries intersect but interiors don't).
    #[inline]
    pub fn touches(&mut self) -> bool {
        self.overlay.touches()
    }

    /// Returns `true` if subject is completely within clip.
    #[inline]
    pub fn within(&mut self) -> bool {
        self.overlay.within()
    }
}

/// Ergonomic trait for spatial predicate operations on shape resources.
///
/// This trait provides convenient methods for testing spatial relationships
/// directly on contours, shapes, and shape collections without explicit
/// overlay construction.
///
/// # Example
///
/// ```
/// use i_overlay::float::relate::FloatRelate;
///
/// let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
/// let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];
///
/// // Overlapping shapes
/// assert!(square.intersects(&other));
/// assert!(square.interiors_intersect(&other));
/// assert!(!square.touches(&other));
///
/// let distant = vec![[100.0, 100.0], [100.0, 110.0], [110.0, 110.0], [110.0, 100.0]];
///
/// // Non-overlapping shapes (fast bounding-box rejection)
/// assert!(!square.intersects(&distant));
/// assert!(square.disjoint(&distant));
/// ```
///
/// # Supported Types
///
/// This trait is implemented for any type implementing `ShapeResource`, including:
/// - `Vec<[f64; 2]>` - single contour
/// - `Vec<Vec<[f64; 2]>>` - multiple contours (shape with holes)
/// - `Vec<Vec<Vec<[f64; 2]>>>` - multiple shapes
pub trait FloatRelate<R1, P, T>
where
    R1: ShapeResource<P, T> + ?Sized,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Returns `true` if this shape intersects with another (shares any point).
    ///
    /// This method uses bounding-box rejection for fast negative results and
    /// early-exit for fast positive results. It's significantly more efficient
    /// than computing a full intersection when you only need a boolean answer.
    ///
    /// Matches the DE-9IM definition: returns `true` for both interior
    /// overlap and boundary contact (shapes sharing an edge).
    fn intersects(&self, other: &R1) -> bool;

    /// Returns `true` if the interiors of this shape and another overlap.
    ///
    /// Unlike `intersects()`, this returns `false` for shapes that only share
    /// boundary points (edges or vertices) without interior overlap.
    fn interiors_intersect(&self, other: &R1) -> bool;

    /// Returns `true` if this shape touches another (boundaries intersect but interiors don't).
    ///
    /// Returns `true` when shapes share boundary points but their interiors don't overlap.
    fn touches(&self, other: &R1) -> bool;

    /// Returns `true` if this shape is completely within another.
    ///
    /// Subject is within clip if everywhere the subject has fill, the clip
    /// also has fill on the same side.
    fn within(&self, other: &R1) -> bool;

    /// Returns `true` if this shape does not intersect with another (no shared points).
    ///
    /// This is the negation of `intersects()`.
    fn disjoint(&self, other: &R1) -> bool;

    /// Returns `true` if this shape completely covers another.
    ///
    /// `covers(A, B)` is equivalent to `within(B, A)`.
    fn covers(&self, other: &R1) -> bool;
}

impl<R0, R1, P, T> FloatRelate<R1, P, T> for R0
where
    R0: ShapeResource<P, T> + ?Sized,
    R1: ShapeResource<P, T> + ?Sized,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn intersects(&self, other: &R1) -> bool {
        FloatPredicateOverlay::with_subj_and_clip(self, other).intersects()
    }

    #[inline]
    fn interiors_intersect(&self, other: &R1) -> bool {
        FloatPredicateOverlay::with_subj_and_clip(self, other).interiors_intersect()
    }

    #[inline]
    fn touches(&self, other: &R1) -> bool {
        FloatPredicateOverlay::with_subj_and_clip(self, other).touches()
    }

    #[inline]
    fn within(&self, other: &R1) -> bool {
        FloatPredicateOverlay::with_subj_and_clip(self, other).within()
    }

    #[inline]
    fn disjoint(&self, other: &R1) -> bool {
        !self.intersects(other)
    }

    #[inline]
    fn covers(&self, other: &R1) -> bool {
        other.within(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_intersects_overlapping() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(square.intersects(&other));
        assert!(other.intersects(&square));
    }

    #[test]
    fn test_intersects_disjoint() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[20.0, 20.0], [20.0, 30.0], [30.0, 30.0], [30.0, 20.0]];

        assert!(!square.intersects(&other));
        assert!(!other.intersects(&square));
    }

    #[test]
    fn test_intersects_contained() {
        let outer = vec![[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let inner = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(outer.intersects(&inner));
        assert!(inner.intersects(&outer));
    }

    #[test]
    fn test_intersects_touching_edge() {
        let left = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let right = vec![[10.0, 0.0], [10.0, 10.0], [20.0, 10.0], [20.0, 0.0]];

        // Shapes sharing an edge intersect (boundary contact) per DE-9IM
        assert!(left.intersects(&right));
        assert!(right.intersects(&left));
    }

    #[test]
    fn test_intersects_empty() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let empty: Vec<[f64; 2]> = vec![];

        assert!(!square.intersects(&empty));
        assert!(!empty.intersects(&square));
    }

    #[test]
    fn test_disjoint() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[20.0, 20.0], [20.0, 30.0], [30.0, 30.0], [30.0, 20.0]];

        assert!(square.disjoint(&other));
        assert!(other.disjoint(&square));

        let overlapping = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];
        assert!(!square.disjoint(&overlapping));
    }

    #[test]
    fn test_interiors_intersect_overlapping() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(square.interiors_intersect(&other));
        assert!(other.interiors_intersect(&square));
    }

    #[test]
    fn test_interiors_intersect_touching_edge() {
        let left = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let right = vec![[10.0, 0.0], [10.0, 10.0], [20.0, 10.0], [20.0, 0.0]];

        // Shapes sharing an edge don't have interior overlap
        assert!(!left.interiors_intersect(&right));
        assert!(!right.interiors_intersect(&left));
    }

    #[test]
    fn test_interiors_intersect_disjoint() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[20.0, 20.0], [20.0, 30.0], [30.0, 30.0], [30.0, 20.0]];

        assert!(!square.interiors_intersect(&other));
    }

    #[test]
    fn test_touches_edge_sharing() {
        let left = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let right = vec![[10.0, 0.0], [10.0, 10.0], [20.0, 10.0], [20.0, 0.0]];

        // Shapes sharing an edge touch
        assert!(left.touches(&right));
        assert!(right.touches(&left));
    }

    #[test]
    fn test_touches_overlapping() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        // Overlapping shapes don't touch (interiors intersect)
        assert!(!square.touches(&other));
        assert!(!other.touches(&square));
    }

    #[test]
    fn test_touches_disjoint() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[20.0, 20.0], [20.0, 30.0], [30.0, 30.0], [30.0, 20.0]];

        // Disjoint shapes don't touch
        assert!(!square.touches(&other));
    }

    #[test]
    fn test_within_contained() {
        let outer = vec![[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let inner = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(inner.within(&outer));
        assert!(!outer.within(&inner));
    }

    #[test]
    fn test_within_overlapping() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        // Overlapping shapes - neither is within the other
        assert!(!square.within(&other));
        assert!(!other.within(&square));
    }

    #[test]
    fn test_within_disjoint() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[20.0, 20.0], [20.0, 30.0], [30.0, 30.0], [30.0, 20.0]];

        assert!(!square.within(&other));
        assert!(!other.within(&square));
    }

    #[test]
    fn test_within_empty() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let empty: Vec<[f64; 2]> = vec![];

        // Empty is not within anything
        assert!(!empty.within(&square));
        // Non-empty is not within empty
        assert!(!square.within(&empty));
    }

    #[test]
    fn test_covers_contained() {
        let outer = vec![[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let inner = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(outer.covers(&inner));
        assert!(!inner.covers(&outer));
    }

    #[test]
    fn test_covers_overlapping() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        // Neither covers the other
        assert!(!square.covers(&other));
        assert!(!other.covers(&square));
    }

    #[test]
    fn test_covers_empty() {
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let empty: Vec<[f64; 2]> = vec![];

        // Anything covers empty (empty is within anything)
        // But empty.within(square) is false, so square.covers(empty) = empty.within(square) = false
        assert!(!square.covers(&empty));
    }

    #[test]
    fn test_predicate_consistency() {
        // Test that predicates are consistent for various scenarios

        // Overlapping squares
        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(square.intersects(&other));
        assert!(!square.disjoint(&other));
        assert!(square.interiors_intersect(&other));
        assert!(!square.touches(&other));
        assert!(!square.within(&other));
        assert!(!square.covers(&other));

        // Disjoint squares
        let distant = vec![[100.0, 100.0], [100.0, 110.0], [110.0, 110.0], [110.0, 100.0]];

        assert!(!square.intersects(&distant));
        assert!(square.disjoint(&distant));
        assert!(!square.interiors_intersect(&distant));
        assert!(!square.touches(&distant));
        assert!(!square.within(&distant));
        assert!(!square.covers(&distant));

        // Contained
        let outer = vec![[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let inner = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        assert!(outer.intersects(&inner));
        assert!(!outer.disjoint(&inner));
        assert!(outer.interiors_intersect(&inner));
        assert!(!outer.touches(&inner));
        assert!(!outer.within(&inner));
        assert!(outer.covers(&inner));

        assert!(inner.intersects(&outer));
        assert!(inner.interiors_intersect(&outer));
        assert!(inner.within(&outer));
        assert!(!inner.covers(&outer));

        // Edge-sharing only
        let left = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let right = vec![[10.0, 0.0], [10.0, 10.0], [20.0, 10.0], [20.0, 0.0]];

        assert!(left.intersects(&right));
        assert!(!left.disjoint(&right));
        assert!(!left.interiors_intersect(&right));
        assert!(left.touches(&right));
        assert!(!left.within(&right));
        assert!(!left.covers(&right));
    }

    #[test]
    fn test_predicate_overlay_with_adapter() {
        use crate::core::overlay::ShapeType;
        use i_float::adapter::FloatPointAdapter;

        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let iter = square.iter().chain(other.iter());
        let adapter = FloatPointAdapter::with_iter(iter);

        let mut overlay = FloatPredicateOverlay::with_adapter(adapter, 16);
        overlay.add_source(&square, ShapeType::Subject);
        overlay.add_source(&other, ShapeType::Clip);
        assert!(overlay.intersects());
    }

    #[test]
    fn test_predicate_overlay_with_adapter_custom() {
        use crate::core::fill_rule::FillRule;
        use crate::core::overlay::ShapeType;
        use crate::core::solver::Solver;
        use i_float::adapter::FloatPointAdapter;

        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let iter = square.iter().chain(other.iter());
        let adapter = FloatPointAdapter::with_iter(iter);

        let mut overlay =
            FloatPredicateOverlay::with_adapter_custom(adapter, FillRule::NonZero, Solver::default(), 16);
        overlay.add_source(&square, ShapeType::Subject);
        overlay.add_source(&other, ShapeType::Clip);
        assert!(overlay.intersects());
    }

    #[test]
    fn test_predicate_overlay_with_subj_and_clip_custom() {
        use crate::core::fill_rule::FillRule;
        use crate::core::solver::Solver;

        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let mut overlay = FloatPredicateOverlay::with_subj_and_clip_custom(
            &square,
            &other,
            FillRule::NonZero,
            Solver::default(),
        );
        assert!(overlay.intersects());
    }

    #[test]
    fn test_predicate_overlay_clear() {
        use crate::core::overlay::ShapeType;

        let square = vec![[0.0, 0.0], [0.0, 10.0], [10.0, 10.0], [10.0, 0.0]];
        let other = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let mut overlay = FloatPredicateOverlay::with_subj_and_clip(&square, &other);
        assert!(overlay.intersects());

        overlay.clear();

        // Use coordinates within the original adapter bounds
        let touching = vec![[10.0, 0.0], [10.0, 10.0], [15.0, 10.0], [15.0, 0.0]];
        overlay.add_source(&square, ShapeType::Subject);
        overlay.add_source(&touching, ShapeType::Clip);
        // After clear and re-add, shapes touch but don't overlap interiors
        assert!(overlay.intersects());
    }

    #[test]
    fn test_predicate_overlay_all_predicates() {
        let outer = vec![[0.0, 0.0], [0.0, 20.0], [20.0, 20.0], [20.0, 0.0]];
        let inner = vec![[5.0, 5.0], [5.0, 15.0], [15.0, 15.0], [15.0, 5.0]];

        let mut overlay = FloatPredicateOverlay::with_subj_and_clip(&inner, &outer);
        assert!(overlay.intersects());

        overlay.clear();
        overlay.add_source(&inner, crate::core::overlay::ShapeType::Subject);
        overlay.add_source(&outer, crate::core::overlay::ShapeType::Clip);
        assert!(overlay.interiors_intersect());

        overlay.clear();
        overlay.add_source(&inner, crate::core::overlay::ShapeType::Subject);
        overlay.add_source(&outer, crate::core::overlay::ShapeType::Clip);
        assert!(!overlay.touches());

        overlay.clear();
        overlay.add_source(&inner, crate::core::overlay::ShapeType::Subject);
        overlay.add_source(&outer, crate::core::overlay::ShapeType::Clip);
        assert!(overlay.within());
    }
}
