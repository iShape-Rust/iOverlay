use crate::space::dual_index::DualIndex;
use crate::space::line_container::LineContainer;
use crate::space::line_indexer::LineIndexer;
use crate::space::line_range::LineRange;
use crate::space::line_segment::LineSegment;


pub struct LineSpace<Id: Copy> {
    pub indexer: LineIndexer,
    buffer: Vec<usize>,
    heaps: Vec<Vec<LineSegment<Id>>>,

}

impl<Id: Copy> LineSpace<Id> {
    pub fn new(level: usize, range: LineRange) -> Self {
        let indexer = LineIndexer::new(level, range);
        let heaps: Vec<Vec<LineSegment<Id>>> = vec![Vec::new(); indexer.size];

        Self { indexer, buffer: Vec::new(), heaps }
    }

    pub fn insert(&mut self, segment: LineSegment<Id>) {
        let index = self.indexer.unsafe_index(segment.range);
        self.heaps[index].push(segment);
    }

    pub fn remove_index(&mut self, index: &DualIndex) {
        if index.minor + 1 < self.heaps[index.major].len() {
            self.heaps[index.major].swap_remove(index.minor);
        } else {
            _ = self.heaps[index.major].pop()
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.heaps.len() {
            self.heaps[i].clear();
        }
    }

    pub fn ids_in_range(&mut self, range: LineRange, ids: &mut Vec<Id>) {
        self.indexer.fill(range, &mut self.buffer);

        for &heap_index in self.buffer.iter() {
            let segments = &self.heaps[heap_index];
            for seg in segments.iter() {
                if seg.range.is_overlap(range) {
                    ids.push(seg.id)
                }
            }
        }

        self.buffer.clear();
    }

    pub fn all_in_range(&mut self, range: LineRange, containers: &mut Vec<LineContainer<Id>>) {
        self.indexer.fill(range, &mut self.buffer);

        for &heap_index in self.buffer.iter() {
            let segments = &self.heaps[heap_index];
            for segment_index in 0..segments.len() {
                if range.is_overlap(segments[segment_index].range) {
                    let index = DualIndex { major: heap_index, minor: segment_index };
                    let id = segments[segment_index].id;
                    let container = LineContainer { id, index };
                    containers.push(container);
                }
            }
        }

        self.buffer.clear();
    }

    pub fn remove_indices(&mut self, indices: &mut Vec<DualIndex>) {
        if indices.len() > 1 {
            indices.sort_by(|a, b| a.order_asc_major_des_minor(b));
            for index in indices {
                self.remove_index(index);
            }
        } else {
            self.remove_index(&indices[0]);
        }
    }

    pub fn all_ids_in_range(&mut self, range: LineRange) -> Vec<Id> {
        let mut result = Vec::new();
        self.ids_in_range(range, &mut result);

        result
    }
}


pub trait IntExtensions {
    fn power_of_two(&self) -> Self;
    fn log_two(&self) -> usize;
}

impl IntExtensions for usize {
    fn power_of_two(&self) -> usize {
        1 << self
    }

    fn log_two(&self) -> usize {
        if *self <= 0 {
            return 0;
        }
        let n = self.leading_zeros();

        (usize::BITS - n) as usize
    }
}