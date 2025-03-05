use i_float::int::point::IntPoint;

pub(super) struct IsoMath;

impl IsoMath {

    #[inline]
    pub(super) fn diagonal(p0: IntPoint, p1: IntPoint) -> (bool, i64) {
        let pos = Self::is_diagonal_pos(p0, p1);

        let x = if pos {
            p0.x as i64 - p0.y as i64
        } else {
            p0.y as i64 + p0.x as i64
        };

        (pos, x)
    }

    #[inline]
    pub(super) fn is_diagonal_pos(p0: IntPoint, p1: IntPoint) -> bool {
        let dp = p1 - p0;
        dp.x > 0 && dp.y > 0 || dp.x < 0 && dp.y < 0
    }

}