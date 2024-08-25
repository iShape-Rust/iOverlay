#[cfg(test)]
mod tests {
    use i_float::f32_point::F32Point;
    use rand::Rng;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::f32::overlay::F32Overlay;


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
}