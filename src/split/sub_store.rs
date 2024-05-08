use std::mem;
use i_tree::node::EMPTY_REF;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::split::sub_store::SubStore::{List, Tree};
use crate::split::sub_store_list::SubStoreList;
use crate::split::sub_store_tree::SubStoreTree;
use crate::x_segment::XSegment;

pub(super) enum SubStore {
    List(SubStoreList),
    Tree(SubStoreTree),
}

impl SubStore {

    #[inline(always)]
    pub(super) fn new(edges: &[ShapeEdge], chunk_list_max_size: usize) -> Self {
        if edges.len() > chunk_list_max_size {
            Tree(SubStoreTree::new(edges))
        } else {
            List(SubStoreList::new(edges))
        }
    }

    #[inline(always)]
    pub(super) fn first(&self) -> u32 {
        match self {
            List(store) => {
                store.first()
            }
            Tree(store) => {
                store.first()
            }
        }
    }

    #[inline(always)]
    pub(super) fn find(&self, x_segment: &XSegment) -> u32 {
        match self {
            List(store) => {
                store.find(x_segment)
            }
            Tree(store) => {
                store.find(x_segment)
            }
        }
    }

    #[inline(always)]
    pub(super) fn find_equal_or_next(&self, x_segment: &XSegment) -> u32 {
        match self {
            List(store) => {
                store.find_equal_or_next(x_segment)
            }
            Tree(store) => {
                store.find_equal_or_next(x_segment)
            }
        }
    }

    #[inline(always)]
    pub(super) fn get_and_remove(&mut self, index: u32) -> ShapeEdge {
        match self {
            List(store) => {
                store.get_and_remove(index)
            }
            Tree(store) => {
                store.get_and_remove(index)
            }
        }
    }

    #[inline(always)]
    pub(super) fn remove_and_next(&mut self, index: u32) -> u32 {
        match self {
            List(store) => {
                store.remove_and_next(index)
            }
            Tree(store) => {
                store.remove_and_next(index)
            }
        }
    }

    #[inline(always)]
    pub(super) fn remove(&mut self, edge: &ShapeEdge) {
        match self {
            List(store) => {
                store.remove(edge)
            }
            Tree(store) => {
                store.remove(edge)
            }
        }
    }

    #[inline(always)]
    pub(super) fn remove_index(&mut self, index: u32) {
        match self {
            List(store) => {
                store.remove_index(index);
            }
            Tree(store) => {
                store.remove_index(index);
            }
        }
    }

    #[inline(always)]
    pub(super) fn update(&mut self, index: u32, count: ShapeCount) {
        match self {
            List(store) => {
                store.update(index, count);
            }
            Tree(store) => {
                store.update(index, count);
            }
        }
    }

    #[inline(always)]
    pub(super) fn merge(&mut self, edge: ShapeEdge) -> u32 {
        match self {
            List(store) => {
                store.merge(edge)
            }
            Tree(store) => {
                store.merge(edge)
            }
        }
    }

    #[inline(always)]
    pub(super) fn next(&self, index: u32) -> u32 {
        match self {
            List(store) => {
                let next = index + 1;
                if next < store.edges.len() as u32 {
                    next
                } else {
                    EMPTY_REF
                }
            }
            Tree(store) => {
                store.tree.next_by_order(index)
            }
        }
    }

    #[inline(always)]
    pub(super) fn get(&self, index: u32) -> ShapeEdge {
        match self {
            List(store) => {
                store.edge(index).clone()
            }
            Tree(store) => {
                store.tree.node(index).value
            }
        }
    }

    #[inline(always)]
    pub(super) fn increase(&mut self, max_list_size: usize) {
        let new_self = match *self {
            List(ref mut store) if store.edges.len() > max_list_size => {
                Tree(SubStoreTree::new(&store.edges))
            }
            _ => return
        };

        let _ = mem::replace(self, new_self);
    }
}