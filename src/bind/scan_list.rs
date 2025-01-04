use crate::bind::segment::IdSegment;
use crate::bind::solver::ScanHoleStore;
use crate::geom::x_segment::XSegment;
use crate::util::log::Int;

pub(crate) struct ScanHoleList {
    buffer: Vec<IdSegment>,
}

impl ScanHoleList {
    #[inline(always)]
    pub(crate) fn new(count: usize) -> Self {
        Self { buffer: Vec::with_capacity(count.log2_sqrt()) }
    }
}

impl ScanHoleStore for ScanHoleList {

    #[inline]
    fn is_emmpty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[inline(always)]
    fn insert(&mut self, segment: IdSegment, _stop: i32) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, segment: XSegment) -> usize {
        if self.buffer.is_empty() {
            return 0;
        }

        let mut i = 0;
        let x = segment.a.x;
        let mut best: Option<XSegment> = None;
        let mut best_id = usize::MAX;
        while i < self.buffer.len() {
            let item = unsafe { self.buffer.get_unchecked(i) };
            if item.x_segment.b.x <= x {
                if i + 1 < self.buffer.len() {
                    self.buffer.swap_remove(i);
                    continue;
                } else {
                    return best_id;
                }
            }

            if item.x_segment.is_under_segment(&segment) {
                if let Some(prev) = best {
                    if prev.is_under_segment(&item.x_segment) {
                        best = Some(item.x_segment);
                        best_id = item.id;
                    }
                } else {
                    best = Some(item.x_segment);
                    best_id = item.id;
                }
            }
            i += 1;
        }

        best_id
    }
}