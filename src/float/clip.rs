use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Path, Paths, Shape};
use i_shape::float::adapter::PathToFloat;
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::float::string_graph::FloatStringGraph;
use crate::float::string_overlay::FloatStringOverlay;
use crate::string::clip::ClipRule;

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatStringGraph<P, T> {
    /// Clips the line strings in the graph based on the specified `ClipRule`.
    ///
    /// - `clip_rule`: The clipping rule specifying whether to invert the selection and include boundaries.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    #[inline]
    pub fn clip_string_lines(&self, clip_rule: ClipRule) -> Paths<P> {
        let lines = self.graph.clip_string_lines(clip_rule);
        lines.into_iter().map(|path| path.to_float(&self.adapter)).collect()
    }
}

pub trait FloatClip<P, T> {
    /// Clips a single path according to the specified fill and clip rules.
    /// - `path`: An array of points representing the path to be clipped.
    /// - `is_open`: Indicates whether the path is open (true) or closed (false).
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip_path(&self, path: &[P], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P>;

    /// Clips multiple paths according to the specified fill and clip rules.
    /// - `paths`: An array of `Path` instances, each representing a path to be clipped.
    /// - `is_open`: Indicates whether the paths are open (true) or closed (false).
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip_paths(&self, paths: &[Path<P>], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P>;
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatClip<P, T> for [Shape<P>] {
    #[inline]
    fn clip_path(&self, path: &[P], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        let iter = self.iter().flatten().flatten().chain(path.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + path.len();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_shapes(self)
            .unsafe_add_string_path(path, is_open)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[Path<P>], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        let iter = self.iter().flatten().flatten().chain(paths.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + paths.points_count();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_shapes(self)
            .unsafe_add_string_paths(paths, is_open)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatClip<P, T> for [Contour<P>] {
    #[inline]
    fn clip_path(&self, path: &[P], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        let iter = self.iter().flatten().chain(path.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + path.len();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contours(self)
            .unsafe_add_string_path(path, is_open)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[Path<P>], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        let iter = self.iter().flatten().chain(paths.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.points_count() + paths.points_count();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contours(self)
            .unsafe_add_string_paths(paths, is_open)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatClip<P, T> for [P] {
    #[inline]
    fn clip_path(&self, path: &[P], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        let iter = self.iter().chain(path.iter());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.len() + path.len();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contour(self)
            .unsafe_add_string_path(path, is_open)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[Path<P>], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        let iter = self.iter().chain(paths.iter().flatten());
        let adapter = FloatPointAdapter::with_iter(iter);
        let capacity = self.len() + paths.points_count();

        FloatStringOverlay::with_adapter(adapter, capacity)
            .unsafe_add_contour(self)
            .unsafe_add_string_paths(paths, is_open)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }
}