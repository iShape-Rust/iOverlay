use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes, PointsCount};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::extension::line::IntLine;
use crate::extension::rule::ExtRule;

pub trait Slice {
    fn slice_by_line(&self, line: &IntLine) -> IntShapes;
    fn slice_by_lines(&self, lines: &[IntLine]) -> IntShapes;
}

impl Slice for IntShapes {

    #[inline]
    fn slice_by_line(&self, line: &IntLine) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + 1);
        overlay.add_shapes(self, ShapeType::Subject);

        let mut ext = overlay.into_ext();
        ext.add_line(line);

        ext.into_graph(FillRule::NonZero).extract_shapes(ExtRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine]) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + lines.len());
        overlay.add_shapes(self, ShapeType::Subject);

        let mut ext = overlay.into_ext();
        ext.add_lines(lines);

        ext.into_graph(FillRule::NonZero).extract_shapes(ExtRule::Slice)
    }
}

impl Slice for IntShape {

    #[inline]
    fn slice_by_line(&self, line: &IntLine) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + 1);
        overlay.add_shape(self, ShapeType::Subject);

        let mut ext = overlay.into_ext();
        ext.add_line(line);

        ext.into_graph(FillRule::NonZero).extract_shapes(ExtRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine]) -> IntShapes {
        let mut overlay = Overlay::new(self.points_count() + lines.len());
        overlay.add_shape(self, ShapeType::Subject);

        let mut ext = overlay.into_ext();
        ext.add_lines(lines);

        ext.into_graph(FillRule::NonZero).extract_shapes(ExtRule::Slice)
    }
}

impl Slice for IntPath {

    #[inline]
    fn slice_by_line(&self, line: &IntLine) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + 1);
        overlay.add_path(self, ShapeType::Subject);

        let mut ext = overlay.into_ext();
        ext.add_line(line);

        ext.into_graph(FillRule::NonZero).extract_shapes(ExtRule::Slice)
    }

    #[inline]
    fn slice_by_lines(&self, lines: &[IntLine]) -> IntShapes {
        let mut overlay = Overlay::new(self.len() + lines.len());
        overlay.add_path(self, ShapeType::Subject);

        let mut ext = overlay.into_ext();
        ext.add_lines(lines);

        ext.into_graph(FillRule::NonZero).extract_shapes(ExtRule::Slice)
    }
}