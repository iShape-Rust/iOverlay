use i_float::fix_vec::FixVec;
use i_float::point::IntPoint;
use i_float::triangle::Triangle;
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
    pub(super) fn is_valid_scan(scan: &XSegment, this: &XSegment) -> bool {
        let is_outdated = scan.b < this.a;
        let is_behind = scan.is_less(this);

        !is_outdated && is_behind
    }

    #[cfg(debug_assertions)]
    fn validate_x(target: &XSegment, other: &XSegment) {
        let test_x = target.a.x > other.a.x && target.a.x > other.b.x
            && target.b.x > other.a.x && target.b.x > other.b.x
            || target.a.x < other.a.x && target.a.x < other.b.x
            && target.b.x < other.a.x && target.b.x < other.b.x;

        debug_assert!(!test_x);
    }

    pub fn scan_cross(target: &XSegment, other: &XSegment) -> Option<CrossResult> {
        // by this time segments already at intersection range by x
        #[cfg(debug_assertions)]
        ScanCrossSolver::validate_x(target, other);

        let test_y = target.a.y > other.a.y && target.a.y > other.b.y
            && target.b.y > other.a.y && target.b.y > other.b.y
            || target.a.y < other.a.y && target.a.y < other.b.y
            && target.b.y < other.a.y && target.b.y < other.b.y;

        if test_y {
            return None;
        }

        let is_end0 = target.a == other.a || target.a == other.b;
        let is_end1 = target.b == other.a || target.b == other.b;

        let a0 = FixVec::new_point(target.a);
        let b0 = FixVec::new_point(target.b);

        let a1 = FixVec::new_point(other.a);
        let b1 = FixVec::new_point(other.b);


        let a0b0a1 = Triangle::clock_direction(a0, b0, a1);
        let a0b0b1 = Triangle::clock_direction(a0, b0, b1);

        let a1b1a0 = Triangle::clock_direction(a1, b1, a0);
        let a1b1b0 = Triangle::clock_direction(a1, b1, b0);

        let is_collinear = a0b0a1 == 0 && a0b0b1 == 0 && a1b1a0 == 0 && a1b1b0 == 0;

        if (is_end0 || is_end1) && is_collinear {
            let dot_product = if is_end0 {
                let p = if a0 == a1 { b1 } else { a1 };
                (a0 - b0).dot_product(a0 - p)
            } else {
                let p = if b0 == a1 { b1 } else { a1 };
                (b0 - a0).dot_product(b0 - p)
            };
            return if dot_product < 0 {
                // only one common end
                None
            } else {
                Some(CrossResult::EndOverlap)
            };
        } else if is_collinear {
            return Some(CrossResult::Overlap);
        } else if is_end0 || is_end1 {
            debug_assert!(!(is_end0 && is_end1));
            return None;
        }

        let not_same0 = a0b0a1 != a0b0b1;
        let not_same1 = a1b1a0 != a1b1b0;

        if !(not_same0 && not_same1) {
            return None;
        }

        if a0b0a1 == 0 || a0b0b1 == 0 || a1b1a0 == 0 || a1b1b0 == 0 {
            // one end is on the other edge
            return if a0b0a1 == 0 {
                Some(CrossResult::OtherEndExact(other.a))
            } else if a0b0b1 == 0 {
                Some(CrossResult::OtherEndExact(other.b))
            } else if a1b1a0 == 0 {
                Some(CrossResult::TargetEndExact(target.a))
            } else {
                Some(CrossResult::TargetEndExact(target.b))
            };
        }

        let p = ScanCrossSolver::cross_point(a0, b0, a1, b1);

        if Triangle::is_line_point(target.a, p, target.b) && Triangle::is_line_point(other.a, p, other.b) {
            return Some(CrossResult::PureExact(p));
        }

        // let p = Self::snap_to_best(p, target, other);

        // still can be common ends because of rounding

        if p == target.a || p == target.b {
            Some(CrossResult::TargetEndRound(p))
        } else if p == other.a || p == other.b {
            Some(CrossResult::OtherEndRound(p))
        } else {
            Some(CrossResult::PureRound(p))
        }
    }

    fn cross_point(a0: FixVec, a1: FixVec, b0: FixVec, b1: FixVec) -> IntPoint {
        // edges are not parallel
        // FixVec(Int64, Int64) where abs(x) and abs(y) < 2^30
        // So the result must be also be in range of 2^30

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
        let a0x = a0.x;
        let a0y = a0.y;

        let a1x = a1.x - a0x;
        let b0x = b0.x - a0x;
        let b1x = b1.x - a0x;

        // move a0.y to 0
        // move all by a0.y
        let a1y = a1.y - a0y;
        let b0y = b0.y - a0y;
        let b1y = b1.y - a0y;

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
            // multiply denominator and discriminant by same value to increase precision
            let a1x_128 = a1x as i128;
            let a1y_128 = a1y as i128;
            let kx = a1x_128 * xy_b as i128;
            let ky = a1y_128 * xy_b as i128;

            let divider = a1y_128 * dx_b as i128 - a1x_128 * dy_b as i128;

            x0 = (kx / divider) as i64;
            y0 = (ky / divider) as i64;
        }

        let x = x0 + a0x;
        let y = y0 + a0y;

        IntPoint::new(x as i32, y as i32)
    }

    fn snap_to_best(p: IntPoint, seg0: &XSegment, seg1: &XSegment) -> IntPoint {
        if p.x != 0 && p.y != 0 {
            let dx = p.x.signum();
            let dy = p.y.signum();

            let points = [
                p,
                IntPoint::new(p.x, p.y + dy),
                IntPoint::new(p.x + dx, p.y),
                IntPoint::new(p.x + dx, p.y + dy)
            ];
            Self::snap_to_points(&points, seg0, seg1)
        } else if p.x == 0 && p.y == 0 {
            let points = [
                IntPoint::new(p.x + 1, p.y - 1),
                IntPoint::new(p.x + 1, p.y),
                IntPoint::new(p.x + 1, p.y + 1),
                IntPoint::new(p.x - 1, p.y - 1),
                IntPoint::new(p.x - 1, p.y),
                IntPoint::new(p.x - 1, p.y + 1),
                IntPoint::new(p.x, p.y - 1),
                p,
                IntPoint::new(p.x, p.y + 1)
            ];
            Self::snap_to_points(&points, seg0, seg1)
        } else if p.x == 0 {
            let dy = p.y.signum();
            let points = [
                IntPoint::new(p.x + 1, p.y + dy),
                IntPoint::new(p.x - 1, p.y + dy),
                IntPoint::new(p.x + 1, p.y),
                IntPoint::new(p.x - 1, p.y),
                IntPoint::new(p.x, p.y + dy),
                p
            ];
            Self::snap_to_points(&points, seg0, seg1)
        } else {
            let dx = p.x.signum();
            let points = [
                IntPoint::new(p.x + dx, p.y + 1),
                IntPoint::new(p.x + dx, p.y - 1),
                IntPoint::new(p.x, p.y + 1),
                IntPoint::new(p.x, p.y - 1),
                IntPoint::new(p.x + dx, p.y),
                p
            ];
            Self::snap_to_points(&points, seg0, seg1)
        }
    }

    fn snap_to_points(points: &[IntPoint], seg0: &XSegment, seg1: &XSegment) -> IntPoint {
        let mut best = AreaPoint { area: i64::MAX, point: IntPoint::ZERO };
        for &p in points {
            let s = Self::area(p, seg0, seg1);
            if s < best.area {
                best.area = s;
                best.point = p;
            }
        }
        return best.point;
    }

    fn area(point: IntPoint, seg0: &XSegment, seg1: &XSegment) -> i64 {
        let s0 = Triangle::area_two_point(point, seg0.a, seg0.b);
        let s1 = Triangle::area_two_point(point, seg1.a, seg1.b);
        s0 + s1
    }
}

struct AreaPoint {
    area: i64,
    point: IntPoint,
}