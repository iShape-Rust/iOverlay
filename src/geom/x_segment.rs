use std::cmp::Ordering;
use i_float::bit_pack::BitPackVec;
use i_float::fix_vec::FixVec;
use i_float::point::Point;
use i_shape::triangle::Triangle;
use crate::space::line_range::LineRange;
use crate::split::shape_edge::ShapeEdge;

#[derive(Debug, Clone, Copy)]
pub(crate) struct XSegment {
    pub(crate) a: Point,
    pub(crate) b: Point,
}

impl XSegment {
    pub(crate) fn y_range(&self) -> LineRange {
        if self.a.y < self.b.y {
            LineRange { min: self.a.y, max: self.b.y }
        } else {
            LineRange { min: self.b.y, max: self.a.y }
        }
    }

    pub(crate) fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub(crate) fn with_edge(edge: &ShapeEdge) -> Self {
        Self {
            a: Point::new_fix_vec(edge.a),
            b: Point::new_fix_vec(edge.b),
        }
    }

    pub(crate) fn is_under_point(&self, p: Point) -> bool {
        assert!(self.a.x <= p.x && p.x <= self.b.x);
        assert!(p != self.a && p != self.b);
        Self::is_clockwise_points(self.a, p, self.b)
    }

    pub(crate) fn is_above_point(&self, p: Point) -> bool {
        assert!(self.a.x <= p.x && p.x <= self.b.x);
        Self::is_clockwise_points(self.a, self.b, p)
    }

    pub(crate) fn is_under_segment(&self, other: XSegment) -> bool {
        if self.a == other.a {
            Self::is_clockwise_points(self.a, other.b, self.b)
        } else if self.a.x < other.a.x {
            Self::is_clockwise_points(self.a, other.a, self.b)
        } else {
            Self::is_clockwise_points(other.a, other.b, self.a)
        }
    }

    pub(crate) fn is_clockwise_points(p0: Point, p1: Point, p2: Point) -> bool {
        Triangle::is_clockwise(FixVec::new_point(p0), FixVec::new_point(p1), FixVec::new_point(p2))
    }

    pub (crate) fn order(&self, other: &Self) -> Ordering {
        let a0 = self.a.bit_pack();
        let a1 = other.a.bit_pack();
        if a0 != a1 {
            if a0 < a1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else if self.b.bit_pack() < other.b.bit_pack() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}