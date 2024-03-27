use crate::array::SwapRemoveIndex;
use crate::x_order::XOrder;
use crate::x_segment::XSegment;
use crate::split::scan_store::{CrossSegment, ScanSplitStore};
use crate::split::version_segment::VersionSegment;

pub(super) struct ScanSplitList {
    buffer: Vec<VersionSegment>,
}

impl ScanSplitList {
    pub(super) fn new(count: usize) -> Self {
        let capacity = ((count << 1) as f64).sqrt() as usize;
        Self { buffer: Vec::with_capacity(capacity) }
    }
}

impl ScanSplitStore for ScanSplitList {
    fn intersect(&mut self, this: XSegment) -> Option<CrossSegment> {
        let mut i = 0;
        let scan_pos= this.a;
        while i < self.buffer.len() {
            let scan = &self.buffer[i];
            if scan.x_segment.b.order_by_line_compare(scan_pos) {
                self.buffer.swap_remove_index(i);
                continue;
            }

            // order is important! this x scan
            if let Some(cross) = this.cross(&scan.x_segment) {
                let index = scan.index.clone();
                self.buffer.swap_remove_index(i);
                return Some(CrossSegment { index, cross });
            }

            i += 1
        }

        None
    }

    fn insert(&mut self, segment: VersionSegment) {
        self.buffer.push(segment);
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}