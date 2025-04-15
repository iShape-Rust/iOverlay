//! This module defines the graph structure that represents the relationships between the paths in
//! subject and clip polygons after boolean operations. The graph helps in extracting final shapes
//! based on the overlay rule applied.

use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::simple::SimplifyContour;
use crate::core::graph::OverlayGraph;
use crate::core::overlay::ContourDirection;
use crate::core::overlay_rule::OverlayRule;
use crate::float::filter::ContourFilter;


/// The `FloatOverlayGraph` struct represents an overlay graph with floating point precision,
/// providing methods to extract geometric shapes from the graph after applying boolean operations.
/// [More information](https://ishape-rust.github.io/iShape-js/overlay/overlay_graph/overlay_graph.html) about Overlay Graph.
pub struct FloatOverlayGraph<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub graph: OverlayGraph,
    pub adapter: FloatPointAdapter<P, T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatOverlayGraph<P, T> {
    /// Creates a new instance of `FloatOverlayGraph`.
    ///
    /// # Parameters
    /// - `graph`: The int overlay graph to be used for shape extraction.
    /// - `adapter`: The point adapter for converting coordinates.
    ///
    /// # Returns
    /// A new `FloatOverlayGraph` instance.
    #[inline]
    pub(crate) fn new(graph: OverlayGraph, adapter: FloatPointAdapter<P, T>) -> Self {
        Self { graph, adapter }
    }

    /// Extracts shapes from the overlay graph based on the specified overlay rule.
    /// This method is used to retrieve the final geometric shapes after boolean operations have been applied.
    /// It's suitable for most use cases where the minimum area of shapes is not a concern.
    ///
    /// # Parameters
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    ///
    /// # Returns
    /// A `Shapes<P>` collection, representing the geometric result of the applied overlay rule.
    ///
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline]
    pub fn extract_shapes(&self, overlay_rule: OverlayRule) -> Shapes<P> {
        self.extract_shapes_custom(overlay_rule, Default::default(), ContourDirection::CounterClockWise)
    }

    /// Extracts shapes from the overlay graph similar to `extract_shapes`, but with an additional constraint on the minimum area of the shapes.
    /// This is useful for filtering out shapes that do not meet a certain size threshold, which can be beneficial for eliminating artifacts or noise from the output.
    ///
    /// # Parameters
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `filter`: `ContourFilter<T>` for optional contour filtering and simplification:
    ///     - `min_area`: Only retain contours with an area larger than this.
    ///     - `simplify`: Simplifies contours and removes degenerate edges if `true`.
    ///
    /// # Returns
    /// A `Shapes<P>` collection, representing the geometric result of the applied overlay rule.
    ///
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Vec<Shape<P>>` represents a set of shapes.
    /// - Each shape `Vec<Contour<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline]
    pub fn extract_shapes_custom(&self, overlay_rule: OverlayRule, filter: ContourFilter<T>, main_direction: ContourDirection) -> Shapes<P> {
        let area = self.adapter.sqr_float_to_int(filter.min_area);
        let shapes = self.graph.extract_shapes_custom(overlay_rule, main_direction, area);
        let mut float = shapes.to_float(&self.adapter);

        if filter.simplify {
            float.simplify_contour(&self.adapter);
        }

        float
    }
}