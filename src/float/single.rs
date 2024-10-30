use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Shape, Shapes};
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::FloatOverlay;


/// Trait `SingleFloatOverlay` provides methods for overlay operations between various geometric entities.
/// This trait supports boolean operations on contours, shapes, and collections of shapes, using customizable overlay and fill rules.
pub trait SingleFloatOverlay<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Overlays the current collection of points with a contour, applying the specified overlay and fill rules.
    ///
    /// - `contour`: A slice of points representing a single closed path contour.
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - Returns: A vector of `Shapes<P>` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    fn overlay_with_contour(&self, contour: &[P], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P>;

    /// Overlays the current collection of points with multiple contours.
    ///
    /// - `contours`: A slice of `Contour<P>` instances, each representing a closed path.
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - Returns: A vector of `Shapes<P>` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    fn overlay_with_contours(&self, contours: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P>;

    /// Overlays the current points with a shape represented by multiple contours.
    ///
    /// - `shape`: A slice of contours making up a shape.
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - Returns: A vector of `Shapes<P>` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    fn overlay_with_shape(&self, shape: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P>;

    /// Overlays the current collection with multiple shapes, each containing one or more contours.
    ///
    /// - `shapes`: A slice of shapes.
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - Returns: A vector of `Shapes<P>` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    fn overlay_with_shapes(&self, shapes: &[Shape<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P>;
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> SingleFloatOverlay<P, T> for [P] {
    #[inline]
    fn overlay_with_contour(&self, contour: &[P], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        FloatOverlay::with_contour(self, contour).overlay(overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_contours(&self, contours: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().chain(contours.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.len() + contours.points_count();

        FloatOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contour(self, ShapeType::Subject)
            .unsafe_add_contours(contours, ShapeType::Clip)
            .overlay(overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_shape(&self, shape: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        self.overlay_with_contours(shape, overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_shapes(&self, shapes: &[Shape<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().chain(shapes.iter().flatten().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.len() + shapes.points_count();

        FloatOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contour(self, ShapeType::Subject)
            .unsafe_add_shapes(shapes, ShapeType::Clip)
            .overlay(overlay_rule, fill_rule)
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> SingleFloatOverlay<P, T> for [Contour<P>] {
    #[inline]
    fn overlay_with_contour(&self, contour: &[P], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().chain(contour.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + contour.len();

        FloatOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contours(self, ShapeType::Subject)
            .unsafe_add_contour(contour, ShapeType::Clip)
            .overlay(overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_contours(&self, contours: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        FloatOverlay::with_contours(self, contours).overlay(overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_shape(&self, shape: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        self.overlay_with_contours(shape, overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_shapes(&self, shapes: &[Shape<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().chain(shapes.iter().flatten().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + shapes.points_count();

        FloatOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contours(self, ShapeType::Subject)
            .unsafe_add_shapes(shapes, ShapeType::Clip)
            .overlay(overlay_rule, fill_rule)
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> SingleFloatOverlay<P, T> for [Shape<P>] {
    #[inline]
    fn overlay_with_contour(&self, contour: &[P], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().flatten().chain(contour.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + contour.len();

        FloatOverlay::with_adapter(adapter, capacity)
            .unsafe_add_shapes(self, ShapeType::Subject)
            .unsafe_add_contour(contour, ShapeType::Clip)
            .overlay(overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_contours(&self, contours: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().flatten().chain(contours.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + contours.points_count();

        FloatOverlay::with_adapter(adapter, capacity)
            .unsafe_add_shapes(self, ShapeType::Subject)
            .unsafe_add_contours(contours, ShapeType::Clip)
            .overlay(overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_shape(&self, shape: &[Contour<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        self.overlay_with_contours(shape, overlay_rule, fill_rule)
    }

    #[inline]
    fn overlay_with_shapes(&self, shapes: &[Shape<P>], overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        FloatOverlay::with_shapes(self, shapes).overlay(overlay_rule, fill_rule)
    }
}