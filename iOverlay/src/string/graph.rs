use alloc::vec::Vec;
use crate::build::builder::GraphNode;
use crate::core::link::OverlayLink;

pub struct StringGraph<'a> {
    pub(crate) nodes: &'a [Vec<usize>],
    pub(crate) links: &'a mut [OverlayLink],
}

impl GraphNode for Vec<usize> {
    #[inline(always)]
    fn with_indices(indices: &[usize]) -> Self {
        indices.to_vec()
    }
}