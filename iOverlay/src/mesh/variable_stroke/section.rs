use crate::mesh::math::Math;
use crate::mesh::variable_stroke::style::StrokeVertex;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use i_float::int::number::int::IntNumber;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RadiusTrend {
    Decreasing,
    Constant,
    Increasing,
}

#[derive(Clone, Copy)]
pub(super) struct Section<P: FloatPointCompatible> {
    pub(super) a: P,
    pub(super) b: P,
    pub(super) a_left: P,
    pub(super) b_left: P,
    pub(super) a_right: P,
    pub(super) b_right: P,
    pub(super) radius_trend: RadiusTrend,
}

impl<P: FloatPointCompatible> Section<P> {
    pub(super) fn try_new<I: IntNumber>(
        a: &StrokeVertex<P>,
        b: &StrokeVertex<P>,
        adapter: &FloatPointAdapter<P, I>,
    ) -> Option<Self> {
        let int_a = adapter.float_to_int(&a.point);
        let int_b = adapter.float_to_int(&b.point);
        if int_a == int_b {
            return None;
        }

        let int_a_radius = adapter.round_len_to_int(a.radius());
        let int_b_radius = adapter.round_len_to_int(b.radius());
        if int_a_radius.max(int_b_radius) <= I::ONE {
            return None;
        }
        let radius_trend = if int_a_radius < int_b_radius {
            RadiusTrend::Increasing
        } else if int_a_radius > int_b_radius {
            RadiusTrend::Decreasing
        } else {
            RadiusTrend::Constant
        };

        let int_radius_delta = int_a_radius.to_wide() - int_b_radius.to_wide();
        let vector = int_b - int_a;
        let int_distance_sqr = vector.sqr_length();

        if int_radius_delta * int_radius_delta >= int_distance_sqr {
            return None;
        }

        let a = adapter.int_to_float(&int_a);
        let b = adapter.int_to_float(&int_b);
        let a_radius = adapter.len_to_float(int_a_radius);
        let b_radius = adapter.len_to_float(int_b_radius);

        Some(Self::new(a_radius, b_radius, &a, &b, radius_trend))
    }

    fn new(a_radius: P::Scalar, b_radius: P::Scalar, a: &P, b: &P, radius_trend: RadiusTrend) -> Self {
        let direction = Math::normal(b, a);
        let center_vector = FloatPointMath::sub(b, a);
        let distance_sqr = FloatPointMath::sqr_length(&center_vector);
        let distance = distance_sqr.sqrt();
        let radius_delta = a_radius - b_radius;
        let k = radius_delta / distance;
        let h = (P::Scalar::ONE - k * k).max(P::Scalar::ZERO).sqrt();

        let normal = P::from_xy(-direction.y(), direction.x());
        let left_normal = P::from_xy(
            k * direction.x() + h * normal.x(),
            k * direction.y() + h * normal.y(),
        );
        let right_normal = P::from_xy(
            k * direction.x() - h * normal.x(),
            k * direction.y() - h * normal.y(),
        );

        let a_left = FloatPointMath::add(a, &FloatPointMath::scale(&left_normal, a_radius));
        let b_left = FloatPointMath::add(b, &FloatPointMath::scale(&left_normal, b_radius));
        let a_right = FloatPointMath::add(a, &FloatPointMath::scale(&right_normal, a_radius));
        let b_right = FloatPointMath::add(b, &FloatPointMath::scale(&right_normal, b_radius));

        Self {
            a: *a,
            b: *b,
            a_left,
            b_left,
            a_right,
            b_right,
            radius_trend,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RadiusTrend, Section};
    use crate::mesh::variable_stroke::StrokeVertex;
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;

    fn adapter() -> FloatPointAdapter<[f64; 2], i32> {
        FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 1.0)
    }

    #[test]
    fn equal_width_has_parallel_tangents() {
        let a = StrokeVertex::new([0.0, 0.0], 4.0);
        let b = StrokeVertex::new([10.0, 0.0], 4.0);
        let section = Section::try_new(&a, &b, &adapter()).unwrap();

        assert_eq!(section.a_left, [0.0, 2.0]);
        assert_eq!(section.b_left, [10.0, 2.0]);
        assert_eq!(section.a_right, [0.0, -2.0]);
        assert_eq!(section.b_right, [10.0, -2.0]);
        assert_eq!(section.radius_trend, RadiusTrend::Constant);
    }

    #[test]
    fn radius_trend_uses_adapter_radii() {
        let increasing = Section::try_new(
            &StrokeVertex::new([0.0, 0.0], 4.0),
            &StrokeVertex::new([10.0, 0.0], 6.0),
            &adapter(),
        )
        .unwrap();
        let decreasing = Section::try_new(
            &StrokeVertex::new([0.0, 0.0], 6.0),
            &StrokeVertex::new([10.0, 0.0], 4.0),
            &adapter(),
        )
        .unwrap();

        assert_eq!(increasing.radius_trend, RadiusTrend::Increasing);
        assert_eq!(decreasing.radius_trend, RadiusTrend::Decreasing);
    }

    #[test]
    fn points_equal_in_int_space_are_zero() {
        let adapter: FloatPointAdapter<[f64; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 10.0);
        let a = StrokeVertex::new([0.01, 0.01], 4.0);
        let b = StrokeVertex::new([0.04, 0.04], 4.0);

        assert!(Section::try_new(&a, &b, &adapter).is_none());
    }

    #[test]
    fn radius_at_most_one_in_int_space_is_zero() {
        let a = StrokeVertex::new([0.0, 0.0], 2.0);
        let b = StrokeVertex::new([10.0, 0.0], 2.0);

        assert!(Section::try_new(&a, &b, &adapter()).is_none());
    }
}
