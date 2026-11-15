use crate::core::solver::Solver;
use i_float::int::number::int::IntNumber;
use i_float::int::number::product_uint::UIntProduct;
use i_float::int::number::uint::UIntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::vector::IntVector;

#[derive(Clone, Copy)]
pub(super) struct SnapRadius<I: IntNumber> {
    pub(super) radius_squared: I::WideUInt,
    grid_shift: u32,
}

impl<I: IntNumber> SnapRadius<I> {
    const MAX_EXPONENT: usize = 2 * (I::BITS as usize - 4);

    pub(super) fn new(radius_squared: I::WideUInt) -> Self {
        Self {
            radius_squared,
            grid_shift: 1,
        }
    }

    pub(super) fn increment(&mut self, step: usize) {
        let exponent = (self.radius_squared.ilog2() as usize)
            .saturating_add(step)
            .min(Self::MAX_EXPONENT);
        self.radius_squared = I::WideUInt::ONE << exponent as u32;
        self.grid_shift = (self.grid_shift + 1).min(I::WideUInt::BITS - 1);
    }

    pub(super) fn is_near_grid(
        &self,
        dir_a: IntVector<I>,
        dir_b: IntVector<I>,
        cross_a: I::Wide,
        cross_b: I::Wide,
    ) -> bool {
        let denominator = dir_a.cross_product(dir_b).unsigned_abs();
        let dx = (cross_a * dir_b.x - cross_b * dir_a.x).unsigned_abs();
        let dy = (cross_a * dir_b.y - cross_b * dir_a.y).unsigned_abs();
        let axis_limit = denominator >> self.grid_shift;
        if dx.max(dy) > axis_limit {
            return false;
        }
        if dx + dy <= axis_limit {
            return true;
        }

        let x = dx << self.grid_shift;
        let y = dy << self.grid_shift;
        let multiply = <I::WideUInt as UIntNumber>::Product::multiply;
        multiply(x, x)
            .checked_add(multiply(y, y))
            .is_some_and(|sum| sum <= multiply(denominator, denominator))
    }
}

impl Solver {
    pub(super) fn snap_radius<I: IntNumber>(&self) -> SnapRadius<I> {
        let exponent = self.precision.start.min(SnapRadius::<I>::MAX_EXPONENT);
        SnapRadius::new(I::WideUInt::ONE << exponent as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::solver::{Precision, Solver};
    use i_float::int::number::int::IntNumber;

    #[test]
    fn squared_radius_preserves_initial_progression() {
        let mut snap = Solver::AUTO.snap_radius::<i16>();
        for (expected, grid_shift) in [(1, 1), (2, 2), (4, 3), (8, 4)] {
            assert_eq!(snap.radius_squared, expected);
            assert_eq!(snap.grid_shift, grid_shift);
            snap.increment(1);
        }
        let mut snap = Solver::with_precision(Precision::ABSOLUTE).snap_radius::<i16>();
        snap.increment(0);
        assert_eq!(snap.radius_squared, 1);
        assert_eq!(snap.grid_shift, 2);
    }

    #[test]
    fn squared_radius_saturates_at_each_engine_limit() {
        macro_rules! check_limit {
            ($int:ty, $exponent:expr) => {{
                let mut snap = Solver::with_precision(Precision {
                    start: $exponent - 1,
                    progression: 1,
                })
                .snap_radius::<$int>();
                let limit: <$int as IntNumber>::WideUInt = 1 << $exponent;
                assert_eq!(snap.radius_squared, limit / 2);
                snap.increment(1);
                assert_eq!(snap.radius_squared, limit);
                for _ in 0..256 {
                    snap.increment(usize::MAX);
                }
                assert_eq!(snap.radius_squared, limit);
                assert_eq!(snap.grid_shift, <$int as IntNumber>::WideUInt::BITS - 1);
            }};
        }
        check_limit!(i16, 24);
        check_limit!(i32, 56);
        check_limit!(i64, 120);
    }
}
