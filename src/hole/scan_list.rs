use i_float::point::Point;
use crate::array::SwapRemoveIndex;
use crate::hole::scan_store::ScanHoleStore;
use crate::hole::segment::IdSegment;
use crate::int::Int;

pub(crate) struct ScanHoleList {
    buffer: Vec<IdSegment>,
}

impl ScanHoleList {
    pub(crate) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanHoleStore for ScanHoleList {
    fn insert(&mut self, segment: IdSegment, _stop: i32) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, p: Point, stop: i32) -> usize {
        let mut i = 0;
        let mut result: Option<IdSegment> = None;
        while i < self.buffer.len() {
            if self.buffer[i].x_segment.b.x <= stop {
                self.buffer.swap_remove_index(i);
            } else {
                let segment = self.buffer[i].x_segment;
                if segment.is_under_point(p) {
                    if let Some(count_seg) = &result {
                        if count_seg.x_segment.is_under_segment(segment) {
                            result = Some(self.buffer[i].clone())
                        }
                    } else {
                        result = Some(self.buffer[i].clone())
                    }
                }

                i += 1
            }
        }

        if let Some(result) = &result {
            result.id
        } else {
            0
        }
    }
}