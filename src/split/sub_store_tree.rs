use std::cmp::Ordering;
use i_tree::node::EMPTY_REF;
use i_tree::tree::Tree;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::x_segment::XSegment;

pub(super) struct SubStoreTree {
    pub(super) tree: Tree<ShapeEdge>,
}

impl SubStoreTree {
    #[inline]
    pub(super) fn new(edges: &[ShapeEdge]) -> Self {
        let n = edges.len();
        debug_assert!(n > 0);
        Self { tree: Tree::with_sorted_array(ShapeEdge::ZERO, edges, 2) }
    }

    #[inline(always)]
    pub(super) fn first(&self) -> u32 {
        self.tree.first_by_order()
    }

    pub(super) fn find(&self, x_segment: &XSegment) -> u32 {
        let mut index = self.tree.root;

        while index != EMPTY_REF {
            let node = self.tree.node(index);
            match node.value.x_segment.cmp(x_segment) {
                Ordering::Equal => {
                    return index;
                }
                Ordering::Less => {
                    index = node.right;
                }
                Ordering::Greater => {
                    index = node.left;
                }
            }
        }

        EMPTY_REF
    }

    pub(super) fn find_equal_or_next(&self, x_segment: &XSegment) -> u32 {
        let mut p_index = EMPTY_REF;
        let mut index = self.tree.root;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            match x_segment.cmp(&node.value.x_segment) {
                Ordering::Equal => {
                    return index;
                }
                Ordering::Less => {
                    p_index = index;
                    index = node.left;
                }
                Ordering::Greater => {
                    p_index = index;
                    index = node.right;
                }
            }
        }

        p_index
    }

    pub(super) fn remove_and_next(&mut self, r_index: u32) -> u32 {
        let x_segment = self.tree.node(r_index).value.x_segment.clone();
        _ = self.tree.delete_index(r_index);

        let mut index = self.tree.root;
        let mut p_index = EMPTY_REF;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            p_index = index;
            if x_segment < node.value.x_segment {
                index = node.left;
            } else {
                index = node.right;
            }
        }

        p_index
    }

    #[inline]
    pub(super) fn get_and_remove(&mut self, index: u32) -> ShapeEdge {
        let edge = self.tree.node(index).value.clone();
        _ = self.tree.delete_index(index);
        edge
    }

    #[inline]
    pub(super) fn remove_index(&mut self, index: u32) {
        _ = self.tree.delete_index(index);
    }

    #[inline]
    pub(super) fn remove(&mut self, edge: &ShapeEdge) {
        _ = self.tree.delete(edge);
    }

    #[inline]
    pub(super) fn update(&mut self, index: u32, count: ShapeCount) {
        _ = self.tree.mut_node(index).value.count = count;
    }

    pub(super) fn merge(&mut self, edge: ShapeEdge) -> u32 {
        let mut p_index = EMPTY_REF;
        let mut index = self.tree.root;
        let mut is_left = false;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            match edge.x_segment.cmp(&node.value.x_segment) {
                Ordering::Equal => {
                    let count = node.value.count.add(edge.count);
                    self.tree.mut_node(index).value.count = count;
                    return index;
                }
                Ordering::Less => {
                    p_index = index;
                    index = node.left;
                    is_left = true;
                }
                Ordering::Greater => {
                    p_index = index;
                    index = node.right;
                    is_left = false;
                }
            }
        }

        if p_index == EMPTY_REF {
            self.tree.insert_root(edge);
            self.tree.root
        } else {
            self.tree.insert_with_parent(edge, p_index, is_left)
        }
    }
}