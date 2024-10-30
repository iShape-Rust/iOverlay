use i_float::f32_point::F32Point;
use i_shape::f32::shape::{F32Path, F32Shape, F32Shapes};
use crate::core::fill_rule::FillRule;
use crate::f32::line::F32Line;
use crate::f32::string::F32StringOverlay;
use crate::string::rule::StringRule;

// #[deprecated(
//     since = "1.8.0",
//     note = "Use FloatSlice<P, T> instead, which provides a more flexible and efficient API"
// )]
pub trait F32Slice {
    fn slice_by_line(&self, line: F32Line, fill_rule: FillRule) -> F32Shapes;
    fn slice_by_lines(&self, lines: &[F32Line], fill_rule: FillRule) -> F32Shapes;
    fn slice_by_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule) -> F32Shapes;
    fn slice_by_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule) -> F32Shapes;
}

impl F32Slice for F32Shapes {
    #[inline]
    fn slice_by_line(&self, line: F32Line, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[F32Line], fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_path(path.clone(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shapes(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}

impl F32Slice for F32Shape {
    #[inline]
    fn slice_by_line(&self, line: F32Line, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[F32Line], fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_path(path.clone(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_paths(self.clone());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}

impl F32Slice for [F32Point] {
    #[inline]
    fn slice_by_line(&self, line: F32Line, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_line(line);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[F32Line], fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_lines(lines.to_vec());
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_path(&self, path: &F32Path, is_open: bool, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_path(path.clone(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }

    #[inline]
    fn slice_by_paths(&self, paths: &[F32Path], is_open: bool, fill_rule: FillRule) -> F32Shapes {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(self.to_vec());
        overlay.add_string_paths(paths.to_vec(), is_open);
        overlay.into_graph(fill_rule).extract_shapes(StringRule::Slice)
    }
}