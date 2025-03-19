use i_shape::int::shape::IntShapes;
use crate::core::fill_rule::FillRule;
use crate::core::graph::OverlayGraph;
use crate::core::link::OverlayLinkBuilder;
use crate::core::overlay::Overlay;
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;

impl Overlay {

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule within the constraints of 45-degree geometry. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    #[inline]
    pub fn into_45geom_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> OverlayGraph {
        let links = OverlayLinkBuilder::iso_build_with_filler_filter(self.segments, fill_rule, solver);
        OverlayGraph::new(solver, links)
    }


    /// Executes a single Boolean operation on the current geometry using the specified overlay and fill rules within the constraints of 45-degree geometry.
    /// This method provides a streamlined approach for performing a Boolean operation without generating
    /// an entire `OverlayGraph`. Ideal for cases where only one Boolean operation is needed, `overlay`
    /// saves on computational resources by building only the necessary links, optimizing CPU usage by 0-20%
    /// compared to a full graph-based approach.
    ///
    /// ### Parameters:
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - Returns: A vector of `IntShape` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    /// ### Usage:
    /// This function is suitable when a single, optimized Boolean operation is required on the provided
    /// geometry. For example:
    ///
    /// ```rust
    /// use i_float::int::point::IntPoint;
    /// use i_float::int_pnt;
    /// use i_overlay::core::fill_rule::FillRule;
    /// use i_overlay::core::overlay::Overlay;
    /// use i_overlay::core::overlay_rule::OverlayRule;
    ///
    /// let left_rect = [int_pnt!(0, 0), int_pnt!(0, 10), int_pnt!(10, 10), int_pnt!(10, 0)];
    /// let right_rect = [int_pnt!(10, 0), int_pnt!(10, 10), int_pnt!(20, 10), int_pnt!(20, 0)];
    /// let overlay = Overlay::with_contour(&left_rect, &right_rect);
    ///
    /// let result = overlay.overlay_45geom(OverlayRule::Union, FillRule::EvenOdd);
    /// ```
    ///
    /// This method is particularly useful in scenarios where the geometry only needs one overlay operation
    /// without subsequent modifications. By excluding unnecessary graph structures, it optimizes performance,
    /// particularly for complex or resource-intensive geometries.
    #[inline]
    pub fn overlay_45geom(self, overlay_rule: OverlayRule, fill_rule: FillRule) -> IntShapes {
        self.overlay_45geom_with_min_area_and_solver(overlay_rule, fill_rule, 0, Default::default())
    }

    /// Executes a single Boolean operation on the current geometry using the specified overlay and fill rules within the constraints of 45-degree geometry.
    /// This method provides a streamlined approach for performing a Boolean operation without generating
    /// an entire `OverlayGraph`. Ideal for cases where only one Boolean operation is needed, `overlay`
    /// saves on computational resources by building only the necessary links, optimizing CPU usage by 0-20%
    /// compared to a full graph-based approach.
    ///
    /// ### Parameters:
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `min_area`: The minimum area threshold for shapes to be included in the result. Shapes with an area smaller than this value will be excluded.
    /// - Returns: A vector of `IntShape` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    /// This method is particularly useful in scenarios where the geometry only needs one overlay operation
    /// without subsequent modifications. By excluding unnecessary graph structures, it optimizes performance,
    /// particularly for complex or resource-intensive geometries.
    #[inline]
    pub fn overlay_45geom_with_min_area_and_solver(self, overlay_rule: OverlayRule, fill_rule: FillRule, min_area: usize, solver: Solver) -> IntShapes {
        let links = OverlayLinkBuilder::iso_build_with_overlay_filter(self.segments, fill_rule, overlay_rule, solver);
        let graph = OverlayGraph::new(solver, links);
        let filter = vec![false; graph.links.len()];
        graph.extract(filter, overlay_rule, min_area)
    }
}