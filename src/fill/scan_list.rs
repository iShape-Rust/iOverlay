use i_float::point::IntPoint;
use crate::fill::count_segment::CountSegment;
use crate::fill::scan_store::ScanFillStore;
use crate::split::shape_count::ShapeCount;
use crate::util::{Int, SwapRemoveIndex};

pub(crate) struct ScanFillList {
    buffer: Vec<CountSegment>,
}

impl ScanFillList {
    pub(crate) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanFillStore for ScanFillList {
    fn insert(&mut self, segment: CountSegment, _stop: i32) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, p: IntPoint, stop: i32) -> Option<ShapeCount> {
        let mut i = 0;
        let mut result: Option<CountSegment> = None;
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

        if let Some(count_seg) = &result {
            Some(count_seg.count)
        } else {
            None
        }
    }
}