use i_float::f64_point::F64Point;
use i_shape::f64::shape::{F64Path, F64Shape, F64Shapes};
use crate::core::fill_rule::FillRule;
use crate::f64::line::F64Line;
use crate::f64::string::{F64StringGraph, F64StringOverlay};
use crate::string::clip::ClipRule;

impl F64StringGraph {
    /// Clips the line strings in the graph based on the specified `ClipRule`, adapting integer-based lines to `F64Line`.
    ///
    /// - `clip_rule`: The clipping rule specifying whether to invert the selection and include boundaries.
    ///
    /// # Returns
    /// A vector of `F64Line` containing the points of lines that meet the clipping conditions.
    #[inline]
    pub fn clip_string_lines(&self, clip_rule: ClipRule) -> Vec<F64Line> {
        let lines = self.graph.clip_string_lines(clip_rule);
        lines.into_iter().map(|line| {
            let a = self.adapter.convert_to_float(&line[0]);
            let b = self.adapter.convert_to_float(&line[0]);
            [a, b]
        }).collect()
    }
}

pub trait F64Clip {
    /// Clips a single line according to the specified fill and clip rules.
    /// - `line`: The line to be clipped, represented by two points.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of line segments.
    /// - `clip_rule`: The rule for clipping, determining how the boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `F64Line` instances representing the clipped sections of the input line.
    fn clip_line(&self, line: F64Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line>;

    /// Clips multiple lines according to the specified fill and clip rules.
    /// - `lines`: A slice of `F64Line` instances representing lines to be clipped.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of line segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the results.
    ///
    /// # Returns
    /// A vector of `F64Line` instances containing the clipped portions of the input lines.
    fn clip_lines(&self, lines: &[F64Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line>;

    /// Clips a single path according to the specified fill and clip rules.
    /// - `path`: A reference to an `F64Path`, which is a sequence of points representing the path to be clipped.
    /// - `is_open`: Indicates whether the path is open (true) or closed (false).
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `F64Line` instances representing the clipped sections of the path.
    fn clip_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line>;

    /// Clips multiple paths according to the specified fill and clip rules.
    /// - `paths`: A slice of `F64Path` instances, each representing a path to be clipped.
    /// - `is_open`: Indicates whether the paths are open (true) or closed (false).
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `F64Line` instances containing the clipped portions of the input paths.
    fn clip_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line>;
}

impl F64Clip for F64Shapes {
    #[inline]
    fn clip_line(&self, line: F64Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[F64Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_path(path.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }
}

impl F64Clip for F64Shape {
    #[inline]
    fn clip_line(&self, line: F64Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[F64Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_path(path.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }
}

impl F64Clip for [F64Point] {
    #[inline]
    fn clip_line(&self, line: F64Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[F64Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_path(path.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F64Line> {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }
}