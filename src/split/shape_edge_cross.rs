use i_float::fix_vec::FixVec;
use i_float::point::Point;
use i_float::triangle::Triangle;
use crate::x_order::XOrder;
use crate::x_segment::XSegment;

#[derive(Debug, Clone, Copy)]
pub struct EdgeCross {
    pub nature: EdgeCrossType,
    pub point: Point,
    pub second: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeCrossType {
    Pure,
    OverlayA,
    Penetrate,
    EndA,
    EndB,
}

impl XSegment {
    pub fn cross(&self, other: &XSegment) -> Option<EdgeCross> {
        let test_y = self.a.y > other.a.y && self.a.y > other.b.y && self.b.y > other.a.y && self.b.y > other.b.y
            || self.a.y < other.a.y && self.a.y < other.b.y && self.b.y < other.a.y && self.b.y < other.b.y;
        if test_y {
            return None;
        }

        let a0 = FixVec::new_point(self.a);
        let a1 = FixVec::new_point(self.b);

        let b0 = FixVec::new_point(other.a);
        let b1 = FixVec::new_point(other.b);

        let a0_area = Triangle::area_two(b0, a0, b1);
        let a1_area = Triangle::area_two(b0, a1, b1);

        if a0_area == 0 && a1_area == 0 {
            // same line
            return Self::same_line_overlay(&self, other);
        }

        let com_a0 = a0 == b0 || a0 == b1;
        let com_a1 = a1 == b0 || a1 == b1;

        let has_same_end = com_a0 || com_a1;

        if has_same_end {
            return None;
        }

        if a0_area == 0 {
            return if other.is_box_contain_point(self.a) {
                Some(EdgeCross { nature: EdgeCrossType::EndA, point: self.a, second: Point::ZERO })
            } else {
                None
            };
        }

        if a1_area == 0 {
            return if other.is_box_contain_point(self.b) {
                Some(EdgeCross { nature: EdgeCrossType::EndA, point: self.b, second: Point::ZERO })
            } else {
                None
            };
        }

        let b0_area = Triangle::area_two(a0, b0, a1);

        if b0_area == 0 {
            return if self.is_box_contain_point(other.a) {
                Some(EdgeCross { nature: EdgeCrossType::EndB, point: other.a, second: Point::ZERO })
            } else {
                None
            };
        }

        let b1_area = Triangle::area_two(a0, b1, a1);

        if b1_area == 0 {
            return if self.is_box_contain_point(other.b) {
                Some(EdgeCross { nature: EdgeCrossType::EndB, point: other.b, second: Point::ZERO })
            } else {
                None
            };
        }

        // areas of triangles must have opposite sign
        let area_a = a0_area > 0 && a1_area < 0 || a0_area < 0 && a1_area > 0;
        let area_b = b0_area > 0 && b1_area < 0 || b0_area < 0 && b1_area > 0;

        if !(area_a && area_b) {
            return None;
        }

        let p = Self::cross_point(a0, a1, b0, b1);

        // still can be common ends cause rounding
        // snap to nearest end with radius 1, (1^2 + 1^2 == 2)

        let ra0 = a0.sqr_distance(p);
        let ra1 = a1.sqr_distance(p);

        let rb0 = b0.sqr_distance(p);
        let rb1 = b1.sqr_distance(p);

        if ra0 <= 2 || ra1 <= 2 || rb0 <= 2 || rb1 <= 2 {
            let ra = ra0.min(ra1);
            let rb = rb0.min(rb1);

            if ra <= rb {
                let a = if ra0 < ra1 { a0 } else { a1 };
                Some(EdgeCross { nature: EdgeCrossType::EndA, point: Point::new_fix_vec(a), second: Point::ZERO })
            } else {
                let b = if rb0 < rb1 { b0 } else { b1 };
                Some(EdgeCross { nature: EdgeCrossType::EndB, point: Point::new_fix_vec(b), second: Point::ZERO })
            }
        } else {
            Some(EdgeCross { nature: EdgeCrossType::Pure, point: Point::new_fix_vec(p), second: Point::ZERO })
        }
    }

    fn cross_point(a0: FixVec, a1: FixVec, b0: FixVec, b1: FixVec) -> FixVec {
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

        FixVec::new(x, y)
    }

    pub fn is_box_contain_point(&self, p: Point) -> bool {
        let x = self.a.x <= p.x && p.x <= self.b.x || self.b.x <= p.x && p.x <= self.a.x;
        let y = self.a.y <= p.y && p.y <= self.b.y || self.b.y <= p.y && p.y <= self.a.y;

        x && y
    }

    pub fn is_box_contain_edge(&self, edge: &XSegment) -> bool {
        let x_contain = self.a.x <= edge.a.x && edge.b.x <= self.b.x;
        if !x_contain {
            return false;
        }

        let (sy_min, sy_max) = if self.a.y <= self.b.y {
            (self.a.y, self.b.y)
        } else {
            (self.b.y, self.a.y)
        };

        let (ey_min, ey_max) = if edge.a.y <= edge.b.y {
            (edge.a.y, edge.b.y)
        } else {
            (edge.b.y, edge.a.y)
        };

        sy_min <= ey_min && ey_max <= sy_max
    }


    fn same_line_overlay(edge_a: &XSegment, edge_b: &XSegment) -> Option<EdgeCross> {
        let is_a = edge_a.is_box_contain_edge(edge_b); // b inside a
        let is_b = edge_b.is_box_contain_edge(edge_a); // a inside b

        if is_a && is_b {
            // edges are equal
            return None;
        }

        if is_b {
            // a inside b
            return Some(edge_b.solve_inside(edge_a, EdgeCrossType::EndA, EdgeCrossType::OverlayA));
        }

        let has_same_end = edge_a.a == edge_b.a || edge_a.a == edge_b.b || edge_a.b == edge_b.a || edge_a.b == edge_b.b;

        if has_same_end {
            return None;
        }

        // debug_assert!(!is_a && !is_b);
        if !is_a && !is_b {
            print!("catch")
        }

        // penetrate

        let ap = if edge_a.is_box_contain_point(edge_b.a) { edge_b.a } else { edge_b.b };
        let bp = if edge_b.is_box_contain_point(edge_a.a) { edge_a.a } else { edge_a.b };

        if Point::order_by_line_compare(ap, bp) {
            Some(EdgeCross { nature: EdgeCrossType::Penetrate, point: ap, second: bp })
        } else {
            Some(EdgeCross { nature: EdgeCrossType::Penetrate, point: bp, second: ap })
        }
    }

    fn solve_inside(&self, other: &XSegment, end: EdgeCrossType, overlay: EdgeCrossType) -> EdgeCross {
        let is_be0 = other.a == self.a || other.a == self.b;
        let is_be1 = other.b == self.a || other.b == self.b;

        return if is_be0 {
            // first point is common
            EdgeCross { nature: end, point: other.b, second: Point::ZERO }
        } else if is_be1 {
            // second point is common
            EdgeCross { nature: end, point: other.a, second: Point::ZERO }
        } else {
            // no common points
            EdgeCross { nature: overlay, point: Point::ZERO, second: Point::ZERO }
        };
    }
}