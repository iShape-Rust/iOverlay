use crate::split::shape_edge::ShapeEdge;
use crate::split::edge_linked_list::EdgeLinkedList;

const EMPTY: usize = std::usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    next: usize,
    prev: usize,
    is_present: bool
}

struct ScanList {
    nodes: Vec<Node>,
    first: usize
}

impl ScanList {

    pub(super) fn new(count: usize) -> Self {
        ScanList {
            nodes: vec![Node { next: EMPTY, prev: EMPTY, is_present: false }; count],
            first: EMPTY
        }
    }

    pub(super) fn next(&self, index: usize) -> usize {
        self.nodes[index].next
    }

    pub(super) fn add(&mut self, index: usize) {
        if index == !1 {
            return;
        }

        if index >= self.nodes.len() {
            self.nodes.resize(index + 1, Node { next: EMPTY, prev: EMPTY, is_present: false });
        }

        if self.nodes[index].is_present {
            return;
        }

        if self.first == EMPTY {
            self.first = index;
            self.nodes[index] = Node {
                next: EMPTY,
                prev: EMPTY,
                is_present: true,
            };
            return;
        }

        let mut next = self.nodes[self.first];
        next.prev = index;
        self.nodes[self.first] = next;

        self.nodes[index] = Node {
            next: self.first,
            prev: EMPTY,
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
        
        if node.prev != EMPTY {
            let mut prev = self.nodes[node.prev];
            prev.next = node.next;
            self.nodes[node.prev] = prev;
        }
        
        if node.next != EMPTY {
            let mut next = self.nodes[node.next];
            next.prev = node.prev;
            self.nodes[node.next] = next;
        }
        
        if self.first == index {
            self.first = node.next;
        }
        
        self.nodes[index] = Node { next: EMPTY, prev: EMPTY, is_present: false };
    }

    fn clear(&mut self) {
        self.nodes.iter_mut().for_each(|node| *node = Node { next: EMPTY, prev: EMPTY, is_present: false });
        self.first = EMPTY;
    }

    fn remove_all_less_or_equal(&mut self, edge: ShapeEdge, list: &EdgeLinkedList) {
        let mut s_index = self.first;
        
        // Try to intersect the current segment with all the segments in the scan list.
        while s_index != EMPTY {
            let scan_edge = list.edge(s_index);
            if edge.is_less_or_equal(scan_edge) {
                s_index = self.remove_and_get_next(s_index);
                continue;
            }
            s_index = self.next(s_index)
        }
    }

    fn remove_and_get_next(&mut self, index: usize) -> usize {
        let node = self.nodes[index];
        
        if node.prev != EMPTY {
            let mut prev = self.nodes[node.prev];
            prev.next = node.next;
            self.nodes[node.prev] = prev;
        } else {
            self.first = node.next;
        }
        
        if node.next != EMPTY {
            let mut next = self.nodes[node.next];
            next.prev = node.prev;
            self.nodes[node.next] = next;
        }
        
        if self.first == index {
            self.first = node.next;
        }
        
        self.nodes[index] = Node { next: EMPTY, prev: EMPTY, is_present: false };
        
        return node.next
    }


}
