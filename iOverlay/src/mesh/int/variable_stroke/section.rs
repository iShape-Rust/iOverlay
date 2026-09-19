use super::math::VariableStrokeMath;
use super::style::IntStrokeVertex;
use crate::mesh::int::math::point;
use i_float::int::{
    number::{int::IntNumber, uint::UIntNumber, wide_int::WideIntNumber},
    point::IntPoint,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RadiusTrend {
    Decreasing,
    Constant,
    Increasing,
}
#[derive(Clone, Copy)]
pub(super) struct Section<I: IntNumber> {
    pub(super) a: IntPoint<I>,
    pub(super) b: IntPoint<I>,
    pub(super) a_left: IntPoint<I>,
    pub(super) a_right: IntPoint<I>,
    pub(super) b_left: IntPoint<I>,
    pub(super) b_right: IntPoint<I>,
    pub(super) radius_trend: RadiusTrend,
}
impl<I: IntNumber> Section<I> {
    pub(super) fn try_new<M: VariableStrokeMath<I>>(
        a: &IntStrokeVertex<I>,
        b: &IntStrokeVertex<I>,
    ) -> Option<Self> {
        let ra = a.radius();
        let rb = b.radius();
        if a.point == b.point || ra.max(rb) <= I::ONE {
            return None;
        }
        let delta = ra.to_wide() - rb.to_wide();
        let v = b.point - a.point;
        let sq = v.sqr_length();
        if (delta * delta).to_uint() >= sq {
            return None;
        }
        let [al, ar, bl, br] = M::tangent_offsets(v, ra, rb);
        Some(Self {
            a: a.point,
            b: b.point,
            a_left: point(a.point, al.x, al.y),
            a_right: point(a.point, ar.x, ar.y),
            b_left: point(b.point, bl.x, bl.y),
            b_right: point(b.point, br.x, br.y),
            radius_trend: if ra < rb {
                RadiusTrend::Increasing
            } else if ra > rb {
                RadiusTrend::Decreasing
            } else {
                RadiusTrend::Constant
            },
        })
    }
    pub(super) fn covers_circle(&self, center: IntPoint<I>, radius: I) -> bool {
        let points = [self.a_left, self.b_left, self.b_right, self.a_right];
        let radius = radius.to_wide().to_uint();
        let first_edge = points[1] - points[0];
        let orientation = first_edge.cross_product(points[2] - points[1]);
        if orientation == I::Wide::ZERO {
            return false;
        }

        for index in 0..points.len() {
            let a = points[index];
            let b = points[(index + 1) % points.len()];
            let edge = b - a;
            let side = edge.cross_product(center - a);
            let interior_distance = if orientation > I::Wide::ZERO { side } else { -side };
            if interior_distance < I::Wide::ZERO {
                return false;
            }

            let length_sqr = edge.sqr_length();
            let mut length = length_sqr.isqrt();
            if length * length < length_sqr {
                length += I::WideUInt::ONE;
            }
            if interior_distance.to_uint() < radius * length {
                return false;
            }
        }

        true
    }
}
