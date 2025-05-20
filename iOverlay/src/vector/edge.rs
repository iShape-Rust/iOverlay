use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_shape::int::path::IntPath;

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
    pub a: IntPoint,
    pub b: IntPoint,
    pub fill: SideFill,
}

impl VectorEdge {
    pub(crate) fn new(fill: SideFill, a: IntPoint, b: IntPoint) -> Self {
        let fill = if a < b {
            fill
        } else {
            fill.reverse()
        };

        Self { a, b, fill }
    }
}

pub trait ToPath {
    fn to_path(&self) -> IntPath;
}

impl ToPath for VectorPath {
    fn to_path(&self) -> IntPath {
        self.iter().map(|e| e.a).collect()
    }
}