use i_tree::node::EMPTY_REF;
use crate::fill::segment::Segment;
use crate::split::range_search::RangeSearch;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::split::store_index::StoreIndex;
use crate::split::store_tree::StoreTree;
use crate::split::sub_store_list::SubStoreList;
use crate::split::sub_store_tree::SubStoreTree;

pub(super) struct StoreList {
    ranges: Vec<i32>,
    sub_stores: Vec<SubStoreList>,
    chunk_start_length: usize,
}

impl StoreList {
    pub(super) fn new(edges: Vec<ShapeEdge>, chunk_start_length: usize) -> Self {
        if edges.len() <= chunk_start_length {
            return Self { ranges: vec![], sub_stores: vec![SubStoreList::with_edges(edges)], chunk_start_length };
        }
        let n = (edges.len() / chunk_start_length).max(1);
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
            sub_stores.push(SubStoreList::new(&edges[i..j]));
            i = j;

            if i < edges.len() {
                ranges.push(x);
            }
        }

        Self { ranges, sub_stores, chunk_start_length }
    }

    pub(super) fn is_tree_conversion_required(&self, chunk_list_max_size: usize) -> bool {
        for sub_store in self.sub_stores.iter() {
            if sub_store.edges.len() > chunk_list_max_size {
                return true;
            }
        }

        false
    }

    #[inline]
    pub(super) fn convert_to_tree(self) -> StoreTree {
        if self.ranges.is_empty() {
            let sub_stores = vec![SubStoreTree::new(&self.sub_stores[0].edges)];
            return StoreTree { ranges: self.ranges, sub_stores, chunk_start_length: self.chunk_start_length }
        }

        let mut ranges = Vec::with_capacity(self.ranges.len());
        let mut sub_stores = Vec::with_capacity(self.sub_stores.len());

        let mut i = 0;
        while i < self.sub_stores.len() {
            let edges = &self.sub_stores[i].edges;
            if !edges.is_empty() {
                sub_stores.push(SubStoreTree::new(edges));
                if i < self.ranges.len() {
                    ranges.push(self.ranges[i]);
                }
            }

            i += 1;
        }

        StoreTree { ranges, sub_stores, chunk_start_length: self.chunk_start_length }
    }

    #[inline]
    pub(super) fn first(&self, index: u32) -> StoreIndex {
        let i0 = index;
        let i1 = self.sub_stores.len() as u32;
        for i in i0..i1 {
            let sub_store = self.sub_store(i);
            if !sub_store.edges.is_empty() {
                return StoreIndex { root: i, node: 0 };
            }
        }

        StoreIndex { root: EMPTY_REF, node: EMPTY_REF }
    }

    #[inline]
    pub(super) fn edge(&self, index: StoreIndex) -> &ShapeEdge {
        self.sub_store(index.root).edge(index.node)
    }

    #[inline]
    pub(super) fn next(&self, index: StoreIndex) -> StoreIndex {
        let sub_store = self.sub_store(index.root);
        let next = index.node + 1;
        if next < sub_store.edges.len() as u32 {
            StoreIndex { root: index.root, node: next }
        } else {
            self.first(index.root + 1)
        }
    }

    #[inline]
    pub(super) fn get_and_remove(&mut self, index: StoreIndex) -> ShapeEdge {
        self.mut_sub_store(index.root).get_and_remove(index.node)
    }

    #[inline]
    pub(super) fn remove(&mut self, edge: &ShapeEdge) {
        let tree = self.find_sub_store(edge.x_segment.a.x);
        self.mut_sub_store(tree).remove(edge);
    }

    #[inline]
    pub(super) fn remove_index(&mut self, index: StoreIndex) {
        self.mut_sub_store(index.root).remove_index(index.node);
    }

    #[inline]
    pub(super) fn remove_and_next(&mut self, index: StoreIndex) -> StoreIndex {
        let next = self.mut_sub_store(index.root).remove_and_next(index.node);
        if next == EMPTY_REF {
            self.first(index.root + 1)
        } else {
            StoreIndex { root: index.root, node: next }
        }
    }

    #[inline]
    pub(super) fn update(&mut self, index: StoreIndex, count: ShapeCount) {
        self.mut_sub_store(index.root).update(index.node, count);
    }

    #[inline]
    pub(super) fn add_and_merge(&mut self, edge: ShapeEdge) -> StoreIndex {
        let root = self.find_sub_store(edge.x_segment.a.x);
        let node = self.mut_sub_store(root).merge(edge);
        StoreIndex { root, node }
    }

    #[inline(always)]
    fn sub_store(&self, index: u32) -> &SubStoreList {
        unsafe {
            self.sub_stores.get_unchecked(index as usize)
        }
    }

    #[inline(always)]
    fn mut_sub_store(&mut self, index: u32) -> &mut SubStoreList {
        unsafe {
            self.sub_stores.get_unchecked_mut(index as usize)
        }
    }

    #[inline(always)]
    fn find_sub_store(&self, x: i32) -> u32 {
        self.ranges.find_index(x)
    }

    pub(super) fn segments(&self) -> Vec<Segment> {
        let capacity = self.sub_stores.iter().fold(0, |acc, x| acc + x.edges.len());
        let mut result = Vec::with_capacity(capacity);

        for sub_store in self.sub_stores.iter() {
            if !sub_store.edges.is_empty() {
                for e in sub_store.edges.iter() {
                    result.push(Segment::new(e));
                }
            }
        }

        result
    }
}