use crate::core::overlay_node::OverlayNode;

impl OverlayNode {
    #[inline(always)]
    pub(super) fn is_contain(&self, link: usize) -> bool {
        match self {
            OverlayNode::Bridge([a, b]) => { *a == link || *b == link }
            OverlayNode::Cross(indices) => { indices.contains(&link) }
        }
    }
}