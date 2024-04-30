use i_tree::node::EMPTY_REF;
use crate::fill::segment::Segment;
use crate::split::edge_sub_tree::EdgeSubTree;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::x_segment::XSegment;

#[derive(Debug, Clone, Copy)]
pub(super) struct StoreIndex {
    pub(super) tree: u32,
    pub(super) node: u32,
}

pub(super) struct EdgeStore {
    ranges: Vec<i32>,
    trees: Vec<EdgeSubTree>,
    range_length: usize,
}

impl EdgeStore {
    pub(super) fn new(edges: &Vec<ShapeEdge>, range_length: usize) -> Self {
        if edges.len() <= range_length {
            return Self { ranges: vec![], trees: vec![EdgeSubTree::new(edges)], range_length };
        }
        let n = edges.len() / range_length;
        let mut ranges = Vec::with_capacity(n - 1);
        let mut trees = Vec::with_capacity(n);

        let mut i = 0;
        while i < edges.len() {
            let mut j = i;
            let mut x = edges[i].x_segment.a.x;
            while j < edges.len() {
                let xj = edges[j].x_segment.a.x;
                if x != xj {
                    if j - i >= range_length {
                        break;
                    }
                    x = xj;
                }
                j += 1;
            }
            trees.push(EdgeSubTree::new(&edges[i..j]));
            i = j;

            if i < edges.len() {
                ranges.push(x);
            }
        }

        Self { ranges, trees, range_length }
    }

    pub(super) fn first(&self, index: u32) -> StoreIndex {
        let i0 = index as usize;
        let i1 = self.trees.len();
        for i in i0..i1 {
            let first_index = self.sub_tree(i).tree.first_by_order();
            if first_index != EMPTY_REF {
                return StoreIndex { tree: i as u32, node: first_index };
            }
        }

        StoreIndex { tree: EMPTY_REF, node: EMPTY_REF }
    }

    pub(super) fn edge(&self, index: StoreIndex) -> ShapeEdge {
        self.sub_tree(index.tree as usize).tree.node(index.node).value
    }

    pub(super) fn find(&self, x_segment: &XSegment) -> StoreIndex {
        let tree = self.find_tree(x_segment.a.x);
        let node = self.sub_tree(tree).find(x_segment);
        StoreIndex { tree: tree as u32, node }
    }

    pub(super) fn find_equal_or_next(&self, tree: u32, x_segment: &XSegment) -> StoreIndex {
        let node = self.sub_tree(tree as usize).find_equal_or_next(x_segment);
        if node == EMPTY_REF {
            self.first(tree + 1)
        } else {
            StoreIndex { tree, node }
        }
    }

    pub(super) fn next(&self, index: StoreIndex) -> StoreIndex {
        let node = self.sub_tree(index.tree as usize).tree.next_by_order(index.node);
        if node == EMPTY_REF {
            self.first(index.tree + 1)
        } else {
            StoreIndex { tree: index.tree, node }
        }
    }

    pub(super) fn get(&self, index: StoreIndex) -> ShapeEdge {
        self.sub_tree(index.tree as usize).tree.node(index.node).value
    }

    pub(super) fn get_and_remove(&mut self, index: StoreIndex) -> ShapeEdge {
        self.mut_sub_tree(index.tree as usize).get_and_remove(index.node)
    }

    pub(super) fn remove(&mut self, edge: &ShapeEdge) {
        let tree = self.find_tree(edge.x_segment.a.x);
        self.mut_sub_tree(tree).remove(edge);
    }

    pub(super) fn remove_index(&mut self, index: StoreIndex) {
        self.mut_sub_tree(index.tree as usize).remove_index(index.node);
    }

    pub(super) fn remove_and_next(&mut self, index: StoreIndex) -> StoreIndex {
        let next = self.mut_sub_tree(index.tree as usize).remove_and_next(index.node);
        if next == EMPTY_REF {
            self.first(index.tree + 1)
        } else {
            StoreIndex { tree: index.tree, node: next }
        }
    }

    pub(super) fn update(&mut self, index: StoreIndex, count: ShapeCount) {
        self.mut_sub_tree(index.tree as usize).update(index.node, count);
    }

    pub(super) fn add_and_merge(&mut self, edge: ShapeEdge) -> StoreIndex {
        let tree = self.find_tree(edge.x_segment.a.x);
        let node = self.mut_sub_tree(tree).merge(edge);
        StoreIndex { tree: tree as u32, node }
    }

    fn find_tree(&self, x: i32) -> usize {
        let mut left = 0;
        let mut right = self.ranges.len();

        while left < right {
            let mid = left + ((right - left) >> 1);
            unsafe {
                let val = *self.ranges.get_unchecked(mid);
                if val == x {
                    return mid;
                } else if val < x {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }

        left
    }

     fn sub_tree(&self, index: usize) -> &EdgeSubTree {
        unsafe {
            self.trees.get_unchecked(index)
        }
    }

    fn mut_sub_tree(&mut self, index: usize) -> &mut EdgeSubTree {
        unsafe {
            self.trees.get_unchecked_mut(index)
        }
    }

    pub(super) fn segments(&self) -> Vec<Segment> {
        let mut result = if self.trees.len() > 1 {
            Vec::with_capacity(self.trees.len() * self.range_length)
        } else {
            Vec::new()
        };

        let mut s_index = self.first(0);

        while s_index.node != EMPTY_REF {
            let tree = &self.sub_tree(s_index.tree as usize).tree;
            let mut n_index = tree.first_by_order();
            while n_index != EMPTY_REF {
                let e = &tree.node(n_index).value;
                result.push(Segment::new(e));
                n_index = tree.next_by_order(n_index);
            }

            s_index = self.first(s_index.tree + 1);
        }

        result
    }
}