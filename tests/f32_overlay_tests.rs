#[cfg(test)]
mod tests {
    use i_float::f32_point::F32Point;
    use i_shape::f32::shape::F32Path;
    use rand::Rng;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::f32::clip::F32Clip;
    use i_overlay::string::rule::StringRule;
    use i_overlay::f32::overlay::F32Overlay;
    use i_overlay::f32::slice::F32Slice;
    use i_overlay::f32::string::F32StringOverlay;
    use i_overlay::string::clip::ClipRule;


    #[test]
    fn test_00() {
        let shape_0 = [
            [
                F32Point::new(0.0, 0.0),
                F32Point::new(0.0, 1.0),
                F32Point::new(1.0, 1.0),
                F32Point::new(1.0, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(1.0, 0.0),
                F32Point::new(1.0, 1.0),
                F32Point::new(2.0, 1.0),
                F32Point::new(2.0, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_01() {
        let a = (1 << 30) as f32;

        let shape_0 = [
            [
                F32Point::new(0.0, 0.0),
                F32Point::new(0.0, a),
                F32Point::new(a, a),
                F32Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(a, 0.0),
                F32Point::new(a, a),
                F32Point::new(2.0 * a, a),
                F32Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_02() {
        let i: usize = 1 << 48;
        let a = i as f32;

        let shape_0 = [
            [
                F32Point::new(0.0, 0.0),
                F32Point::new(0.0, a),
                F32Point::new(a, a),
                F32Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(a, 0.0),
                F32Point::new(a, a),
                F32Point::new(2.0 * a, a),
                F32Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_03() {
        let i: usize = 1 << 48;
        let a = 1.0 / i as f32;

        let shape_0 = [
            [
                F32Point::new(0.0, 0.0),
                F32Point::new(0.0, a),
                F32Point::new(a, a),
                F32Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(a, 0.0),
                F32Point::new(a, a),
                F32Point::new(2.0 * a, a),
                F32Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_04() {
        let a = 0.9;

        let shape_0 = [
            [
                F32Point::new(0.0, 0.0),
                F32Point::new(0.0, a),
                F32Point::new(a, a),
                F32Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(a, 0.0),
                F32Point::new(a, a),
                F32Point::new(2.0 * a, a),
                F32Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_05() {
        let a = 0.99999_99999_99999_9;

        let shape_0 = [
            [
                F32Point::new(0.0, 0.0),
                F32Point::new(0.0, a),
                F32Point::new(a, a),
                F32Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(a, 0.0),
                F32Point::new(a, a),
                F32Point::new(2.0 * a, a),
                F32Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_06() {
        let a = 1.99999_99999_99999;

        let shape_0 = [
            [
                F32Point::new(0.0, 0.0),
                F32Point::new(0.0, a),
                F32Point::new(a, a),
                F32Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(a, 0.0),
                F32Point::new(a, a),
                F32Point::new(2.0 * a, a),
                F32Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_07() {
        let shape_0 = [
            [
                F32Point::new(-10.0, -10.0),
                F32Point::new(-10.0, 10.0),
                F32Point::new(10.0, 10.0),
                F32Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F32Point::new(-5.0, -5.0),
                F32Point::new(-5.0, 15.0),
                F32Point::new(15.0, 15.0),
                F32Point::new(15.0, -5.0),
            ].to_vec()
        ].to_vec();

        let overlay = F32Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 8);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        for n in 5..=10 {
            let mut points = vec![F32Point::ZERO; n];
            for _ in 0..=1000 {
                for i in 0..n {
                    let x = rng.gen_range(-1.0..=1.0);
                    let y = rng.gen_range(-1.0..=1.0);
                    points[i] = F32Point::new(x, y);
                }
            }
        }
    }

    #[test]
    fn test_empty_0() {
        let graph = F32Overlay::new().into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.is_empty(), true);
    }

    #[test]
    fn test_empty_1() {
        let shape = [
            [
                F32Point::new(-10.0, -10.0),
                F32Point::new(-10.0, 10.0),
                F32Point::new(10.0, 10.0),
                F32Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();

        let mut overlay = F32Overlay::new();
        overlay.add_paths(shape, ShapeType::Subject);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_2() {
        let shape_0 = [
            [
                F32Point::new(-10.0, -10.0),
                F32Point::new(-10.0, 10.0),
                F32Point::new(10.0, 10.0),
                F32Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();

        let shape_1 = [
            [
                F32Point::new(-500.0, -500.0)
            ].to_vec()
        ].to_vec();


        let overlay = F32Overlay::with_paths(shape_0, shape_1);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_3() {
        let shape_0 = [
            [
                F32Point::new(-10.0, -10.0),
                F32Point::new(-10.0, 10.0),
                F32Point::new(10.0, 10.0),
                F32Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();

        let shape_1 = [
            [
                F32Point::new(-500.0, -500.0),
                F32Point::new(-500.0, 500.0)
            ].to_vec()
        ].to_vec();


        let overlay = F32Overlay::with_paths(shape_0, shape_1);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_4() {
        let mut overlay = F32Overlay::new();
        overlay.add_path(vec![F32Point::new(0.0, 0.0)], ShapeType::Subject);
        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_5() {
        let mut overlay = F32Overlay::new();
        overlay.add_path(vec![F32Point::new(0.0, 0.0)], ShapeType::Subject);
        overlay.add_path(vec![F32Point::new(1.0, 0.0)], ShapeType::Clip);
        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_6() {
        let mut overlay = F32Overlay::new();
        overlay.add_path(vec![F32Point::new(0.0, 0.0), F32Point::new(1.0, 0.0)], ShapeType::Subject);
        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_slice_0() {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(vec![
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ]);

        overlay.add_string_line([F32Point::new(0.0, -15.0), F32Point::new(0.0, 15.0)]);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_1() {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(vec![
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ]);

        overlay.add_string_line([F32Point::new(0.0, -5.0), F32Point::new(0.0, 5.0)]);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_slice_2() {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(vec![
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ]);

        overlay.add_string_path(
            [
                F32Point::new(-15.0, -15.0),
                F32Point::new(0.0, 0.0),
                F32Point::new(-15.0, 15.0)
            ].to_vec(), true);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_3() {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(vec![
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ]);

        overlay.add_string_path(
            [
                F32Point::new(0.0, -5.0),
                F32Point::new(0.0, 5.0),
                F32Point::new(15.0, 5.0),
                F32Point::new(15.0, -5.0),
            ].to_vec(), false);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_4() {
        let mut overlay = F32StringOverlay::new();
        overlay.add_shape_path(vec![
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ]);

        overlay.add_string_path(vec![
            F32Point::new(-5.0, -5.0),
            F32Point::new(-5.0, 5.0),
            F32Point::new(5.0, 5.0),
            F32Point::new(5.0, -5.0),
        ], false);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_0() {
        let shapes = [
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ].slice_by_line([F32Point::new(0.0, -15.0), F32Point::new(0.0, 15.0)], FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_1() {
        let shapes = [
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ].slice_by_line([F32Point::new(0.0, -5.0), F32Point::new(0.0, 5.0)], FillRule::NonZero);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_ext_slice_2() {
        let shapes = [
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ].slice_by_path(&vec![
            F32Point::new(-15.0, -15.0),
            F32Point::new(0.0, 0.0),
            F32Point::new(-15.0, 15.0),
        ], true, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_3() {
        let shapes = [
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ].slice_by_path(&vec![
            F32Point::new(0.0, -5.0),
            F32Point::new(0.0, 5.0),
            F32Point::new(15.0, 5.0),
            F32Point::new(15.0, -5.0),
        ], false, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_4() {
        let shapes = [
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ].slice_by_path(&vec![
            F32Point::new(-5.0, -5.0),
            F32Point::new(-5.0, 5.0),
            F32Point::new(5.0, 5.0),
            F32Point::new(5.0, -5.0),
        ], false, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_clip_empty_path() {
        let path: F32Path = vec![];
        let result_0 = path.clip_line(
            [F32Point::new(0.0, 0.0), F32Point::new(1.0, 0.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [F32Point::new(0.0, 0.0), F32Point::new(1.0, 0.0)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert!(result_0.is_empty());
        assert_eq!(result_1.len(), 1);
    }

    #[test]
    fn test_clip_simple() {
        let path: F32Path = vec![
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ];
        let result_0 = path.clip_line(
            [F32Point::new(0.0, -15.0), F32Point::new(0.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [F32Point::new(0.0, -15.0), F32Point::new(0.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert_eq!(result_0.len(), 1);
        assert_eq!(result_1.len(), 2);
    }

    #[test]
    fn test_clip_boundary() {
        let path: F32Path = vec![
            F32Point::new(-10.0, -10.0),
            F32Point::new(-10.0, 10.0),
            F32Point::new(10.0, 10.0),
            F32Point::new(10.0, -10.0),
        ];
        let result_0 = path.clip_line(
            [F32Point::new(-10.0, -15.0), F32Point::new(-10.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [F32Point::new(-10.0, -15.0), F32Point::new(-10.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: true },
        );

        assert_eq!(result_0.len(), 0);
        assert_eq!(result_1.len(), 1);
    }
}