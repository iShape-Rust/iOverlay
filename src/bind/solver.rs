use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShape;
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

    fn private_solve<S: ScanHoleStore>(scan_store: S, shape_count: usize, hole_segments: Vec<IdSegment>, segments: Vec<IdSegment>) -> BindSolution {
        let children_count = hole_segments.len();
        let mut scan_store = scan_store;

        let mut parent_for_child = vec![0; children_count];
        let mut children_count_for_parent = vec![0; shape_count];

        let mut i = 0;
        let mut j = 0;

        while i < hole_segments.len() {
            let x = hole_segments[i].x_segment.a.x;

            while j < segments.len() {
                let id_segment = &segments[j];
                if id_segment.x_segment.a.x > x {
                    break;
                }

                if id_segment.x_segment.b.x > x {
                    scan_store.insert(*id_segment, x);
                }
                j += 1
            }

            while i < hole_segments.len() && hole_segments[i].x_segment.a.x == x {
                let parent_index = scan_store.find_under_and_nearest(hole_segments[i].x_segment);
                let child_index = hole_segments[i].id;

                parent_for_child[child_index] = parent_index;
                children_count_for_parent[parent_index] += 1;

                i += 1;
            }
        }

        BindSolution { parent_for_child, children_count_for_parent }
    }
}

pub(crate) trait JoinHoles {
    fn join_unsorted_holes(&mut self, solver: &Solver, holes: Vec<IntPath>);
    fn join_sorted_holes(&mut self, solver: &Solver, holes: Vec<IntPath>);
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
            .map(|(id, path)| IdSegment { id, x_segment: most_left_bottom(path) })
            .collect();

        hole_segments.sort_by(|a, b| a.x_segment.a.cmp(&b.x_segment.a));

        self.scan_join(solver, holes, hole_segments);
    }

    #[inline]
    fn join_sorted_holes(&mut self, solver: &Solver, holes: Vec<IntPath>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].reserve(holes.len());
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
            return;
        }

        let hole_segments: Vec<_> = holes.iter().enumerate()
            .map(|(id, path)| {
                let a = path[1];
                let b = path[2];
                debug_assert!(a < b);

                let x_segment = XSegment { a, b };

                debug_assert_eq!(x_segment, most_left_bottom(path));
                IdSegment { id, x_segment }
            })
            .collect();

        debug_assert!(is_sorted(&hole_segments));

        self.scan_join(solver, holes, hole_segments);
    }

    fn scan_join(&mut self, solver: &Solver, holes: Vec<IntPath>, hole_segments: Vec<IdSegment>) {
        let x_min = hole_segments[0].x_segment.a.x;
        let x_max = hole_segments[hole_segments.len() - 1].x_segment.a.x;

        let capacity = self.iter().fold(0, |s, it| s + it[0].len()) / 2;
        let mut segments = Vec::with_capacity(capacity);
        for (i, shape) in self.iter().enumerate() {
            shape[0].append_id_segments(&mut segments, i, x_min, x_max);
        }

        segments.smart_bin_sort_by(solver, |a, b| a.x_segment.a.x.cmp(&b.x_segment.a.x));

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

#[inline]
fn most_left_bottom(path: &IntPath) -> XSegment {
    let mut index = 0;
    let mut a = path[0];
    for (i, &p) in path.iter().enumerate().skip(1) {
        if p < a {
            a = p;
            index = i;
        }
    }
    let n = path.len();
    let b0 = path[(index + 1) % n];
    let b1 = path[(index + n - 1) % n];

    let s0 = XSegment { a, b: b0 };
    let s1 = XSegment { a, b: b1 };

    if s0.is_under_segment(&s1) { s0 } else { s1 }
}

#[inline]
fn is_sorted(segments: &[IdSegment]) -> bool {
    segments.windows(2).all(|slice| slice[0].x_segment.a <= slice[1].x_segment.a)
}