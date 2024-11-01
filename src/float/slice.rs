use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use crate::core::fill_rule::FillRule;
use crate::float::source::resource::OverlayResource;
use crate::float::string_overlay::FloatStringOverlay;
use crate::string::rule::StringRule;

/// The `FloatSlice` trait provides methods to slice geometric shapes using a given path or set of paths,
/// allowing for boolean operations based on the specified fill rule.
pub trait FloatSlice<S, P, T: FloatNumber>
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Slices the current shapes by paths.
    ///
    /// - `source`: A source for string paths.
    ///   `ContourSource` can be one of the following:
    ///     - `Path`: A single open path.
    ///     - `Paths`: An array of open paths.
    ///     - `Shapes`: A two-dimensional array where each element defines a separate open path.
    /// - `fill_rule`: Fill rule to determine filled areas within shapes.
    ///
    /// Returns a `Shapes<P>` collection representing the sliced geometry.
    fn slice(&self, source: &S, fill_rule: FillRule) -> Shapes<P>;
}


impl<S, P, T> FloatSlice<S, P, T> for S
    where
        S: OverlayResource<P, T>,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
    #[inline]
    fn slice(&self, source: &S, fill_rule: FillRule) -> Shapes<P> {
        FloatStringOverlay::with_shape_and_string(self, source)
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