use super::style::IntStrokeVertex;
use crate::mesh::int::math::{mul_div, point};
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
    pub(super) fn try_new(a: &IntStrokeVertex<I>, b: &IntStrokeVertex<I>) -> Option<Self> {
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
        // Keep fractional bits of sqrt(length² - delta²). Taking an unscaled
        // integer root would noticeably shrink wide strokes on short sections.
        // Using sq to choose the shift also bounds every numerator by sq << shift.
        let shift = (sq.leading_zeros() - 1) / 2;
        let tangent = I::Wide::from_uint(((sq - (delta * delta).to_uint()) << (2 * shift)).isqrt());
        let denominator = I::Wide::from_uint(sq << shift);
        let dx = (delta * v.x) << shift;
        let dy = (delta * v.y) << shift;
        let left_x = dx - tangent * v.y;
        let left_y = dy + tangent * v.x;
        let right_x = dx + tangent * v.y;
        let right_y = dy - tangent * v.x;
        let contact = |p, r: I, x, y| {
            point(
                p,
                mul_div::<I>(r.to_wide(), x, denominator),
                mul_div::<I>(r.to_wide(), y, denominator),
            )
        };
        Some(Self {
            a: a.point,
            b: b.point,
            a_left: contact(a.point, ra, left_x, left_y),
            a_right: contact(a.point, ra, right_x, right_y),
            b_left: contact(b.point, rb, left_x, left_y),
            b_right: contact(b.point, rb, right_x, right_y),
            radius_trend: if ra < rb {
                RadiusTrend::Increasing
            } else if ra > rb {
                RadiusTrend::Decreasing
            } else {
                RadiusTrend::Constant
            },
        })
    }
}
