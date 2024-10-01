use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes, PointsCount};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::extension::line::IntLine;

pub trait Slice {
    fn slice_by_line(&self, line: &IntLine, fill_rule: FillRule, min_area: i64) -> IntShapes;
    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule, min_area: i64) -> IntShapes;
    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes;
    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes;
    fn slice_by_open_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes;
    fn slice_by_open_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes;
}

impl Slice for IntShapes {
    fn slice_by_line(&self, line: &IntLine, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + 1);
        overlay.add_shapes(&self, ShapeType::Subject);
        overlay.add_line(line, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + lines.len());
        overlay.add_shapes(&self, ShapeType::Subject);
        overlay.add_lines(lines, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + path.len());
        overlay.add_shapes(&self, ShapeType::Subject);
        overlay.add_path(&path, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + paths.points_count());
        overlay.add_shapes(&self, ShapeType::Subject);
        overlay.add_paths(&paths, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_open_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + path.len() - 1);
        overlay.add_shapes(&self, ShapeType::Subject);
        overlay.add_open_path(path, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_open_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + paths.points_count() - paths.len());
        overlay.add_shapes(&self, ShapeType::Subject);
        overlay.add_open_paths(paths, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }
}

impl Slice for IntShape {
    fn slice_by_line(&self, line: &IntLine, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + 1);
        overlay.add_shape(&self, ShapeType::Subject);
        overlay.add_line(line, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + lines.len());
        overlay.add_shape(&self, ShapeType::Subject);
        overlay.add_lines(lines, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + path.len());
        overlay.add_shape(&self, ShapeType::Subject);
        overlay.add_path(&path, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + paths.points_count());
        overlay.add_shape(&self, ShapeType::Subject);
        overlay.add_paths(&paths, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_open_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + path.len() - 1);
        overlay.add_shape(&self, ShapeType::Subject);
        overlay.add_open_path(path, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_open_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + paths.points_count() - paths.len());
        overlay.add_shape(&self, ShapeType::Subject);
        overlay.add_open_paths(paths, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }
}

impl Slice for IntPath {
    fn slice_by_line(&self, line: &IntLine, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + 1);
        overlay.add_path(&self, ShapeType::Subject);
        overlay.add_line(line, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_lines(&self, lines: &[IntLine], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + lines.len());
        overlay.add_path(&self, ShapeType::Subject);
        overlay.add_lines(lines, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + path.len());
        overlay.add_path(&self, ShapeType::Subject);
        overlay.add_path(&path, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + paths.points_count());
        overlay.add_path(&self, ShapeType::Subject);
        overlay.add_paths(&paths, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_open_path(&self, path: &IntPath, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + path.len() - 1);
        overlay.add_path(&self, ShapeType::Subject);
        overlay.add_open_path(path, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }

    fn slice_by_open_paths(&self, paths: &[IntPath], fill_rule: FillRule, min_area: i64) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + paths.points_count() - paths.len());
        overlay.add_path(&self, ShapeType::Subject);
        overlay.add_open_paths(paths, ShapeType::Clip);
        overlay.slice(fill_rule, min_area)
    }
}
