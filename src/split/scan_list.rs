use crate::util::{Int, SwapRemoveIndex};
use crate::split::cross_solver::ScanCrossSolver;
use crate::x_segment::XSegment;
use crate::split::scan_store::{CrossSegment, ScanSplitStore};

pub(super) struct ScanSplitList {
    buffer: Vec<XSegment>,
}

impl ScanSplitList {
    pub(super) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanSplitStore for ScanSplitList {
    fn intersect_and_remove_other(&mut self, this: XSegment) -> Option<CrossSegment> {
        let mut i = 0;
        while i < self.buffer.len() {
            let scan = &self.buffer[i];

            let is_valid = ScanCrossSolver::is_valid_scan(&scan, &this);
            if !is_valid {
                self.buffer.swap_remove_index(i);
                continue;
            }

            // order is important! this x scan
            if let Some(cross) = ScanCrossSolver::scan_cross(&this, &scan) {
                let scan = scan.clone();
                self.buffer.swap_remove_index(i);
                return Some(CrossSegment { other: scan, cross });
            }

            i += 1
        }

        None
    }

    fn insert(&mut self, segment: XSegment) {
        self.buffer.push(segment);
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}