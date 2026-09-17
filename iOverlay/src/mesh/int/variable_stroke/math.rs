use crate::mesh::int::math::{backend::MeshMath, float::FloatMath, integer::IntegerMath, mul_div};
use i_float::float::number::FloatNumber;
use i_float::int::{
    number::{int::IntNumber, uint::UIntNumber, wide_int::WideIntNumber},
    vector::IntVector,
};

/// Construction of the external tangent contacts of two non-containing circles.
pub(super) trait VariableStrokeMath<I: IntNumber>: MeshMath<I> {
    /// Offsets from the centers, ordered as a-left, a-right, b-left, b-right.
    /// The caller guarantees |ra - rb|² < |v|².
    fn tangent_offsets(v: IntVector<I>, ra: I, rb: I) -> [IntVector<I>; 4];
}

impl<I: IntNumber> VariableStrokeMath<I> for IntegerMath {
    fn tangent_offsets(v: IntVector<I>, ra: I, rb: I) -> [IntVector<I>; 4] {
        let delta = ra.to_wide() - rb.to_wide();
        let sq = v.sqr_length();
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
        let contact = |r: I, x, y| {
            IntVector::new(
                mul_div::<I>(r.to_wide(), x, denominator),
                mul_div::<I>(r.to_wide(), y, denominator),
            )
        };
        [
            contact(ra, left_x, left_y),
            contact(ra, right_x, right_y),
            contact(rb, left_x, left_y),
            contact(rb, right_x, right_y),
        ]
    }
}

impl<I: IntNumber> VariableStrokeMath<I> for FloatMath {
    fn tangent_offsets(v: IntVector<I>, ra: I, rb: I) -> [IntVector<I>; 4] {
        let delta = ra.to_wide() - rb.to_wide();
        let sq = v.sqr_length();
        // Subtract before converting to float: nearly contained circles can
        // have a tiny positive difference even at large integer coordinates.
        let tangent = FloatNumber::sqrt((sq - (delta * delta).to_uint()).to_f64());
        let denominator = sq.to_f64();
        let dx = (delta * v.x).to_f64();
        let dy = (delta * v.y).to_f64();
        let tx = tangent * v.x.to_f64();
        let ty = tangent * v.y.to_f64();
        let contact = |r: I, x: f64, y: f64| {
            let scale = r.to_f64() / denominator;
            IntVector::new(
                I::Wide::from_rounded_float(scale * x),
                I::Wide::from_rounded_float(scale * y),
            )
        };
        [
            contact(ra, dx - ty, dy + tx),
            contact(ra, dx + ty, dy - tx),
            contact(rb, dx - ty, dy + tx),
            contact(rb, dx + ty, dy - tx),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_tangents_keep_the_small_difference_near_containment() {
        let d = 1_i64 << 59;
        let [al, ar, _, _] = FloatMath::tangent_offsets(IntVector::<i64>::new(d as i128, 1), d + 2, 2);
        assert_eq!(al.y, 2);
        assert_eq!(ar.y, 0);
    }

    #[test]
    fn both_backends_preserve_short_section_contact_precision() {
        fn check<M: VariableStrokeMath<i32>>() {
            for (x, y) in [(1, 1), (2, 2), (3, 2), (1, 7)] {
                for delta in [0, 1] {
                    let offsets = M::tangent_offsets(IntVector::new(x, y), 4096, 4096 - delta);
                    let sq = (x * x + y * y) as f64;
                    let tangent = (sq - (delta * delta) as f64).sqrt();
                    for (i, p) in offsets.iter().enumerate() {
                        let radius = if i < 2 { 4096.0 } else { (4096 - delta) as f64 };
                        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                        let ex = radius * (delta as f64 * x as f64 - sign * tangent * y as f64) / sq;
                        let ey = radius * (delta as f64 * y as f64 + sign * tangent * x as f64) / sq;
                        assert!((p.x as f64 - ex).abs() <= 0.51);
                        assert!((p.y as f64 - ey).abs() <= 0.51);
                    }
                }
            }
        }
        check::<IntegerMath>();
        check::<FloatMath>();
    }
}
