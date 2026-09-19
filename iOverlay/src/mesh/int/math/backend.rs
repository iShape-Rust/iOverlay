use crate::mesh::int::arc::ArcMath;
use i_float::int::{
    angle::Angle, number::int::IntNumber, point::IntPoint, unit_vector::UnitIntVector, vector::IntVector,
};

/// Five degrees: avoid amplifying rounding errors in nearly parallel offset lines.
pub(crate) const DEFAULT_MITER_MIN_TURN: u32 = (1u32 << 31) / 36;

/// Construction arithmetic only; coordinates and topology remain integer.
pub(crate) trait MeshMath<I: IntNumber>: Copy {
    type Arc: ArcMath<UnitIntVector<I>>;
    /// Backend-specific minimum interior angle, in Angle bits.
    /// The configurable near-straight cutoff is independent of this clipping limit.
    const MIN_MITER_ANGLE: u32;
    fn sin_cos(angle: Angle) -> (i32, i32);
    fn angle_between(from: UnitIntVector<I>, to: UnitIntVector<I>) -> Angle;

    fn normalize(vector: IntVector<I>) -> Option<UnitIntVector<I>>;
    fn scale(direction: UnitIntVector<I>, distance: I) -> IntVector<I>;
    fn rotate(direction: UnitIntVector<I>, local: IntPoint<I>) -> IntVector<I>;
    fn guard_padding(padding: I::Wide) -> I::Wide;
}
