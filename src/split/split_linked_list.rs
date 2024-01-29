use crate::split::shape_edge::ShapeEdge;
use crate::index::EMPTY_INDEX;
use crate::split::shape_count::ShapeCount;

#[derive(Debug, Clone, Copy)]
pub(super) struct SplitLinkedListNode {
    pub(super) version: usize,
    pub(super) next: usize,
    prev: usize,
    pub(super) edge: ShapeEdge,
}

impl SplitLinkedListNode {
    pub(super) fn is_removed(&self) -> bool {
        self.version == 0
    }

    pub(super) fn clear(&mut self) {
        self.next = EMPTY_INDEX;
        self.prev = EMPTY_INDEX;
        self.edge.count = ShapeCount::new(0, 0);
        self.version += 1;
    }

    pub(super) fn update_edge(&mut self, edge: ShapeEdge) -> usize {
        self.version += 1;
        self.edge = edge;

        self.version
    }

    pub(super) fn update_count(&mut self, count: ShapeCount) -> usize {
        self.version += 1;
        self.edge.count = count;

        self.version
    }
}

pub(super) struct SplitLinkedList {
    free: Vec<usize>,
    pub(super) nodes: Vec<SplitLinkedListNode>,
    first: usize,
}

impl SplitLinkedList {

    pub(super) fn new(edges: &[ShapeEdge]) -> Self {
        let extra_capacity = 16.min(edges.len() / 2);
        let capacity = edges.len() + extra_capacity;

        let mut nodes = Vec::with_capacity(capacity);

        let mut index: usize = 0;
        for edge in edges.iter() {
            let node = SplitLinkedListNode { version: 1, next: index + 1, prev: index.wrapping_sub(1), edge: edge.clone() };
            nodes.push(node);
            index += 1;
        }

        nodes[edges.len() - 1].next = EMPTY_INDEX;

        let n = nodes.len();
        let mut free = Vec::with_capacity(capacity - n);
        let mut i = capacity - 1;
        while i >= n {
            free.push(i);
            nodes.push(SplitLinkedListNode { version: 0, next: EMPTY_INDEX, prev: EMPTY_INDEX, edge: ShapeEdge::ZERO });
            i -= 1
        }

        Self { free, nodes, first: 0 }
    }

    pub(super) fn first(&self) -> usize {
        self.first
    }

    pub(super) fn remove(&mut self, index: usize) {
        let node = self.nodes[index];

        if node.prev != EMPTY_INDEX {
            let mut prev = self.nodes[node.prev];
            prev.next = node.next;
            self.nodes[node.prev] = prev;
        } else {
            self.first = node.next
        }

        if node.next != EMPTY_INDEX {
            let mut next = self.nodes[node.next];
            next.prev = node.prev;
            self.nodes[node.next] = next;
        }

        self.nodes[index].clear();

        self.free.push(index);
    }

    pub(super) fn update_edge(&mut self, index: usize, edge: ShapeEdge) -> usize {
        self.nodes[index].update_edge(edge)
    }

    pub(super) fn update_count(&mut self, index: usize, count: ShapeCount) -> usize {
        self.nodes[index].update_count(count)
    }

    pub(super) fn find_from_start(&mut self, edge: &ShapeEdge) -> usize {
        if self.first != EMPTY_INDEX {
            let first_edge = self.nodes[self.first].edge;
            if first_edge.is_equal(edge) {
                self.first
            } else if edge.is_less(&first_edge) {
                let old_first = self.first;
                self.first = self.any_free();
                self.nodes[old_first].prev = self.first;
                self.nodes[self.first].next = old_first;
                self.first
            } else {
                self.find_forward(self.first, edge)
            }
        } else {
            self.first = self.any_free();
            self.first
        }
    }

    fn find_back(&mut self, from_index: usize, edge: &ShapeEdge) -> usize {
        let mut node_prev = self.nodes[from_index].prev;
        let mut next_index = from_index;

        while node_prev != EMPTY_INDEX {
            let prev_edge = self.nodes[node_prev].edge;
            if prev_edge.is_less(edge) {
                // insert new
                let new_index = self.any_free();

                self.nodes[node_prev].next = new_index;
                self.nodes[new_index].next = next_index;
                self.nodes[new_index].prev = node_prev;
                self.nodes[next_index].prev = new_index;

                return new_index;
            } else if prev_edge.is_equal(edge) {
                return node_prev;
            }

            next_index = node_prev;
            node_prev = self.nodes[node_prev].prev;
        }

        // insert new as first
        self.first = self.any_free();

        self.nodes[self.first].next = next_index;
        self.nodes[next_index].prev = self.first;

        self.first
    }

    fn find_forward(&mut self, from_index: usize, edge: &ShapeEdge) -> usize {
        let mut prev_next = self.nodes[from_index].next;
        let mut prev_index = from_index;

        while prev_next != EMPTY_INDEX {
            let next_index = prev_next;
            let next_edge = self.nodes[next_index].edge;
            if edge.is_less(&next_edge) {
                // insert new
                let new_index = self.any_free();

                self.nodes[prev_index].next = new_index;
                self.nodes[new_index].next = next_index;
                self.nodes[new_index].prev = prev_index;
                self.nodes[next_index].prev = new_index;

                return new_index;
            } else if next_edge.is_equal(edge) {
                return next_index;
            }

            prev_index = next_index;
            prev_next = self.nodes[next_index].next;
        }

        // insert new as last
        let new_index = self.any_free();

        self.nodes[prev_index].next = new_index;
        self.nodes[new_index].prev = prev_index;

        new_index
    }

    pub(super) fn find(&mut self, anchor_index: usize, edge: &ShapeEdge) -> usize {
        let anchor = &self.nodes[anchor_index];
        if anchor.is_removed() {
            return self.find_from_start(edge);
        }

        if edge.is_equal(&anchor.edge) {
            anchor_index
        } else if edge.is_less(&anchor.edge) {
            self.find_back(anchor_index, edge)
        } else {
            self.find_forward(anchor_index, edge)
        }
    }

    fn any_free(&mut self) -> usize {
        if self.free.is_empty() {
            let new_index = self.nodes.len();
            self.nodes.push(SplitLinkedListNode { version: 1, next: EMPTY_INDEX, prev: EMPTY_INDEX, edge: ShapeEdge::ZERO });
            new_index
        } else {
            self.free.pop().unwrap()
        }
    }
}