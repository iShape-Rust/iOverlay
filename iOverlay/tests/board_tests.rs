#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_shape::int::path::IntPath;

    #[test]
    fn test_0() {
        assert_eq!(1, test(1, OverlayRule::Xor));
    }

    #[test]
    fn test_1() {
        assert_eq!(5, test(2, OverlayRule::Xor));
    }

    #[test]
    fn test_2() {
        assert_eq!(9 + 4, test(3, OverlayRule::Xor));
    }

    #[test]
    fn test_12() {
        let s = 12 * 12 + 11 * 11;
        assert_eq!(s, test(12, OverlayRule::Xor));
    }

    #[test]
    fn test_n() {
        for i in 1..20 {
            let s = i * i + (i - 1) * (i - 1);
            assert_eq!(s, test(i, OverlayRule::Xor));
        }
    }

    fn test(n: usize, rule: OverlayRule) -> usize {
        let subj_paths = many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

        let mut overlay = Overlay::new(8 * n * n);
        overlay.add_contours(&subj_paths, ShapeType::Subject);
        overlay.add_contours(&clip_paths, ShapeType::Clip);

        let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();
        let result = graph.extract_shapes(rule, &mut Default::default());

        result.len()
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
