use i_float::fix_vec::FixVec;
use i_shape::fix_edge::FixEdge;
use crate::split::shape_count::ShapeCount;

#[derive(Debug, Clone, Copy)]
pub (crate) struct ShapeEdge {
    pub (crate) a: FixVec,
    pub (crate) b: FixVec,
    a_bit_pack: i64,
    b_bit_pack: i64,
    pub (super) count: ShapeCount,
    max_y: i64,
    min_y: i64,
}

impl ShapeEdge {

    pub (super) const ZERO: ShapeEdge = ShapeEdge {
        a: FixVec::ZERO,
        b: FixVec::ZERO,
        count: ShapeCount { subj: 0, clip: 0 },
        a_bit_pack: 0,
        b_bit_pack: 0,
        max_y: 0,
        min_y: 0,
    };

    pub fn edge(&self) -> FixEdge {
        FixEdge {
            e0: self.a,
            e1: self.b,
        }
    }

    pub fn new(a: FixVec, b: FixVec, count: ShapeCount) -> Self {
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

    pub fn from_parent(parent: ShapeEdge, count: ShapeCount) -> Self {
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

    pub fn merge(&self, other: ShapeEdge) -> ShapeEdge {
        ShapeEdge::new(self.a, self.b, self.count.add(other.count))
    }

    pub fn is_less(&self, other: ShapeEdge) -> bool {
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

    pub fn is_less_or_equal(&self, other: ShapeEdge) -> bool {
        let a0 = self.a_bit_pack;
        let a1 = other.a_bit_pack;
        if a0 != a1 {
            a0 < a1
        } else {
            let b0 = self.b_bit_pack;
            let b1 = other.b_bit_pack;
            b0 <= b1
        }
    }

    pub fn is_equal(&self, other: ShapeEdge) -> bool {
        let a0 = self.a_bit_pack;
        let a1 = other.a_bit_pack;
        let b0 = self.b_bit_pack;
        let b1 = other.b_bit_pack;
        a0 == a1 && b0 == b1
    }
}