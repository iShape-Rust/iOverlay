#[cfg(test)]
mod tests {
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

    /*
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
    */

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

    /*
    fn discrete_spiral(count: usize, a: i32) -> Vec<IntContour> {
        let mut rects = Vec::with_capacity(8 * count);

        let a2 = 2 * a;
        let a3 = 3 * a;
        let a4 = 4 * a;
        let a4 = 6 * a;

        // horizontal rects
        let mut s = 0;
        for i in 0..count {

            // bottom
            let r0x0 = -(s + a);
            let r0x1 = r0x0;
            let r0x2 = s + a;
            let r0x3 = r0x2;

            let r0y0 = s - a;
            let r0y1 = r0y0 - a2;
            let r0y2 = r0y1;
            let r0y3 = r0y0;

            // top
            let r1x0 = r0x0 - a4;
            let r1x1 = r1x0;
            let r1x2 = r0x2;
            let r1x3 = r1x2;

            let r1y0 = -r0y1;
            let r1y1 = r1y0 - a2;
            let r1y2 = r1y1;
            let r1y3 = r1y0;

            // left
            let r2x0 = -s - a6;
            let r2x1 = r2x0;
            let r2x2 = r2x1 + a2;
            let r2x3 = r0x2;

            let r2y0 = -s + a3;
            let r2y1 = r2y0 - s;
            let r2y2 = r0y1;
            let r2y3 = r0y0;

            // right
            let r3x0 = -r2x0;
            let r3x1 = r2x2;
            let r3x2 = r2x2 + a2;
            let r3x3 = r2x3;

            let r3y0 = r2y0;
            let r3y1 = r0y0 - s;
            let r3y2 = r0y1;
            let r3y3 = r0y0;

            s += a4;

            rects.push(vec![IntPoint::new(r0x0, r0y0), IntPoint::new(r0x1, r0y1), IntPoint::new(r0x2, r0y2), IntPoint::new(r0x3, r0y3)]);
            rects.push(vec![IntPoint::new(r1x0, r1y0), IntPoint::new(r1x1, r1y1), IntPoint::new(r1x2, r1y2), IntPoint::new(r1x3, r1y3)]);
            rects.push(vec![IntPoint::new(r2x0, r2y0), IntPoint::new(r2x1, r2y1), IntPoint::new(r2x2, r2y2), IntPoint::new(r2x3, r2y3)]);
            rects.push(vec![IntPoint::new(r3x0, r3y0), IntPoint::new(r3x1, r3y1), IntPoint::new(r3x2, r3y2), IntPoint::new(r3x3, r3y3)]);
        }

        // top rects
        let mut y = -3 * a;
        let mut s = 0;
        for i in 0..count {
            let x0 = -s;
            let x1 = x0;
            let x2 = s + a;
            let x3 = x2;

            let y0 = y;
            let y1 = y - a2;
            let y2 = y1;
            let y3 = y0;

            s += a4;

            rects.push(vec![
                IntPoint::new(x0, y0),
                IntPoint::new(x1, y1),
                IntPoint::new(x2, y2),
                IntPoint::new(x3, y3),
            ])
        }
    }
    */
}