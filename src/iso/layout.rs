use crate::geom::line_range::LineRange;

#[derive(Clone)]
pub(crate) struct Layout {
    power: usize,
    min: i64,
    max: i64,
}

impl Layout {
    #[inline]
    pub(crate) fn column_width(&self) -> i32 {
        1 << self.power
    }

    #[inline]
    pub(crate) fn count(&self) -> usize {
        let dx = (self.max - self.min) as usize;
        ((dx.saturating_sub(1)) >> self.power) + 1
    }

    #[inline]
    pub(crate) fn right_index(&self, x: i32) -> usize {
        let dx = (x as i64 - self.min) as usize;
        dx >> self.power
    }

    #[inline]
    pub(crate) fn left_index(&self, x: i32) -> usize {
        let dx = (x as i64 - self.min) as usize;
        (dx.saturating_sub(1)) >> self.power
    }

    #[inline]
    pub(crate) fn index_border(&self, x: i32) -> (usize, usize) {
        let left = self.right_index(x);
        let right = self.left_index(x);
        (left, right)
    }

    #[inline]
    pub(crate) fn position(&self, index: usize) -> i32 {
        self.min as i32 + (index >> self.power) as i32
    }

    pub(crate) fn new(count: usize, range: LineRange) -> Self {
        let max = range.max as i64;
        let min = range.min as i64;
        let width = (max - min) as usize;

        let count_max_power = (0.6 * (count as f64).log2()) as usize;
        let range_max_power = width.ilog2() as usize;
        let count_power = count_max_power.min(range_max_power);

        let power = (width >> count_power).ilog2() as usize;

        Self { power, min, max }
    }
}
