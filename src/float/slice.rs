use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Path, Shape, Shapes};
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::float::string_overlay::FloatStringOverlay;
use crate::string::rule::StringRule;

/// The `FloatSlice` trait provides methods to slice geometric shapes using a given path or set of paths,
/// allowing for boolean operations based on the specified fill rule.
pub trait FloatSlice<P, T: FloatNumber> {
    /// Slices the current shapes by a single path.
    ///
    /// - `path`: A slice of points representing the slicing path.
    /// - `is_open`: Specifies if the path is open (true) or closed (false).
    /// - `fill_rule`: Fill rule to determine filled areas within shapes.
    ///
    /// Returns a `Shapes<P>` collection representing the sliced geometry.
    fn slice_by_path(&self, path: &[P], is_open: bool, fill_rule: FillRule) -> Shapes<P>;

    /// Slices the current shapes by multiple paths.
    ///
    /// - `paths`: A slice of paths, each a vector of points.
    /// - `is_open`: Specifies if each path is open (true) or closed (false).
    /// - `fill_rule`: Fill rule to determine filled areas within shapes.
    ///
    /// Returns a `Shapes<P>` collection representing the sliced geometry.
    fn slice_by_paths(&self, paths: &[Vec<P>], is_open: bool, fill_rule: FillRule) -> Shapes<P>;
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatSlice<P, T> for [Shape<P>] {
    #[inline]
    fn slice_by_path(&self, path: &[P], is_open: bool, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().flatten().chain(path.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + path.len();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_shapes(self)
            .unsafe_add_string_path(path, is_open)
            .into_graph(fill_rule)
            .extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[Vec<P>], is_open: bool, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().flatten().chain(paths.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + paths.points_count();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_shapes(self)
            .unsafe_add_string_paths(paths, is_open)
            .into_graph(fill_rule)
            .extract_shapes(StringRule::Slice)
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatSlice<P, T> for [Contour<P>] {
    #[inline]
    fn slice_by_path(&self, path: &[P], is_open: bool, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().chain(path.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + path.len();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contours(self)
            .unsafe_add_string_path(path, is_open)
            .into_graph(fill_rule)
            .extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[Path<P>], is_open: bool, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().flatten().chain(paths.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + paths.points_count();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contours(self)
            .unsafe_add_string_paths(paths, is_open)
            .into_graph(fill_rule)
            .extract_shapes(StringRule::Slice)
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatSlice<P, T> for [P] {
    #[inline]
    fn slice_by_path(&self, path: &[P], is_open: bool, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().chain(path.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.len() + path.len();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contour(self)
            .unsafe_add_string_path(path, is_open)
            .into_graph(fill_rule)
            .extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[Path<P>], is_open: bool, fill_rule: FillRule) -> Shapes<P> {
        let iter = self.iter().chain(paths.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.len() + paths.points_count();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contour(self)
            .unsafe_add_string_paths(paths, is_open)
            .into_graph(fill_rule)
            .extract_shapes(StringRule::Slice)
    }
}