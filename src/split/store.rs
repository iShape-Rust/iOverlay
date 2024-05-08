use i_tree::node::EMPTY_REF;
use crate::fill::segment::Segment;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::split::sub_store::SubStore;
use crate::split::sub_store::SubStore::{List, Tree};
use crate::split::sub_store_list::SubStoreList;
use crate::x_segment::XSegment;

#[derive(Debug, Clone, Copy)]
pub(super) struct StoreIndex {
    pub(super) root: u32,
    pub(super) node: u32,
}

pub(super) struct EdgeStore {
    ranges: Vec<i32>,
    sub_stores: Vec<SubStore>,
    chunk_start_length: usize,
    chunk_list_max_size: usize,
}

impl EdgeStore {
    pub(super) fn new(edges: &Vec<ShapeEdge>, chunk_start_length: usize, chunk_list_max_size: usize) -> Self {
        if edges.len() <= chunk_start_length {
            return Self { ranges: vec![], sub_stores: vec![List(SubStoreList::new(edges))], chunk_start_length, chunk_list_max_size };
        }
        let n = edges.len() / chunk_start_length;
        let mut ranges = Vec::with_capacity(n - 1);
        let mut sub_stores = Vec::with_capacity(n);

        let mut i = 0;
        while i < edges.len() {
            let mut j = i;
            let mut x = edges[i].x_segment.a.x;
            while j < edges.len() {
                let xj = edges[j].x_segment.a.x;
                if x != xj {
                    if j - i >= chunk_start_length {
                        break;
                    }
                    x = xj;
                }
                j += 1;
            }
            sub_stores.push(SubStore::new(&edges[i..j], chunk_list_max_size));
            i = j;

            if i < edges.len() {
                ranges.push(x);
            }
        }

        Self { ranges, sub_stores, chunk_start_length, chunk_list_max_size }
    }

    #[inline]
    pub(super) fn first(&self, index: u32) -> StoreIndex {
        let i0 = index;
        let i1 = self.sub_stores.len() as u32;
        for i in i0..i1 {
            let first_index = self.sub_tree(i).first();
            if first_index != EMPTY_REF {
                return StoreIndex { root: i, node: first_index };
            }
        }

        StoreIndex { root: EMPTY_REF, node: EMPTY_REF }
    }

    #[inline]
    pub(super) fn edge(&self, index: StoreIndex) -> ShapeEdge {
        self.sub_tree(index.root).get(index.node)
    }

    #[inline]
    pub(super) fn find(&self, x_segment: &XSegment) -> StoreIndex {
        let tree = self.find_sub_store(x_segment.a.x);
        let node = self.sub_tree(tree).find(x_segment);
        StoreIndex { root: tree, node }
    }

    #[inline]
    pub(super) fn find_equal_or_next(&self, tree: u32, x_segment: &XSegment) -> StoreIndex {
        let node = self.sub_tree(tree).find_equal_or_next(x_segment);
        if node == EMPTY_REF {
            self.first(tree + 1)
        } else {
            StoreIndex { root: tree, node }
        }
    }

    #[inline]
    pub(super) fn next(&self, index: StoreIndex) -> StoreIndex {
        let node = self.sub_tree(index.root).next(index.node);
        if node == EMPTY_REF {
            self.first(index.root + 1)
        } else {
            StoreIndex { root: index.root, node }
        }
    }

    #[inline]
    pub(super) fn get(&self, index: StoreIndex) -> ShapeEdge {
        self.sub_tree(index.root).get(index.node)
    }

    #[inline]
    pub(super) fn get_and_remove(&mut self, index: StoreIndex) -> ShapeEdge {
        self.mut_sub_tree(index.root).get_and_remove(index.node)
    }

    #[inline]
    pub(super) fn remove(&mut self, edge: &ShapeEdge) {
        let tree = self.find_sub_store(edge.x_segment.a.x);
        self.mut_sub_tree(tree).remove(edge);
    }

    #[inline]
    pub(super) fn remove_index(&mut self, index: StoreIndex) {
        self.mut_sub_tree(index.root).remove_index(index.node);
    }

    #[inline]
    pub(super) fn remove_and_next(&mut self, index: StoreIndex) -> StoreIndex {
        let next = self.mut_sub_tree(index.root).remove_and_next(index.node);
        if next == EMPTY_REF {
            self.first(index.root + 1)
        } else {
            StoreIndex { root: index.root, node: next }
        }
    }

    #[inline]
    pub(super) fn update(&mut self, index: StoreIndex, count: ShapeCount) {
        self.mut_sub_tree(index.root).update(index.node, count);
    }

    #[inline]
    pub(super) fn add_and_merge(&mut self, edge: ShapeEdge) -> StoreIndex {
        let root = self.find_sub_store(edge.x_segment.a.x);
        let chunk_list_max_size = self.chunk_list_max_size;
        self.mut_sub_tree(root).increase(chunk_list_max_size);

        let node = self.mut_sub_tree(root).merge(edge);
        StoreIndex { root, node }
    }

    fn find_sub_store(&self, x: i32) -> u32 {
        let mut left = 0;
        let mut right = self.ranges.len();

        while left < right {
            let mid = left + ((right - left) >> 1);
            unsafe {
                let val = *self.ranges.get_unchecked(mid);
                if val == x {
                    return mid as u32;
                } else if val < x {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }

        left as u32
    }

    #[inline(always)]
     fn sub_tree(&self, index: u32) -> &SubStore {
        unsafe {
            self.sub_stores.get_unchecked(index as usize)
        }
    }

    #[inline(always)]
    fn mut_sub_tree(&mut self, index: u32) -> &mut SubStore {
        unsafe {
            self.sub_stores.get_unchecked_mut(index as usize)
        }
    }

    pub(super) fn segments(&self) -> Vec<Segment> {
        let mut result = if self.sub_stores.len() > 1 {
            Vec::with_capacity(self.sub_stores.len() * self.chunk_start_length)
        } else {
            Vec::new()
        };

        let mut s_index = self.first(0);

        while s_index.node != EMPTY_REF {
            match &self.sub_tree(s_index.root) {
                List(store) => {
                    for e in store.edges.iter() {
                        result.push(Segment::new(e));
                    }
                }
                Tree(store) => {
                    let tree = &store.tree;
                    let mut n_index = tree.first_by_order();
                    while n_index != EMPTY_REF {
                        let e = &tree.node(n_index).value;
                        result.push(Segment::new(e));
                        n_index = tree.next_by_order(n_index);
                    }
                }
            }
            s_index = self.first(s_index.root + 1);
        }

        result
    }
}