use crate::split::shape_edge::ShapeEdge;
use crate::split::edge_linked_list::EdgeLinkedList;
use crate::index::EMPTY_INDEX;

const EMPTY_NODE: Node = Node { next: EMPTY_INDEX, prev: EMPTY_INDEX, is_present: false };

#[derive(Debug, Clone, Copy)]
struct Node {
    next: usize,
    prev: usize,
    is_present: bool
}

pub (super) struct ScanList {
    nodes: Vec<Node>,
    first: usize
}

impl ScanList {

    pub(super) fn new(count: usize) -> Self {
        ScanList {
            nodes: vec![EMPTY_NODE; count],
            first: EMPTY_INDEX
        }
    }

    pub(super) fn first(&self) -> usize {
        self.first
    }

    pub(super) fn next(&self, index: usize) -> usize {
        self.nodes[index].next
    }

    pub(super) fn add(&mut self, index: usize) {
        if index == !1 {
            return;
        }

        if index >= self.nodes.len() {
            self.nodes.resize(index + 1, EMPTY_NODE);
        }

        if self.nodes[index].is_present {
            return;
        }

        if self.first == EMPTY_INDEX {
            self.first = index;
            self.nodes[index] = Node {
                next: EMPTY_INDEX,
                prev: EMPTY_INDEX,
                is_present: true,
            };
            return;
        }

        let mut next = self.nodes[self.first];
        next.prev = index;
        self.nodes[self.first] = next;

        self.nodes[index] = Node {
            next: self.first,
            prev: EMPTY_INDEX,
            is_present: true,
        };

        self.first = index;
    }

    pub(super) fn remove(&mut self, index: usize) {
        if index >= self.nodes.len() {
            return
        }
        
        let node = self.nodes[index];

        if !node.is_present {
            return
        }
        
        if node.prev != EMPTY_INDEX {
            let mut prev = self.nodes[node.prev];
            prev.next = node.next;
            self.nodes[node.prev] = prev;
        }
        
        if node.next != EMPTY_INDEX {
            let mut next = self.nodes[node.next];
            next.prev = node.prev;
            self.nodes[node.next] = next;
        }
        
        if self.first == index {
            self.first = node.next;
        }
        
        self.nodes[index] = EMPTY_NODE;
    }

    pub(super) fn clear(&mut self) {
        self.nodes.iter_mut().for_each(|node| *node = EMPTY_NODE);
        self.first = EMPTY_INDEX;
    }

    pub(super) fn remove_all_less_or_equal(&mut self, edge: ShapeEdge, list: &EdgeLinkedList) {
        let mut s_index = self.first;
        
        // Try to intersect the current segment with all the segments in the scan list.
        while s_index != EMPTY_INDEX {
            let scan_edge = list.edge(s_index);
            if edge.is_less_or_equal(scan_edge) {
                s_index = self.remove_and_get_next(s_index);
                continue;
            }
            s_index = self.next(s_index)
        }
    }

    pub(super) fn remove_and_get_next(&mut self, index: usize) -> usize {
        let node = self.nodes[index];
        
        if node.prev != EMPTY_INDEX {
            let mut prev = self.nodes[node.prev];
            prev.next = node.next;
            self.nodes[node.prev] = prev;
        } else {
            self.first = node.next;
        }
        
        if node.next != EMPTY_INDEX {
            let mut next = self.nodes[node.next];
            next.prev = node.prev;
            self.nodes[node.next] = next;
        }
        
        if self.first == index {
            self.first = node.next;
        }
        
        self.nodes[index] = EMPTY_NODE;
        
        return node.next
    }


}
