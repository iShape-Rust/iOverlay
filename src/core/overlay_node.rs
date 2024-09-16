#[derive(Debug)]
pub(crate) struct OverlayNode {
    pub(crate) indices: Vec<usize>,
}

impl OverlayNode {
    #[inline]
    pub(crate) fn other(&self, index: usize) -> usize {
        debug_assert_eq!(self.indices.len(), 2);
        let (i0, i1) = unsafe {
            let i0 = *self.indices.get_unchecked(0);
            let i1 = *self.indices.get_unchecked(1);
            (i0, i1)
        };

        if i0 == index {
            i1
        } else {
            i0
        }
    }

}