use i_float::point::IntPoint;
use crate::bind::segment::IdSegment;
use crate::id_point::IdPoint;
use crate::bind::scan_list::ScanHoleList;
use crate::bind::scan_tree::ScanHoleTree;

pub struct BindSolution {
    pub parent_for_child: Vec<usize>,
    pub children_count_for_parent: Vec<usize>,
}

pub struct ShapeBinder;

pub(crate) trait ScanHoleStore {
    fn insert(&mut self, segment: IdSegment, stop: i32);

    fn find_under_and_nearest(&mut self, p: IntPoint) -> usize;
}

impl ShapeBinder {
    #[inline]
    pub fn bind(shape_count: usize, i_points: Vec<IdPoint>, segments: Vec<IdSegment>) -> BindSolution {
        if i_points.len() < 128 {
            let scan_list = ScanHoleList::new(segments.len());
            Self::private_solve(scan_list, shape_count, i_points, segments)
        } else {
            let scan_tree = ScanHoleTree::new(segments.len());
            Self::private_solve(scan_tree, shape_count, i_points, segments)
        }
    }

    fn private_solve<S: ScanHoleStore>(scan_store: S, shape_count: usize, i_points: Vec<IdPoint>, segments: Vec<IdSegment>) -> BindSolution {
        let children_count = i_points.len();
        let mut scan_store = scan_store;

        let mut parent_for_child = vec![0; children_count];
        let mut children_count_for_parent = vec![0; shape_count];

        let mut i = 0;
        let mut j = 0;

        while i < i_points.len() {
            let x = i_points[i].point.x;

            while j < segments.len() && segments[j].x_segment.a.x <= x {
                let id_segment = &segments[j];
                if id_segment.x_segment.b.x > x {
                    scan_store.insert(*id_segment, x);
                }
                j += 1
            }

            while i < i_points.len() && i_points[i].point.x == x {
                let p = i_points[i].point;

                let parent_index = scan_store.find_under_and_nearest(p);
                let child_index = i_points[i].id;

                parent_for_child[child_index] = parent_index;
                children_count_for_parent[parent_index] += 1;

                i += 1;
            }
        }

        BindSolution { parent_for_child, children_count_for_parent }
    }
}