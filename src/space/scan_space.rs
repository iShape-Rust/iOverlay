use crate::space::dual_index::DualIndex;
use crate::space::line_indexer::LineIndexer;
use crate::space::line_range::LineRange;
use crate::space::line_space::IntExtensions;

#[derive(Clone)]
pub struct ScanSegment<Id: Copy, Unit> {
    pub id: Id,
    pub range: LineRange,
    pub stop: Unit,
}

pub struct ScanItem<Id: Copy> {
    pub id: Id,
    pub index: DualIndex,
}

pub struct ScanSpace<Id: Copy, Unit: Ord + Copy> {
    pub indexer: LineIndexer,
    heaps: Vec<Vec<ScanSegment<Id, Unit>>>,
    index_buffer: Vec<usize>,
}

impl<Id: Copy, Unit: Ord + Copy> ScanSpace<Id, Unit> {
    pub(crate) fn new(range: LineRange, count: usize) -> Self {
        let max_level = 2.max(((count as f64).sqrt() as usize).log_two());
        let indexer = LineIndexer::new(max_level, range);
        let heaps: Vec<Vec<ScanSegment<Id, Unit>>> = vec![Vec::new(); indexer.size];

        Self { indexer, heaps, index_buffer: Vec::new() }
    }

    pub(crate) fn insert(&mut self, segment: ScanSegment<Id, Unit>) {
        let index = self.indexer.unsafe_index(segment.range);
        unsafe {
            self.heaps.get_unchecked_mut(index).push(segment);
        }
    }

    pub fn clear(&mut self) {
        for segments in self.heaps.iter_mut() {
            segments.clear();
        }
    }

    pub fn ids_in_range(&mut self, range: LineRange, stop: Unit, ids: &mut Vec<Id>) {
        self.indexer.fill_unsafe(range, &mut self.index_buffer);

        for &major in self.index_buffer.iter() {
            unsafe {
                let segments = self.heaps.get_unchecked_mut(major);
                let mut minor = 0;
                while minor < segments.len() {
                    let seg = segments.get_unchecked(minor);
                    if seg.stop <= stop {
                        if minor + 1 < segments.len() {
                            segments.swap_remove(minor);
                        } else {
                            _ = segments.pop()
                        }
                    } else {
                        if seg.range.is_overlap(range) {
                            ids.push(seg.id);
                        }
                        minor += 1;
                    }
                }
            }
        }
        self.index_buffer.clear();
    }

    pub fn items_in_range(&mut self, range: LineRange, stop: Unit, items: &mut Vec<ScanItem<Id>>) {
        self.indexer.fill_unsafe(range, &mut self.index_buffer);

        for &major in self.index_buffer.iter() {
            unsafe {
                let segments = self.heaps.get_unchecked_mut(major);
                let mut minor = 0;
                while minor < segments.len() {
                    let seg = segments.get_unchecked(minor);
                    if seg.stop <= stop {
                        if minor + 1 < segments.len() {
                            segments.swap_remove(minor);
                        } else {
                            _ = segments.pop()
                        }
                    } else {
                        if seg.range.is_overlap(range) {
                            items.push(ScanItem { id: seg.id, index: DualIndex { major, minor } });
                        }
                        minor += 1;
                    }
                }
            }
        }
        self.index_buffer.clear();
    }

    pub fn remove_indices(&mut self, indices: &mut Vec<DualIndex>) {
        let n = indices.len();
        if n == 0 {
            return;
        }

        if n > 1 {
            indices.sort_by(|a, b| a.order_asc_major_des_minor(b));
            let mut i = 0;
            while i < indices.len() {
                unsafe {
                    let index = indices.get_unchecked(i);
                    let segments = self.heaps.get_unchecked_mut(index.major);
                    while i < indices.len() && index.major == indices.get_unchecked(i).major {
                        segments.swap_remove(index.minor);
                        i += 1;
                    }
                }
            }
        } else {
            let index = indices[0];
            self.remove_index(index.major, index.minor);
        }
        indices.clear();
    }

    pub fn remove_index(&mut self, major: usize, minor: usize) {
        if minor + 1 < self.heaps[major].len() {
            self.heaps[major].swap_remove(minor);
        } else {
            _ = self.heaps[major].pop()
        }
    }
}