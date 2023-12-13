use std::cmp::Ordering;

use i_float::fix_vec::FixVec;
use i_shape::{fix_edge::{FixEdge, EdgeCross}, triangle::Triangle};
use crate::split::shape_count::ShapeCount;

#[derive(Debug, Clone, Copy)]
pub (crate) struct ShapeEdge {
    pub (crate) a: FixVec,
    pub (crate) b: FixVec,
    pub (super) count: ShapeCount,
}

impl ShapeEdge {

    pub (crate) const ZERO: ShapeEdge = ShapeEdge {
        a: FixVec::ZERO,
        b: FixVec::ZERO,
        count: ShapeCount { subj: 0, clip: 0 }
    };

    pub (super) fn edge(&self) -> FixEdge {
        FixEdge {
            e0: self.a,
            e1: self.b,
        }
    }

    pub (crate) fn new(a: FixVec, b: FixVec, count: ShapeCount) -> Self {
        if a.bit_pack() <= b.bit_pack() {
            Self { a, b, count }
        } else {
            Self { b, a, count }
        }
    }

    pub (super) fn from_parent(parent: ShapeEdge, count: ShapeCount) -> Self {
        Self { a: parent.a, b: parent.b, count }
    }

    pub (crate) fn is_less(&self, other: ShapeEdge) -> bool {
        let a0 = self.a.bit_pack();
        let a1 = other.a.bit_pack();
        if a0 != a1 {
            a0 < a1
        } else {
            self.b.bit_pack() < other.b.bit_pack()
        }
    }

    pub (crate) fn is_equal(&self, other: ShapeEdge) -> bool {
        self.a == other.a && self.b == other.b
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
        } else {
            let b0 = self.b.bit_pack();
            let b1 = other.b.bit_pack();
            if b0 == b1 {
                Ordering::Equal
            } else if b0 < b1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}