use i_float::bit_pack::BitPackVec;
use i_float::fix_vec::FixVec;

pub type SideFill = u8;
pub type VectorPath = Vec<VectorEdge>;
pub type VectorShape = Vec<VectorPath>;

pub const SUBJ_LEFT: u8 = 0b0001;
pub const SUBJ_RIGHT: u8 = 0b0010;
pub const CLIP_LEFT: u8 = 0b0100;
pub const CLIP_RIGHT: u8 = 0b1000;

pub trait Reverse {
    fn reverse(self) -> Self;
}

impl Reverse for SideFill {
    fn reverse(self) -> Self {
        let subj_left = self & SUBJ_LEFT;
        let subj_right = self & SUBJ_RIGHT;
        let clip_left = self & CLIP_LEFT;
        let clip_right = self & CLIP_RIGHT;

        (subj_left << 1) | (subj_right >> 1) | (clip_left << 1) | (clip_right >> 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VectorEdge {
    pub a: FixVec,
    pub b: FixVec,
    pub fill: SideFill,
}

impl VectorEdge {
    pub(crate) fn new(fill: SideFill, a: FixVec, b: FixVec) -> Self {
        let fill = if a.bit_pack() < b.bit_pack() {
            fill
        } else {
            fill.reverse()
        };

        Self { a, b, fill }
    }

    pub(crate) fn reverse(&mut self) {
        let c = self.a;
        self.a = self.b;
        self.b = c;
        self.fill = self.fill.reverse();
    }
}