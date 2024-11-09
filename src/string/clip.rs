use i_float::int::point::IntPoint;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes};
use crate::core::fill_rule::FillRule;
use crate::core::link::OverlayLink;
use crate::geom::id_point::IdPoint;
use crate::segm::segment::SegmentFill;
use crate::segm::shape_count::{STRING_BACK_CLIP, STRING_FORWARD_CLIP};
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
    pub(super) fn clip_string_lines(self) -> Vec<IntPath> {
        let mut paths = Vec::new();

        let mut links = self.links;
        let nodes = self.nodes;

        for (start_node_index, start_node) in nodes.iter().enumerate() {
            for &link_index in start_node.iter() {
                let mlink = unsafe { links.get_unchecked_mut(link_index) };
                let (a, mut b) = if mlink.a.id == start_node_index { (mlink.a, mlink.b) } else { (mlink.b, mlink.a) };

                let is_move_possible = mlink.visit_if_possible(a.point, b.point);

                if !is_move_possible {
                    continue;
                }

                let mut sub_path = Vec::new();
                sub_path.push(a.point);
                sub_path.push(b.point);

                while let Some(c) = Self::find_next_point(&nodes, &mut links, b) {
                    b = c;
                    sub_path.push(b.point);
                }

                paths.push(sub_path);
            }
        }

        paths
    }

    #[inline]
    fn find_next_point(nodes: &[Vec<usize>], links: &mut [OverlayLink], b: IdPoint) -> Option<IdPoint> {
        let node = unsafe { nodes.get_unchecked(b.id) };
        for &index in node.iter() {
            let mlink = unsafe { links.get_unchecked_mut(index) };
            let c = mlink.other(b.id);
            let is_move_possible = mlink.visit_if_possible(b.point, c.point);
            if is_move_possible {
                return Some(c);
            }
        }
        None
    }
}

const CLIP_BACK: SegmentFill = STRING_BACK_CLIP << 2;
const CLIP_FORWARD: SegmentFill = STRING_FORWARD_CLIP << 2;

impl OverlayLink {
    #[inline]
    fn visit_if_possible(&mut self, a: IntPoint, b: IntPoint) -> bool {
        let is_forward = a > b;
        if is_forward {
            let is_link_forward = self.fill & CLIP_FORWARD == CLIP_FORWARD;
            if is_link_forward {
                self.fill &= !CLIP_FORWARD;
            }
            is_link_forward
        } else {
            let is_link_back = self.fill & CLIP_BACK == CLIP_BACK;
            if is_link_back {
                self.fill &= !CLIP_BACK;
            }
            is_link_back
        }
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

        let result_1 = rect.clip_path(&path, FillRule::NonZero,
                                      ClipRule { invert: false, boundary_included: true },
        );

        assert_eq!(result_0.len(), 3);
        assert_eq!(result_1.len(), 2);
    }
}