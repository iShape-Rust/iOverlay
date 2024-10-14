use i_float::f32_point::F32Point;
use i_shape::f32::adapter::PathToFloat;
use i_shape::f32::shape::{F32Path, F32Shape, F32Shapes};
use crate::core::fill_rule::FillRule;
use crate::f32::line::F32Line;
use crate::f32::string::{F32StringGraph, F32StringOverlay};
use crate::string::clip::ClipRule;

impl F32StringGraph {
    /// Clips the line strings in the graph based on the specified `ClipRule`, adapting integer-based lines to `F32Line`.
    ///
    /// - `clip_rule`: The clipping rule specifying whether to invert the selection and include boundaries.
    ///
    /// # Returns
    /// A vector of `F32Path` containing the points of lines that meet the clipping conditions.
    #[inline]
    pub fn clip_string_lines(&self, clip_rule: ClipRule) -> Vec<F32Path> {
        let lines = self.graph.clip_string_lines(clip_rule);
        lines.into_iter().map(|path| path.to_float(&self.adapter)).collect()
    }
}

pub trait F32Clip {
    /// Clips a single line according to the specified fill and clip rules.
    /// - `line`: The line to be clipped, represented by two points.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of line segments.
    /// - `clip_rule`: The rule for clipping, determining how the boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `F32Path` instances representing the clipped sections of the input line.
    fn clip_line(&self, line: F32Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path>;

    /// Clips multiple lines according to the specified fill and clip rules.
    /// - `lines`: A slice of `F32Line` instances representing lines to be clipped.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of line segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the results.
    ///
    /// # Returns
    /// A vector of `F32Path` instances containing the clipped portions of the input lines.
    fn clip_lines(&self, lines: &[F32Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path>;

    /// Clips a single path according to the specified fill and clip rules.
    /// - `path`: A reference to an `F32Path`, which is a sequence of points representing the path to be clipped.
    /// - `is_open`: Indicates whether the path is open (true) or closed (false).
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `F32Path` instances representing the clipped sections of the path.
    fn clip_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path>;

    /// Clips multiple paths according to the specified fill and clip rules.
    /// - `paths`: A slice of `F32Path` instances, each representing a path to be clipped.
    /// - `is_open`: Indicates whether the paths are open (true) or closed (false).
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `F32Path` instances containing the clipped portions of the input paths.
    fn clip_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path>;
}

impl F32Clip for F32Shapes {
    #[inline]
    fn clip_line(&self, line: F32Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[F32Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_path(path.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }
}

impl F32Clip for F32Shape {
    #[inline]
    fn clip_line(&self, line: F32Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[F32Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_path(path.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }
}

impl F32Clip for [F32Point] {
    #[inline]
    fn clip_line(&self, line: F32Line, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[F32Line], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_path(path.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<F32Path> {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).clip_string_lines(clip_rule)
    }
}