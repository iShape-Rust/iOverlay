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
    fn slice_by_path(&self, path: &IntPath, is_open: bool, fill_rule: FillRule) -> IntShapes;
    fn slice_by_paths(&self, paths: &[IntPath], is_open: bool, fill_rule: FillRule) -> IntShapes;
}

impl IntSlice for IntShapes {
    #[inline]
    fn slice_by_line(&self, line: IntLine, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_lines(lines);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &IntPath, is_open: bool, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_path(path, is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[IntPath], is_open: bool, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shapes(self);
        overlay.add_string_paths(paths, is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}

impl IntSlice for IntShape {
    #[inline]
    fn slice_by_line(&self, line: IntLine, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_lines(lines);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &IntPath, is_open: bool, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_path(path, is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[IntPath], is_open: bool, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape(self);
        overlay.add_string_paths(paths, is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}

impl IntSlice for [IntPoint] {
    #[inline]
    fn slice_by_line(&self, line: IntLine, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_path(self);
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_path(self);
        overlay.add_string_lines(lines);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &IntPath, is_open: bool, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_path(self);
        overlay.add_string_path(path, is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[IntPath], is_open: bool, fill_rule: FillRule) -> IntShapes {
        let mut overlay = StringOverlay::with_shape_path(self);
        overlay.add_string_paths(paths, is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}