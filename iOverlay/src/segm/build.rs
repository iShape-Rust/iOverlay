use crate::core::overlay::ShapeType;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use crate::segm::winding::WindingCount;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;

pub(crate) trait BuildSegments {
    fn append_path_iter<I: Iterator<Item = IntPoint>>(
        &mut self,
        iter: I,
        shape_type: ShapeType,
        keep_same_line_points: bool,
    ) -> bool;
}

impl<C: WindingCount> BuildSegments for Vec<Segment<C>> {
    #[inline]
    fn append_path_iter<I: Iterator<Item = IntPoint>>(
        &mut self,
        iter: I,
        shape_type: ShapeType,
        keep_same_line_points: bool,
    ) -> bool {
        if keep_same_line_points {
            build_segments_with_filter::<DropOppositeCollinear, I, C>(self, iter, shape_type)
        } else {
            build_segments_with_filter::<DropCollinear, I, C>(self, iter, shape_type)
        }
    }
}

fn build_segments_with_filter<F: PointFilter, I: Iterator<Item = IntPoint>, C: WindingCount>(
    segments: &mut Vec<Segment<C>>,
    mut iter: I,
    shape_type: ShapeType,
) -> bool {
    // our goal add all not degenerate segments
    let mut p0 = if let Some(p) = iter.next() { p } else { return false; };
    let mut p1 = if let Some(p) = iter.find(|p| p0.ne(p)) { p } else { return true; };

    let mut filtered = false;

    let q0 = p0;

    for p2 in &mut iter {
        if F::include_point(p0, p1, p2) {
            p0 = p1;
            p1 = p2;
            break;
        }
        p1 = p2;
        filtered = true;
    }

    let q1 = p0;

    let (direct, invert) = C::with_shape_type(shape_type);

    // We close the loop with the first two points
    for p2 in &mut iter.chain([q0, q1]) {
        if !F::include_point(p0, p1, p2) {
            p1 = p2;
            filtered = true;
            continue;
        }
        segments.push(Segment::with_ab(p0, p1, direct, invert));

        p0 = p1;
        p1 = p2;
    }

    let add_last = p1 != p0;
    filtered |= !add_last;
    if add_last {
        segments.push(Segment::with_ab(p0, p1, direct, invert));
    }

    filtered
}

trait PointFilter {
    fn include_point(a: IntPoint, b: IntPoint, c: IntPoint) -> bool;
}

struct DropOppositeCollinear;
struct DropCollinear;

impl PointFilter for DropOppositeCollinear {
    #[inline]
    fn include_point(p0: IntPoint, p1: IntPoint, p2: IntPoint) -> bool {
        let a = p1.subtract(p0);
        let b = p1.subtract(p2);

        if a.cross_product(b) != 0 {
            // not collinear
            return true;
        }

        // collinear – keep only if we keep going same direction
        a.dot_product(b) < 0
    }
}

impl PointFilter for DropCollinear {
    #[inline]
    fn include_point(p0: IntPoint, p1: IntPoint, p2: IntPoint) -> bool {
        let a = p1.subtract(p0);
        let b = p1.subtract(p2);
        a.cross_product(b) != 0
    }
}

impl<C: Send> Segment<C> {
    #[inline]
    pub(crate) fn with_ab(p0: IntPoint, p1: IntPoint, direct: C, invert: C) -> Self {
        if p0 < p1 {
            Self {
                x_segment: XSegment { a: p0, b: p1 },
                count: direct,
            }
        } else {
            Self {
                x_segment: XSegment { a: p1, b: p0 },
                count: invert,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::overlay::ShapeType;
    use crate::segm::boolean::ShapeCountBoolean;
    use crate::segm::build::BuildSegments;
    use crate::segm::merge::ShapeSegmentsMerge;
    use crate::segm::segment::Segment;
    use alloc::vec::Vec;
    use i_float::int::point::IntPoint;

    #[test]
    fn test_0() {
        let points = [
            IntPoint::new(2, 0),
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
        ];

        test_count(&points, 0, false);
    }

    #[test]
    fn test_1() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ];

        test_count(&points, 0, false);
    }

    #[test]
    fn test_2() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(0, 2),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ];

        test_count(&points, 4, true);
    }

    #[test]
    fn test_roll_0() {
        let points = [
            IntPoint::new(1, 0),
            IntPoint::new(1, 0),
            IntPoint::new(1, 0),
            IntPoint::new(1, 0),
        ];

        test_roll_count(&points, 0, false);
        test_roll_count(&points, 0, true);
    }

    #[test]
    fn test_roll_1() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
        ];

        test_roll_count(&points, 0, false);
        test_roll_count(&points, 0, true);
    }

    #[test]
    fn test_roll_2() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(0, 0),
        ];

        test_roll_count(&points, 0, false);
        test_roll_count(&points, 0, true);
    }

    #[test]
    fn test_roll_3() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
            IntPoint::new(2, 2),
        ];

        test_roll_count(&points, 3, false);
        test_roll_count(&points, 3, true);
    }

    #[test]
    fn test_roll_4() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(0, 2),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ];

        test_roll_count(&points, 4, false);
        test_roll_count(&points, 4, true);
    }

    #[test]
    fn test_roll_5() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(1, 0),
            IntPoint::new(2, 0),
        ];

        test_roll_count(&points, 0, false);
        test_roll_count(&points, 0, true);
    }

    #[test]
    fn test_roll_6() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(1, 0),
            IntPoint::new(2, 0),
            IntPoint::new(3, 0),
        ];

        test_roll_count(&points, 0, false);
        test_roll_count(&points, 0, true);
    }

    #[test]
    fn test_roll_7() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ];

        test_roll_count(&points, 0, false);
        test_roll_count(&points, 0, true);
    }

    #[test]
    fn test_roll_8() {
        let points = [
            IntPoint::new(0, 3),
            IntPoint::new(-4, -3),
            IntPoint::new(4, -3),
            IntPoint::new(3, -3),
            IntPoint::new(0, 3),
        ];

        test_roll_count(&points, 3, false);
        test_roll_count(&points, 3, true);
    }

    #[test]
    fn test_roll_9() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
            IntPoint::new(4, 2),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ];

        test_roll_count(&points, 0, false);
        test_roll_count(&points, 0, true);
    }

    #[test]
    fn test_roll_10() {
        let points = [
            IntPoint::new(-10, 0),
            IntPoint::new(-10, -10),
            IntPoint::new(0, -10),
            IntPoint::new(10, -10),
            IntPoint::new(10, 0),
            IntPoint::new(10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(-10, 10),
        ];

        test_roll_count(&points, 4, false);
        test_roll_count(&points, 8, true);
    }

    #[test]
    fn test_roll_11() {
        let points = [
            IntPoint::new(-1, 2),
            IntPoint::new(-1, 1),
            IntPoint::new(-2, 1),
            IntPoint::new(-1, 1),
            IntPoint::new(-1, -1),
            IntPoint::new(-1, -2),
            IntPoint::new(-1, -1),
            IntPoint::new(-2, -1),
            IntPoint::new(-1, -1),
            IntPoint::new(1, -1),
            IntPoint::new(2, -1),
            IntPoint::new(1, -1),
            IntPoint::new(1, -2),
            IntPoint::new(1, -1),
            IntPoint::new(1, 1),
            IntPoint::new(1, 2),
            IntPoint::new(1, 1),
            IntPoint::new(2, 1),
            IntPoint::new(1, 1),
            IntPoint::new(-1, 1),
        ];

        test_roll_count(&points, 4, false);
        test_roll_count(&points, 4, true);
    }

    #[test]
    fn test_roll_12() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(0, 2),
            IntPoint::new(1, 2),
            IntPoint::new(2, 2),
            IntPoint::new(3, 2),
            IntPoint::new(4, 2),
            IntPoint::new(5, 0),
        ];

        test_roll_count(&points, 4, false);
        test_roll_count(&points, 7, true);
    }

    #[test]
    fn test_roll_13() {
        let points = [
            IntPoint::new(0, 2),
            IntPoint::new(5, 2),
            IntPoint::new(4, 2),
            IntPoint::new(4, 0),
            IntPoint::new(1, 0),
            IntPoint::new(1, 2),
        ];

        test_roll_count(&points, 4, false);
        test_roll_count(&points, 4, true);
    }

    fn test_count(points: &[IntPoint], count: usize, keep_same_line_points: bool) {
        let mut segments: Vec<Segment<ShapeCountBoolean>> = Vec::new();
        segments.append_path_iter(
            points.iter().copied(),
            ShapeType::Subject,
            keep_same_line_points,
        );
        segments.merge_if_needed();

        assert_eq!(segments.len(), count);
    }

    fn test_roll_count(slice: &[IntPoint], count: usize, keep_same_line_points: bool) {
        let mut points = slice.to_vec();
        let n = points.len();
        let mut segments: Vec<Segment<ShapeCountBoolean>> = Vec::with_capacity(n);
        for _ in 0..n {
            segments.append_path_iter(
                points.iter().copied(),
                ShapeType::Subject,
                keep_same_line_points,
            );
            segments.merge_if_needed();

            assert_eq!(segments.len(), count);

            segments.clear();
            roll_points(&mut points);
        }
    }

    fn roll_points(points: &mut Vec<IntPoint>) {
        if points.len() <= 1 {
            return;
        }

        if let Some(last) = points.pop() {
            points.insert(0, last);
        }
    }
}
