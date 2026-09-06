use crate::core::solver::Solver;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;

pub(super) struct SnapRadius {
    current: usize,
    step: usize,
}

impl SnapRadius {
    pub(super) fn increment(&mut self) {
        self.current = self.current.saturating_add(self.step);
    }

    /// Squared-distance threshold for snapping to an existing endpoint.
    pub(super) fn radius_squared<I: IntNumber>(&self) -> I::Wide {
        let exponent = self.current.min((2 * (I::BITS - 4)) as usize) as u32;
        I::Wide::ONE << exponent
    }
}

impl Solver {
    pub(super) fn snap_radius(&self) -> SnapRadius {
        SnapRadius {
            current: self.precision.start,
            step: self.precision.progression,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SnapRadius;

    #[test]
    fn squared_radius_preserves_initial_progression() {
        let mut snap = SnapRadius { current: 0, step: 1 };
        for expected in [1, 2, 4, 8] {
            assert_eq!(snap.radius_squared::<i16>(), expected);
            snap.increment();
        }
    }

    #[test]
    fn squared_radius_saturates_at_each_engine_limit() {
        macro_rules! check_limit {
            ($int:ty, $exponent:expr) => {{
                let mut snap = SnapRadius {
                    current: $exponent - 1,
                    step: 1,
                };
                let limit: <$int as i_float::int::number::int::IntNumber>::Wide = 1 << $exponent;
                assert_eq!(snap.radius_squared::<$int>(), limit / 2);
                snap.increment();
                assert_eq!(snap.radius_squared::<$int>(), limit);
                snap.increment();
                assert_eq!(snap.radius_squared::<$int>(), limit);
                snap.step = usize::MAX;
                snap.increment();
                assert_eq!(snap.radius_squared::<$int>(), limit);
                snap.increment();
                assert_eq!(snap.radius_squared::<$int>(), limit);
            }};
        }
        check_limit!(i16, 24);
        check_limit!(i32, 56);
        check_limit!(i64, 120);
    }
}
