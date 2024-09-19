use i_float::point::IntPoint;
use crate::util::Int;
use crate::bind::segment::IdSegment;
use crate::bind::solver::ScanHoleStore;

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
    #[inline(always)]
    fn insert(&mut self, segment: IdSegment, _stop: i32) {
        self.buffer.push(segment)
    }

    fn find_under_and_nearest(&mut self, p: IntPoint) -> usize {
        if self.buffer.is_empty() {
            return 0
        }

        let mut i = 0;
        let mut j = usize::MAX;
        let mut n = self.buffer.len();
        while i < n {
            let item = unsafe { self.buffer.get_unchecked(i) };
            if item.x_segment.b.x <= p.x {
                let last = *unsafe { self.buffer.get_unchecked(n - 1) };
                *unsafe { self.buffer.get_unchecked_mut(i) } = last;
                n -= 1;
                continue;
            }

            if item.x_segment.is_under_point(p) {
                if j == usize::MAX {
                    j = i;
                } else {
                    let prev = unsafe { self.buffer.get_unchecked(j) };
                    if prev.x_segment.is_under_segment(&item.x_segment) {
                        j = i;
                    }
                }
            }
            i += 1;
        }

        if n != self.buffer.len() {
            self.buffer.truncate(n);
        }

        if j == usize::MAX {
            0
        } else {
            unsafe { self.buffer.get_unchecked(j) }.id
        }
    }
}