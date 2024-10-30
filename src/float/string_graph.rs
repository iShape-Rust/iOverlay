use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use crate::float::adapter::AdapterExt;
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;

/// The `FloatStringGraph` struct represents a graph structure with floating-point precision,
/// providing methods to extract geometric shapes from the graph after applying string-based operations.
pub struct FloatStringGraph<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub graph: StringGraph,
    pub adapter: FloatPointAdapter<P, T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatStringGraph<P, T> {
    /// Extracts shapes from the overlay graph based on the specified string rule.
    /// This method is used to retrieve the final geometric shapes after boolean operations have been applied.
    /// It's suitable for most use cases where the minimum area of shapes is not a concern.
    ///
    /// # Parameters
    /// - `string_rule`: The string operation rule to apply when extracting shapes from the graph, such as slice.
    ///
    /// # Returns
    /// A `Shapes<P>` collection, representing the geometric result of the applied string rule.
    ///
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Shapes<P>` represents a set of shapes.
    /// - Each shape `Shape<P>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Contour<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline(always)]
    pub fn extract_shapes(&self, string_rule: StringRule) -> Shapes<P> {
        self.extract_shapes_min_area(string_rule, T::from_float(0.0))
    }

    /// Extracts shapes from the overlay graph similar to `extract_shapes`, but with an additional constraint on the minimum area of the shapes.
    /// This is useful for filtering out shapes that do not meet a certain size threshold, which can be beneficial for eliminating artifacts or noise from the output.
    ///
    /// # Parameters
    /// - `string_rule`: The string operation rule to apply when extracting shapes from the graph, such as slice.
    /// - `min_area`: The minimum area threshold for shapes to be included in the result. Shapes with an area smaller than this value will be excluded.
    ///
    /// # Returns
    /// A `Shapes<P>` collection, representing the geometric result of the applied string rule.
    ///
    /// # Shape Representation
    /// The output is a `Shapes<P>`, where:
    /// - The outer `Shapes<P>` represents a set of shapes.
    /// - Each shape `Shape<P>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Contour<P>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline]
    pub fn extract_shapes_min_area(&self, string_rule: StringRule, min_area: T) -> Shapes<P> {
        let area = self.adapter.convert_area(min_area);
        let shapes = self.graph.extract_shapes_min_area(string_rule, area);
        shapes.to_float(&self.adapter)
    }
}