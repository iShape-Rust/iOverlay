#[derive(Debug)]
pub(crate) struct OverlayNode {
    pub(crate) indices: Vec<usize>,
}

impl OverlayNode {
    #[inline]
    pub(crate) fn other(&self, index: usize) -> usize {
        debug_assert_eq!(self.indices.len(), 2);
        let i0 = unsafe { *self.indices.get_unchecked(0) };
        if i0 == index {
            unsafe { *self.indices.get_unchecked(1) }
        } else {
            i0
        }
    }
}