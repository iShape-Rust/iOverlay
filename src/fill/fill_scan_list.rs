use crate::dual_index::DualIndex;
use crate::fill::segment::Segment;
use crate::space::line_range::LineRange;
use crate::space::line_space::{IntExtensions, LineContainer, LineSegment, LineSpace};

pub(super) struct FillScanList {
    space: LineSpace<usize>,
    bottom: i32,
    delta: i32,
}

impl FillScanList {
    pub(super) fn new(segments: &Vec<Segment>) -> Self {
        let mut y_min: i64 = i64::MAX;
        let mut y_max: i64 = i64::MIN;
        for segment in segments.iter() {
            if segment.a.y > segment.b.y {
                y_min = y_min.min(segment.b.y.value());
                y_max = y_max.max(segment.a.y.value());
            } else {
                y_min = y_min.min(segment.a.y.value());
                y_max = y_max.max(segment.b.y.value());
            }
        }

        let max_level = ((segments.len() as f64).sqrt() as usize).log_two();
        let space = LineSpace::new(max_level, LineRange { min: y_min as i32, max: y_max as i32 });
        let bottom = y_min as i32;
        let delta = 1 << space.scale();
        Self { space, bottom, delta }
    }

    pub(super) fn iterator_to_bottom(&self, start: i32) -> LineRange {
        let min_y = self.bottom.max(start - self.delta);
        LineRange { min: min_y, max: start }
    }

    pub(super) fn next(&self, range: LineRange) -> LineRange {
        if range.min > self.bottom {
            let radius = (range.max - range.min) << 1;
            let min_y = self.bottom.max(range.min - radius);
            return LineRange { min: min_y, max: range.min };
        } else {
            LineRange { min: i32::MIN, max: i32::MAX }
        }
    }

    pub(super) fn all_in_range(&mut self, range: LineRange) -> &Vec<LineContainer<usize>> {
        self.space.all_in_range(range)
    }

    pub(super) fn insert(&mut self, segment: LineSegment<usize>) {
        self.space.insert(segment);
    }

    pub(super) fn remove(&mut self, indices: &mut Vec<DualIndex>) {
        if indices.len() > 1 {
            indices.sort_by(|a, b| a.order_asc_major_des_minor(b));
            for index in indices {
                self.space.remove(index);
            }
        } else {
            self.space.remove(&indices[0]);
        }
    }
}