use std::f64::consts::PI;
use i_overlay::i_float::float::point::FloatPoint;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::base::data::Contour;
use i_overlay::i_shape::int::path::IntPath;

pub(super) struct Util;

impl Util {

    pub(super) fn many_squares(start: IntPoint, size: i32, offset: i32, n: usize) -> Vec<IntPath> {
        let mut result = Vec::with_capacity(n * n);
        let mut y = start.y;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let path: IntPath = vec![
                    IntPoint::new(x, y),
                    IntPoint::new(x, y + size),
                    IntPoint::new(x + size, y + size),
                    IntPoint::new(x + size, y),
                ];
                result.push(path);
                x += offset;
            }
            y += offset;
        }

        result
    }


    pub(super) fn many_windows(start: IntPoint, a: i32, b: i32, offset: i32, n: usize) -> (Vec<IntPath>, Vec<IntPath>) {
        let mut boundaries = Vec::with_capacity(n * n);
        let mut holes = Vec::with_capacity(n * n);
        let mut y = start.y;
        let c = (a - b) / 2;
        let d = b + c;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let boundary: IntPath = vec![
                    IntPoint::new(x, y),
                    IntPoint::new(x, y + a),
                    IntPoint::new(x + a, y + a),
                    IntPoint::new(x + a, y),
                ];
                boundaries.push(boundary);

                let hole: IntPath = vec![
                    IntPoint::new(x + c, y + c),
                    IntPoint::new(x + c, y + d),
                    IntPoint::new(x + d, y + d),
                    IntPoint::new(x + d, y + c),
                ];
                holes.push(hole);

                x += offset;
            }
            y += offset;
        }

        (boundaries, holes)
    }

    pub(super) fn concentric_squares(a: i32, n: usize) -> (Vec<IntPath>, Vec<IntPath>) {
        let mut vert = Vec::with_capacity(2 * n);
        let mut horz = Vec::with_capacity(2 * n);
        let s = 2 * a;
        let mut r = s;
        for _ in 0..n {
            let hz_top: IntPath = vec![
                IntPoint::new(-r, r - a),
                IntPoint::new(-r, r),
                IntPoint::new(r, r),
                IntPoint::new(r, r - a),
            ];
            let hz_bot: IntPath = vec![
                IntPoint::new(-r, -r),
                IntPoint::new(-r, -r + a),
                IntPoint::new(r, -r + a),
                IntPoint::new(r, -r),
            ];
            horz.push(hz_top);
            horz.push(hz_bot);

            let vt_left: IntPath = vec![
                IntPoint::new(-r, -r),
                IntPoint::new(-r, r),
                IntPoint::new(-r + a, r),
                IntPoint::new(-r + a, -r),
            ];
            let vt_right: IntPath = vec![
                IntPoint::new(r - a, -r),
                IntPoint::new(r - a, r),
                IntPoint::new(r, r),
                IntPoint::new(r, -r),
            ];
            vert.push(vt_left);
            vert.push(vt_right);

            r += s;
        }

        (vert, horz)
    }

    pub(super) fn many_lines_x(a: i32, n: usize) -> Vec<IntPath> {
        let w = a / 2;
        let s = a * (n as i32) / 2;
        let mut x = -s + w / 2;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let path: IntPath = vec![
                IntPoint::new(x, -s),
                IntPoint::new(x, s),
                IntPoint::new(x + w, s),
                IntPoint::new(x + w, -s),
            ];
            result.push(path);
            x += a;
        }

        result
    }

    pub(super) fn many_lines_y(a: i32, n: usize) -> Vec<IntPath> {
        let h = a / 2;
        let s = a * (n as i32) / 2;
        let mut y = -s + h / 2;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let path: IntPath = vec![
                IntPoint::new(-s, y),
                IntPoint::new(s, y),
                IntPoint::new(s, y - h),
                IntPoint::new(-s, y - h),
            ];
            result.push(path);
            y += a;
        }

        result
    }

    pub(super) fn spiral(count: usize, radius: f64) -> Contour<FloatPoint<f64>> {
        let mut a_path = Vec::with_capacity(4 * count);
        let mut b_path = Vec::with_capacity(2 * count);

        let mut a: f64 = 0.0;
        let mut r = radius;
        let w = 0.1 * radius;

        let c0 = FloatPoint { x: 0.0, y: 0.0 };
        let mut p0 = c0;

        for i in 0..count {
            let (sy, sx) = a.sin_cos();

            let rr = if i % 2 == 0 {
                r + 0.2 * radius
            } else {
                r - 0.2 * radius
            };

            let p = FloatPoint { x: rr * sx, y: rr * sy };
            let n = (p - p0).normalize();
            let t = FloatPoint { x: w * -n.y, y: w * n.x };

            a_path.push(p0 + t);
            a_path.push(p + t);
            b_path.push(p0 - t);
            b_path.push(p - t);

            a += radius / r;
            r = radius * (1.0 + a / (2.0 * PI));
            p0 = p;
        }

        b_path.reverse();
        a_path.append(&mut b_path);

        a_path
    }
}