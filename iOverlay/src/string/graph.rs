use alloc::vec::Vec;
use crate::build::builder::GraphNode;
use crate::core::link::OverlayLink;

pub struct StringGraph<'a> {
    pub(crate) nodes: &'a [Vec<usize>],
    pub(crate) links: &'a mut [OverlayLink],
}
impl StringGraph<'_> {
    #[inline(always)]
    pub(super) fn node(&self, index: usize) -> &[usize] {
        unsafe { self.nodes.get_unchecked(index) }
    }

    #[inline(always)]
    pub(super) fn link(&self, index: usize) -> &OverlayLink {
        unsafe { self.links.get_unchecked(index) }
    }
}

impl GraphNode for Vec<usize> {
    #[inline(always)]
    fn with_indices(indices: &[usize]) -> Self {
        indices.to_vec()
    }
}