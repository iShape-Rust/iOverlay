mod util;

#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_shape::int::path::IntPath;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;
    use i_shape::int::shape::IntContour;
    use crate::util::overlay::JsonPrint;

    #[test]
    fn test_many_squares() {
        let fill = FillRule::NonZero;
        let rule = OverlayRule::Xor;

        for n in [25usize, 50, 100] {
            let subj_paths = many_squares(IntPoint::new(0, 0), 20, 30, n);
            let clip_paths = many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

            let list_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::LIST)
                .overlay(rule, fill);

            let tree_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::TREE)
                .overlay(rule, fill);

            let frag_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::FRAG)
                .overlay(rule, fill);

            assert_eq!(list_result, tree_result);
            assert_eq!(list_result, frag_result);
        }
    }

    #[test]
    fn test_no_overlap() {
        let fill = FillRule::NonZero;
        let rule = OverlayRule::Xor;

        for n in [25usize, 50, 100] {
            let subj_paths = repeat_xy(square(0, 0, 2), 0, 0, 10, 10, n);
            let clip_paths = repeat_xy(romb(0, 0, 4), 5, 5, 10, 10, n - 1);

            let list_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::LIST)
                .overlay(rule, fill);

            let tree_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::TREE)
                .overlay(rule, fill);

            let frag_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::FRAG)
                .overlay(rule, fill);

            assert_eq!(list_result, tree_result);
            assert_eq!(list_result, frag_result);
            assert_eq!(list_result.len(), n * n + (n - 1) * (n - 1));
        }
    }

    #[test]
    fn test_many_lines() {
        let fill = FillRule::NonZero;
        let rule = OverlayRule::Intersect;

        for n in [25usize, 50, 100] {
            let subj_paths = many_lines_x(20, n);
            let clip_paths = many_lines_y(20, n);

            let list_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::LIST)
                .overlay(rule, fill);

            let tree_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::TREE)
                .overlay(rule, fill);

            let frag_result = Overlay::with_contours_custom(&subj_paths, &clip_paths, Default::default(), Solver::FRAG)
                .overlay(rule, fill);

            assert_eq!(list_result, tree_result);
            assert_eq!(list_result, frag_result);
        }
    }

    #[test]
    fn test_spiral() {
        let fill = FillRule::NonZero;
        let rule = OverlayRule::Subject;

        for n in [25usize, 50, 100] {
            let contours = discrete_spiral(n, 4);

            let mut list_overlay = Overlay::new(n * 8);
            list_overlay.add_contours(&contours, ShapeType::Subject);

            let list_result = list_overlay.overlay(rule, fill);

            let mut tree_overlay = Overlay::new(n * 8);
            tree_overlay.add_contours(&contours, ShapeType::Subject);

            let tree_result = tree_overlay.overlay(rule, fill);

            let mut frag_overlay = Overlay::new(n * 8);
            frag_overlay.add_contours(&contours, ShapeType::Subject);

            let frag_result = frag_overlay.overlay(rule, fill);

            assert_eq!(list_result, tree_result);
            assert_eq!(list_result, frag_result);
        }
    }

    #[test]
    fn test_crosses() {
        let n = 10;

        let subj_cross = vec![
            IntPoint::new(-4, 3),
            IntPoint::new(-4, 1),
            IntPoint::new(-3, 0),
            IntPoint::new(-4, -1),
            IntPoint::new(-4, -3),
            IntPoint::new(-3, -4),
            IntPoint::new(-1, -4),
            IntPoint::new(0, -3),
            IntPoint::new(1, -4),
            IntPoint::new(3, -4),
            IntPoint::new(4, -3),
            IntPoint::new(4, -1),
            IntPoint::new(3, 0),
            IntPoint::new(4, 1),
            IntPoint::new(4, 3),
            IntPoint::new(3, 4),
            IntPoint::new(1, 4),
            IntPoint::new(0, 3),
            IntPoint::new(-1, 4),
            IntPoint::new(-3, 4),
        ];

        let clip_rect_0 = vec![
            IntPoint::new(-3, 2),
            IntPoint::new(-2, 3),
            IntPoint::new(3, -2),
            IntPoint::new(2, -3),
        ];

        let clip_rect_1 = vec![
            IntPoint::new(-2, -3),
            IntPoint::new(-3, -2),
            IntPoint::new(2, 3),
            IntPoint::new(3, 2),
        ];

        let subj_paths = repeat_xy(subj_cross, 0, 0, 10, 10, n);
        let clip_rects_0 = repeat_xy(clip_rect_0, 0, 0, 10, 10, n);
        let mut clip_rects_1 = repeat_xy(clip_rect_1, 0, 0, 10, 10, n);
        let mut clip_paths = clip_rects_0;
        clip_paths.append(&mut clip_rects_1);

        println!("subj_paths: {}", subj_paths.json_print());
        println!("clip_paths: {}", clip_paths.json_print());
    }

    #[test]
    fn test_romb() {
        let n = 10;
        let a = 2;
        let l = 2 * a * n;
        let vert_line = vec![
            IntPoint::new(0, l + a),
            IntPoint::new(0, 0),
            IntPoint::new(a, 0),
            IntPoint::new(a, l + a),
        ];
        let horz_line = vec![
            IntPoint::new(0, a),
            IntPoint::new(0, 0),
            IntPoint::new(l + a, 0),
            IntPoint::new(l + a, a),
        ];

        let clip = vec![
            IntPoint::new(a / 2, l / 2 + a / 2),
            IntPoint::new(l / 2 + a / 2, a / 2),
            IntPoint::new(l + a / 2, l / 2 + a / 2),
            IntPoint::new(l / 2 + a / 2, l + a / 2),
        ];

        let subj_0 = repeat_x(vert_line, 0, 0, 2 * a, 1 + n as usize);
        let mut subj_1 = repeat_y(horz_line, 0, 0, 2 * a, 1 + n as usize);

        let mut subj = subj_0;
        subj.append(&mut subj_1);

        println!("subj: {}", subj.json_print());
        println!("clip: {}", clip.json_print());
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

    fn discrete_spiral(count: usize, a: i32) -> Vec<IntContour> {
        let mut rects = Vec::with_capacity(8 * count);

        let a2 = 2 * a;
        let a4 = 4 * a;
        let a6 = 6 * a;

        // horizontal rects
        let mut r = 0;
        for _ in 0..count {

            // bottom
            let r0x0 = -r;
            let r0x1 = r0x0;
            let r0x2 = r + a2;
            let r0x3 = r0x2;

            let r0y0 = -r;
            let r0y1 = r0y0 - a2;
            let r0y2 = r0y1;
            let r0y3 = r0y0;

            // top
            let r1x0 = r0x0 - a4;
            let r1x1 = r1x0;
            let r1x2 = r0x2;
            let r1x3 = r1x2;

            let r1y0 = r + a6;
            let r1y1 = r1y0 - a2;
            let r1y2 = r1y1;
            let r1y3 = r1y0;

            // left
            let r2x0 = -r - a6;
            let r2x1 = r2x0;
            let r2x2 = r2x1 + a2;
            let r2x3 = r2x2;

            let r2y0 = r + a4;
            let r2y1 = -(r + a4);
            let r2y2 = r2y1;
            let r2y3 = r2y0;

            // right
            let r3x0 = r + a2;
            let r3x1 = r3x0;
            let r3x2 = r3x1 + a2;
            let r3x3 = r3x2;

            let r3y0 = r + a4;
            let r3y1 = -r;
            let r3y2 = r3y1;
            let r3y3 = r3y0;

            r += a4;

            rects.push(vec![IntPoint::new(r0x0, r0y0), IntPoint::new(r0x1, r0y1), IntPoint::new(r0x2, r0y2), IntPoint::new(r0x3, r0y3)]);
            rects.push(vec![IntPoint::new(r1x0, r1y0), IntPoint::new(r1x1, r1y1), IntPoint::new(r1x2, r1y2), IntPoint::new(r1x3, r1y3)]);
            rects.push(vec![IntPoint::new(r2x0, r2y0), IntPoint::new(r2x1, r2y1), IntPoint::new(r2x2, r2y2), IntPoint::new(r2x3, r2y3)]);
            rects.push(vec![IntPoint::new(r3x0, r3y0), IntPoint::new(r3x1, r3y1), IntPoint::new(r3x2, r3y2), IntPoint::new(r3x3, r3y3)]);

            rects.push(romb(-r, r, a2));
            rects.push(romb(-r, -r, a2));
            rects.push(romb(r - a2, a4 - r, a2));
            rects.push(romb(r - a2, r, a2));
        }

        rects
    }

    fn romb(x: i32, y: i32, a: i32) -> IntContour {
        vec![
            IntPoint::new(x - a, y),
            IntPoint::new(x, y - a),
            IntPoint::new(x + a, y),
            IntPoint::new(x, y + a),
        ]
    }

    fn square(x: i32, y: i32, a: i32) -> IntContour {
        vec![
            IntPoint::new(x - a, y + a),
            IntPoint::new(x - a, y - a),
            IntPoint::new(x + a, y - a),
            IntPoint::new(x + a, y + a),
        ]
    }

    fn repeat_xy(origin: IntContour, x0: i32, y0: i32, dx: i32, dy: i32, count: usize) -> Vec<IntContour> {
        let mut contours = Vec::with_capacity(8 * count);
        let mut x = x0;
        for _ in 0..count {
            let mut y = y0;
            for _ in 0..count {
                let mut contour = origin.clone();
                for p in contour.iter_mut() {
                    p.x += x;
                    p.y += y;
                }
                contours.push(contour);
                y += dy;
            }
            x += dx;
        }

        contours
    }

    fn repeat_x(origin: IntContour, x0: i32, y0: i32, dx: i32, count: usize) -> Vec<IntContour> {
        let mut contours = Vec::with_capacity(8 * count);
        let mut x = x0;
        for _ in 0..count {
            let mut contour = origin.clone();
            for p in contour.iter_mut() {
                p.x += x;
                p.y += y0;
            }
            contours.push(contour);
            x += dx;
        }

        contours
    }

    fn repeat_y(origin: IntContour, x0: i32, y0: i32, dy: i32, count: usize) -> Vec<IntContour> {
        let mut contours = Vec::with_capacity(8 * count);
        let mut y = y0;
        for _ in 0..count {
            let mut contour = origin.clone();
            for p in contour.iter_mut() {
                p.x += x0;
                p.y += y;
            }
            contours.push(contour);
            y += dy;
        }

        contours
    }
}