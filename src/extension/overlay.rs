use i_shape::int::path::IntPath;
use crate::core::overlay::{Overlay, ShapeType};
use crate::extension::line::IntLine;
use crate::segm::segment::{Segment, ToSegment};
use crate::segm::shape_count::ShapeCount;

impl Overlay {

    #[inline]
    pub(super) fn add_line(&mut self, line: &IntLine, shape_type: ShapeType) {
        self.segments.push(line.to_segment(shape_type));
    }

    #[inline]
    pub(super) fn add_lines(&mut self, lines: &[IntLine], shape_type: ShapeType) {
        self.segments.extend(lines.iter().map(|line| line.to_segment(shape_type)));
    }

    #[inline]
    pub(super) fn add_open_path(&mut self, path: &IntPath, shape_type: ShapeType) {
        let count = ShapeCount::with_shape_type(shape_type);
        self.segments.extend(
            path.windows(2)
                .map(|w| Segment::create_and_validate(w[0], w[1], count))
        );
    }

    #[inline]
    pub(super) fn add_open_paths(&mut self, paths: &[IntPath], shape_type: ShapeType) {
        let count = ShapeCount::with_shape_type(shape_type);
        for path in paths {
            self.segments.extend(
                path.windows(2).map(|w| {
                    Segment::create_and_validate(w[0], w[1], count)
                })
            );
        }
    }
}