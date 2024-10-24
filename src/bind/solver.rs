use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShape;
use crate::bind::segment::{IdSegment, IdSegments};
use crate::bind::scan_list::ScanHoleList;
use crate::bind::scan_tree::ScanHoleTree;
use crate::core::solver::Solver;
use crate::geom::id_point::IdPoint;
use crate::util::sort::SmartBinSort;

pub(crate) struct BindSolution {
    pub(crate) parent_for_child: Vec<usize>,
    pub(crate) children_count_for_parent: Vec<usize>,
}

pub(crate) struct ShapeBinder;

pub(crate) trait ScanHoleStore {
    fn insert(&mut self, segment: IdSegment, stop: i32);
    fn find_under_and_nearest(&mut self, path_point: IdPoint) -> usize;
}

impl ShapeBinder {
    #[inline]
    pub(crate) fn bind(shape_count: usize, hole_points: Vec<IdPoint>, segments: Vec<IdSegment>) -> BindSolution {
        if shape_count < 128 {
            let scan_list = ScanHoleList::new(segments.len());
            Self::private_solve(scan_list, shape_count, hole_points, segments)
        } else {
            let scan_tree = ScanHoleTree::new(segments.len());
            Self::private_solve(scan_tree, shape_count, hole_points, segments)
        }
    }

    fn private_solve<S: ScanHoleStore>(scan_store: S, shape_count: usize, hole_points: Vec<IdPoint>, segments: Vec<IdSegment>) -> BindSolution {
        let children_count = hole_points.len();
        let mut scan_store = scan_store;

        let mut parent_for_child = vec![0; children_count];
        let mut children_count_for_parent = vec![0; shape_count];

        let mut i = 0;
        let mut j = 0;

        while i < hole_points.len() {
            let x = hole_points[i].point.x;

            while j < segments.len() && segments[j].x_segment.a.x <= x {
                let id_segment = &segments[j];
                if id_segment.x_segment.b.x > x {
                    scan_store.insert(*id_segment, x);
                }
                j += 1
            }

            while i < hole_points.len() && hole_points[i].point.x == x {
                let parent_index = scan_store.find_under_and_nearest(hole_points[i]);
                let child_index = hole_points[i].id;

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
    fn scan_join(&mut self, solver: &Solver, holes: Vec<IntPath>, hole_points: Vec<IdPoint>);
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
        } else {
            let mut hole_points: Vec<_> = holes.iter().enumerate()
                .map(|(i, path)| IdPoint::new(i, *path.first().unwrap()))
                .collect();

            hole_points.smart_bin_sort_by(solver, |a, b| a.point.cmp(&b.point));
            self.scan_join(solver, holes, hole_points);
        }
    }

    fn scan_join(&mut self, solver: &Solver, holes: Vec<IntPath>, hole_points: Vec<IdPoint>) {
        let x_min = hole_points[0].point.x;
        let x_max = hole_points[hole_points.len() - 1].point.x;

        let capacity = self.iter().fold(0, |s, it| s + it[0].len()) / 2;
        let mut segments = Vec::with_capacity(capacity);
        for (i, shape) in self.iter().enumerate() {
            shape[0].append_id_segments(&mut segments, i, x_min, x_max);
        }

        segments.smart_bin_sort_by(solver, |a, b| a.x_segment.a.x.cmp(&b.x_segment.a.x));

        let solution = ShapeBinder::bind(self.len(), hole_points, segments);

        for (shape_index, &capacity) in solution.children_count_for_parent.iter().enumerate() {
            self[shape_index].reserve(capacity);
        }

        for (hole_index, hole) in holes.into_iter().enumerate() {
            let shape_index = solution.parent_for_child[hole_index];
            self[shape_index].push(hole);
        }
    }
}