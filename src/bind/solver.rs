use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntContour, IntShape};
use crate::bind::segment::{IdSegment, IdSegments};
use crate::bind::scan_list::ScanHoleList;
use crate::bind::scan_tree::ScanHoleTree;
use crate::core::solver::Solver;
use crate::geom::x_segment::XSegment;
use crate::util::sort::SmartBinSort;

pub(crate) struct BindSolution {
    pub(crate) parent_for_child: Vec<usize>,
    pub(crate) children_count_for_parent: Vec<usize>,
}

pub(crate) struct ShapeBinder;

pub(crate) trait ScanHoleStore {
    fn insert(&mut self, segment: IdSegment, stop: i32);
    fn find_under_and_nearest(&mut self, segment: XSegment) -> usize;
    fn is_emmpty(&self) -> bool;
}

impl ShapeBinder {
    #[inline]
    pub(crate) fn bind(shape_count: usize, hole_segments: Vec<IdSegment>, segments: Vec<IdSegment>) -> BindSolution {
        if shape_count < 128 {
            let scan_list = ScanHoleList::new(segments.len());
            Self::private_solve(scan_list, shape_count, hole_segments, segments)
        } else {
            let scan_tree = ScanHoleTree::new(segments.len());
            Self::private_solve(scan_tree, shape_count, hole_segments, segments)
        }
    }

    fn private_solve<S: ScanHoleStore>(scan_store: S, shape_count: usize, anchors: Vec<IdSegment>, segments: Vec<IdSegment>) -> BindSolution {
        let children_count = anchors.len();
        let mut scan_store = scan_store;

        let mut parent_for_child = vec![0; children_count];
        let mut children_count_for_parent = vec![0; shape_count];


        let mut j = 0;

        for anchor in anchors.iter() {
            let p = anchor.x_segment.a;

            while j < segments.len() {
                let id_segment = &segments[j];
                if id_segment.x_segment.a >= p {
                    break;
                }

                if id_segment.x_segment.b.x > p.x {
                    scan_store.insert(*id_segment, p.x);
                }
                j += 1
            }

            // debug_assert!(!scan_store.is_emmpty(), "scan_store can not be empty!");

            let target_id = scan_store.find_under_and_nearest(anchor.x_segment);
            let is_shape = target_id & 1 == 0;
            let index = target_id >> 1;
            let parent_index = if is_shape {
                index
            } else {
                // index is a hole index
                // at this moment this hole parent is known
                parent_for_child[index]
            };

            let child_index = anchor.id;

            parent_for_child[child_index] = parent_index;
            children_count_for_parent[parent_index] += 1;
        }

        BindSolution { parent_for_child, children_count_for_parent }
    }
}

pub(crate) trait JoinHoles {
    fn join_unsorted_holes(&mut self, solver: &Solver, holes: Vec<IntContour>);
    fn join_sorted_holes(&mut self, solver: &Solver, holes: Vec<IntContour>, anchors: Vec<IdSegment>);
    fn scan_join(&mut self, solver: &Solver, holes: Vec<IntPath>, hole_segments: Vec<IdSegment>);
}

impl JoinHoles for Vec<IntShape> {
    #[inline]
    fn join_unsorted_holes(&mut self, solver: &Solver, holes: Vec<IntPath>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].reserve(holes.len());
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
            return;
        }

        let mut hole_segments: Vec<_> = holes.iter().enumerate()
            .map(|(id, path)| IdSegment { id, x_segment: path.left_bottom_segment() })
            .collect();

        hole_segments.sort_by(|a, b| a.x_segment.a.cmp(&b.x_segment.a));

        self.scan_join(solver, holes, hole_segments);
    }

    #[inline]
    fn join_sorted_holes(&mut self, solver: &Solver, holes: Vec<IntContour>, anchors: Vec<IdSegment>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
            return;
        }
        debug_assert!(is_sorted(&anchors));

        self.scan_join(solver, holes, anchors);
    }

    fn scan_join(&mut self, solver: &Solver, holes: Vec<IntPath>, hole_segments: Vec<IdSegment>) {
        let x_min = hole_segments[0].x_segment.a.x;
        let x_max = hole_segments[hole_segments.len() - 1].x_segment.a.x;

        let capacity = self.iter().fold(0, |s, it| s + it[0].len()) / 2;
        let mut segments = Vec::with_capacity(capacity);
        for (i, shape) in self.iter().enumerate() {
            shape[0].append_hull_segments(&mut segments, i, x_min, x_max);
        }

        for (i, hole) in holes.iter().enumerate() {
            hole.append_hole_segments(&mut segments, i, x_min, x_max);
        }

        segments.smart_bin_sort_by(solver, |a, b| a.x_segment.a.cmp(&b.x_segment.a));

        let solution = ShapeBinder::bind(self.len(), hole_segments, segments);

        for (shape_index, &capacity) in solution.children_count_for_parent.iter().enumerate() {
            self[shape_index].reserve(capacity);
        }

        for (hole_index, hole) in holes.into_iter().enumerate() {
            let shape_index = solution.parent_for_child[hole_index];
            self[shape_index].push(hole);
        }
    }
}

pub(crate) trait LeftBottomSegment {
    fn left_bottom_segment(&self) -> XSegment;
}

impl LeftBottomSegment for IntContour {
    fn left_bottom_segment(&self) -> XSegment {
        let mut index = 0;
        let mut a = *self.first().unwrap();
        for (i, &p) in self.iter().enumerate().skip(1) {
            if p < a {
                a = p;
                index = i;
            }
        }
        let n = self.len();
        let b0 = self[(index + 1) % n];
        let b1 = self[(index + n - 1) % n];

        let s0 = XSegment { a, b: b0 };
        let s1 = XSegment { a, b: b1 };

        if s0.is_under_segment(&s1) { s0 } else { s1 }
    }
}

#[inline]
fn is_sorted(segments: &[IdSegment]) -> bool {
    segments.windows(2).all(|slice| slice[0].x_segment.a <= slice[1].x_segment.a)
}