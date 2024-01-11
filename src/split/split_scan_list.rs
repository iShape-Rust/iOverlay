use crate::space::dual_index::DualIndex;
use crate::space::line_indexer::LineIndexer;
use crate::space::line_range::LineRange;
use crate::space::line_space::IntExtensions;
use crate::split::shape_edge::ShapeEdge;
use crate::split::version_index::VersionedIndex;

#[derive(Clone)]
pub(super) struct SplitSegment {
    pub(super) id: VersionedIndex,
    pub(super) range: LineRange,
    pub(super) stop: i64,
}

#[derive(Clone)]
pub(super) struct SplitCandidate {
    pub(super) id: VersionedIndex,
    pub(super) index: DualIndex,
}

pub(super) struct SplitList {
    indexer: LineIndexer,
    heaps: Vec<Vec<SplitSegment>>,
    index_buffer: Vec<usize>
}

impl SplitList {
    pub(super) fn new(edges: &Vec<ShapeEdge>) -> Self {
        let mut y_min = i64::MAX;
        let mut y_max = i64::MIN;
        for edge in edges.iter() {
            if edge.a.y > edge.b.y {
                y_min = y_min.min(edge.b.y);
                y_max = y_max.max(edge.a.y);
            } else {
                y_min = y_min.min(edge.a.y);
                y_max = y_max.max(edge.b.y);
            }
        }

        let max_level = ((edges.len() as f64).sqrt() as usize).log_two();
        let range = LineRange { min: y_min as i32, max: y_max as i32 };

        let indexer = LineIndexer::new(max_level, range);
        let heaps: Vec<Vec<SplitSegment>> = vec![Vec::new(); indexer.size];

        Self { indexer, heaps, index_buffer: Vec::new() }
    }

    pub(super) fn insert(&mut self, handler: SplitSegment) {
        let index = self.indexer.unsafe_index(handler.range);
        self.heaps[index].push(handler);
    }

    pub fn clear(&mut self) {
        for i in 0..self.heaps.len() {
            self.heaps[i].clear();
        }
    }

    pub(super) fn candidates_in_range(&mut self, range: LineRange, stop: i64, candidates: &mut Vec<SplitCandidate>) {
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
                            candidates.push(SplitCandidate { id: seg.id, index: DualIndex { major, minor } });
                        }
                        minor += 1;
                    }
                }
            }
        }
        self.index_buffer.clear();
    }

    pub(super) fn remove_indices(&mut self, indices: &mut Vec<DualIndex>) {
        let n = indices.len();
        if n == 0 {
            return;
        }

        if n > 1 {
            indices.sort_by(|a, b| a.order_asc_major_des_minor(b));
            for index in indices.into_iter() {
                self.remove_index(index.major, index.minor);
            }
        } else {
            let index = indices[0];
            self.remove_index(index.major, index.minor);
        }
        indices.clear();
    }

    fn remove_index(&mut self, major: usize, minor: usize) {
        if minor + 1 < self.heaps[major].len() {
            self.heaps[major].swap_remove(minor);
        } else {
            _ = self.heaps[major].pop()
        }
    }
}