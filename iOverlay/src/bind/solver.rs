use alloc::vec;
use alloc::vec::Vec;
use crate::bind::segment::{ContourIndex, IdSegment, IdSegments};
use crate::geom::v_segment::VSegment;
use crate::util::log::Int;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntContour, IntShape};
use i_tree::key::exp::KeyExpCollection;
use i_tree::key::list::KeyExpList;
use i_tree::key::tree::KeyExpTree;
use core::cmp::Ordering;
use i_key_sort::sort::key_sort::KeyBinSort;

pub(crate) struct BindSolution {
    pub(crate) parent_for_child: Vec<usize>,
    pub(crate) children_count_for_parent: Vec<usize>,
}

pub(crate) struct ShapeBinder;

impl ShapeBinder {
    #[inline]
    pub(crate) fn bind(
        shape_count: usize,
        hole_segments: Vec<IdSegment>,
        segments: Vec<IdSegment>,
    ) -> BindSolution {
        if shape_count < 32 {
            let capacity = segments.len().log2_sqrt().max(4) * 2;
            let list = KeyExpList::new(capacity);
            Self::private_solve::<KeyExpList<VSegment, i32, ContourIndex>>(
                list,
                shape_count,
                hole_segments,
                segments,
            )
        } else {
            let capacity = segments.len().log2_sqrt().max(8);
            let list = KeyExpTree::new(capacity);
            Self::private_solve::<KeyExpTree<VSegment, i32, ContourIndex>>(
                list,
                shape_count,
                hole_segments,
                segments,
            )
        }
    }

    fn private_solve<S: KeyExpCollection<VSegment, i32, ContourIndex>>(
        mut scan_list: S,
        shape_count: usize,
        anchors: Vec<IdSegment>,
        segments: Vec<IdSegment>,
    ) -> BindSolution {
        let children_count = anchors.len();
        let mut parent_for_child = {
            #[cfg(debug_assertions)]
            {
                // prefer crash in debug mode
                vec![usize::MAX; children_count]
            }
            #[cfg(not(debug_assertions))]
            {
                vec![0; children_count]
            }
        };
        let mut children_count_for_parent = vec![0; shape_count];

        let mut j = 0;

        for anchor in anchors.iter() {
            let p = anchor.v_segment.a;

            while j < segments.len() {
                let id_segment = &segments[j];
                if id_segment.cmp_by_a_then_by_angle(anchor) == Ordering::Greater {
                    break;
                }

                if id_segment.v_segment.b.x > p.x {
                    scan_list.insert(id_segment.v_segment, id_segment.contour_index, p.x);
                }
                j += 1
            }

            let target_id =
                scan_list.first_less(anchor.v_segment.a.x, ContourIndex::EMPTY, anchor.v_segment);
            let parent_index = if target_id.is_hole() {
                // index is a hole index
                // at this moment this hole parent is known
                parent_for_child[target_id.index()]
            } else {
                target_id.index()
            };

            let child_index = anchor.contour_index.index();

            parent_for_child[child_index] = parent_index;
            children_count_for_parent[parent_index] += 1;
        }

        BindSolution {
            parent_for_child,
            children_count_for_parent,
        }
    }
}

pub(crate) trait JoinHoles {
    fn join_unsorted_holes(&mut self, holes: Vec<IntContour>, clockwise: bool);
    fn join_sorted_holes(
        &mut self,
        holes: Vec<IntContour>,
        anchors: Vec<IdSegment>,
        clockwise: bool
    );
    fn scan_join(&mut self, holes: Vec<IntPath>, hole_segments: Vec<IdSegment>, clockwise: bool);
}

impl JoinHoles for Vec<IntShape> {
    #[inline]
    fn join_unsorted_holes(&mut self, holes: Vec<IntPath>, clockwise: bool) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].reserve(holes.len());
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
            return;
        }

        let mut hole_segments: Vec<_> = holes
            .iter()
            .enumerate()
            .map(|(id, path)| IdSegment {
                contour_index: ContourIndex::new_hole(id),
                v_segment: path.left_bottom_segment(),
            })
            .collect();

        hole_segments.sort_by_a_then_by_angle();

        self.scan_join(holes, hole_segments, clockwise);
    }

    #[inline]
    fn join_sorted_holes(
        &mut self,
        holes: Vec<IntContour>,
        anchors: Vec<IdSegment>,
        clockwise: bool
    ) {
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
        self.scan_join(holes, anchors, clockwise);
    }

    fn scan_join(&mut self, holes: Vec<IntPath>, hole_segments: Vec<IdSegment>, clockwise: bool) {
        let x_min = hole_segments[0].v_segment.a.x;
        let x_max = hole_segments[hole_segments.len() - 1].v_segment.a.x;

        let capacity = self.iter().fold(0, |s, it| s + it[0].len()) / 2;
        let mut segments = Vec::with_capacity(capacity);
        for (i, shape) in self.iter().enumerate() {
            shape[0].append_id_segments(&mut segments, ContourIndex::new_shape(i), x_min, x_max, clockwise);
        }

        for (i, hole) in holes.iter().enumerate() {
            hole.append_id_segments(&mut segments, ContourIndex::new_hole(i), x_min, x_max, clockwise);
        }

        segments.sort_by_a_then_by_angle();

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
    fn left_bottom_segment(&self) -> VSegment;
}

impl LeftBottomSegment for IntContour {
    fn left_bottom_segment(&self) -> VSegment {
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

        let s0 = VSegment { a, b: b0 };
        let s1 = VSegment { a, b: b1 };

        if s0.is_under_segment(&s1) { s0 } else { s1 }
    }
}

#[inline]
fn is_sorted(segments: &[IdSegment]) -> bool {
    segments
        .windows(2)
        .all(|slice| slice[0].v_segment.a <= slice[1].v_segment.a)
}

impl IdSegment {
    #[inline]
    fn cmp_by_a_then_by_angle(&self, other: &Self) -> Ordering {
        self.v_segment
            .a
            .cmp(&other.v_segment.a)
            .then_with(|| self.v_segment.cmp_by_angle(&other.v_segment))
    }
}

pub(crate) trait SortByAngle {
    fn sort_by_a_then_by_angle(&mut self);
    fn add_sort_by_angle(&mut self);
}

impl SortByAngle for [IdSegment] {
    #[inline]
    fn sort_by_a_then_by_angle(&mut self) {
        self.sort_with_bins(|s0, s1| s0.cmp_by_a_then_by_angle(s1));
    }

    #[inline]
    fn add_sort_by_angle(&mut self) {
        // there is a very small chance that sort is required that's why we don't use regular sort

        let mut start = 0;
        while start < self.len() {
            let a = self[start].v_segment.a;
            let mut end = start + 1;

            while end < self.len() && self[end].v_segment.a == a {
                end += 1;
            }

            if end > start + 1 {
                self[start..end].sort_by(|s0, s1| s0.v_segment.cmp_by_angle(&s1.v_segment));
            }

            start = end;
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use crate::bind::solver::JoinHoles;
    use crate::geom::v_segment::VSegment;
    use i_float::int::point::IntPoint;
    use core::cmp::Ordering;

    #[test]
    fn test_0() {
        let mut shapes = vec![
            vec![vec![
                IntPoint::new(-1, 2),
                IntPoint::new(-1, 4),
                IntPoint::new(-3, 4),
                IntPoint::new(-3, 2),
            ]],
            vec![vec![
                IntPoint::new(6, 0),
                IntPoint::new(6, 6),
                IntPoint::new(3, 6),
                IntPoint::new(2, 3),
                IntPoint::new(3, 0),
            ]],
            vec![vec![
                IntPoint::new(0, -1),
                IntPoint::new(0, -2),
                IntPoint::new(10, -2),
                IntPoint::new(10, -1),
            ]],
        ];

        let holes = vec![
            vec![
                IntPoint::new(2, 3),
                IntPoint::new(4, 4),
                IntPoint::new(4, 3),
            ],
            vec![
                IntPoint::new(2, 3),
                IntPoint::new(4, 2),
                IntPoint::new(3, 1),
            ],
        ];

        shapes.join_unsorted_holes(holes, false);

        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[1].len(), 3);
    }

    #[test]
    fn test_sort() {
        let s0 = VSegment {
            a: IntPoint::new(0, -2),
            b: IntPoint::new(10, -2),
        };
        let s1 = VSegment {
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
