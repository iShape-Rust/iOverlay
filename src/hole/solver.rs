use crate::hole::segment::IdSegment;
use crate::hole::id_point::IdPoint;
use crate::hole::scan_list::ScanHoleList;
use crate::hole::scan_store::ScanHoleStore;
use crate::hole::scan_tree::ScanHoleTree;

pub(crate) struct HolesSolution {
    pub(crate) hole_shape: Vec<usize>,
    pub(crate) hole_counter: Vec<usize>,
}

pub(crate) struct HoleSolver;

impl HoleSolver {

    pub(crate) fn solve(shape_count: usize, i_points: Vec<IdPoint>, segments: Vec<IdSegment>) -> HolesSolution {
        if i_points.len() < 128 {
            let scan_list = ScanHoleList::new(segments.len());
            Self::private_solve(scan_list, shape_count, i_points, segments)
        } else {
            let scan_tree = ScanHoleTree::new(segments.len());
            Self::private_solve(scan_tree, shape_count, i_points, segments)
        }
    }


    pub(crate) fn private_solve<S: ScanHoleStore>(scan_store: S, shape_count: usize, i_points: Vec<IdPoint>, segments: Vec<IdSegment>) -> HolesSolution {
        let hole_count = i_points.len();
        let mut scan_store = scan_store;

        let mut hole_shape = vec![0; hole_count];
        let mut hole_counter = vec![0; shape_count];

        let mut i = 0;
        let mut j = 0;

        while i < i_points.len() {
            let x = i_points[i].point.x;

            while j < segments.len() && segments[j].x_segment.a.x <= x {
                let id_segment = &segments[j];
                if id_segment.x_segment.b.x > x {
                    scan_store.insert(id_segment.clone(), x);
                }
                j += 1
            }

            while i < i_points.len() && i_points[i].point.x == x {
                let p = i_points[i].point;

                let shape_index = scan_store.find_under_and_nearest(p, x);
                let hole_index = i_points[i].id;

                hole_shape[hole_index] = shape_index;
                hole_counter[shape_index] += 1;

                i += 1;
            }
        }

        HolesSolution { hole_shape, hole_counter }
    }
}