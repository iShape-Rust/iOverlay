//! This module defines the graph structure that represents the relationships between the paths in
//! subject and clip polygons after boolean operations. The graph helps in extracting final shapes
//! based on the overlay rule applied.

use i_float::adapter::FloatPointAdapter;
use i_float::float::Float;
use i_float::float_point::FloatPointCompatible;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_rule::OverlayRule;

/// The `FloatOverlayGraph` struct represents an overlay graph with floating point precision,
/// providing methods to extract geometric shapes from the graph after applying boolean operations.
/// [More information](https://ishape-rust.github.io/iShape-js/overlay/overlay_graph/overlay_graph.html) about Overlay Graph.
pub struct FloatOverlayGraph<T: Float> {
    pub graph: OverlayGraph,
    pub adapter: FloatPointAdapter<T>,
}

impl<T: Float> FloatOverlayGraph<T> {
    /// Creates a new instance of `FloatOverlayGraph`.
    ///
    /// # Parameters
    /// - `graph`: The int overlay graph to be used for shape extraction.
    /// - `adapter`: The point adapter for converting coordinates.
    ///
    /// # Returns
    /// A new `FloatOverlayGraph` instance.
    #[inline(always)]
    pub(crate) fn new(graph: OverlayGraph, adapter: FloatPointAdapter<T>) -> Self {
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
    /// The output is a `Vec<Vec<Vec<P>>>`, where:
    /// - The outer `Vec<Vec<Vec<P>>>` represents a set of shapes.
    /// - Each shape `Vec<Vec<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline(always)]
    pub fn extract_shapes<P: FloatPointCompatible<T>>(&self, overlay_rule: OverlayRule) -> Vec<Vec<Vec<P>>> {
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
    /// The output is a `Vec<Vec<Vec<P>>>`, where:
    /// - The outer `Vec<Vec<Vec<P>>>` represents a set of shapes.
    /// - Each shape `Vec<Vec<P>>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline]
    pub fn extract_shapes_min_area<P: FloatPointCompatible<T>>(&self, overlay_rule: OverlayRule, min_area: f64) -> Vec<Vec<Vec<P>>> {
        let scale = self.adapter.dir_scale.to_f64();
        let sqr_scale = scale * scale;
        let area = (sqr_scale * min_area) as usize;
        let shapes = self.graph.extract_shapes_min_area(overlay_rule, area);

        shapes
            .into_iter()
            .map(|v| v.into_iter().map(|inner| inner.into_iter()
                .map(|p| FloatPointCompatible::from_float_point(self.adapter.convert_to_float(p)))
                .collect()).collect())
            .collect()
    }
}