use super::arc::ArcDirection;
use super::math::backend::{ArcMath, MeshMath};
use super::math::integer::IntegerMath;
use super::math::{abs, mul_div, point, scaled_point, vector};
use super::style::IntLineJoin;
use crate::mesh::subject::SubjectSegments;
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::angle::Angle;
use i_float::int::number::{int::IntNumber, wide_int::WideIntNumber};
use i_float::int::{point::IntPoint, unit_vector::UnitIntVector};

pub(super) enum Join<I: IntNumber, M: MeshMath<I> = IntegerMath> {
    Bevel,
    Miter { minimum: u32, sin: i32, cos: i32 },
    Round(M::Arc),
}

impl<I: IntNumber, M: MeshMath<I>> Join<I, M> {
    pub(super) fn new(style: IntLineJoin) -> Self {
        match style {
            IntLineJoin::Bevel => Self::Bevel,
            IntLineJoin::Round(options) => Self::Round(M::Arc::new(options)),
            IntLineJoin::Miter(angle) => {
                let minimum = angle
                    .bits()
                    .clamp((1u32 << 31) / 100, ((1u64 << 31) * 99 / 100) as u32);
                let (sin, cos) = M::sin_cos(Angle::from_bits(minimum / 2));
                Self::Miter { minimum, sin, cos }
            }
        }
    }

    pub(super) fn padding(style: IntLineJoin, radius: I) -> I::Wide {
        let radius = abs(radius);
        if radius == I::Wide::ZERO {
            return radius;
        }
        match style {
            IntLineJoin::Miter(_) => {
                let Self::Miter { sin, .. } = Self::new(style) else {
                    unreachable!()
                };
                mul_div::<I>(radius, I::Wide::from_u32(1 << 30), I::Wide::from_u32(sin as u32)) + I::Wide::TWO
            }
            _ => radius,
        }
    }

    /// Joins the two outward offset rays, in boundary traversal order.
    pub(super) fn add(
        &mut self,
        center: IntPoint<I>,
        a: IntPoint<I>,
        b: IntPoint<I>,
        incoming: UnitIntVector<I>,
        outgoing: UnitIntVector<I>,
        radius: I,
        direction: ArcDirection,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        if a == b {
            return;
        }
        match self {
            Self::Bevel => segments.push_non_degenerate(a, b),
            Self::Round(arc) => {
                let Some(from) = M::normalize(a - center) else {
                    segments.push_non_degenerate(a, b);
                    return;
                };
                let Some(to) = M::normalize(b - center) else {
                    segments.push_non_degenerate(a, b);
                    return;
                };
                let mut previous = a;
                for &direction in arc.build(from, to, direction) {
                    let next = scaled_point(center, direction, radius);
                    segments.push_non_degenerate(previous, next);
                    previous = next;
                }
                segments.push_non_degenerate(previous, b);
            }
            Self::Miter { minimum, sin, cos } => {
                let va = vector(incoming);
                let vb = vector(outgoing);
                let cross = va.cross_product(vb);
                if cross == I::Wide::ZERO {
                    segments.push_non_degenerate(a, b);
                    return;
                }
                let turn = M::angle_between(incoming, outgoing).bits();
                let turn = turn.min(turn.wrapping_neg());
                if (1u32 << 31) - turn < *minimum {
                    let extension = mul_div::<I>(
                        abs(radius),
                        I::Wide::from_u32(*cos as u32),
                        I::Wide::from_u32(*sin as u32),
                    );
                    let scale = UnitIntVector::<I>::DENOMINATOR;
                    let ac = point(
                        a,
                        mul_div::<I>(va.x, extension, scale),
                        mul_div::<I>(va.y, extension, scale),
                    );
                    let bc = point(
                        b,
                        -mul_div::<I>(vb.x, extension, scale),
                        -mul_div::<I>(vb.y, extension, scale),
                    );
                    segments.push_non_degenerate(a, ac);
                    segments.push_non_degenerate(ac, bc);
                    segments.push_non_degenerate(bc, b);
                } else {
                    let numerator = (b - a).cross_product(vb);
                    let peak = point(
                        a,
                        mul_div::<I>(va.x, numerator, cross),
                        mul_div::<I>(va.y, numerator, cross),
                    );
                    segments.push_non_degenerate(a, peak);
                    segments.push_non_degenerate(peak, b);
                }
            }
        }
    }
}
