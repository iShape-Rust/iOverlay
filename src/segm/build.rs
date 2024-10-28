use i_float::point::IntPoint;
use i_float::triangle::Triangle;
use crate::core::overlay::ShapeType;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use crate::segm::shape_count::ShapeCount;

pub(crate) trait BuildSegments {
    fn append_iter<I: Iterator<Item=IntPoint>>(&mut self, iter: I, shape_type: ShapeType);
}

impl BuildSegments for Vec<Segment> {

    fn append_iter<I: Iterator<Item=IntPoint>>(&mut self, iter: I, shape_type: ShapeType) {
        let count = match shape_type {
            ShapeType::Subject => ShapeCount::new(1, 0),
            ShapeType::Clip => ShapeCount::new(0, 1),
        };

        private_append_iter(self, iter, count);
    }
}

fn private_append_iter<I: Iterator<Item=IntPoint>>(segments: &mut Vec<Segment>, mut iter: I, shape_count: ShapeCount) {
    // our goal add all not degenerate segments
    let mut p0 = if let Some(p) = iter.next() { p } else { return; };
    let mut p1 = if let Some(p) = iter.next() { p } else { return; };

    let q0 = p0;
    while let Some(p) = iter.next() {
        if Triangle::is_not_line_point(p0, p1, p) {
            p0 = p1;
            p1 = p;
            break;
        }
        p1 = p;
    }

    let q1 = p0;

    let direct = shape_count;
    let invert = shape_count.invert();

    while let Some(p) = iter.next() {
        dbg!(p);
        if Triangle::is_line_point(p0, p1, p) {
            p1 = p;
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
            if p0 != q1 {
                segments.push(Segment::with_ab(p0, q1, direct, invert));
            }
        }
        (true, false) => {
            // p0, p1, q0 is on same line
            if p0 != q0 {
                segments.push(Segment::with_ab(p0, q0, direct, invert));
            }
            segments.push(Segment::with_ab(q0, q1, direct, invert));
        }
        (false, true) => {
            // p1, q0, q1 is on same line
            segments.push(Segment::with_ab(p0, p1, direct, invert));
            if p1 != q1 {
                segments.push(Segment::with_ab(p1, q1, direct, invert));
            }
        }
    }
}

impl Segment {
    #[inline]
    fn with_ab(p0: IntPoint, p1: IntPoint, direct: ShapeCount, invert: ShapeCount) -> Self {
        if p0 < p1 {
            Self { x_segment: XSegment { a: p0, b: p1 }, count: direct }
        } else {
            Self { x_segment: XSegment { a: p1, b: p0 }, count: invert }
        }
    }
}

#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use crate::core::overlay::ShapeType;
    use crate::segm::build::BuildSegments;
    use crate::segm::merge::ShapeSegmentsMerge;
    use crate::segm::segment::Segment;

    #[test]
    fn test_0() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(0, 2),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0),
        ].to_vec();

        test_count(points, 4);
    }

    #[test]
    fn test_1() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(1, 0),
            IntPoint::new(2, 0)
        ].to_vec();

        test_count(points, 0);
    }

    #[test]
    fn test_2() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(1, 0),
            IntPoint::new(2, 0),
            IntPoint::new(3, 0)
        ].to_vec();

        test_count(points, 0);
    }

    #[test]
    fn test_3() {
        let points = [
            IntPoint::new(0, 0),
            IntPoint::new(2, 0),
            IntPoint::new(2, 2),
            IntPoint::new(2, 0)
        ].to_vec();

        test_count(points, 0);
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

        test_count(points, 0);
    }

    fn test_count(mut points: Vec<IntPoint>, count: usize) {
        let n = points.len();
        let mut segments: Vec<Segment> = Vec::with_capacity(n);
        for _ in 0..n {
            segments.append_iter(points.iter().copied(), ShapeType::Subject);
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