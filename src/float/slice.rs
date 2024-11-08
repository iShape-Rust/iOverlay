use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use crate::core::fill_rule::FillRule;
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
    ///
    /// Returns a `Shapes<P>` collection representing the sliced geometry.
    fn slice_by(&self, resource: &R, fill_rule: FillRule) -> Shapes<P>;
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
}


#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::float::simplify::Simplify;

    #[test]
    fn test_contour_slice() {
        let rect = [[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.as_slice().simplify(FillRule::NonZero, 0.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contour_vec() {
        let rect = vec![[0.0, 0.0], [0.0, 0.5], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let shapes = rect.simplify(FillRule::NonZero, 0.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }
}