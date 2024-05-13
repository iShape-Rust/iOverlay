use i_tree::node::EMPTY_REF;
use crate::fill::segment::Segment;
use crate::split::range_search::RangeSearch;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::split::store_index::StoreIndex;
use crate::split::sub_store_tree::SubStoreTree;
use crate::x_segment::XSegment;

pub(super) struct StoreTree {
    pub(super) ranges: Vec<i32>,
    pub(super) sub_stores: Vec<SubStoreTree>,
    pub(super) chunk_start_length: usize,
}


impl StoreTree {
    pub(super) fn new(edges: Vec<ShapeEdge>, chunk_start_length: usize) -> Self {
        if edges.len() <= chunk_start_length {
            return Self { ranges: vec![], sub_stores: vec![SubStoreTree::new(&edges)], chunk_start_length };
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
            sub_stores.push(SubStoreTree::new(&edges[i..j]));
            i = j;

            if i < edges.len() {
                ranges.push(x);
            }
        }

        Self { ranges, sub_stores, chunk_start_length }
    }

    #[inline]
    pub(super) fn first(&self, index: u32) -> StoreIndex {
        let mut i = index;
        let n = self.sub_stores.len() as u32;
        while i < n {
            let first_index = self.sub_tree(i).first();
            if first_index != EMPTY_REF {
                return StoreIndex { root: i, node: first_index };
            }
            i += 1;
        }

        StoreIndex { root: EMPTY_REF, node: EMPTY_REF }
    }

    #[inline]
    pub(super) fn edge(&self, index: StoreIndex) -> ShapeEdge {
        self.sub_tree(index.root).tree.node(index.node).value
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
        let node = self.sub_tree(index.root).tree.next_by_order(index.node);
        if node == EMPTY_REF {
            self.first(index.root + 1)
        } else {
            StoreIndex { root: index.root, node }
        }
    }

    #[inline]
    pub(super) fn get(&self, index: StoreIndex) -> ShapeEdge {
        self.sub_tree(index.root).tree.node(index.node).value
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

        let node = self.mut_sub_tree(root).merge(edge);
        StoreIndex { root, node }
    }

    #[inline(always)]
    fn sub_tree(&self, index: u32) -> &SubStoreTree {
        unsafe {
            self.sub_stores.get_unchecked(index as usize)
        }
    }

    #[inline(always)]
    fn mut_sub_tree(&mut self, index: u32) -> &mut SubStoreTree {
        unsafe {
            self.sub_stores.get_unchecked_mut(index as usize)
        }
    }

    #[inline(always)]
    fn find_sub_store(&self, x: i32) -> u32 {
        if self.ranges.is_empty() {
            0
        } else {
            self.ranges.find_index(x)
        }
    }

    pub(super) fn segments(&self) -> Vec<Segment> {
        let mut result = Vec::with_capacity(self.sub_stores.len() * self.chunk_start_length);

        for sub_store in self.sub_stores.iter() {
            let tree = &sub_store.tree;
            let mut n_index = tree.first_by_order();
            while n_index != EMPTY_REF {
                let e = &tree.node(n_index).value;
                if !e.count.is_empty() {
                    result.push(Segment::new(e));
                }
                n_index = tree.next_by_order(n_index);
            }
        }

        result
    }
}