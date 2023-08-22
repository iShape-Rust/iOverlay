use crate::split::shape_edge::ShapeEdge;

const EMPTY: usize = std::usize::MAX;

#[derive(Debug, Clone, Copy)]
struct EdgeNode {
    next: usize,
    prev: usize,
    edge: ShapeEdge
}

impl EdgeNode {
    const EMPTY: Self = EdgeNode { next: EMPTY, prev: EMPTY, edge: ShapeEdge::ZERO };
}

pub (super) struct EdgeLinkedList {
    free: Vec<usize>,
    nodes: Vec<EdgeNode>,
    first: usize,
}

impl EdgeLinkedList {

    pub (super) fn new(edges: &[ShapeEdge]) -> Self {
        let plus_capacity = 16;
        let mut nodes = vec![EdgeNode::EMPTY; edges.len() + plus_capacity];
        let mut free = Vec::new();

        for i in 0..edges.len() - 1 {
            nodes[i] = EdgeNode {
                next: i + 1,
                prev: i - 1,
                edge: edges[i],
            };
        }

        nodes[edges.len() - 1] = EdgeNode {
            next: EMPTY,
            prev: edges.len() - 2,
            edge: edges[edges.len() - 1],
        };

        for i in (edges.len()..edges.len() + plus_capacity).rev() {
            free.push(i);
            nodes[i] = EdgeNode::EMPTY;
        }

        EdgeLinkedList {
            free,
            nodes,
            first: 0,
        }
    }

    pub (super) fn edge(&self, index: usize) -> ShapeEdge {
        self.nodes[index].edge
    }

    // Continue to define other methods as required, converting the Swift logic to Rust
}