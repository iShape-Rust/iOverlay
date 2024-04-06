#[derive(Debug)]
pub(crate) struct OverlayNode {
    pub(crate) indices: Vec<usize>,
}

impl OverlayNode {
    pub(crate) fn other(&self, index: usize) -> usize {
        debug_assert_eq!(self.indices.len(), 2);
        if self.indices[0] == index {
            self.indices[1]
        } else {
            self.indices[0]
        }
    }
}