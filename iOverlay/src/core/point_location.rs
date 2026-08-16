//! Batched point-in-polygon queries for integer geometry.

use crate::core::integer::OverlayInt;
use crate::core::overlay::ShapeType;
use crate::geom::end::End;
use crate::geom::v_segment::VSegment;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::build::BuildSegments;
use crate::segm::segment::Segment;
use crate::segm::sort::ShapeSegmentsSort;
use crate::segm::winding::WindingCount;
use crate::util::log::Int;
use alloc::vec;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use i_key_sort::sort::two_keys::TwoKeysSort;
use i_shape::int::shape::{IntContour, IntShape};
use i_tree::key::exp::KeyExpCollection;
use i_tree::key::list::KeyExpList;
use i_tree::key::tree::KeyExpTree;

const MAX_LIST_EDGE_COUNT: usize = 8_000;

const EMPTY_COUNT: ShapeCountBoolean = ShapeCountBoolean { subj: 0, clip: 0 };

#[derive(Clone, Copy)]
struct QueryPoint<I: OverlayInt> {
    point: IntPoint<I>,
    index: usize,
}

/// Convenience methods for one-shot batched point-in-polygon queries.
///
/// Every shape must have resolved topology, for example as the result of a
/// `simplify_shape` operation. A collection of shapes is evaluated as their
/// union.
///
/// # Example
///
/// ```
/// use i_overlay::core::point_location::IntPointContainment;
/// use i_overlay::i_float::int::point::IntPoint;
///
/// let contour = [
///     IntPoint::new(0, 0),
///     IntPoint::new(10, 0),
///     IntPoint::new(10, 10),
///     IntPoint::new(0, 10),
/// ];
/// let points = [IntPoint::new(5, 5), IntPoint::new(20, 5)];
///
/// assert_eq!(contour.contains_points(&points), [true, false]);
/// ```
pub trait IntPointContainment<I: OverlayInt> {
    /// Tests whether each point is strictly inside this geometry.
    ///
    /// Points on contour boundaries are outside the method's contract.
    fn contains_points(&self, points: &[IntPoint<I>]) -> Vec<bool>;
}

impl<I: OverlayInt> IntPointContainment<I> for [IntPoint<I>] {
    #[inline]
    fn contains_points(&self, points: &[IntPoint<I>]) -> Vec<bool> {
        contains_points_in_valid_contours(core::iter::once(self), points)
    }
}

impl<I: OverlayInt> IntPointContainment<I> for [IntContour<I>] {
    #[inline]
    fn contains_points(&self, points: &[IntPoint<I>]) -> Vec<bool> {
        contains_points_in_valid_contours(self.iter().map(Vec::as_slice), points)
    }
}

impl<I: OverlayInt> IntPointContainment<I> for [IntShape<I>] {
    #[inline]
    fn contains_points(&self, points: &[IntPoint<I>]) -> Vec<bool> {
        let mut result = vec![false; points.len()];
        for shape in self {
            let shape_result = contains_points_in_valid_contours(shape.iter().map(Vec::as_slice), points);
            for (contains, shape_contains) in result.iter_mut().zip(shape_result) {
                *contains |= shape_contains;
            }
        }
        result
    }
}

fn contains_points_in_valid_contours<'a, I, It>(contours: It, points: &[IntPoint<I>]) -> Vec<bool>
where
    I: OverlayInt + 'a,
    It: IntoIterator<Item = &'a [IntPoint<I>]>,
{
    if points.is_empty() {
        return Vec::new();
    }

    let mut queries: Vec<_> = points
        .iter()
        .copied()
        .enumerate()
        .map(|(index, point)| QueryPoint { point, index })
        .collect();
    queries.sort_by_two_keys(false, |query| query.point.x, |query| query.point.y);

    let mut result = vec![false; points.len()];
    let mut contour_result = vec![false; points.len()];

    for contour in contours {
        let mut segments = Vec::with_capacity(contour.len());
        segments.append_path_iter(contour.iter().copied(), ShapeType::Subject, false);
        if segments.is_empty() {
            continue;
        }
        segments.sort_by_ab(false);
        contour_result.fill(false);

        if segments.len() < MAX_LIST_EDGE_COUNT {
            let capacity = segments.len().log2_sqrt().max(4) * 2;
            let mut list = KeyExpList::new(capacity);
            contains_with_scan(&mut list, &segments, &queries, &mut contour_result);
        } else {
            let capacity = segments.len().log2_sqrt().max(8);
            let mut tree = KeyExpTree::new(capacity);
            contains_with_scan(&mut tree, &segments, &queries, &mut contour_result);
        }

        for (contains, contour_contains) in result.iter_mut().zip(&contour_result) {
            *contains ^= contour_contains;
        }
    }

    result
}

fn contains_with_scan<I, S>(
    scan: &mut S,
    segments: &[Segment<ShapeCountBoolean, I>],
    queries: &[QueryPoint<I>],
    result: &mut [bool],
) where
    I: OverlayInt,
    S: KeyExpCollection<VSegment<I>, I, ShapeCountBoolean>,
{
    let mut node = Vec::with_capacity(4);
    let mut segment_index = 0;

    for query in queries {
        while segment_index < segments.len() && segments[segment_index].x_segment.a.x <= query.point.x {
            let p = segments[segment_index].x_segment.a;
            node.push(End {
                index: segment_index,
                point: segments[segment_index].x_segment.b,
            });
            segment_index += 1;

            while segment_index < segments.len() && segments[segment_index].x_segment.a == p {
                node.push(End {
                    index: segment_index,
                    point: segments[segment_index].x_segment.b,
                });
                segment_index += 1;
            }

            if node.len() > 1 {
                node.sort_by(|a, b| Triangle::clock_order(p, b.point, a.point));
            }

            let mut sum = scan.first_less_or_equal_by(p.x, EMPTY_COUNT, |s| s.is_under_point_order(p));

            for end in &node {
                let segment = &segments[end.index];
                sum = sum.add(segment.count);

                if segment.x_segment.is_not_vertical() {
                    scan.insert(segment.x_segment.into(), sum, p.x);
                }
            }

            node.clear();
        }

        let count = scan.first_less_or_equal_by(query.point.x, EMPTY_COUNT, |segment| {
            Triangle::clock_order(segment.a, query.point, segment.b)
        });

        result[query.index] = is_filled(count.subj);
    }
}

#[inline(always)]
fn is_filled(count: i32) -> bool {
    count & 1 != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay::IntOverlayOptions;
    use crate::core::simplify::Simplify;
    use alloc::vec;
    use i_shape::int::path::ContourExtension;

    fn ccw_square(min: i32, max: i32) -> IntContour<i32> {
        vec![
            IntPoint::new(min, min),
            IntPoint::new(max, min),
            IntPoint::new(max, max),
            IntPoint::new(min, max),
        ]
    }

    #[test]
    fn contains_inside_points_and_preserves_order() {
        let contour = ccw_square(0, 10);
        let points = [
            IntPoint::new(5, 5),
            IntPoint::new(-1, 5),
            IntPoint::new(2, 8),
            IntPoint::new(20, 5),
            IntPoint::new(5, 5),
        ];

        assert_eq!(
            contour.contains_points(&points),
            vec![true, false, true, false, true]
        );
    }

    #[test]
    fn supports_holes_and_preserves_query_order() {
        let mut hole = ccw_square(3, 7);
        hole.reverse();
        let shape = [ccw_square(0, 10), hole];
        let points = [
            IntPoint::new(5, 5),
            IntPoint::new(1, 1),
            IntPoint::new(2, 5),
            IntPoint::new(5, 5),
            IntPoint::new(20, 20),
        ];

        assert_eq!(
            shape.contains_points(&points),
            vec![false, true, true, false, false]
        );
    }

    #[test]
    fn accepts_either_outer_contour_direction() {
        let ccw = ccw_square(0, 10);
        let mut cw = ccw.clone();
        cw.reverse();
        let point = [IntPoint::new(5, 5)];

        assert_eq!(ccw.contains_points(&point), vec![true]);
        assert_eq!(cw.contains_points(&point), vec![true]);
    }

    #[test]
    fn supports_i64_and_multiple_vertical_ranges() {
        let contours = [
            vec![
                IntPoint::<i64>::new(0, 0),
                IntPoint::new(10, 0),
                IntPoint::new(10, 10),
                IntPoint::new(0, 10),
            ],
            vec![
                IntPoint::new(0, 20),
                IntPoint::new(10, 20),
                IntPoint::new(10, 30),
                IntPoint::new(0, 30),
            ],
        ];
        let points = [IntPoint::new(5, 5), IntPoint::new(5, 15), IntPoint::new(5, 25)];

        assert_eq!(contours.contains_points(&points), vec![true, false, true]);
    }

    #[test]
    fn contains_points_in_simplified_shapes() {
        let contours = [ccw_square(0, 10), ccw_square(5, 15)];
        let shapes = contours
            .as_slice()
            .simplify(FillRule::NonZero, IntOverlayOptions::default());
        let points = [
            IntPoint::new(2, 2),
            IntPoint::new(7, 7),
            IntPoint::new(12, 12),
            IntPoint::new(2, 12),
        ];

        assert_eq!(shapes.contains_points(&points), vec![true, true, true, false]);
    }

    #[test]
    fn vertical_edges_update_winding_but_are_not_stored_in_scan() {
        let contour = [
            IntPoint::new(0, 0),
            IntPoint::new(10, 0),
            IntPoint::new(10, 10),
            IntPoint::new(5, 10),
            IntPoint::new(5, 5),
            IntPoint::new(0, 5),
        ];
        let points = [
            IntPoint::new(5, 2),
            IntPoint::new(4, 7),
            IntPoint::new(6, 7),
            IntPoint::new(5, 12),
        ];

        assert_eq!(contour.contains_points(&points), vec![true, false, true, false]);
    }

    #[test]
    fn random_simplified_contours_match_contains_point_in_50_by_50_space() {
        for iteration in 0..256_u64 {
            let seed = next_test_seed(iteration);
            let mut rng = TestRng::new(seed);
            let contour_count = rng.range_usize(1, 4);
            let mut contours = Vec::with_capacity(contour_count);

            for _ in 0..contour_count {
                let point_count = rng.range_usize(3, 20);
                let mut contour = Vec::with_capacity(point_count);
                for _ in 0..point_count {
                    contour.push(IntPoint::new(rng.range_i32(0, 50), rng.range_i32(0, 50)));
                }
                contours.push(contour);
            }

            let shapes = contours
                .as_slice()
                .simplify(FillRule::NonZero, IntOverlayOptions::ogc());
            let mut points = Vec::with_capacity(51 * 51);
            for y in 0..=50 {
                for x in 0..=50 {
                    let point = IntPoint::new(x, y);
                    if !is_on_boundary(&shapes, point) {
                        points.push(point);
                    }
                }
            }
            rng.shuffle(&mut points);

            let actual = shapes.contains_points(&points);
            for (index, &point) in points.iter().enumerate() {
                let expected = shapes.iter().any(|shape| {
                    shape.iter().fold(false, |contains, contour| {
                        contains ^ contour.contains_point(point)
                    })
                });
                assert_eq!(
                    actual[index], expected,
                    "iteration={iteration} seed={seed} point={point} contours={contours:?} shapes={shapes:?}"
                );
            }
        }
    }

    fn is_on_boundary(shapes: &[IntShape<i32>], point: IntPoint<i32>) -> bool {
        shapes.iter().flatten().any(|contour| {
            let Some(&last) = contour.last() else {
                return false;
            };
            let mut a = last;
            contour.iter().any(|&b| {
                let contains = Triangle::is_line(a, point, b)
                    && a.x.min(b.x) <= point.x
                    && point.x <= a.x.max(b.x)
                    && a.y.min(b.y) <= point.y
                    && point.y <= a.y.max(b.y);
                a = b;
                contains
            })
        })
    }

    struct TestRng {
        state: u64,
    }

    impl TestRng {
        fn new(seed: u64) -> Self {
            Self {
                state: seed ^ 0xa076_1d64_78bd_642f,
            }
        }

        fn range_usize(&mut self, min: usize, max: usize) -> usize {
            min + self.next_u32() as usize % (max - min + 1)
        }

        fn range_i32(&mut self, min: i32, max: i32) -> i32 {
            min + (self.next_u32() % (max - min + 1) as u32) as i32
        }

        fn shuffle<T>(&mut self, values: &mut [T]) {
            for index in (1..values.len()).rev() {
                let target = self.range_usize(0, index);
                values.swap(index, target);
            }
        }

        fn next_u32(&mut self) -> u32 {
            self.state = self
                .state
                .wrapping_mul(0xe703_7ed1_a0b4_28db)
                .wrapping_add(0x8ebc_6af0_9c88_c6e3);
            (self.state >> 32) as u32
        }
    }

    fn next_test_seed(seed: u64) -> u64 {
        seed.wrapping_mul(0xe703_7ed1_a0b4_28db)
            .wrapping_add(0x8ebc_6af0_9c88_c6e3)
    }
}
