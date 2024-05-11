use std::cmp::Ordering;
use i_tree::node::EMPTY_REF;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::x_segment::XSegment;

pub(super) struct SubStoreList {
    pub(super) edges: Vec<ShapeEdge>,
}

struct SearchResult {
    is_exist: bool,
    index: u32,
}

impl SubStoreList {
    #[inline(always)]
    pub(super) fn new(edges: &[ShapeEdge]) -> Self {
        let n = edges.len();
        debug_assert!(n > 0);
        Self { edges: edges.to_vec() }
    }

    #[inline(always)]
    pub(super) fn with_edges(edges: Vec<ShapeEdge>) -> Self {
        let n = edges.len();
        debug_assert!(n > 0);
        Self { edges }
    }

    #[inline(always)]
    pub(super) fn first(&self) -> u32 {
        if self.edges.is_empty() {
            EMPTY_REF
        } else {
            0
        }
    }

    #[inline(always)]
    pub(super) fn get_and_remove(&mut self, index: u32) -> ShapeEdge {
        self.edges.remove(index as usize)
    }

    #[inline(always)]
    pub(super) fn remove_and_next(&mut self, index: u32) -> u32 {
        self.edges.remove(index as usize);
        if index < self.edges.len() as u32 {
            index
        } else {
            EMPTY_REF
        }
    }

    #[inline(always)]
    pub(super) fn next(&self, index: u32) -> u32 {
        let next = index + 1;
        if next < self.edges.len() as u32 {
            next
        } else {
            EMPTY_REF
        }
    }

    #[inline(always)]
    pub(super) fn remove(&mut self, edge: &ShapeEdge) {
        self.edges.remove(self.index(&edge.x_segment).index as usize);
    }

    #[inline(always)]
    pub(super) fn remove_index(&mut self, index: u32) {
        self.edges.remove(index as usize);
    }

    #[inline(always)]
    pub(super) fn update(&mut self, index: u32, count: ShapeCount) {
        self.mut_edge(index).count = count
    }

    #[inline(always)]
    pub(super) fn merge(&mut self, edge: ShapeEdge) -> u32 {
        let result = self.index(&edge.x_segment);
        if result.is_exist {
            self.mut_edge(result.index).count.accumulate(edge.count);
        } else {
            self.edges.insert(result.index as usize, edge);
        }

        result.index
    }

    #[inline]
    fn index(&self, target: &XSegment) -> SearchResult {
        let mut left = 0;
        let mut right = self.edges.len() as u32;

        while left < right {
            let mid = left + ((right - left) >> 1);
            match self.edge(mid).x_segment.cmp(target) {
                Ordering::Equal => {
                    return SearchResult { is_exist: true, index: mid };
                }
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Greater => {
                    right = mid;
                }
            }
        }

        SearchResult { is_exist: false, index: left }
    }

    #[inline(always)]
    pub(super) fn edge(&self, index: u32) -> &ShapeEdge {
        unsafe { self.edges.get_unchecked(index as usize) }
    }

    #[inline(always)]
    fn mut_edge(&mut self, index: u32) -> &mut ShapeEdge {
        unsafe { self.edges.get_unchecked_mut(index as usize) }
    }
}