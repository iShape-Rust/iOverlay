use i_float::f64_point::F64Point;
use i_shape::f64::shape::{F64Path, F64Shape, F64Shapes};
use crate::core::fill_rule::FillRule;
use crate::f64::line::F64Line;
use crate::f64::string::F64StringOverlay;
use crate::string::rule::StringRule;

pub trait F64Slice {
    fn slice_by_line(&self, line: F64Line, fill_rule: FillRule) -> F64Shapes;
    fn slice_by_lines(&self, lines: &[F64Line], fill_rule: FillRule) -> F64Shapes;
    fn slice_by_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule) -> F64Shapes;
    fn slice_by_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule) -> F64Shapes;
}

impl F64Slice for F64Shapes {
    #[inline]
    fn slice_by_line(&self, line: F64Line, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[F64Line], fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_path(path.clone(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}

impl F64Slice for F64Shape {
    #[inline]
    fn slice_by_line(&self, line: F64Line, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[F64Line], fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_path(path.clone(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}

impl F64Slice for [F64Point] {
    #[inline]
    fn slice_by_line(&self, line: F64Line, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[F64Line], fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &F64Path, is_open: bool, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_path(path.clone(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[F64Path], is_open: bool, fill_rule: FillRule) -> F64Shapes {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}