use crate::space::line_range::LineRange;
use crate::space::scan_space::ScanSpace;

pub(crate) struct XScanList {
    pub(crate) space: ScanSpace<usize, i32>,
    delta: i32,
}

impl XScanList {
    pub(crate) fn new(range: LineRange, count: usize) -> Self {
        let space = ScanSpace::new(range, count);
        let delta = 1 << space.indexer.scale;
        Self { space, delta }
    }

    pub(crate) fn iterator_to_bottom(&self, start: i32) -> LineRange {
        let range = self.space.indexer.range;
        let top = range.max.min(start);
        let min_y = range.min.max(top - self.delta);
        LineRange { min: min_y, max: top }
    }

    pub(crate) fn next(&self, range: LineRange) -> LineRange {
        let bottom = self.space.indexer.range.min;
        if range.min > bottom {
            let radius = (range.max - range.min) << 1;
            let min_y = bottom.max(range.min - radius);
            return LineRange { min: min_y, max: range.min };
        } else {
            LineRange { min: i32::MIN, max: i32::MAX }
        }
    }

}