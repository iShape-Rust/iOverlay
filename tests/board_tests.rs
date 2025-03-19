#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_shape::int::path::IntPath;
    use i_overlay::core::solver::Solver;

    #[test]
    fn test_1() {
        test(1, OverlayRule::Xor);
    }

    #[test]
    fn test_2() {
        test(2, OverlayRule::Xor);
    }

    #[test]
    fn test_12() {
        test(12, OverlayRule::Xor)
    }

    #[test]
    fn test_n() {
        for i in 1..20 {
            test(i, OverlayRule::Xor)
        }
    }

    #[test]
    fn test_iso_1() {
        test_iso(1, OverlayRule::Xor)
    }

    #[test]
    fn test_iso_2() {
        test_iso(2, OverlayRule::Xor)
    }

    #[test]
    fn test_iso_3() {
        test_iso(3, OverlayRule::Xor)
    }

    #[test]
    fn test_iso_100() {
        test_iso(100, OverlayRule::Xor)
    }

    #[test]
    fn test_iso_n() {
        for i in 1..20 {
            test_iso(i, OverlayRule::Xor)
        }
    }


    fn test(n: usize, rule: OverlayRule) {
        let subj_paths = many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

        let mut overlay = Overlay::new(8 * n * n);
        overlay.add_contours(&subj_paths, ShapeType::Subject);
        overlay.add_contours(&clip_paths, ShapeType::Clip);

        let graph = overlay.into_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        let s = n * n + (n - 1) * (n - 1);
        assert_eq!(s, result.len());
    }

    fn test_iso(n: usize, rule: OverlayRule) {
        let subj_paths = many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

        let overlay = Overlay::with_contours(&subj_paths, &clip_paths);
        let graph = overlay.into_45geom_graph_with_solver(FillRule::NonZero, Solver::default());
        let result = graph.extract_shapes(rule);

        let s = n * n + (n - 1) * (n - 1);
        assert_eq!(s, result.len());
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
}
