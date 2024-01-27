use crate::space::line_range::LineRange;
use crate::space::scan_space::ScanSpace;

pub(crate) struct XScanList {
    pub(crate) space: ScanSpace<usize, i32>,
    bottom: i32,
    delta: i32,
}

impl XScanList {
    pub(crate) fn new(range: LineRange, count: usize) -> Self {
        let space = ScanSpace::new(range, count);
        let bottom = range.min;
        let delta = 1 << space.indexer.scale;
        Self { space, bottom, delta }
    }

    pub(crate) fn iterator_to_bottom(&self, start: i32) -> LineRange {
        let min_y = self.bottom.max(start - self.delta);
        LineRange { min: min_y, max: start }
    }

    pub(crate) fn next(&self, range: LineRange) -> LineRange {
        if range.min > self.bottom {
            let radius = (range.max - range.min) << 1;
            let min_y = self.bottom.max(range.min - radius);
            return LineRange { min: min_y, max: range.min };
        } else {
            LineRange { min: i32::MIN, max: i32::MAX }
        }
    }
}