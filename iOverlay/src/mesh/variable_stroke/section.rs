use crate::mesh::variable_stroke::style::StrokeVertex;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;

#[derive(Clone, Copy)]
pub(super) struct Section<P: FloatPointCompatible> {
    pub(super) a: P,
    pub(super) b: P,
    pub(super) a_radius: P::Scalar,
    pub(super) b_radius: P::Scalar,
    pub(super) a_left: P,
    pub(super) b_left: P,
    pub(super) a_right: P,
    pub(super) b_right: P,
    pub(super) direction: P,
}

#[derive(Clone, Copy)]
pub(super) struct CoveredSection<P: FloatPointCompatible> {
    pub(super) center: P,
    pub(super) radius: P::Scalar,
    pub(super) outward: P,
}

pub(super) enum SectionKind<P: FloatPointCompatible> {
    Regular(Section<P>),
    Covered(CoveredSection<P>),
    Coincident,
    Empty,
}

impl<P: FloatPointCompatible> Section<P> {
    pub(super) fn classify(a: &StrokeVertex<P>, b: &StrokeVertex<P>) -> SectionKind<P> {
        if !Self::is_finite(a) || !Self::is_finite(b) {
            return SectionKind::Empty;
        }

        let a_radius = a.radius();
        let b_radius = b.radius();
        if a_radius <= P::Scalar::ZERO && b_radius <= P::Scalar::ZERO {
            return SectionKind::Empty;
        }

        let vector = FloatPointMath::sub(&b.point, &a.point);
        let distance_sqr = FloatPointMath::sqr_length(&vector);
        if distance_sqr <= P::Scalar::ZERO {
            return SectionKind::Coincident;
        }

        let radius_delta = a_radius - b_radius;
        if radius_delta * radius_delta >= distance_sqr {
            let direction = FloatPointMath::normalize(&vector);
            return if a_radius >= b_radius {
                SectionKind::Covered(CoveredSection {
                    center: a.point,
                    radius: a_radius,
                    outward: P::from_xy(-direction.x(), -direction.y()),
                })
            } else {
                SectionKind::Covered(CoveredSection {
                    center: b.point,
                    radius: b_radius,
                    outward: direction,
                })
            };
        }

        let distance = distance_sqr.sqrt();
        let direction = FloatPointMath::scale(&vector, P::Scalar::ONE / distance);
        let k = radius_delta / distance;
        let h = (P::Scalar::ONE - k * k).max(P::Scalar::ZERO).sqrt();
        if h <= P::Scalar::ZERO {
            return SectionKind::Empty;
        }

        let normal = P::from_xy(-direction.y(), direction.x());
        let left_normal = P::from_xy(
            k * direction.x() + h * normal.x(),
            k * direction.y() + h * normal.y(),
        );
        let right_normal = P::from_xy(
            k * direction.x() - h * normal.x(),
            k * direction.y() - h * normal.y(),
        );

        let a_left = FloatPointMath::add(&a.point, &FloatPointMath::scale(&left_normal, a_radius));
        let b_left = FloatPointMath::add(&b.point, &FloatPointMath::scale(&left_normal, b_radius));
        let a_right = FloatPointMath::add(&a.point, &FloatPointMath::scale(&right_normal, a_radius));
        let b_right = FloatPointMath::add(&b.point, &FloatPointMath::scale(&right_normal, b_radius));

        SectionKind::Regular(Self {
            a: a.point,
            b: b.point,
            a_radius,
            b_radius,
            a_left,
            b_left,
            a_right,
            b_right,
            direction,
        })
    }

    #[inline]
    fn is_finite(vertex: &StrokeVertex<P>) -> bool {
        vertex.point.x().is_finite() && vertex.point.y().is_finite() && vertex.width.is_finite()
    }
}

#[cfg(test)]
mod tests {
    use super::{Section, SectionKind};
    use crate::mesh::variable_stroke::StrokeVertex;

    #[test]
    fn equal_width_has_parallel_tangents() {
        let a = StrokeVertex::new([0.0, 0.0], 4.0);
        let b = StrokeVertex::new([10.0, 0.0], 4.0);
        let SectionKind::Regular(section) = Section::classify(&a, &b) else {
            panic!("expected regular section");
        };

        assert_eq!(section.a_left, [0.0, 2.0]);
        assert_eq!(section.b_left, [10.0, 2.0]);
        assert_eq!(section.a_right, [0.0, -2.0]);
        assert_eq!(section.b_right, [10.0, -2.0]);
    }

    #[test]
    fn larger_end_covers_smaller_start() {
        let a = StrokeVertex::new([0.0, 0.0], 2.0);
        let b = StrokeVertex::new([2.0, 0.0], 6.0);
        assert!(matches!(Section::classify(&a, &b), SectionKind::Covered(_)));
    }
}
