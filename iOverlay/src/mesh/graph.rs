use crate::core::graph::OverlayNode;
use crate::core::link::OverlayLink;

pub struct OffsetGraph<'a> {
    pub(crate) nodes: &'a [OverlayNode],
    pub(crate) links: &'a [OverlayLink],
}