//! This module defines the graph structure that represents the relationships between the paths in
//! subject and clip polygons after boolean operations. The graph helps in extracting final shapes
//! based on the overlay rule applied.

use alloc::vec::Vec;
use super::link::OverlayLink;
use crate::build::builder::GraphNode;
use crate::core::overlay::IntOverlayOptions;

/// A representation of geometric shapes organized for efficient boolean operations.
///
/// `OverlayGraph` is a core structure designed to facilitate the execution of boolean operations on shapes, such as union, intersection, and difference. It organizes and preprocesses geometric data, making it optimized for these operations. This struct is the result of compiling shape data into a form where boolean operations can be applied directly, efficiently managing the complex relationships between different geometric entities.
///
/// Use `OverlayGraph` to perform boolean operations on the geometric shapes you've added to an `Overlay`, after it has processed the shapes according to the specified build and overlay rules.
/// [More information](https://ishape-rust.github.io/iShape-js/overlay/overlay_graph/overlay_graph.html) about Overlay Graph.
pub struct OverlayGraph<'a> {
    pub(crate) options: IntOverlayOptions,
    pub(crate) nodes: &'a [OverlayNode],
    pub(crate) links: &'a [OverlayLink],
}

#[derive(Debug)]
pub(crate) enum OverlayNode {
    Bridge([usize; 2]),
    Cross(Vec<usize>),
}

impl GraphNode for OverlayNode {
    #[inline]
    fn with_indices(indices: &[usize]) -> Self {
        if indices.len() == 2 {
            Self::Bridge(unsafe { [*indices.get_unchecked(0), *indices.get_unchecked(1)] })
        } else {
            Self::Cross(indices.to_vec())
        }
    }
}

impl OverlayGraph<'_> {
    pub fn validate(&self) {
        for node in self.nodes.iter() {
            if let OverlayNode::Cross(indices) = node {
                debug_assert!(indices.len() > 1, "indices: {}", indices.len());
                debug_assert!(
                    self.nodes.len() <= self.links.len(),
                    "nodes is more then links"
                );
            }
        }
    }
}