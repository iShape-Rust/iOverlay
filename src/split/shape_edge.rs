use std::cmp::Ordering;

use i_float::fix_vec::FixVec;
use i_shape::{fix_edge::{FixEdge, EdgeCross}, triangle::Triangle};
use crate::split::shape_count::ShapeCount;

#[derive(Debug, Clone, Copy)]
pub (crate) struct ShapeEdge {
    pub (crate) a: FixVec,
    pub (crate) b: FixVec,
    pub (super) a_bit_pack: i64,
    pub (super) b_bit_pack: i64,
    pub (super) count: ShapeCount,
    max_y: i64,
    min_y: i64,
}

impl ShapeEdge {

    pub (crate) const ZERO: ShapeEdge = ShapeEdge {
        a: FixVec::ZERO,
        b: FixVec::ZERO,
        count: ShapeCount { subj: 0, clip: 0 },
        a_bit_pack: 0,
        b_bit_pack: 0,
        max_y: 0,
        min_y: 0,
    };

    pub (super) fn edge(&self) -> FixEdge {
        FixEdge {
            e0: self.a,
            e1: self.b,
        }
    }

    pub (crate) fn new(a: FixVec, b: FixVec, count: ShapeCount) -> Self {
        let a_bit_pack = a.bit_pack();
        let b_bit_pack = b.bit_pack();
        let (a, b, a_bit_pack, b_bit_pack) = if a_bit_pack <= b_bit_pack {
            (a, b, a_bit_pack, b_bit_pack)
        } else {
            (b, a, b_bit_pack, a_bit_pack)
        };
        let (max_y, min_y) = if a.y < b.y { (b.y.value(), a.y.value()) } else { (a.y.value(), b.y.value()) };

        Self {
            a,
            b,
            a_bit_pack,
            b_bit_pack,
            count,
            max_y,
            min_y,
        }
    }

    pub (super) fn from_parent(parent: ShapeEdge, count: ShapeCount) -> Self {
        Self {
            a: parent.a,
            b: parent.b,
            min_y: parent.min_y,
            max_y: parent.max_y,
            count,
            a_bit_pack: parent.a_bit_pack,
            b_bit_pack: parent.b_bit_pack,
        }
    }

    pub (crate) fn merge(&self, other: ShapeEdge) -> ShapeEdge {
        ShapeEdge::new(self.a, self.b, self.count.add(other.count))
    }

    pub (crate) fn is_less(&self, other: ShapeEdge) -> bool {
        let a0 = self.a_bit_pack;
        let a1 = other.a_bit_pack;
        if a0 != a1 {
            a0 < a1
        } else {
            let b0 = self.b_bit_pack;
            let b1 = other.b_bit_pack;
            b0 < b1
        }
    }

    pub (crate) fn is_equal(&self, other: ShapeEdge) -> bool {
        let a0 = self.a_bit_pack;
        let a1 = other.a_bit_pack;
        let b0 = self.b_bit_pack;
        let b1 = other.b_bit_pack;
        a0 == a1 && b0 == b1
    }

    pub (super) fn cross(&self, edge: ShapeEdge) -> EdgeCross {
        if edge.min_y <= self.max_y && edge.max_y >= self.min_y {
            self.edge().cross(edge.edge())
        } else {
            EdgeCross::NOT_CROSS
        }
    }

    pub (super) fn is_not_same_line(&self, point: FixVec) -> bool {
        Triangle::is_not_line(self.a, self.b, point)
    }

    pub (crate) fn is_even(&self) -> bool {
        self.count.is_even()
    }

    pub (crate) fn is_odd_clip(&self) -> bool {
        self.count.clip % 2 == 1
    }

    pub (crate) fn is_odd_subj(&self) -> bool {
        self.count.subj % 2 == 1
    }

    pub (crate) fn order(&self, other: &Self) -> Ordering {
        let a0 = self.a_bit_pack;
        let a1 = other.a_bit_pack;
        if a0 != a1 {
            if a0 < a1 {
                return Ordering::Less
            } else {
                return Ordering::Greater
            }
        } else {
            let b0 = self.b_bit_pack;
            let b1 = other.b_bit_pack;
            if b0 == b1 {
                return Ordering::Equal
            } else if b0 < b1 {
                return Ordering::Less
            } else {
                return Ordering::Greater
            }
        }
    }

}