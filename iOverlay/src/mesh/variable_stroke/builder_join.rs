use crate::mesh::variable_stroke::builder::PolygonBuilder;
use crate::mesh::variable_stroke::builder_cap::RoundFanBuilder;
use crate::mesh::variable_stroke::section::Section;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use i_float::int::number::int::IntNumber;

pub(super) struct RoundJoinBuilder<T: FloatNumber> {
    _angle: T,
}

impl<T: FloatNumber> RoundJoinBuilder<T> {
    #[inline]
    pub(super) fn new(angle: T) -> Self {
        Self { _angle: angle }
    }

    pub(super) fn add<P, I>(
        &self,
        previous: &Section<P>,
        next: &Section<P>,
        fan: &RoundFanBuilder<T>,
        output: &mut PolygonBuilder<P, I>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        let cross = FloatPointMath::cross_product(&previous.direction, &next.direction);
        let dot = FloatPointMath::dot_product(&previous.direction, &next.direction);
        let epsilon = T::from_float(0.0001);

        if Self::is_reversal(dot) {
            fan.add_end_cap(previous, output);
            fan.add_start_cap(next, output);
            return;
        }

        if cross.abs() <= epsilon {
            output.add_triangle(previous.b, previous.b_left, next.a_left);
            output.add_triangle(previous.b, next.a_right, previous.b_right);
            return;
        }

        let angle = dot.max(-T::ONE).min(T::ONE).acos();
        let turn = if cross > T::ZERO { angle } else { -angle };

        if cross > T::ZERO {
            output.add_triangle(previous.b, next.a_left, previous.b_left);
            fan.add_join_fan(
                previous.b,
                previous.b_radius.max(next.a_radius),
                previous.b_right,
                next.a_right,
                turn,
                output,
            );
        } else {
            output.add_triangle(previous.b, previous.b_right, next.a_right);
            fan.add_join_fan(
                previous.b,
                previous.b_radius.max(next.a_radius),
                previous.b_left,
                next.a_left,
                turn,
                output,
            );
        }
    }

    #[inline]
    fn is_reversal(dot: T) -> bool {
        dot <= T::from_float(-0.999)
    }
}

#[cfg(test)]
mod tests {
    use super::RoundJoinBuilder;

    #[test]
    fn near_antiparallel_edges_are_reversal() {
        assert!(RoundJoinBuilder::<f64>::is_reversal(-0.999_838_686));
        assert!(!RoundJoinBuilder::<f64>::is_reversal(-0.99));
    }
}
