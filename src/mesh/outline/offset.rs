use crate::float::filter::ContourFilter;
use crate::mesh::style::OutlineStyle;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;

pub trait OutlineOffset<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Generates an outline shapes for contours, or shapes.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    fn outline(&self, style: OutlineStyle<T>) -> Shapes<P>;

    /// Generates an outline shapes for contours, or shapes with optional filtering.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    /// - `filter`: Defines optional contour filtering and simplification:
    ///     - `min_area`: Retains only contours with an area larger than this value.
    ///     - `simplify`: If `true`, simplifies contours and removes degenerate edges.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    fn outline_with_filter(&self, style: OutlineStyle<T>, filter: ContourFilter<T>) -> Shapes<P>;
}
