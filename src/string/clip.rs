use i_float::int::point::IntPoint;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes};
use crate::core::fill_rule::FillRule;
use crate::string::graph::StringGraph;
use crate::string::line::IntLine;
use crate::string::overlay::StringOverlay;

#[derive(Debug, Clone, Copy)]
pub struct ClipRule {
    /// Configuration for clipping lines with rules to determine inclusion or exclusion based on boundary and inversion.
    /// - `invert`: If true, inverts the clipping area selection, affecting which lines are included in the output.
    /// - `boundary_included`: If true, includes boundary lines in the clipping result. When combined with `invert`, this toggles whether lines on the boundary are considered inside or outside.
    pub invert: bool,
    pub boundary_included: bool,
}

impl StringGraph {

    #[inline]
    pub(super) fn clip_string_lines(&self) -> Vec<IntPath> {
        let mut visited = vec![false; self.links.len()];

        let mut link_index = 0;
        let mut paths = Vec::new();
        while link_index < visited.len() {
            if visited[link_index] {
                link_index += 1;
                continue;
            }

            visited[link_index] = true;

            let mut sub_path = Vec::new();
            let mut link = self.link(link_index);
            let mut a = link.a;
            sub_path.push(a.point);
            loop {
                let b = link.other(a.id);
                sub_path.push(b.point);
                let node = self.node(b.id);
                let next_index = if let Some(&not_visited_index) = node.iter().find(|&&index| !visited[index]) {
                    not_visited_index
                } else {
                    break;
                };
                visited[next_index] = true;
                link = self.link(next_index);
                a = b;
            }

            paths.push(sub_path);
        }

        paths
    }
}

pub trait IntClip {
    /// Clips a single line according to the specified fill and clip rules.
    /// - `line`: The line to be clipped, represented by two points.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of line segments.
    /// - `clip_rule`: The rule for clipping, determining how the boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `IntPath` instances representing the clipped sections of the input line.
    fn clip_line(&self, line: IntLine, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath>;

    /// Clips multiple lines according to the specified fill and clip rules.
    /// - `lines`: A slice of `IntLine` instances representing lines to be clipped.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of line segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the results.
    ///
    /// # Returns
    /// A vector of `IntPath` instances containing the clipped portions of the input lines.
    fn clip_lines(&self, lines: &[IntLine], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath>;

    /// Clips a single path according to the specified fill and clip rules.
    /// - `path`: A reference to an `IntPath`, which is a sequence of points representing the path to be clipped.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `IntPath` instances representing the clipped sections of the path.
    fn clip_path(&self, path: &IntPath, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath>;

    /// Clips multiple paths according to the specified fill and clip rules.
    /// - `paths`: A slice of `IntPath` instances, each representing a path to be clipped.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A vector of `IntPath` instances containing the clipped portions of the input paths.
    fn clip_paths(&self, paths: &[IntPath], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath>;
}

impl IntClip for IntShapes {
    #[inline]
    fn clip_line(&self, line: IntLine, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_line(line);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[IntLine], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_lines(lines);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &IntPath, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_path(path);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[IntPath], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_paths(paths);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }
}

impl IntClip for IntShape {
    #[inline]
    fn clip_line(&self, line: IntLine, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_line(line);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[IntLine], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_lines(lines);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &IntPath, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_path(path);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[IntPath], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_paths(paths);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }
}

impl IntClip for [IntPoint] {
    #[inline]
    fn clip_line(&self, line: IntLine, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_line(line);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_lines(&self, lines: &[IntLine], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_lines(lines);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_path(&self, path: &IntPath, fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_path(path);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }

    #[inline]
    fn clip_paths(&self, paths: &[IntPath], fill_rule: FillRule, clip_rule: ClipRule) -> Vec<IntPath> {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_paths(paths);
        overlay.clip_string_lines(fill_rule, clip_rule)
    }
}


#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_shape::int::path::IntPath;
    use crate::core::fill_rule::FillRule;
    use crate::string::clip::{ClipRule, IntClip};

    #[test]
    fn test_empty_path() {
        let path: IntPath = vec![];
        let result_0 = path.clip_line(
            [IntPoint::new(0, 0), IntPoint::new(0, 0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [IntPoint::new(0, 0), IntPoint::new(10, 0)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert!(result_0.is_empty());
        assert_eq!(result_1.len(), 1);
    }

    #[test]
    fn test_simple() {
        let path = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let result_0 = path.clip_line(
            [IntPoint::new(0, -15), IntPoint::new(0, 15)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [IntPoint::new(0, -15), IntPoint::new(0, 15)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert_eq!(result_0.len(), 1);
        assert_eq!(result_1.len(), 2);
    }

    #[test]
    fn test_boundary() {
        let path = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let result_0 = path.clip_line(
            [IntPoint::new(-10, -15), IntPoint::new(-10, 15)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [IntPoint::new(-10, -15), IntPoint::new(-10, 15)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: true },
        );

        assert_eq!(result_0.len(), 0);
        assert_eq!(result_1.len(), 1);
    }

    #[test]
    fn test_complex() {
        let rect = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let path = vec![
            IntPoint::new(-20, 10),
            IntPoint::new(-20, 0),
            IntPoint::new(0, 0),
            IntPoint::new(0, 10),
            IntPoint::new(5, 10),
            IntPoint::new(20, -5),
            IntPoint::new(5, -5),
            IntPoint::new(0, -10),
            IntPoint::new(-5, -5),
            IntPoint::new(-15, -15)
        ];

        let result_0 = rect.clip_path(&path, FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = rect.clip_path(&path,FillRule::NonZero,
            ClipRule { invert: false, boundary_included: true },
        );

        assert_eq!(result_0.len(), 3);
        assert_eq!(result_1.len(), 2);
    }
}