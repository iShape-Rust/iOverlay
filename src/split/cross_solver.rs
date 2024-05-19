use i_float::point::IntPoint;
use i_float::triangle::Triangle;
use i_float::u128::UInt128;
use crate::x_segment::XSegment;

pub enum CrossResult {
    PureExact(IntPoint),
    PureRound(IntPoint),
    EndOverlap,
    Overlap,
    TargetEndExact(IntPoint),
    TargetEndRound(IntPoint),
    OtherEndExact(IntPoint),
    OtherEndRound(IntPoint),
}

pub struct ScanCrossSolver;

impl ScanCrossSolver {
    #[inline(always)]
    pub(super) fn is_valid_scan(scan: &XSegment, this: &XSegment) -> bool {
        scan.b >= this.a && scan < this
    }

    #[cfg(debug_assertions)]
    pub fn test_x(target: &XSegment, other: &XSegment) -> bool {
        target.a.x > other.a.x && target.a.x > other.b.x
            && target.b.x > other.a.x && target.b.x > other.b.x
            || target.a.x < other.a.x && target.a.x < other.b.x
            && target.b.x < other.a.x && target.b.x < other.b.x
    }

    #[inline(always)]
    pub fn test_y(target: &XSegment, other: &XSegment) -> bool {
        target.a.y > other.a.y && target.a.y > other.b.y
            && target.b.y > other.a.y && target.b.y > other.b.y
            || target.a.y < other.a.y && target.a.y < other.b.y
            && target.b.y < other.a.y && target.b.y < other.b.y
    }

    pub fn cross(target: &XSegment, other: &XSegment) -> Option<CrossResult> {
        // by this time segments already at intersection range by x
        #[cfg(debug_assertions)]
        debug_assert!(!ScanCrossSolver::test_x(target, other));

        #[cfg(debug_assertions)]
        debug_assert!(!ScanCrossSolver::test_y(target, other));

        let a0b0a1 = Triangle::clock_direction_point(target.a, target.b, other.a);
        let a0b0b1 = Triangle::clock_direction_point(target.a, target.b, other.b);

        let a1b1a0 = Triangle::clock_direction_point(other.a, other.b, target.a);
        let a1b1b0 = Triangle::clock_direction_point(other.a, other.b, target.b);

        let s = (1 & (a0b0a1 + 1)) + (1 & (a0b0b1 + 1)) + (1 & (a1b1a0 + 1)) + (1 & (a1b1b0 + 1));

        let is_not_cross = a0b0a1 == a0b0b1 || a1b1a0 == a1b1b0;

        if s == 2 || (is_not_cross && s != 4) {
            return None;
        }

        if s != 0 {
            // special case
            return if s == 4 {
                // collinear

                let aa = target.a == other.a;
                let ab = target.a == other.b;
                let ba = target.b == other.a;
                let bb = target.b == other.b;

                let is_end0 = aa || ab;
                let is_end1 = ba || bb;

                if is_end0 || is_end1 {
                    let p = if aa || ba { other.b } else { other.a };
                    let v0 = target.a.subtract(p);
                    let v1 = if is_end0 {
                        target.a.subtract(target.b)
                    } else {
                        target.b.subtract(target.a)
                    };
                    let dot_product = v1.dot_product(v0);
                    if dot_product >= 0 {
                        return Some(CrossResult::EndOverlap);
                    } else {
                        // end to end connection
                        None
                    }
                } else {
                    Some(CrossResult::Overlap)
                }
            } else {
                if a0b0a1 == 0 {
                    Some(CrossResult::OtherEndExact(other.a))
                } else if a0b0b1 == 0 {
                    Some(CrossResult::OtherEndExact(other.b))
                } else if a1b1a0 == 0 {
                    Some(CrossResult::TargetEndExact(target.a))
                } else {
                    Some(CrossResult::TargetEndExact(target.b))
                }
            };
        }

        let p = ScanCrossSolver::cross_point(&target, &other);

        if Triangle::is_line_point(target.a, p, target.b) && Triangle::is_line_point(other.a, p, other.b) {
            return Some(CrossResult::PureExact(p));
        }

        // still can be common ends because of rounding
        // snap to nearest end with r (1^2 + 1^2 == 2)

        let ra0 = target.a.sqr_distance(p);
        let rb0 = target.b.sqr_distance(p);

        let ra1 = other.a.sqr_distance(p);
        let rb1 = other.b.sqr_distance(p);

        if ra0 <= 2 || ra1 <= 2 || rb0 <= 2 || rb1 <= 2 {
            let r0 = ra0.min(rb0);
            let r1 = ra1.min(rb1);

            if r0 <= r1 {
                let p = if ra0 < rb0 { target.a } else { target.b };
                // ignore if it's a clean point
                if Triangle::is_not_line_point(other.a, p, other.b) {
                    return Some(CrossResult::TargetEndRound(p));
                }
            } else {
                let p = if ra1 < rb1 { other.a } else { other.b };

                // ignore if it's a clean point
                if Triangle::is_not_line_point(target.a, p, target.b) {
                    return Some(CrossResult::OtherEndRound(p));
                }
            }
        }

        Some(CrossResult::PureRound(p))
    }

    fn cross_point(target: &XSegment, other: &XSegment) -> IntPoint {
        // edges are not parallel
        // any abs(x) and abs(y) < 2^30
        // The result must be < 2^30

        // Classic approach:

        // let dxA = a0.x - a1.x
        // let dyB = b0.y - b1.y
        // let dyA = a0.y - a1.y
        // let dxB = b0.x - b1.x
        //
        // let xyA = a0.x * a1.y - a0.y * a1.x
        // let xyB = b0.x * b1.y - b0.y * b1.x
        //
        // overflow is possible!
        // let kx = xyA * dxB - dxA * xyB
        //
        // overflow is possible!
        // let ky = xyA * dyB - dyA * xyB
        //
        // let divider = dxA * dyB - dyA * dxB
        //
        // let x = kx / divider
        // let y = ky / divider
        //
        // return FixVec(x, y)

        // offset approach
        // move all picture by -a0. Point a0 will be equal (0, 0)

        // move a0.x to 0
        // move all by a0.x
        let a0x = target.a.x as i64;
        let a0y = target.a.y as i64;

        let a1x = target.b.x as i64 - a0x;
        let b0x = other.a.x as i64 - a0x;
        let b1x = other.b.x as i64 - a0x;

        // move a0.y to 0
        // move all by a0.y
        let a1y = target.b.y as i64 - a0y;
        let b0y = other.a.y as i64 - a0y;
        let b1y = other.b.y as i64 - a0y;

        let dy_b = b0y - b1y;
        let dx_b = b0x - b1x;

        // let xyA = 0
        let xy_b = b0x * b1y - b0y * b1x;

        let x0: i64;
        let y0: i64;

        // a1y and a1x cannot be zero simultaneously, because we will get edge a0<>a1 zero length and it is impossible

        if a1x == 0 {
            // dxB is not zero because it will be parallel case and it's impossible
            x0 = 0;
            y0 = xy_b / dx_b;
        } else if a1y == 0 {
            // dyB is not zero because it will be parallel case and it's impossible
            y0 = 0;
            x0 = -xy_b / dy_b;
        } else {
            // divider
            let div = a1y * dx_b - a1x * dy_b;

            // calculate result sign
            let s = div.signum() * xy_b.signum();
            let sx = a1x.signum() * s;
            let sy = a1y.signum() * s;

            // use custom u128 bit math with rounding
            let uxy_b = xy_b.abs() as u64;
            let udiv = div.abs() as u64;

            let kx = UInt128::multiply(a1x.abs() as u64, uxy_b);
            let ky = UInt128::multiply(a1y.abs() as u64, uxy_b);

            let ux = kx.divide_with_rounding(udiv);
            let uy = ky.divide_with_rounding(udiv);

            // get i64 bit result
            x0 = sx * ux as i64;
            y0 = sy * uy as i64;
        }

        let x = (x0 + a0x) as i32;
        let y = (y0 + a0y) as i32;

        IntPoint::new(x, y)
    }
}

const LAST_BIT_INDEX: usize = 63;

trait RoundDivide {
    fn divide_with_rounding(&self, divisor: u64) -> u64;
}

impl RoundDivide for UInt128 {
    fn divide_with_rounding(&self, divisor: u64) -> u64 {
        if self.high == 0 {
            let result = self.low / divisor;
            let remainder = self.low - result * divisor;
            return if remainder >= (divisor + 1) >> 1 {
                result + 1
            } else {
                result
            };
        }

        let dn = divisor.leading_zeros();
        let norm_divisor = divisor << dn;
        let mut norm_dividend_high = self.high << dn | self.low >> (u64::BITS - dn);
        let mut norm_dividend_low = self.low << dn;

        let mut quotient = 0;
        let one = 1 << LAST_BIT_INDEX;

        for _ in 0..u64::BITS {
            let bit = (norm_dividend_high & one) != 0;
            norm_dividend_high = (norm_dividend_high << 1) | (norm_dividend_low >> LAST_BIT_INDEX);
            norm_dividend_low <<= 1;
            quotient <<= 1;
            if norm_dividend_high >= norm_divisor || bit {
                norm_dividend_high = norm_dividend_high.wrapping_sub(norm_divisor);
                quotient |= 1;
            }
        }

        // Check remainder for rounding
        let remainder = (norm_dividend_high << (u64::BITS - dn)) | (norm_dividend_low >> dn);
        if remainder >= (divisor + 1) >> 1 {
            quotient += 1;
        }

        quotient
    }
}