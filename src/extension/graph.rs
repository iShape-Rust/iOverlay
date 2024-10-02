use std::mem;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_node::OverlayNode;

impl OverlayGraph {
    pub(super) fn remove_leaf_links(&mut self) {
        let mut buffer = Vec::new();
        for i in 0..self.links.len() {
            self.remove_leaf_link(i, &mut buffer);
        }

        let mut next_buffer = Vec::new();
        while !buffer.is_empty() {
            for &i in buffer.iter() {
                self.remove_leaf_link(i, &mut next_buffer);
            }
            buffer.clear();
            mem::swap(&mut next_buffer, &mut buffer);
        }
    }

    #[inline]
    fn remove_leaf_link(&mut self, index: usize, buffer: &mut Vec<usize>) {
        let link = self.link(index);
        let a = self.node(link.a.id);
        let b = self.node(link.b.id);
        if !(a.is_single() || b.is_single()) {
            return;
        }
        let a_id = link.a.id;
        let b_id = link.b.id;

        if let Some(link) = self.mut_node(a_id).remove(index) {
            buffer.push(link);
        }
        if let Some(link) = self.mut_node(b_id).remove(index) {
            buffer.push(link);
        }
    }

    #[inline(always)]
    fn mut_node(&mut self, index: usize) -> &mut OverlayNode {
        unsafe { self.nodes.get_unchecked_mut(index) }
    }
/*
    #[inline(always)]
    pub(super) fn sort_indices(&self, node_id: usize, sorted_buffer: &mut Vec<IdPoint>, visited: &[u8]) {
        sorted_buffer.clear();
        let node = self.node(node_id);
        match node {
            OverlayNode::Bridge([a, b]) => {
                if visited[*a] > 0 {
                    let point = self.link(*a).other(node_id).point;
                    sorted_buffer.push(IdPoint { id: *a, point });
                }
                if visited[*b] > 0 {
                    let point = self.link(*b).other(node_id).point;
                    sorted_buffer.push(IdPoint { id: *b, point });
                }
            }
            OverlayNode::Cross(array) => {
                for a in array.iter() {
                    if visited[*a] > 0 {
                        let point = self.link(*a).other(node_id).point;
                        sorted_buffer.push(IdPoint { id: *a, point });
                    }
                }
            }
        }
        if sorted_buffer.len() <= 1 {
            return;
        }

        let any_link_index = sorted_buffer[0].id;
        let any_link = self.link(any_link_index);

        // center
        let c = if any_link.a.id == node_id { any_link.a.point } else { any_link.b.point };

        sorted_buffer.sort_by(|a, b|{
            let order = Triangle::area_two_point(c, a.point, b.point).cmp(&0);
            match order {
                Ordering::Equal => {
                    // x are equal
                    b.point.y.cmp(&a.point.y)
                }
                _ => {
                    order
                }
            }
        });
    }

    fn sort_in_clock_wise_direction
*/
}

impl OverlayNode {

    #[inline(always)]
    fn is_single(&self) -> bool {
        match self {
            OverlayNode::Bridge(_) => { false }
            OverlayNode::Cross(indices) => { indices.len() == 1 }
        }
    }

    fn remove(&mut self, link: usize) -> Option<usize> {
        match self {
            OverlayNode::Bridge([a, b]) => {
                let other = if *a == link {
                    *b
                } else {
                    *a
                };

                *self = OverlayNode::Cross([other].to_vec());
                Some(other)
            }
            OverlayNode::Cross(indices) => {
                if indices.len() == 1 {
                    indices.clear();
                    return None;
                }

                if let Some(pos) = indices.iter().position(|x| *x == link) {
                    indices.swap_remove(pos);
                }

                if indices.len() == 2 {
                    *self = OverlayNode::Bridge([indices[0], indices[1]]);
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use crate::core::overlay_link::OverlayLink;
    use crate::id_point::IdPoint;
    use super::*;

    impl PartialEq for OverlayNode {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (OverlayNode::Bridge(a), OverlayNode::Bridge(b)) => a == b,
                (OverlayNode::Cross(a), OverlayNode::Cross(b)) => a == b,
                _ => false
            }
        }
    }

    impl IdPoint {
        fn id(id: usize) -> Self {
            IdPoint { id, point: IntPoint::ZERO }
        }
    }

    impl OverlayLink {
        fn test(node_a: usize, node_b: usize) -> Self {
            OverlayLink { a: IdPoint::id(node_a), b: IdPoint::id(node_b), fill: 0 }
        }
    }

    #[test]
    fn test_remove_leaf_links_single_leaf() {
        let nodes = vec![
            OverlayNode::Bridge([0, 1]),
            OverlayNode::Cross(vec![1]),
            OverlayNode::Cross(vec![0]),
        ];

        let links = vec![
            OverlayLink::test(0, 2),
            OverlayLink::test(0, 1),
        ];

        let mut graph = OverlayGraph { solver: Default::default(), nodes, links };

        graph.remove_leaf_links();

        assert_eq!(graph.nodes[0], OverlayNode::Cross(vec![]));
        assert_eq!(graph.nodes[1], OverlayNode::Cross(vec![]));
        assert_eq!(graph.nodes[2], OverlayNode::Cross(vec![]));
    }

    #[test]
    fn test_remove_leaf_links_multiple_leafs() {
        let nodes = vec![
            OverlayNode::Cross(vec![0, 1, 2]),
            OverlayNode::Cross(vec![0]),
            OverlayNode::Cross(vec![1]),
            OverlayNode::Cross(vec![2]),
        ];

        let links = vec![
            OverlayLink::test(0, 1),
            OverlayLink::test(0, 2),
            OverlayLink::test(0, 3),
        ];

        let mut graph = OverlayGraph { solver: Default::default(), nodes, links };
        graph.remove_leaf_links();

        assert_eq!(graph.nodes[0], OverlayNode::Cross(vec![]));
    }

    #[test]
    fn test_no_leafs_initially() {
        let nodes = vec![
            OverlayNode::Bridge([0, 1]),
            OverlayNode::Bridge([1, 0]),
        ];

        let links = vec![
            OverlayLink::test(0, 1),
            OverlayLink::test(1, 0),
        ];

        let mut graph = OverlayGraph { solver: Default::default(), nodes, links };
        graph.remove_leaf_links();

        assert_eq!(graph.links.len(), 2);
    }

    #[test]
    fn test_remove_multiple_leafs_in_sequence() {
        let nodes = vec![
            OverlayNode::Bridge([0, 1]),         // 0
            OverlayNode::Bridge([0, 2]),         // 1
            OverlayNode::Cross(vec![1, 3, 4]),   // 2
            OverlayNode::Cross(vec![2, 3, 5]),   // 3
            OverlayNode::Cross(vec![4]),         // 4
            OverlayNode::Bridge([5, 6]),         // 5
            OverlayNode::Bridge([6, 7]),         // 6
            OverlayNode::Cross(vec![7]),         // 7
        ];

        let links = vec![
            OverlayLink::test(0, 1), // 0
            OverlayLink::test(0, 2), // 1
            OverlayLink::test(1, 3), // 2
            OverlayLink::test(2, 3), // 3
            OverlayLink::test(2, 4), // 4
            OverlayLink::test(3, 5), // 5
            OverlayLink::test(5, 6), // 6
            OverlayLink::test(6, 7), // 7
        ];

        let mut graph = OverlayGraph { solver: Default::default(), nodes, links };
        graph.remove_leaf_links();

        assert_eq!(graph.nodes[0], OverlayNode::Bridge([0, 1]));
        assert_eq!(graph.nodes[1], OverlayNode::Bridge([0, 2]));
        assert_eq!(graph.nodes[2], OverlayNode::Bridge([1, 3]));
        assert_eq!(graph.nodes[3], OverlayNode::Bridge([2, 3]));
    }
}
