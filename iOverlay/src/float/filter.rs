use i_float::float::number::FloatNumber;

/// Configuration for filtering and cleaning polygon contours.
#[derive(Clone, Copy)]
pub struct ContourFilter<T> {
    /// Minimum allowed area for a contour.
    /// Contours with area less than this value will be excluded.
    pub min_area: T,

    /// If true, collinear points will be removed from the contour.
    pub simplify_contour: bool,

    /// If true, the result will be cleaned from precision-related issues
    /// such as duplicate or nearly identical points. Especially useful for `f32` coordinates.
    pub clean_result: bool,
}
impl<T: FloatNumber> Default for ContourFilter<T> {
    fn default() -> Self {
        // f32 precision is not enough to cover i32
        let clean_result = T::bit_width() <= 32;
        ContourFilter { min_area: T::from_float(0.0), simplify_contour: true, clean_result }
    }
}