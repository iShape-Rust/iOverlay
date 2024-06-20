use i_float::adapter::PointAdapter;
use i_shape::f64::adapter::ShapesToFloat;
use i_shape::f64::shape::F64Shapes;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_rule::OverlayRule;

/// The `FloatOverlayGraph` struct represents an overlay graph with floating point precision,
/// providing methods to extract geometric shapes from the graph after applying boolean operations.
pub struct FloatOverlayGraph {
    pub graph: OverlayGraph,
    pub adapter: PointAdapter,
}

impl FloatOverlayGraph {

    /// Creates a new instance of `FloatOverlayGraph`.
    ///
    /// # Parameters
    /// - `graph`: The int overlay graph to be used for shape extraction.
    /// - `adapter`: The point adapter for converting coordinates.
    ///
    /// # Returns
    /// A new `FloatOverlayGraph` instance.
    #[inline(always)]
    pub(super) fn new(graph: OverlayGraph, adapter: PointAdapter) -> Self {
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
    /// A vector of `F64Shape`, representing the geometric result of the applied overlay rule.
    ///
    /// # Shape Representation
    /// The output is a `Vec<Vec<Vec<F64Point>>>`, where:
    /// - The outer `Vec` represents a set of shapes.
    /// - Each shape `Vec` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec` represents a collection of points, where every two consecutive points (cyclically) make up the boundary edge of the polygon.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline(always)]
    pub fn extract_shapes(&self, overlay_rule: OverlayRule) -> F64Shapes {
        self.extract_shapes_min_area(overlay_rule, 0.0)
    }

    /// Extracts shapes from the overlay graph similar to `extract_shapes`, but with an additional constraint on the minimum area of the shapes.
    /// This is useful for filtering out shapes that do not meet a certain size threshold, which can be beneficial for eliminating artifacts or noise from the output.
    ///
    /// # Parameters
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `min_area`: The minimum area threshold for shapes to be included in the result. Shapes with an area smaller than this value will be excluded.
    ///
    /// # Returns
    /// A vector of `F64Shapes` that meet the specified area criteria, representing the cleaned-up geometric result.
    ///
    /// # Shape Representation
    /// The output is a `Vec<Vec<Vec<F64Point>>>`, where:
    /// - The outer `Vec` represents a set of shapes.
    /// - Each shape `Vec` represents a collection of polygons, where the first polygon is the outer boundary, and all subsequent polygons are holes in this boundary.
    /// - Each polygon `Vec` represents a collection of points, where every two consecutive points (cyclically) make up the boundary edge of the polygon.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline]
    pub fn extract_shapes_min_area(&self, overlay_rule: OverlayRule, min_area: f64) -> F64Shapes {
        let sqr_scale = self.adapter.dir_scale * self.adapter.dir_scale;
        let area = (sqr_scale * min_area) as i64;
        let shapes = self.graph.extract_shapes_min_area(overlay_rule, area);

        shapes.to_float(&self.adapter)
    }
}