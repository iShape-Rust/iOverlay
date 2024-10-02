use crate::bind::hole_point::HolePoint;
use crate::bind::segment::IdSegment;
use crate::bind::scan_list::ScanHoleList;
use crate::bind::scan_tree::ScanHoleTree;

pub(crate) struct BindSolution {
    pub(crate) parent_for_child: Vec<usize>,
    pub(crate) children_count_for_parent: Vec<usize>,
}

pub(crate) struct ShapeBinder;

pub(crate) trait ScanHoleStore {
    fn insert(&mut self, segment: IdSegment, stop: i32);
    fn find_under_and_nearest<P: HolePoint>(&mut self, path_point: &P) -> usize;
}

impl ShapeBinder {
    #[inline]
    pub(crate) fn bind<P: HolePoint>(shape_count: usize, hole_points: Vec<P>, segments: Vec<IdSegment>) -> BindSolution {
        if shape_count < 128 {
            let scan_list = ScanHoleList::new(segments.len());
            Self::private_solve(scan_list, shape_count, hole_points, segments)
        } else {
            let scan_tree = ScanHoleTree::new(segments.len());
            Self::private_solve(scan_tree, shape_count, hole_points, segments)
        }
    }

    fn private_solve<S: ScanHoleStore, P: HolePoint>(scan_store: S, shape_count: usize, hole_points: Vec<P>, segments: Vec<IdSegment>) -> BindSolution {
        let children_count = hole_points.len();
        let mut scan_store = scan_store;

        let mut parent_for_child = vec![0; children_count];
        let mut children_count_for_parent = vec![0; shape_count];

        let mut i = 0;
        let mut j = 0;

        while i < hole_points.len() {
            let x = hole_points[i].point().x;

            while j < segments.len() && segments[j].x_segment.a.x <= x {
                let id_segment = &segments[j];
                if id_segment.x_segment.b.x > x {
                    scan_store.insert(*id_segment, x);
                }
                j += 1
            }

            while i < hole_points.len() && hole_points[i].point().x == x {
                let parent_index = scan_store.find_under_and_nearest(&hole_points[i]);
                let child_index = hole_points[i].hole_id();

                parent_for_child[child_index] = parent_index;
                children_count_for_parent[parent_index] += 1;

                i += 1;
            }
        }

        BindSolution { parent_for_child, children_count_for_parent }
    }
}