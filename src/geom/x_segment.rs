use std::cmp::Ordering;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use i_key_sort::index::{BinKey, BinLayout};
use crate::geom::line_range::LineRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct XSegment {
    pub(crate) a: IntPoint,
    pub(crate) b: IntPoint,
}

impl XSegment {
    #[inline(always)]
    pub(crate) fn y_range(&self) -> LineRange {
        if self.a.y < self.b.y {
            LineRange { min: self.a.y, max: self.b.y }
        } else {
            LineRange { min: self.b.y, max: self.a.y }
        }
    }

    #[inline(always)]
    pub(crate) fn is_not_vertical(&self) -> bool {
        self.a.x != self.b.x
    }

    #[inline(always)]
    pub(crate) fn is_under_segment(&self, other: &XSegment) -> bool {
        match self.a.cmp(&other.a) {
            Ordering::Less => {
                Triangle::is_clockwise_point(self.a, other.a, self.b)
            }
            Ordering::Equal => {
                Triangle::is_clockwise_point(self.a, other.b, self.b)
            }
            Ordering::Greater => {
                Triangle::is_clockwise_point(other.a, other.b, self.a)
            }
        }
    }

    #[inline(always)]
    pub(crate) fn is_under_segment_order(&self, other: &XSegment) -> Ordering {
        match self.a.cmp(&other.a) {
            Ordering::Less => Self::clockwise_order(self.a, other.a, self.b),
            Ordering::Equal => Self::clockwise_order(self.a, other.b, self.b),
            Ordering::Greater => Self::clockwise_order(other.a, other.b, self.a),
        }
    }


    #[inline(always)]
    pub(crate) fn is_not_intersect_y_range(&self, range: &LineRange) -> bool {
        range.min > self.a.y && range.min > self.b.y || range.max < self.a.y && range.max < self.b.y
    }

    #[inline(always)]
    pub(crate) fn cmp_by_angle(&self, other: &Self) -> Ordering {
        // sort angles counterclockwise
        debug_assert!(self.a == other.a);
        let v0 = self.b.subtract(self.a);
        let v1 = other.b.subtract(other.a);
        let cross = v0.cross_product(v1);
        0.cmp(&cross)
    }

    #[inline(always)]
    fn clockwise_order(p0: IntPoint, p1: IntPoint, p2: IntPoint) -> Ordering {
        let area = Triangle::area_two_point(p0, p1, p2);
        0.cmp(&area)
    }
}

impl PartialOrd for XSegment {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for XSegment {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.a.cmp(&other.a);
        if a == Ordering::Equal {
            self.b.cmp(&other.b)
        } else {
            a
        }
    }
}

impl BinKey<i32> for XSegment {
    #[inline(always)]
    fn bin_key(&self) -> i32 {
        self.a.x
    }

    #[inline(always)]
    fn bin_index(&self, layout: &BinLayout<i32>) -> usize {
        layout.index(self.a.x)
    }
}