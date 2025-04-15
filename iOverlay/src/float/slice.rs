use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use crate::core::fill_rule::FillRule;
use crate::core::solver::Solver;
use crate::float::filter::ContourFilter;
use crate::float::source::resource::OverlayResource;
use crate::float::string_overlay::FloatStringOverlay;
use crate::string::rule::StringRule;

/// The `FloatSlice` trait provides methods to slice geometric shapes using a given path or set of paths,
/// allowing for boolean operations based on the specified fill rule.
pub trait FloatSlice<R, P, T: FloatNumber>
where
    R: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Slices the current shapes by string lines.
    ///
    /// - `resource`: A string lines.
    ///   `OverlayResource` can be one of the following:
    ///     - `Path`: A path representing a string line.
    ///     - `Paths`: A collection of paths, each representing a string line.
    ///     - `Vec<Paths>`: A collection of grouped paths, where each group may consist of multiple paths.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `filter`: `ContourFilter<T>` for optional contour filtering and simplification:
    ///     - `min_area`: Only retain contours with an area larger than this.
    ///     - `simplify`: Simplifies contours and removes degenerate edges if `true`.
    /// - `solver`: Type of solver to use.
    ///
    /// Returns a `Shapes<P>` collection representing the sliced geometry.
    fn slice_by(&self, resource: &R, fill_rule: FillRule) -> Shapes<P>;

    /// Slices the current shapes by string lines.
    ///
    /// - `resource`: A string lines.
    ///   `OverlayResource` can be one of the following:
    ///     - `Path`: A path representing a string line.
    ///     - `Paths`: A collection of paths, each representing a string line.
    ///     - `Vec<Paths>`: A collection of grouped paths, where each group may consist of multiple paths.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    ///
    /// Returns a `Shapes<P>` collection representing the sliced geometry.
    fn slice_by_with_filter_and_solver(&self, resource: &R, fill_rule: FillRule, filter: ContourFilter<T>, solver: Solver) -> Shapes<P>;
}


impl<R0, R1, P, T> FloatSlice<R0, P, T> for R1
where
    R0: OverlayResource<P, T>,
    R1: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn slice_by(&self, resource: &R0, fill_rule: FillRule) -> Shapes<P> {
        FloatStringOverlay::with_shape_and_string(self, resource)
            .into_graph(fill_rule)
            .extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_with_filter_and_solver(&self, resource: &R0, fill_rule: FillRule, filter: ContourFilter<T>, solver: Solver) -> Shapes<P> {
        FloatStringOverlay::with_shape_and_string(self, resource)
            .into_graph_with_solver(fill_rule, solver)
            .extract_shapes_with_filter(StringRule::Slice, filter)
    }
}


#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay::ContourDirection;
    use crate::float::simplify::SimplifyShape;

    #[test]
    fn test_contour_slice() {
        let rect = [[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.as_slice().simplify_shape(FillRule::NonZero, ContourDirection::CounterClockwise, 0.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_vec() {
        let rect = vec![[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.simplify_shape(FillRule::NonZero, ContourDirection::CounterClockwise, 0.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }
}