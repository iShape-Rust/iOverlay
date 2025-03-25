use std::cmp::Ordering;
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
                if id_segment.cmp_by_a_then_by_angle(anchor) == Ordering::Greater {
                    break;
                }


                if id_segment.x_segment.b.x > p.x {
                    scan_store.insert(*id_segment, p.x);
                }
                j += 1
            }

            // debug_assert!(!scan_store.is_empty(), "scan_store can not be empty!");

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

        hole_segments.sort_by_a_then_by_angle(solver);

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

        let mut anchors = anchors;
        anchors.add_sort_by_angle();
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

        segments.sort_by_a_then_by_angle(solver);

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

impl IdSegment {
    #[inline]
    fn cmp_by_a_then_by_angle(&self, other: &Self) -> Ordering {
        self.x_segment
            .a
            .cmp(&other.x_segment.a)
            .then_with(|| self.x_segment.cmp_by_angle(&other.x_segment))
    }
}

pub(crate) trait SortByAngle {
    fn sort_by_a_then_by_angle(&mut self, solver: &Solver);
    fn add_sort_by_angle(&mut self);
}

impl SortByAngle for [IdSegment] {

    #[inline]
    fn sort_by_a_then_by_angle(&mut self, solver: &Solver) {
        self.smart_bin_sort_by(solver, |s0, s1| s0.cmp_by_a_then_by_angle(s1));
    }

    fn add_sort_by_angle(&mut self) {
        // there is a very small chance that sort is required that's why we don't use regular sort

        let mut start = 0;
        while start < self.len() {
            let a = self[start].x_segment.a;
            let mut end = start + 1;

            while end < self.len() && self[end].x_segment.a == a {
                end += 1;
            }

            if end > start + 1 {
                self[start..end].sort_by(|s0, s1| s0.x_segment.cmp_by_angle(&s1.x_segment));
            }

            start = end;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bind::solver::JoinHoles;
    use crate::core::solver::Solver;
    use crate::geom::x_segment::XSegment;
    use i_float::int::point::IntPoint;
    use std::cmp::Ordering;

    #[test]
    fn test_0() {
        let mut shapes = vec![
            vec![vec![
                IntPoint::new(-3, 2),
                IntPoint::new(-3, 4),
                IntPoint::new(-1, 4),
                IntPoint::new(-1, 2),
            ]],
            vec![vec![
                IntPoint::new(3, 0),
                IntPoint::new(2, 3),
                IntPoint::new(3, 6),
                IntPoint::new(6, 6),
                IntPoint::new(6, 0),
            ]],
            vec![vec![
                IntPoint::new(0, -2),
                IntPoint::new(0, -1),
                IntPoint::new(10, -1),
                IntPoint::new(10, -2),
            ]],
        ];

        let holes = vec![
            vec![
                IntPoint::new(4, 3),
                IntPoint::new(4, 4),
                IntPoint::new(2, 3),
            ],
            vec![
                IntPoint::new(3, 1),
                IntPoint::new(4, 2),
                IntPoint::new(2, 3),
            ],
        ];

        shapes.join_unsorted_holes(&Solver::default(), holes);

        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[1].len(), 3);
    }

    #[test]
    fn test_sort() {
        let s0 = XSegment {
            a: IntPoint::new(0, -2),
            b: IntPoint::new(10, -2),
        };
        let s1 = XSegment {
            a: IntPoint::new(2, 3),
            b: IntPoint::new(3, 0),
        };
        let by_a = s0.a.cmp(&s1.a);
        let long_result = match by_a {
            Ordering::Equal => s0.cmp_by_angle(&s1),
            _ => by_a,
        };

        let short_result = s0.a.cmp(&s1.b).then_with(|| s0.cmp_by_angle(&s1));

        assert_eq!(short_result, long_result);
        assert_eq!(Ordering::Less, long_result);
    }
}
