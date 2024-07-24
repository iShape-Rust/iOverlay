use i_float::rect::IntRect;
use crate::line_range::LineRange;
use crate::x_segment::XSegment;

#[derive(Debug, Clone)]
pub(super) struct Fragment {
    pub(super) index: usize,
    pub(super) rect: IntRect,
    pub(super) x_segment: XSegment,
}

impl Fragment {
    pub(super) fn with_index_and_segment(index: usize, x_segment: XSegment) -> Self {
        let (min_y, max_y) = if x_segment.a.y < x_segment.b.y {
            (x_segment.a.y, x_segment.b.y)
        } else {
            (x_segment.b.y, x_segment.a.y)
        };

        let rect = IntRect {
            min_x: x_segment.a.x,
            max_x: x_segment.b.x,
            min_y,
            max_y,
        };

        Self {
            index,
            rect,
            x_segment,
        }
    }

    pub(super) fn y_range(&self) -> LineRange {
        LineRange { min: self.rect.min_y, max: self.rect.max_y }
    }
}