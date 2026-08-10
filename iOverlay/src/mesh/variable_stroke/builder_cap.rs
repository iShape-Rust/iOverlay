use crate::mesh::rotator::Rotator;
use crate::mesh::variable_stroke::builder::PolygonBuilder;
use crate::mesh::variable_stroke::section::{CoveredSection, Section};
use core::f64::consts::PI;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use i_float::int::number::int::IntNumber;

#[derive(Clone, Copy)]
pub(super) struct RoundFanBuilder<T: FloatNumber> {
    angle: T,
}

impl<T: FloatNumber> RoundFanBuilder<T> {
    #[inline]
    pub(super) fn new(angle: T) -> Self {
        Self { angle }
    }

    #[inline]
    pub(super) fn capacity(&self) -> usize {
        (T::from_float(2.0 * PI) / self.angle)
            .to_usize()
            .saturating_add(1)
            * 3
    }

    pub(super) fn add_start_cap<P, I>(&self, section: &Section<P>, output: &mut PolygonBuilder<P, I>)
    where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        let through = P::from_xy(-section.direction.x(), -section.direction.y());
        self.add_round_fan(
            section.a,
            section.a_radius,
            section.a_right,
            section.a_left,
            through,
            output,
        );
    }

    pub(super) fn add_end_cap<P, I>(&self, section: &Section<P>, output: &mut PolygonBuilder<P, I>)
    where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        self.add_round_fan(
            section.b,
            section.b_radius,
            section.b_left,
            section.b_right,
            section.direction,
            output,
        );
    }

    pub(super) fn add_covered_cap<P, I>(&self, section: &CoveredSection<P>, output: &mut PolygonBuilder<P, I>)
    where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if section.radius <= T::ZERO {
            return;
        }
        let normal = P::from_xy(-section.outward.y(), section.outward.x());
        let from = FloatPointMath::add(&section.center, &FloatPointMath::scale(&normal, section.radius));
        let to = FloatPointMath::sub(&section.center, &FloatPointMath::scale(&normal, section.radius));
        self.add_round_fan(section.center, section.radius, from, to, section.outward, output);
    }

    pub(super) fn add_round_fan<P, I>(
        &self,
        center: P,
        radius: T,
        from: P,
        to: P,
        through: P,
        output: &mut PolygonBuilder<P, I>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if radius <= T::ZERO {
            return;
        }

        let from_vector = FloatPointMath::sub(&from, &center);
        let to_vector = FloatPointMath::sub(&to, &center);
        if FloatPointMath::sqr_length(&from_vector) <= T::ZERO
            || FloatPointMath::sqr_length(&to_vector) <= T::ZERO
        {
            return;
        }

        let from_unit = FloatPointMath::normalize(&from_vector);
        let to_unit = FloatPointMath::normalize(&to_vector);
        let through_unit = FloatPointMath::normalize(&through);
        let sweep = Self::sweep_through(&from_unit, &to_unit, &through_unit);
        self.add_fan(center, from, to, from_vector, sweep, output);
    }

    pub(super) fn add_join_fan<P, I>(
        &self,
        center: P,
        radius: T,
        from: P,
        to: P,
        turn: T,
        output: &mut PolygonBuilder<P, I>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if radius <= T::ZERO {
            return;
        }

        let from_vector = FloatPointMath::sub(&from, &center);
        let to_vector = FloatPointMath::sub(&to, &center);
        if FloatPointMath::sqr_length(&from_vector) <= T::ZERO
            || FloatPointMath::sqr_length(&to_vector) <= T::ZERO
        {
            return;
        }

        let from_unit = FloatPointMath::normalize(&from_vector);
        let to_unit = FloatPointMath::normalize(&to_vector);
        let sweep = Self::join_sweep(&from_unit, &to_unit, turn);
        self.add_fan(center, from, to, from_vector, sweep, output);
    }

    fn add_fan<P, I>(
        &self,
        center: P,
        from: P,
        to: P,
        from_vector: P,
        sweep: T,
        output: &mut PolygonBuilder<P, I>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        let count = (sweep.abs() / self.angle)
            .to_usize()
            .saturating_add(1)
            .clamp(1, 1024);
        let delta = sweep / T::from_usize(count);
        let rotator = Rotator::with_angle(delta);

        let mut vector = from_vector;
        let mut a = from;
        for i in 1..=count {
            let b = if i == count {
                to
            } else {
                vector = rotator.rotate(&vector);
                FloatPointMath::add(&center, &vector)
            };
            output.add_triangle(center, a, b);
            a = b;
        }
    }

    fn join_sweep<P: FloatPointCompatible<Scalar = T>>(from: &P, to: &P, turn: T) -> T {
        let counterclockwise = Self::ccw_angle(from, to);
        let full = T::from_float(2.0 * PI);
        let epsilon = T::from_float(1.0e-6);
        if counterclockwise <= epsilon {
            return counterclockwise;
        }

        let clockwise = counterclockwise - full;
        if (counterclockwise - turn).abs() <= (clockwise - turn).abs() {
            counterclockwise
        } else {
            clockwise
        }
    }

    fn sweep_through<P: FloatPointCompatible<Scalar = T>>(from: &P, to: &P, through: &P) -> T {
        let full = T::from_float(2.0 * PI);
        let total = Self::ccw_angle(from, to);
        let via = Self::ccw_angle(from, through);
        let epsilon = T::from_float(1.0e-6);
        if via <= total + epsilon {
            total
        } else {
            total - full
        }
    }

    fn ccw_angle<P: FloatPointCompatible<Scalar = T>>(a: &P, b: &P) -> T {
        let dot = FloatPointMath::dot_product(a, b)
            .max(T::from_float(-1.0))
            .min(T::ONE);
        let angle = dot.acos();
        if FloatPointMath::cross_product(a, b) >= T::ZERO {
            angle
        } else {
            T::from_float(2.0 * PI) - angle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RoundFanBuilder;

    #[test]
    fn join_uses_complement_when_it_is_closer_to_centerline_turn() {
        let from_angle = -48.871_721_f64.to_radians();
        let to_angle = -72.717_667_f64.to_radians();
        let from = [from_angle.cos(), from_angle.sin()];
        let to = [to_angle.cos(), to_angle.sin()];
        let turn = 41.208_797_f64.to_radians();
        let sweep = RoundFanBuilder::<f64>::join_sweep(&from, &to, turn);

        assert!(sweep < 0.0);
        assert!(sweep.abs() < core::f64::consts::PI);
        assert!((sweep.to_degrees() + 23.845_946).abs() < 0.000_1);
    }

    #[test]
    fn join_follows_centerline_turn_beyond_half_circle() {
        let from_angle = 131.658_745_f64.to_radians();
        let to_angle = -36.261_842_f64.to_radians();
        let from = [from_angle.cos(), from_angle.sin()];
        let to = [to_angle.cos(), to_angle.sin()];
        let turn = 169.324_035_f64.to_radians();
        let sweep = RoundFanBuilder::<f64>::join_sweep(&from, &to, turn);

        assert!(sweep > core::f64::consts::PI);
        assert!((sweep.to_degrees() - 192.079_413).abs() < 0.000_1);
    }
}
