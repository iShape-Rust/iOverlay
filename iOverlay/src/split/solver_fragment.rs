use crate::core::edge_data::OverlayEdgeData;
use crate::core::integer::OverlayInt;
use crate::core::solver::Solver;
use crate::segm::segment::Segment;
use crate::segm::winding::WindingCount;
use crate::split::fragment::Fragment;
use crate::split::grid_layout::{BorderVSegment, FragmentBuffer, GridLayout};
use crate::split::line_mark::LineMark;
use crate::split::snap_radius::SnapRadius;
use crate::split::solver::SplitSolver;
use alloc::vec::Vec;

impl<I> SplitSolver<I>
where
    I: OverlayInt,
{
    pub(super) fn fragment_split<C: WindingCount, D: OverlayEdgeData<C>>(
        &mut self,
        snap_radius: SnapRadius,
        segments: &mut Vec<Segment<C, I, D>>,
        solver: &Solver,
        store: &mut D::Store,
    ) -> bool {
        let layout =
            if let Some(layout) = GridLayout::new(segments.iter().map(|it| it.x_segment), segments.len()) {
                layout
            } else {
                return self.tree_split(snap_radius, segments, solver, store);
            };

        let mut reusable_buffer = Vec::new();
        let mut buffer = FragmentBuffer::new(layout);

        let mut need_to_fix = true;
        let mut any_intersection = false;

        let mut snap_radius = snap_radius;

        while need_to_fix && segments.len() > 2 {
            self.marks.clear();

            buffer.init_fragment_buffer(segments.iter().map(|it| it.x_segment));
            for (i, segment) in segments.iter().enumerate() {
                buffer.add_segment(i, segment.x_segment);
            }

            need_to_fix = self.process(snap_radius.radius_squared::<I>(), &mut buffer, solver);

            #[cfg(debug_assertions)]
            debug_assert!(buffer.is_on_border_sorted());
            let mut j = 0;
            while j < buffer.on_border.len() {
                let j0 = j;
                let x = buffer.on_border[j].x;
                j += 1;
                while j < buffer.on_border.len() && x == buffer.on_border[j].x {
                    j += 1;
                }

                // Segments ending at the border are stored in the left group.
                // on_border contains only borders with a positive group index.
                let index = buffer.layout.index(x) - 1;
                if let Some(fragments) = buffer.groups.get(index) {
                    self.on_border_split(x, fragments, &mut buffer.on_border[j0..j]);
                }
            }

            if self.marks.is_empty() {
                return any_intersection;
            }

            any_intersection = true;
            buffer.clear();

            self.apply(segments, &mut reusable_buffer, solver, store);

            snap_radius.increment();
        }

        any_intersection
    }

    #[inline]
    fn process(&mut self, radius_squared: I::Wide, buffer: &mut FragmentBuffer<I>, _solver: &Solver) -> bool {
        #[cfg(feature = "allow_multithreading")]
        {
            if _solver.multithreading.is_some() {
                return self.parallel_split(radius_squared, buffer);
            }
        }

        self.serial_split(radius_squared, buffer)
    }

    #[inline]
    fn serial_split(&mut self, radius_squared: I::Wide, buffer: &mut FragmentBuffer<I>) -> bool {
        let mut is_any_round = false;
        for group in buffer.groups.iter_mut() {
            if group.is_empty() {
                continue;
            }
            let any_round = Self::bin_split(radius_squared, group, &mut self.marks);
            is_any_round = is_any_round || any_round;
        }
        is_any_round
    }

    #[cfg(feature = "allow_multithreading")]
    fn parallel_split(&mut self, radius_squared: I::Wide, buffer: &mut FragmentBuffer<I>) -> bool {
        use rayon::iter::IntoParallelRefMutIterator;
        use rayon::iter::ParallelIterator;

        struct TaskResult<I: OverlayInt> {
            any_round: bool,
            marks: Vec<LineMark<I>>,
        }

        debug_assert!(!buffer.groups.is_empty(), "groups.len() >= 1");
        let marks_capacity = self.marks.capacity() / buffer.groups.len();

        let results: Vec<TaskResult<I>> = buffer
            .groups
            .par_iter_mut()
            .map(|group| {
                let mut marks = Vec::with_capacity(marks_capacity);
                let any_round = Self::bin_split(radius_squared, group, &mut marks);
                TaskResult { any_round, marks }
            })
            .collect();

        let mut is_any_round = false;
        let mut size = 0;
        for result in results.iter() {
            is_any_round = is_any_round || result.any_round;
            size += result.marks.len();
        }

        if size == 0 {
            return false;
        }

        if self.marks.capacity() < size {
            let additional = size - self.marks.capacity();
            self.marks.reserve(additional);
        }

        for mut result in results.into_iter() {
            self.marks.append(&mut result.marks);
        }

        is_any_round
    }

    fn bin_split(
        radius_squared: I::Wide,
        fragments: &mut [Fragment<I>],
        marks: &mut Vec<LineMark<I>>,
    ) -> bool {
        if fragments.len() < 2 {
            return false;
        }

        fragments.sort_unstable_by_key(|a| a.rect.min_y);

        let mut any_round = false;

        for (i, fi) in fragments.iter().enumerate().take(fragments.len() - 1) {
            for fj in fragments.iter().skip(i + 1) {
                if fi.rect.max_y < fj.rect.min_y {
                    break;
                }
                if !fi.rect.is_intersect_border_include(&fj.rect) {
                    continue;
                }

                // MARK: the intersection, ensuring the right order for deterministic results

                let is_round = if fi.x_segment < fj.x_segment {
                    Self::cross_fragments(fi, fj, radius_squared, marks)
                } else {
                    Self::cross_fragments(fj, fi, radius_squared, marks)
                };

                any_round = any_round || is_round
            }
        }

        any_round
    }

    fn on_border_split(
        &mut self,
        border_x: I,
        fragments: &[Fragment<I>],
        vertical_segments: &mut [BorderVSegment<I>],
    ) {
        let mut points = Vec::new();
        for fragment in fragments.iter() {
            if fragment.x_segment.b.x == border_x {
                points.push(fragment.x_segment.b)
            }
        }

        if points.is_empty() {
            return;
        }

        points.sort_unstable_by_key(|p0| p0.y);
        vertical_segments.sort_by_key(|s0| s0.y_range.min);

        let mut i = 0;
        for s in vertical_segments.iter() {
            while i < points.len() && points[i].y <= s.y_range.min {
                i += 1;
            }
            let mut j = i;
            while j < points.len() && points[j].y < s.y_range.max {
                self.marks.push(LineMark {
                    index: s.id,
                    point: points[j],
                });
                j += 1;
            }
        }
    }

    fn cross_fragments(
        fi: &Fragment<I>,
        fj: &Fragment<I>,
        radius_squared: I::Wide,
        marks: &mut Vec<LineMark<I>>,
    ) -> bool {
        // Fragments select candidate pairs; marks belong to the complete segments.
        // Repeated marks from different columns are deduplicated in apply().
        Self::cross(
            fi.index,
            fj.index,
            &fi.x_segment,
            &fj.x_segment,
            marks,
            radius_squared,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::solver::Solver;
    use crate::geom::x_segment::XSegment;
    use crate::segm::boolean::ShapeCountBoolean;
    use crate::segm::segment::Segment;
    use crate::split::grid_layout::GridLayout;
    use crate::split::solver::SplitSolver;
    use alloc::vec::Vec;
    use i_float::int::point::IntPoint;

    #[test]
    fn collinear_overlap_starts_in_a_later_column() {
        for (start, end, far_end) in [(80, 100, 150), (180, 300, 400)] {
            for slope in [-1, 0, 1] {
                let segment = |a, b, count| Segment {
                    x_segment: XSegment { a, b },
                    count: ShapeCountBoolean { subj: count, clip: 0 },
                    data: (),
                };
                let point = |x| IntPoint::new(x, slope * x);
                let padding = [
                    segment(IntPoint::new(0, 1000), IntPoint::new(far_end, 1000), 1),
                    segment(IntPoint::new(0, 2000), IntPoint::new(far_end, 2000), 1),
                ];
                let mut input = alloc::vec![
                    segment(point(0), point(end), 1),
                    segment(point(start), point(far_end), 1)
                ];
                input.extend(padding.iter().copied());
                input.sort_unstable();
                let layout = GridLayout::new(input.iter().map(|s| s.x_segment), input.len()).unwrap();
                assert!(layout.index(start) > layout.index(0));
                let mut expected = alloc::vec![
                    segment(point(0), point(start), 1),
                    segment(point(start), point(end), 2),
                    segment(point(end), point(far_end), 1),
                ];
                expected.extend(padding);
                expected.sort_unstable();
                let expected: Vec<_> = expected.iter().map(|s| (s.x_segment, s.count)).collect();
                for solver in [Solver::LIST, Solver::TREE, Solver::FRAG] {
                    let mut actual = input.clone();
                    SplitSolver::new().split_segments(&mut actual, &solver);
                    let actual: Vec<_> = actual.iter().map(|s| (s.x_segment, s.count)).collect();
                    assert_eq!(
                        actual, expected,
                        "{:?}, slope={slope}, start={start}",
                        solver.strategy
                    );
                }
            }
        }
    }

    #[test]
    fn fragments_can_report_a_crossing_outside_their_column() {
        use crate::split::grid_layout::FragmentBuffer;
        let edges = [
            XSegment {
                a: IntPoint::new(0, 0),
                b: IntPoint::new(100, 1),
            },
            XSegment {
                a: IntPoint::new(0, 1),
                b: IntPoint::new(100, 0),
            },
        ];
        let layout = GridLayout::new(edges.iter().copied(), 4).unwrap();
        assert_eq!(layout.pos(1), 32);
        let mut buffer = FragmentBuffer::new(layout);
        for (index, edge) in edges.into_iter().enumerate() {
            buffer.add_segment(index, edge);
        }
        let mut marks = Vec::new();
        // The first column's enclosures overlap, but the rounded crossing at
        // (50, 1) is in a later column. Marks belong to the complete segments.
        assert!(SplitSolver::<i32>::bin_split(
            1,
            &mut buffer.groups[0],
            &mut marks
        ));
        assert_eq!(marks.len(), 2);
        assert_eq!(marks[0].index, 0);
        assert_eq!(marks[1].index, 1);
        for mark in marks {
            assert_eq!(mark.point, IntPoint::new(50, 1));
        }
    }

    // Exercise the complete splitter so the tests include selection of the border's
    // neighboring group, not just on_border_split with an already selected group.
    fn assert_border_split(edges: &[[i32; 4]], expected_verticals: &[[i32; 4]]) {
        // Keep the x range at [0, 8]. With 4..15 edges the column width is 4.
        // These isolated edges do not intersect the geometry under test.
        let padding = [[0, 100, 8, 100], [0, 102, 8, 102]];
        for dx in [-11, 0, 7] {
            let segment = |&[ax, ay, bx, by]: &[i32; 4]| Segment {
                x_segment: XSegment {
                    a: IntPoint::new(ax + dx, ay),
                    b: IntPoint::new(bx + dx, by),
                },
                count: ShapeCountBoolean::SUBJ_DIRECT,
                data: (),
            };
            let mut input: Vec<_> = edges.iter().chain(&padding).map(segment).collect();
            input.sort_unstable();
            let layout = GridLayout::new(input.iter().map(|s| s.x_segment), input.len()).unwrap();
            assert_eq!(layout.pos(1), dx + 4);
            assert_eq!(layout.pos(2), dx + 8);

            let mut expected: Vec<_> = edges
                .iter()
                .filter(|e| e[0] != e[2])
                .chain(expected_verticals)
                .chain(&padding)
                .map(segment)
                .collect();
            expected.sort_unstable();
            let expected: Vec<_> = expected.iter().map(|s| (s.x_segment, s.count)).collect();

            for solver in [Solver::LIST, Solver::TREE, Solver::FRAG] {
                for multithreading in [None, solver.multithreading] {
                    let solver = Solver {
                        multithreading,
                        ..solver
                    };
                    let mut actual = input.clone();
                    SplitSolver::new().split_segments(&mut actual, &solver);
                    let actual: Vec<_> = actual.iter().map(|s| (s.x_segment, s.count)).collect();
                    assert_eq!(
                        actual,
                        expected,
                        "strategy={:?}, dx={dx}, multithreading={}",
                        solver.strategy,
                        multithreading.is_some(),
                    );
                }
            }
        }
    }

    #[test]
    fn border_horizontal_endpoint_from_left() {
        // Cover both the first internal border and the rightmost border, where
        // the right group contains only the vertical segment.
        for x in [4, 8] {
            assert_border_split(&[[x, 0, x, 6], [0, 3, x, 3]], &[[x, 0, x, 3], [x, 3, x, 6]]);
        }
    }

    #[test]
    fn border_sloped_endpoints_at_same_point() {
        // Rising and falling edges create duplicate marks at (4, 3).
        assert_border_split(
            &[[4, 0, 4, 6], [0, 0, 4, 3], [0, 6, 4, 3]],
            &[[4, 0, 4, 3], [4, 3, 4, 6]],
        );
    }

    #[test]
    fn border_multiple_points_and_verticals() {
        assert_border_split(
            &[
                [4, 0, 4, 3],
                [4, 5, 4, 8],
                [0, 1, 4, 1],
                [0, 2, 4, 2],
                [0, 6, 4, 6],
                [0, 7, 4, 7],
            ],
            &[
                [4, 0, 4, 1],
                [4, 1, 4, 2],
                [4, 2, 4, 3],
                [4, 5, 4, 6],
                [4, 6, 4, 7],
                [4, 7, 4, 8],
            ],
        );
    }

    #[test]
    fn border_multiple_columns() {
        assert_border_split(
            &[[4, 0, 4, 6], [8, 0, 8, 6], [0, 1, 4, 1], [6, 3, 8, 3]],
            &[[4, 0, 4, 1], [4, 1, 4, 6], [8, 0, 8, 3], [8, 3, 8, 6]],
        );
    }

    #[test]
    fn border_shared_ends_and_outside_points_do_not_split() {
        assert_border_split(
            &[
                [4, 0, 4, 6],
                [0, -1, 4, -1],
                [0, 0, 4, 0],
                [0, 6, 4, 6],
                [0, 7, 4, 7],
            ],
            &[[4, 0, 4, 6]],
        );
    }

    #[test]
    fn border_endpoint_from_right() {
        // This contact is handled inside the right group by bin_split.
        assert_border_split(&[[4, 0, 4, 6], [4, 3, 8, 3]], &[[4, 0, 4, 3], [4, 3, 4, 6]]);
    }

    #[test]
    fn border_first_column_has_no_left_neighbor() {
        assert_border_split(&[[0, 0, 0, 6], [0, 3, 8, 3]], &[[0, 0, 0, 3], [0, 3, 0, 6]]);
    }
}
