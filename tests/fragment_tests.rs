#[cfg(test)]
mod tests {
    use i_overlay::float::overlay::FloatOverlay;
    use i_overlay::float::filter::ContourFilter;
    use std::f64::consts::PI;
    use i_shape::base::data::Contour;
    use i_float::int::point::IntPoint;
    use i_shape::int::path::IntPath;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::Overlay;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;

    #[test]
    fn test_many_squares() {
        let fill = FillRule::NonZero;
        let rule = OverlayRule::Xor;

        for n in [25usize, 50, 100] {
            let subj_paths = many_squares(IntPoint::new(0, 0), 20, 30, n);
            let clip_paths = many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

            let list_result = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, fill, 0, Solver::LIST);

            let tree_result = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, fill, 0, Solver::TREE);

            let frag_result = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, fill, 0, Solver::FRAG);

            assert_eq!(list_result, tree_result);
            assert_eq!(list_result, frag_result);
        }
    }

    #[test]
    fn test_many_lines() {
        let fill = FillRule::NonZero;
        let rule = OverlayRule::Intersect;

        for n in [25usize, 50, 100] {
            let subj_paths = many_lines_x(20, n);
            let clip_paths = many_lines_y(20, n);

            let list_result = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, fill, 0, Solver::LIST);

            let tree_result = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, fill, 0, Solver::TREE);

            let frag_result = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, fill, 0, Solver::FRAG);

            assert_eq!(list_result, tree_result);
            assert_eq!(list_result, frag_result);
        }
    }

    #[test]
    fn test_spiral() {
        let fill = FillRule::NonZero;
        let rule = OverlayRule::Subject;
        let filter = ContourFilter { min_area: 0.0, simplify: true };

        for n in [25usize, 50, 100] {
            let subj_path = spiral(n, 100.0);

            let list_result = FloatOverlay::with_subj(&subj_path)
                .overlay_with_filter_and_solver(rule, fill, filter, Default::default());

            let tree_result = FloatOverlay::with_subj(&subj_path)
                .overlay_with_filter_and_solver(rule, fill, filter, Default::default());

            let frag_result = FloatOverlay::with_subj(&subj_path)
                .overlay_with_filter_and_solver(rule, fill, filter, Default::default());

            assert_eq!(list_result, tree_result);
            assert_eq!(list_result, frag_result);
        }
    }

    fn many_squares(start: IntPoint, size: i32, offset: i32, n: usize) -> Vec<IntPath> {
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

    fn many_lines_x(a: i32, n: usize) -> Vec<IntPath> {
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

    fn many_lines_y(a: i32, n: usize) -> Vec<IntPath> {
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

    fn spiral(count: usize, radius: f64) -> Contour<[f64; 2]> {
        let mut a_path = Vec::with_capacity(4 * count);
        let mut b_path = Vec::with_capacity(2 * count);

        let mut a: f64 = 0.0;
        let mut r = radius;
        let w = 0.1 * radius;

        let c0 = [0.0, 0.0];
        let mut p0 = c0;

        for i in 0..count {
            let (sy, sx) = a.sin_cos();

            let rr = if i % 2 == 0 {
                r + 0.2 * radius
            } else {
                r - 0.2 * radius
            };

            let p = [rr * sx, rr * sy];
            let nx = p[0] - p0[0];
            let ny = p[1] - p0[1];
            let l = (nx * nx + ny * ny).sqrt();

            let n = [nx / l, ny/ l];
            let t = [w * -n[1], w * n[0]];

            a_path.push([p0[0] + t[0], p0[1] + t[1]]);
            a_path.push([p[0] + t[0], p[1] + t[1]]);
            a_path.push([p0[0] - t[0], p0[1] - t[1]]);
            a_path.push([p[0] - t[0], p[1] - t[1]]);

            a += radius / r;
            r = radius * (1.0 + a / (2.0 * PI));
            p0 = p;
        }

        b_path.reverse();
        a_path.append(&mut b_path);

        a_path
    }

}