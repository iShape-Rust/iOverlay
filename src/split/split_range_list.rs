use crate::dual_index::DualIndex;
use crate::fill::segment::Segment;
use crate::index::EMPTY_INDEX;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge::ShapeEdge;
use crate::split::split_linked_list::SplitLinkedList;
use crate::split::version_index::VersionedIndex;

pub(super) struct SplitRangeList {
    ranges: Vec<i64>,
    lists: Vec<SplitLinkedList>,
}

impl SplitRangeList {
    const RANGE_LENGTH: usize = 2;

    pub fn edge(&self, index: DualIndex) -> &ShapeEdge {
        &self.lists[index.major].nodes[index.minor].edge
    }

    pub(super) fn validate_edge(&self, v_index: VersionedIndex) -> Option<ShapeEdge> {
        let node = self.lists[v_index.index.major].nodes[v_index.index.minor];
        if node.version != v_index.version {
            None
        } else {
            Some(node.edge)
        }
    }

    pub(super) fn first(&self) -> VersionedIndex {
        self.first_by_index(0)
    }

    fn first_by_index(&self, index: usize) -> VersionedIndex {
        for i in index..self.lists.len() {
            let first_index = self.lists[i].first();
            if first_index != EMPTY_INDEX {
                let node = self.lists[i].nodes[first_index];
                return VersionedIndex { version: node.version, index: DualIndex { major: i, minor: first_index } };
            }
        }

        VersionedIndex::EMPTY
    }

    pub(super) fn next(&self, index: DualIndex) -> VersionedIndex {
        let node = &self.lists[index.major].nodes[index.minor];
        if node.next != EMPTY_INDEX {
            let version = self.lists[index.major].nodes[node.next].version;
            return VersionedIndex { version, index: DualIndex { major: index.major, minor: node.next } };
        } else if (index.major) < self.lists.len() {
            self.first_by_index(index.major + 1)
        } else {
            VersionedIndex::EMPTY
        }
    }

    pub(super) fn remove_and_next(&mut self, index: DualIndex) -> VersionedIndex {
        let next_index = self.next(index);
        self.lists[index.major].remove(index.minor);
        next_index
    }

    pub(super) fn remove(&mut self, index: DualIndex) {
        self.lists[index.major].remove(index.minor);
    }

    pub(super) fn update_edge(&mut self, index: DualIndex, edge: ShapeEdge) -> usize {
        self.lists[index.major].update_edge(index.minor, edge)
    }

    pub(super) fn update_count(&mut self, index: DualIndex, count: ShapeCount) -> usize {
        self.lists[index.major].update_count(index.minor, count)
    }

    pub(super) fn add_and_merge(&mut self, anchor_index: DualIndex, new_edge: ShapeEdge) -> VersionedIndex {
        let index = self.find_index(anchor_index, &new_edge);
        let edge = self.edge(index);
        let version = if edge.is_equal(&new_edge) {
            self.update_count(index, edge.count.add(new_edge.count))
        } else {
            self.update_edge(index, new_edge)
        };

        VersionedIndex { version, index }
    }

    pub(super) fn find_index(&mut self, anchor_index: DualIndex, edge: &ShapeEdge) -> DualIndex {
        let a = edge.a.bit_pack();
        let base: usize;
        let node: usize;
        if self.ranges[anchor_index.major] < a && a <= self.ranges[(anchor_index.major) + 1] {
            base = anchor_index.major;
            node = self.lists[base].find(anchor_index.minor, edge);
        } else {
            base = self.ranges.find_index(a) - 1; // -1 is ranges offset
            node = self.lists[base].find_from_start(edge);
        }

        DualIndex { major: base, minor: node }
    }

    pub(super) fn new(edges: &Vec<ShapeEdge>) -> Self {
        // array must be sorted

        let n = (edges.len() - 1) / Self::RANGE_LENGTH + 1;
        let length = edges.len() / n;

        let mut ranges = Vec::with_capacity(n + 1);
        ranges.push(i64::MIN);

        let mut lists = Vec::with_capacity(n);

        let min_length = Self::RANGE_LENGTH / 2 + 1;

        let mut i = 0;
        while i < edges.len() {
            let i0 = i;

            i = (edges.len() - 1).min(i + length);
            let a = edges[i].a;
            i += 1;
            while i < edges.len() && edges[i].a == a {
                i += 1
            }

            if i + min_length >= edges.len() {
                let slice = &edges[i0..edges.len()];
                lists.push(SplitLinkedList::new(slice));
                ranges.push(i64::MAX);
                break;
            } else {
                let slice = &edges[i0..i];
                lists.push(SplitLinkedList::new(slice));
                ranges.push(a.bit_pack());
            }
        }

        Self { ranges, lists }
    }

    pub(super) fn segments(&self) -> Vec<Segment> {
        let mut n = 0;
        for list in self.lists.iter() {
            n += list.nodes.len();
        }
        let mut result = Vec::with_capacity(n);
        let mut v_index = self.first();

        while v_index.is_not_nil() {
            let e = self.edge(v_index.index);
            result.push(Segment::new(e));
            v_index = self.next(v_index.index);
        }

        result
    }
}

trait FindIndex {
    fn find_index(&self, target: i64) -> usize;
}

impl FindIndex for [i64] {
    fn find_index(&self, target: i64) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if self[mid] == target {
                return mid;
            } else if self[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}