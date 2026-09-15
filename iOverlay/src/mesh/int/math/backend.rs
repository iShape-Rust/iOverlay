use crate::mesh::int::arc::{ArcDirection, ArcOptions};
use i_float::int::{
    angle::Angle, number::int::IntNumber, point::IntPoint, unit_vector::UnitIntVector, vector::IntVector,
};

/// Construction arithmetic only; coordinates and topology remain integer.
pub(crate) trait MeshMath<I: IntNumber>: Copy {
    type Arc: ArcMath<UnitIntVector<I>>;
    fn sin_cos(angle: Angle) -> (i32, i32);
    fn angle_between(from: UnitIntVector<I>, to: UnitIntVector<I>) -> Angle;

    fn normalize(vector: IntVector<I>) -> Option<UnitIntVector<I>>;
    fn scale(direction: UnitIntVector<I>, distance: I) -> IntVector<I>;
    fn rotate(direction: UnitIntVector<I>, local: IntPoint<I>) -> IntVector<I>;
    fn guard_padding(padding: I::Wide) -> I::Wide;
}

pub(crate) trait ArcMath<D> {
    fn new(options: ArcOptions) -> Self;
    fn build(&mut self, from: D, to: D, direction: ArcDirection) -> &[D];
}
