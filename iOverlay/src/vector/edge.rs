use crate::core::edge_data::OverlayEdgeData;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::point::IntPoint;
use i_shape::int::path::IntPath;

pub type SideFill = u8;
pub type DataVectorPath<D = (), I = i32> = Vec<DataVectorEdge<D, I>>;
pub type DataVectorShape<D = (), I = i32> = Vec<DataVectorPath<D, I>>;
pub type VectorPath<I = i32> = DataVectorPath<(), I>;
pub type VectorShape<I = i32> = DataVectorShape<(), I>;

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
pub struct DataVectorEdge<D = (), I: IntNumber = i32> {
    pub a: IntPoint<I>,
    pub b: IntPoint<I>,
    pub fill: SideFill,
    pub data: D,
}

impl<D: OverlayEdgeData, I: IntNumber> DataVectorEdge<D, I> {
    pub(crate) fn new(fill: SideFill, a: IntPoint<I>, b: IntPoint<I>, data: D) -> Self {
        let (fill, data) = if a < b {
            (fill, data)
        } else {
            (fill.reverse(), data.reversed())
        };

        Self { a, b, fill, data }
    }
}

pub trait ToPath<I: IntNumber> {
    fn to_path(&self) -> IntPath<I>;
}

impl<I: IntNumber> ToPath<I> for VectorPath<I> {
    fn to_path(&self) -> IntPath<I> {
        self.iter().map(|e| e.a).collect()
    }
}
