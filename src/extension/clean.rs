use std::mem;
use crate::extension::unstable_graph::UnstableGraph;

impl UnstableGraph {
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
        if a.is_not_leaf() && b.is_not_leaf() {
            return;
        }
        let a_id = link.a.id;
        let b_id = link.b.id;

        if let Some(link) = self.mut_node(a_id).remove_link(index) {
            buffer.push(link);
        }
        if let Some(link) = self.mut_node(b_id).remove_link(index) {
            buffer.push(link);
        }
    }

    #[inline(always)]
    fn mut_node(&mut self, index: usize) -> &mut Vec<usize> {
        unsafe { self.nodes.get_unchecked_mut(index) }
    }
}

trait NodeIndices {
    fn is_not_leaf(&self) -> bool;
    fn remove_link(&mut self, link: usize) -> Option<usize>;
}

impl NodeIndices for Vec<usize> {
    #[inline(always)]
    fn is_not_leaf(&self) -> bool {
        self.len() > 1
    }

    fn remove_link(&mut self, link: usize) -> Option<usize> {
        if self.len() == 1 {
            if self[0] == link {
                self.clear();
            }
            return None;
        }

        if let Some(pos) = self.iter().position(|x| *x == link) {
            self.swap_remove(pos);
            if self.len() == 1 {
                return Some(self[0]);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use crate::core::overlay_link::OverlayLink;
    use crate::id_point::IdPoint;
    use super::*;

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
            vec![0, 1],
            vec![1],
            vec![0],
        ];

        let links = vec![
            OverlayLink::test(0, 2),
            OverlayLink::test(0, 1),
        ];

        let mut graph = UnstableGraph { solver: Default::default(), nodes, links };

        graph.remove_leaf_links();

        assert_eq!(graph.nodes[0], vec![]);
        assert_eq!(graph.nodes[1], vec![]);
        assert_eq!(graph.nodes[2], vec![]);
    }

    #[test]
    fn test_remove_leaf_links_multiple_leafs() {
        let nodes = vec![
            vec![0, 1, 2],
            vec![0],
            vec![1],
            vec![2],
        ];

        let links = vec![
            OverlayLink::test(0, 1),
            OverlayLink::test(0, 2),
            OverlayLink::test(0, 3),
        ];

        let mut graph = UnstableGraph { solver: Default::default(), nodes, links };
        graph.remove_leaf_links();

        assert_eq!(graph.nodes[0], vec![]);
    }

    #[test]
    fn test_no_leafs_initially() {
        let nodes = vec![
            vec![0, 1],
            vec![1, 0],
        ];

        let links = vec![
            OverlayLink::test(0, 1),
            OverlayLink::test(1, 0),
        ];

        let mut graph = UnstableGraph { solver: Default::default(), nodes, links };
        graph.remove_leaf_links();

        assert_eq!(graph.links.len(), 2);
    }

    #[test]
    fn test_remove_multiple_leafs_in_sequence() {
        let nodes = vec![
            vec![0, 1],      // 0
            vec![0, 2],      // 1
            vec![1, 3, 4],   // 2
            vec![2, 3, 5],   // 3
            vec![4],         // 4
            vec![5, 6],      // 5
            vec![6, 7],      // 6
            vec![7],         // 7
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

        let mut graph = UnstableGraph { solver: Default::default(), nodes, links };
        graph.remove_leaf_links();

        assert_eq!(graph.nodes[0], vec![0, 1]);
        assert_eq!(graph.nodes[1], vec![0, 2]);
        assert_eq!(graph.nodes[2], vec![1, 3]);
        assert_eq!(graph.nodes[3], vec![2, 3]);
    }
}