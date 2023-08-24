use crate::index::EMPTY_INDEX;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OverlayNode {
    pub(crate) data0: usize,
    pub(crate) data1: usize,
    pub(crate) count: usize
}

impl OverlayNode {

    pub (super) fn new(data0: usize, data1: usize, count: usize) -> OverlayNode {
        OverlayNode { data0, data1, count }
    }

    pub (crate) fn other(&self, index: usize) -> usize {
        if self.data0 == index { self.data1 } else { self.data0 }
    }

    pub (super) fn add(&mut self, index: usize, indices: &mut Vec<usize>) {
        if self.count <= 2 {
            if self.data0 == EMPTY_INDEX {
                self.data0 = index;
            } else {
                self.data1 = index;
            }
        } else {
            indices[self.data0 + self.data1] = index;
            self.data1 += 1;
        }
    }

}