use crate::split::shape_edge::ShapeEdge;
use crate::index::EMPTY_INDEX;

const EMPTY_NODE: EdgeNode = EdgeNode { next: EMPTY_INDEX, prev: EMPTY_INDEX, edge: ShapeEdge::ZERO };

#[derive(Debug, Clone, Copy)]
pub (super) struct EdgeNode {
    pub (super) next: usize,
    prev: usize,
    pub (super) edge: ShapeEdge
}

impl EdgeNode {
    pub (super) fn is_removed(&self) -> bool {
        self.next == EMPTY_INDEX && self.prev == EMPTY_INDEX
    }
}

pub (super) struct EdgeLinkedList {
    free: Vec<usize>,
    pub (super) nodes: Vec<EdgeNode>,
    first: usize,
}

impl EdgeLinkedList {

    pub (super) fn new(edges: &[ShapeEdge]) -> Self {
        let plus_capacity = 16;
        let mut nodes = vec![EMPTY_NODE; edges.len() + plus_capacity];
        let mut free = Vec::with_capacity(plus_capacity);

        for i in 0..edges.len() - 1 {
            nodes[i] = EdgeNode {
                next: i + 1,
                prev: i.wrapping_sub(1), // i = 0 => EMPTY_INDEX
                edge: edges[i],
            };
        }

        nodes[edges.len() - 1] = EdgeNode {
            next: EMPTY_INDEX,
            prev: edges.len() - 2,
            edge: edges[edges.len() - 1],
        };

        for i in (edges.len()..edges.len() + plus_capacity).rev() {
            free.push(i);
        }

        EdgeLinkedList {
            free,
            nodes,
            first: 0,
        }
    }

    pub (super) fn first(&self) -> usize {
        self.first
    }

    pub (super) fn node(&self, index: usize) -> EdgeNode {
        self.nodes[index]
    }

    pub (super) fn remove(&mut self, index: usize) {
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
        
        self.nodes[index] = EMPTY_NODE;
        
        self.free.push(index);
      }

      pub (super) fn add_and_merge(&mut self, anchor_index: usize, new_edge: ShapeEdge) -> usize {
        if self.free.is_empty() {
            let new_index = self.nodes.len();
            self.nodes.push(EMPTY_NODE);
            self.free.push(new_index);
        }

        let anchor = self.nodes[anchor_index];
            
        if new_edge.is_less(anchor.edge) {
            // search back
            let mut next_ix = anchor_index;
            let mut next = anchor;
            let mut i = anchor.prev;
            while i != EMPTY_INDEX {
                let mut node = self.nodes[i];
                if new_edge.is_less(node.edge) {
                    next_ix = i;
                    next = node;
                    i = node.prev;
                } else if node.edge.is_equal(new_edge) {
                    
                    // merge
                    
                    let count = node.edge.count.add(new_edge.count);
                    
                    node.edge = ShapeEdge::from_parent(new_edge, count);
                    self.nodes[i] = node;
                    
                    return i
                } else {
                    
                    // insert new
                    
                    let free = self.free.pop().unwrap();
                    node.next = free;
                    next.prev = free;
                    self.nodes[i] = node;
                    self.nodes[free] = EdgeNode { next: next_ix, prev: i, edge: new_edge };
                    self.nodes[next_ix] = next;
                    
                    return free
                }
            }
            
            // nothing is found
            // add as first

            let free = self.free.pop().unwrap();
            self.first = free;
            next.prev = free;
            self.nodes[free] = EdgeNode { next: next_ix, prev: EMPTY_INDEX, edge: new_edge };
            self.nodes[next_ix] = next;
            
            return free
        } else {
            // search forward
            let mut prev_ix = anchor_index;
            let mut prev = anchor;
            let mut i = anchor.next;
            while i != EMPTY_INDEX {
                let mut node = self.nodes[i];
                if node.edge.is_less(new_edge) {
                    prev_ix = i;
                    prev = node;
                    i = node.next;
                } else if node.edge.is_equal(new_edge) {
                    
                    // merge
                    
                    let count = node.edge.count.add(new_edge.count);

                    node.edge = ShapeEdge::from_parent(new_edge, count );
                    self.nodes[i] = node;
                    
                    return i
                } else {
                    
                    // insert new
                    
                    let free = self.free.pop().unwrap();
                    node.prev = free;
                    prev.next = free;
                    self.nodes[i] = node;
                    self.nodes[prev_ix] = prev;
                    self.nodes[free] = EdgeNode { next: i, prev: prev_ix, edge: new_edge };
                    
                    return free
                }
            }
            
            // nothing is found
            // add as last

            let free = self.free.pop().unwrap();
            prev.next = free;
            self.nodes[free] = EdgeNode { next: EMPTY_INDEX, prev: prev_ix, edge: new_edge };
            self.nodes[prev_ix] = prev;
            
            return free
        }
    }

      pub (crate) fn edges(&self) -> Vec<ShapeEdge> {
        let mut edges = Vec::with_capacity(self.nodes.len());
        let mut index = self.first;
        while index != EMPTY_INDEX {
            let node = self.nodes[index];
            
            if !node.edge.count.is_even() {
                edges.push(node.edge);
            }

            index = node.next
        }
        return edges
    }
    
}