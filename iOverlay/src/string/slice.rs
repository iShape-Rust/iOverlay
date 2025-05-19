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