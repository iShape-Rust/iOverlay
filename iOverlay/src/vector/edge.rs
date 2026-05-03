use crate::core::edge_data::OverlayEdgeData;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_shape::int::path::IntPath;

pub type SideFill = u8;
pub type DataVectorPath<D> = Vec<DataVectorEdge<D>>;
pub type DataVectorShape<D> = Vec<DataVectorPath<D>>;
pub type VectorPath = DataVectorPath<()>;
pub type VectorShape = DataVectorShape<()>;

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
pub struct DataVectorEdge<D = ()> {
    pub a: IntPoint,
    pub b: IntPoint,
    pub fill: SideFill,
    pub data: D,
}

impl<D: OverlayEdgeData> DataVectorEdge<D> {
    pub(crate) fn new(fill: SideFill, a: IntPoint, b: IntPoint, data: D) -> Self {
        let (fill, data) = if a < b {
            (fill, data)
        } else {
            (fill.reverse(), data.reversed())
        };

        Self { a, b, fill, data }
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
