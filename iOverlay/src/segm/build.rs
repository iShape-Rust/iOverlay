use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use crate::core::overlay::ShapeType;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use crate::segm::winding_count::WindingCount;

pub(crate) trait BuildSegments {
    fn append_path_iter<I: Iterator<Item=IntPoint>>(&mut self, iter: I, shape_type: ShapeType, keep_same_line_points: bool) -> bool;
}

impl<C: WindingCount> BuildSegments for Vec<Segment<C>> {
    #[inline]
    fn append_path_iter<I: Iterator<Item=IntPoint>>(&mut self, iter: I, shape_type: ShapeType, keep_same_line_points: bool) -> bool {
        if keep_same_line_points {
            append_iter_keeping_same_line_points(self, iter, shape_type)
        } else {
            append_iter_removing_same_line_points(self, iter, shape_type)
        }
    }
}

fn append_iter_removing_same_line_points<I: Iterator<Item=IntPoint>, C: WindingCount>(segments: &mut Vec<Segment<C>>, mut iter: I, shape_type: ShapeType) -> bool {
    // our goal add all not degenerate segments
    let mut modified = false;
    let mut p0 = if let Some(p) = iter.next() { p } else { return modified; };
    let mut p1 = if let Some(p) = iter.next() { p } else { return modified; };

    let q0 = p0;
    for p in &mut iter {
        if Triangle::is_not_line_point(p0, p1, p) {
            p0 = p1;
            p1 = p;
            break;
        }
        modified = true;
        p1 = p;
    }

    let q1 = p0;

    let (direct, invert) = C::with_shape_type(shape_type);

    for p in &mut iter {
        if Triangle::is_line_point(p0, p1, p) {
            p1 = p;
            modified = true;
            continue;
        }
        segments.push(Segment::with_ab(p0, p1, direct, invert));

        p0 = p1;
        p1 = p;
    }

    let is_q0 = Triangle::is_line_point(p0, p1, q0);
    let is_p1 = Triangle::is_line_point(q0, q1, p1);

    match (is_q0, is_p1) {
        (false, false) => {
            // no one is collinear, most common case
            segments.push(Segment::with_ab(p0, p1, direct, invert));
            segments.push(Segment::with_ab(p1, q0, direct, invert));
            segments.push(Segment::with_ab(q0, q1, direct, invert));
        }
        (true, true) => {
            // all collinear
            modified = true;
            if p0 != q1 {
                segments.push(Segment::with_ab(p0, q1, direct, invert));
            }
        }
        (true, false) => {
            // p0, p1, q0 is on same line
            modified = true;
            if p0 != q0 {
                segments.push(Segment::with_ab(p0, q0, direct, invert));
            }
            segments.push(Segment::with_ab(q0, q1, direct, invert));
        }
        (false, true) => {
            // p1, q0, q1 is on same line
            modified = true;
            segments.push(Segment::with_ab(p0, p1, direct, invert));
            if p1 != q1 {
                segments.push(Segment::with_ab(p1, q1, direct, invert));
            }
        }
    }
    modified
}

fn append_iter_keeping_same_line_points<I: Iterator<Item=IntPoint>, C: WindingCount>(segments: &mut Vec<Segment<C>>, mut iter: I, shape_type: ShapeType) -> bool {
    // our goal add all not degenerate segments escaping same line points
    let mut modified = false;
    let mut p0 = if let Some(p) = iter.next() { p } else { return modified; };
    let mut p1 = if let Some(p) = iter.next() { p } else { return modified; };

    let q0 = p0;
    for p in &mut iter {
        if test_keep_points(p0, p1, p) {
            p0 = p1;
            p1 = p;
            break;
        }
        modified = true;
        p1 = p;
    }

    let q1 = p0;

    let (direct, invert) = C::with_shape_type(shape_type);

    for p in &mut iter {
        if !test_keep_points(p0, p1, p) {
            p1 = p;
            modified = true;
            continue;
        }
        segments.push(Segment::with_ab(p0, p1, direct, invert));

        p0 = p1;
        p1 = p;
    }

    let is_q0 = !test_keep_points(p0, p1, q0);
    let is_p1 = !test_keep_points(q1, q0, p1);

    match (is_q0, is_p1) {
        (false, false) => {
            // no one is collinear, most common case
            segments.push(Segment::with_ab(p0, p1, direct, invert));
            segments.push(Segment::with_ab(p1, q0, direct, invert));
            segments.push(Segment::with_ab(q0, q1, direct, invert));
        }
        (true, true) => {
            // all collinear
            modified = true;
            if p0 != q1 {
                segments.push(Segment::with_ab(p0, q1, direct, invert));
            }
        }
        (true, false) => {
            // p0, p1, q0 is on same line
            modified = true;
            if p0 != q0 {
                segments.push(Segment::with_ab(p0, q0, direct, invert));
            }
            segments.push(Segment::with_ab(q0, q1, direct, invert));
        }
        (false, true) => {
            // p1, q0, q1 is on same line
            modified = true;
            segments.push(Segment::with_ab(p0, p1, direct, invert));
            if p1 != q1 {
                segments.push(Segment::with_ab(p1, q1, direct, invert));
            }
        }
    }
    modified
}

#[inline]
fn test_keep_points(p0: IntPoint, p1: IntPoint, p2: IntPoint) -> bool {
    let a = p1.subtract(p0);
    let b = p1.subtract(p2);

    let cross = a.cross_product(b);
    let dot = a.dot_product(b);

    // true to keep
    let keep = cross != 0 || dot < 0;
    keep
}

impl<C: Send> Segment<C> {
    #[inline]
    pub(crate) fn with_ab(p0: IntPoint, p1: IntPoint, direct: C, invert: C) -> Self {
        if p0 < p1 {
            Self { x_segment: XSegment { a: p0, b: p1 }, count: direct }
        } else {
            Self { x_segment: XSegment { a: p1, b: p0 }, count: invert }
        }
    }
}

#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use crate::core::overlay::ShapeType;
    use crate::segm::build::BuildSegments;
    use crate::segm::merge::ShapeSegmentsMerge;
    use crate::segm::segment::Segment;
    use crate::segm::winding_count::ShapeCountBoolean;

    #[test]
    fn test_0() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(0, 2),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ].to_vec();

        test_count(points.clone(), 4, false);
        test_count(points, 4, true);
    }

    #[test]
    fn test_1() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(1, 0),
            IntPoint::new(2, 0)
        ].to_vec();

        test_count(points.clone(), 0, false);
        test_count(points, 0, true);
    }

    #[test]
    fn test_2() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(1, 0),
            IntPoint::new(2, 0),
            IntPoint::new(3, 0)
        ].to_vec();

        test_count(points.clone(), 0, false);
        test_count(points, 0, true);
    }

    #[test]
    fn test_3() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0)
        ].to_vec();

        test_count(points.clone(), 0, false);
        test_count(points, 0, true);
    }

    #[test]
    fn test_4() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
            IntPoint::new(4, 2),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ].to_vec();

        test_count(points.clone(), 0, false);
        test_count(points, 0, true);
    }

    #[test]
    fn test_5() {
        let points = vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, -10),
            IntPoint::new(0, -10),
            IntPoint::new(10, -10),
            IntPoint::new(10, 0),
            IntPoint::new(10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(-10, 10),
        ];

        test_count(points.clone(), 4, false);
        test_count(points, 8, true);
    }

    fn test_count(mut points: Vec<IntPoint>, count: usize, keep_same_line_points: bool) {
        let n = points.len();
        let mut segments: Vec<Segment<ShapeCountBoolean>> = Vec::with_capacity(n);
        for _ in 0..n {
            segments.append_path_iter(points.iter().copied(), ShapeType::Subject, keep_same_line_points);
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