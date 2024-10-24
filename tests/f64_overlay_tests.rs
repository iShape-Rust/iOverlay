#[cfg(test)]
mod tests {
    use i_float::f64_point::F64Point;
    use i_shape::f64::shape::F64Path;
    use rand::Rng;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::f64::clip::F64Clip;
    use i_overlay::f64::overlay::F64Overlay;
    use i_overlay::f64::slice::F64Slice;
    use i_overlay::f64::string::F64StringOverlay;
    use i_overlay::string::clip::ClipRule;
    use i_overlay::string::rule::StringRule;


    #[test]
    fn test_00() {
        let shape_0 = [
            [
                F64Point::new(0.0, 0.0),
                F64Point::new(0.0, 1.0),
                F64Point::new(1.0, 1.0),
                F64Point::new(1.0, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(1.0, 0.0),
                F64Point::new(1.0, 1.0),
                F64Point::new(2.0, 1.0),
                F64Point::new(2.0, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_01() {
        let a = (1 << 30) as f64;

        let shape_0 = [
            [
                F64Point::new(0.0, 0.0),
                F64Point::new(0.0, a),
                F64Point::new(a, a),
                F64Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(a, 0.0),
                F64Point::new(a, a),
                F64Point::new(2.0 * a, a),
                F64Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_02() {
        let i: usize = 1 << 48;
        let a = i as f64;

        let shape_0 = [
            [
                F64Point::new(0.0, 0.0),
                F64Point::new(0.0, a),
                F64Point::new(a, a),
                F64Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(a, 0.0),
                F64Point::new(a, a),
                F64Point::new(2.0 * a, a),
                F64Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_03() {
        let i: usize = 1 << 48;
        let a = 1.0 / i as f64;

        let shape_0 = [
            [
                F64Point::new(0.0, 0.0),
                F64Point::new(0.0, a),
                F64Point::new(a, a),
                F64Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(a, 0.0),
                F64Point::new(a, a),
                F64Point::new(2.0 * a, a),
                F64Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
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
                F64Point::new(0.0, 0.0),
                F64Point::new(0.0, a),
                F64Point::new(a, a),
                F64Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(a, 0.0),
                F64Point::new(a, a),
                F64Point::new(2.0 * a, a),
                F64Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
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
                F64Point::new(0.0, 0.0),
                F64Point::new(0.0, a),
                F64Point::new(a, a),
                F64Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(a, 0.0),
                F64Point::new(a, a),
                F64Point::new(2.0 * a, a),
                F64Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
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
                F64Point::new(0.0, 0.0),
                F64Point::new(0.0, a),
                F64Point::new(a, a),
                F64Point::new(a, 0.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(a, 0.0),
                F64Point::new(a, a),
                F64Point::new(2.0 * a, a),
                F64Point::new(2.0 * a, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
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
                F64Point::new(-10.0, -10.0),
                F64Point::new(-10.0, 10.0),
                F64Point::new(10.0, 10.0),
                F64Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(-5.0, -5.0),
                F64Point::new(-5.0, 15.0),
                F64Point::new(15.0, 15.0),
                F64Point::new(15.0, -5.0),
            ].to_vec()
        ].to_vec();

        let overlay = F64Overlay::with_paths(shape_0, shape_1);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 8);
    }

    #[test]
    fn test_08() {
        let subj = [
            [
                [
                    F64Point::new(2.308208703994751, -44.961544036865234),
                    F64Point::new(37.17605972290039, -16.19365692138672),
                    F64Point::new(33.88911819458008, -25.311290740966797),
                    F64Point::new(33.86614990234375, -25.540952682495117),
                    F64Point::new(40.45701217651367, -15.621655464172363),
                    F64Point::new(46.20624923706055, -37.305298805236816),
                ].to_vec()
            ].to_vec(),
            [
                [
                    F64Point::new(11.95650577545166, 14.7280387878418),
                    F64Point::new(30.346940994262695, 38.30845260620117),
                    F64Point::new(38.4443359375, 40.48749923706055),
                    F64Point::new(41.69062423706055, 18.96145248413086),
                    F64Point::new(33.89716720581055, 20.76250076293945),
                ].to_vec()
            ].to_vec(),
            [
                [
                    F64Point::new(35.20156717300415, 39.084500551223755),
                    F64Point::new(30.97531795501709, 38.142252922058105),
                    F64Point::new(34.15063834190369, 21.34823846817017)
                ].to_vec(),
                [
                    F64Point::new(38.073265075683594, 39.88750076293945),
                    F64Point::new(37.69073486328125, 33.43665313720703),
                    F64Point::new(37.88459777832031, 33.34542465209961)
                ].to_vec()
            ].to_vec()
        ].to_vec();
        let clip = [[
            [
                F64Point::new(36.77085494995117, -16.330209732055664),
                F64Point::new(37.04164505004883, -15.794790267944336),
                F64Point::new(38.42919921875, -16.496543884277344),
                F64Point::new(40.78192138671875, -18.3913516998291),
                F64Point::new(40.40557861328125, -18.8586483001709),
                F64Point::new(38.10205078125, -17.003456115722656),
            ].to_vec()
        ].to_vec()].to_vec();

        let overlay = F64Overlay::with_shapes(subj, clip);
        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 2);
        assert_eq!(union[0].len(), 2);
        assert_eq!(union[0][0].len(), 8);
    }


    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        for n in 5..=10 {
            let mut points = vec![F64Point::ZERO; n];
            for _ in 0..=1000 {
                for i in 0..n {
                    let x = rng.gen_range(-1.0..=1.0);
                    let y = rng.gen_range(-1.0..=1.0);
                    points[i] = F64Point::new(x, y);
                }
            }
        }
    }

    #[test]
    fn test_empty_0() {
        let graph = F64Overlay::new().into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.is_empty(), true);
    }

    #[test]
    fn test_empty_1() {
        let shape = [
            [
                F64Point::new(-10.0, -10.0),
                F64Point::new(-10.0, 10.0),
                F64Point::new(10.0, 10.0),
                F64Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();

        let mut overlay = F64Overlay::new();
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
                F64Point::new(-10.0, -10.0),
                F64Point::new(-10.0, 10.0),
                F64Point::new(10.0, 10.0),
                F64Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();

        let shape_1 = [
            [
                F64Point::new(-500.0, -500.0)
            ].to_vec()
        ].to_vec();


        let overlay = F64Overlay::with_paths(shape_0, shape_1);

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
                F64Point::new(-10.0, -10.0),
                F64Point::new(-10.0, 10.0),
                F64Point::new(10.0, 10.0),
                F64Point::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();

        let shape_1 = [
            [
                F64Point::new(-500.0, -500.0),
                F64Point::new(-500.0, 500.0)
            ].to_vec()
        ].to_vec();


        let overlay = F64Overlay::with_paths(shape_0, shape_1);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_4() {
        let mut overlay = F64Overlay::new();
        overlay.add_path(vec![F64Point::new(0.0, 0.0)], ShapeType::Subject);
        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_5() {
        let mut overlay = F64Overlay::new();
        overlay.add_path(vec![F64Point::new(0.0, 0.0)], ShapeType::Subject);
        overlay.add_path(vec![F64Point::new(1.0, 0.0)], ShapeType::Clip);
        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_6() {
        let mut overlay = F64Overlay::new();
        overlay.add_path(vec![F64Point::new(0.0, 0.0), F64Point::new(1.0, 0.0)], ShapeType::Subject);
        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }


    #[test]
    fn test_slice_0() {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(vec![
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ]);

        overlay.add_string_line([F64Point::new(0.0, -15.0), F64Point::new(0.0, 15.0)]);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_1() {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(vec![
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ]);

        overlay.add_string_line([F64Point::new(0.0, -5.0), F64Point::new(0.0, 5.0)]);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_slice_2() {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(vec![
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ]);

        overlay.add_string_path(
            [
                F64Point::new(-15.0, -15.0),
                F64Point::new(0.0, 0.0),
                F64Point::new(-15.0, 15.0)
            ].to_vec(), true);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_3() {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(vec![
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ]);

        overlay.add_string_path(
            [
                F64Point::new(0.0, -5.0),
                F64Point::new(0.0, 5.0),
                F64Point::new(15.0, 5.0),
                F64Point::new(15.0, -5.0),
            ].to_vec(), false);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_4() {
        let mut overlay = F64StringOverlay::new();
        overlay.add_shape_path(vec![
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ]);

        overlay.add_string_path(vec![
            F64Point::new(-5.0, -5.0),
            F64Point::new(-5.0, 5.0),
            F64Point::new(5.0, 5.0),
            F64Point::new(5.0, -5.0),
        ], false);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }


    #[test]
    fn test_ext_slice_0() {
        let shapes = [
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ].slice_by_line([F64Point::new(0.0, -15.0), F64Point::new(0.0, 15.0)], FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_1() {
        let shapes = [
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ].slice_by_line([F64Point::new(0.0, -5.0), F64Point::new(0.0, 5.0)], FillRule::NonZero);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_ext_slice_2() {
        let shapes = [
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ].slice_by_path(&vec![
            F64Point::new(-15.0, -15.0),
            F64Point::new(0.0, 0.0),
            F64Point::new(-15.0, 15.0),
        ], true, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_3() {
        let shapes = [
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ].slice_by_path(&vec![
            F64Point::new(0.0, -5.0),
            F64Point::new(0.0, 5.0),
            F64Point::new(15.0, 5.0),
            F64Point::new(15.0, -5.0),
        ], false, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_4() {
        let shapes = [
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ].slice_by_path(&vec![
            F64Point::new(-5.0, -5.0),
            F64Point::new(-5.0, 5.0),
            F64Point::new(5.0, 5.0),
            F64Point::new(5.0, -5.0),
        ], false, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_clip_empty_path() {
        let path: F64Path = vec![];
        let result_0 = path.clip_line(
            [F64Point::new(0.0, 0.0), F64Point::new(1.0, 0.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [F64Point::new(0.0, 0.0), F64Point::new(1.0, 0.0)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert!(result_0.is_empty());
        assert_eq!(result_1.len(), 1);
    }

    #[test]
    fn test_clip_simple() {
        let path: F64Path = vec![
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ];
        let result_0 = path.clip_line(
            [F64Point::new(0.0, -15.0), F64Point::new(0.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [F64Point::new(0.0, -15.0), F64Point::new(0.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert_eq!(result_0.len(), 1);
        assert_eq!(result_1.len(), 2);
    }

    #[test]
    fn test_clip_boundary() {
        let path: F64Path = vec![
            F64Point::new(-10.0, -10.0),
            F64Point::new(-10.0, 10.0),
            F64Point::new(10.0, 10.0),
            F64Point::new(10.0, -10.0),
        ];
        let result_0 = path.clip_line(
            [F64Point::new(-10.0, -15.0), F64Point::new(-10.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [F64Point::new(-10.0, -15.0), F64Point::new(-10.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: true },
        );

        assert_eq!(result_0.len(), 0);
        assert_eq!(result_1.len(), 1);
    }
}