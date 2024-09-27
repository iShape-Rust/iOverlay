use std::cmp::Ordering;
use i_float::point::IntPoint;
use i_float::triangle::Triangle;
use i_key_sort::index::{BinKey, BinLayout};
use crate::line_range::LineRange;

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
    pub(crate) fn is_under_point(&self, p: IntPoint) -> bool {
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        debug_assert!(p != self.a && p != self.b);
        Triangle::area_two_point(self.a, p, self.b) > 0
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
    pub(crate) fn is_not_intersect_y_range(&self, range: &LineRange) -> bool {
        range.min > self.a.y && range.min > self.b.y || range.max < self.a.y && range.max < self.b.y
    }
}

impl PartialOrd for XSegment {
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