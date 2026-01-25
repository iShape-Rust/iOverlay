use i_float::int::point::IntPoint;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes};
use crate::core::fill_rule::FillRule;
use crate::string::line::IntLine;
use crate::string::overlay::StringOverlay;
use crate::string::rule::StringRule;

pub trait IntSlice {
    fn slice_by_line(&self, line: IntLine, fill_rule: FillRule) -> IntShapes;
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule) -> IntShapes;
    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule) -> IntShapes;
    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule) -> IntShapes;
}

impl IntSlice for IntShapes {
    #[inline]
    fn slice_by_line(&self, line: IntLine, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_line(line);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_lines(lines);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_path(path);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_paths(paths);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }
}

impl IntSlice for IntShape {
    #[inline]
    fn slice_by_line(&self, line: IntLine, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_line(line);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_lines(lines);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_path(path);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_paths(paths);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }
}

impl IntSlice for [IntPoint] {
    #[inline]
    fn slice_by_line(&self, line: IntLine, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_line(line);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_lines(lines);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_path(path);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_contour(self);
        overlay.add_string_paths(paths);
        overlay
            .build_graph_view(fill_rule)
            .map(|graph| graph.extract_shapes(StringRule::Slice))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use i_float::int::point::IntPoint;
    use crate::core::fill_rule::FillRule;
    use crate::string::slice::IntSlice;

    #[test]
    fn test_empty_input() {
        #[rustfmt::skip]
        let shapes = [].slice_by_line(
            [IntPoint::new(0, 0), IntPoint::new(0, 0)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_0() {
        #[rustfmt::skip]
        let shapes = vec![
            IntPoint::new(-2, -2),
            IntPoint::new(2, -2),
            IntPoint::new(2, 2),
            IntPoint::new(-2, 2),
        ]
        .slice_by_line(
            [IntPoint::new(-5, 5), IntPoint::new(5, -5)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0][0].len(), 3);
        assert_eq!(shapes[1][0].len(), 3);        
    }

    #[test]
    fn test_1() {
        #[rustfmt::skip]
        let shapes = vec![
            IntPoint::new(-2, -2),
            IntPoint::new(2, -2),
            IntPoint::new(2, 2),
            IntPoint::new(-2, 2),
        ]
        .slice_by_line(
            [IntPoint::new(-5, 5), IntPoint::new(5, 5)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_2() {
        #[rustfmt::skip]
        let shapes = vec![
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 3),
            IntPoint::new(1, 1),
            IntPoint::new(0, 3),
        ]
        .slice_by_line(
            [IntPoint::new(-5, 2), IntPoint::new(5, 2)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 3);
        assert_eq!(shapes[0][0].len(), 5);
        assert_eq!(shapes[1][0].len(), 3);
        assert_eq!(shapes[2][0].len(), 3);
    }

    #[test]
    fn test_3() {
        #[rustfmt::skip]
        let shapes = vec![
            IntPoint::new(-2, -2),
            IntPoint::new(2, -2),
            IntPoint::new(2, 2),
            IntPoint::new(-2, 2),
        ]
        .slice_by_line(
            [IntPoint::new(-2, 5), IntPoint::new(2, -5)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_4() {
        #[rustfmt::skip]
        let shapes = vec![
            IntPoint::new(-2, 0),
            IntPoint::new(2, 0),
            IntPoint::new(0, 2),
        ]
        .slice_by_line(
            [IntPoint::new(-5, 2), IntPoint::new(5, 2)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 1);
    }
}