#[cfg(test)]
mod tests {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::compatible::FloatPointCompatible;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{ContourDirection, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::clip::FloatClip;
    use i_overlay::float::overlay::{FloatOverlay, OverlayOptions};
    use i_overlay::float::slice::FloatSlice;
    use i_overlay::string::clip::ClipRule;
    use rand::Rng;

    #[derive(Clone, Copy)]
    struct FPoint {
        x: f32,
        y: f32,
    }

    impl FPoint {
        fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }

    impl FloatPointCompatible<f32> for FPoint {
        fn from_xy(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        fn x(&self) -> f32 {
            self.x
        }

        fn y(&self) -> f32 {
            self.y
        }
    }

    #[test]
    fn test_00() {
        let shape_0 = vec![vec![
            FPoint::new(0.0, 0.0),
            FPoint::new(0.0, 1.0),
            FPoint::new(1.0, 1.0),
            FPoint::new(1.0, 0.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(1.0, 0.0),
            FPoint::new(1.0, 1.0),
            FPoint::new(2.0, 1.0),
            FPoint::new(2.0, 0.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_01() {
        let a = (1 << 30) as f32;

        let shape_0 = vec![vec![
            FPoint::new(0.0, 0.0),
            FPoint::new(0.0, a),
            FPoint::new(a, a),
            FPoint::new(a, 0.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(a, 0.0),
            FPoint::new(a, a),
            FPoint::new(2.0 * a, a),
            FPoint::new(2.0 * a, 0.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_02() {
        let i: usize = 1 << 48;
        let a = i as f32;

        let shape_0 = vec![vec![
            FPoint::new(0.0, 0.0),
            FPoint::new(0.0, a),
            FPoint::new(a, a),
            FPoint::new(a, 0.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(a, 0.0),
            FPoint::new(a, a),
            FPoint::new(2.0 * a, a),
            FPoint::new(2.0 * a, 0.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_03() {
        let i: usize = 1 << 48;
        let a = 1.0 / i as f32;

        let shape_0 = vec![vec![
            FPoint::new(0.0, 0.0),
            FPoint::new(0.0, a),
            FPoint::new(a, a),
            FPoint::new(a, 0.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(a, 0.0),
            FPoint::new(a, a),
            FPoint::new(2.0 * a, a),
            FPoint::new(2.0 * a, 0.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_04() {
        let a = 0.9;

        let shape_0 = vec![vec![
            FPoint::new(0.0, 0.0),
            FPoint::new(0.0, a),
            FPoint::new(a, a),
            FPoint::new(a, 0.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(a, 0.0),
            FPoint::new(a, a),
            FPoint::new(2.0 * a, a),
            FPoint::new(2.0 * a, 0.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_05() {
        let a = 0.99999_99999_99999_9;

        let shape_0 = vec![vec![
            FPoint::new(0.0, 0.0),
            FPoint::new(0.0, a),
            FPoint::new(a, a),
            FPoint::new(a, 0.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(a, 0.0),
            FPoint::new(a, a),
            FPoint::new(2.0 * a, a),
            FPoint::new(2.0 * a, 0.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_06() {
        let a = 1.99999_99999_99999;

        let shape_0 = vec![vec![
            FPoint::new(0.0, 0.0),
            FPoint::new(0.0, a),
            FPoint::new(a, a),
            FPoint::new(a, 0.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(a, 0.0),
            FPoint::new(a, a),
            FPoint::new(2.0 * a, a),
            FPoint::new(2.0 * a, 0.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_07() {
        let shape_0 = vec![vec![
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]];
        let shape_1 = vec![vec![
            FPoint::new(-5.0, -5.0),
            FPoint::new(-5.0, 15.0),
            FPoint::new(15.0, 15.0),
            FPoint::new(15.0, -5.0),
        ]];

        let union = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero).unwrap()
            .extract_shapes(OverlayRule::Union, &mut Default::default());

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 8);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::rng();
        for n in 5..=10 {
            let mut points = vec![FPoint::new(0.0, 0.0); n];
            for _ in 0..=1000 {
                for i in 0..n {
                    let x = rng.random_range(-1.0..=1.0);
                    let y = rng.random_range(-1.0..=1.0);
                    points[i] = FPoint::new(x, y);
                }
            }
        }
    }

    #[test]
    fn test_empty_0() {
        let path = vec![FPoint::new(-10.0, -10.0), FPoint::new(-10.0, 10.0)];

        let shapes =
            FloatOverlay::with_adapter(FloatPointAdapter::with_iter(path.iter()), path.len())
                .build_graph_view(FillRule::NonZero)
                .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut Default::default()));

        assert_eq!(shapes.is_empty(), true);
    }

    #[test]
    fn test_empty_1() {
        let shape = [[
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .to_vec()];

        let shapes = FloatOverlay::with_adapter(
            FloatPointAdapter::with_iter(shape.iter().flatten()),
            shape.len(),
        )
            .unsafe_add_source(&shape, ShapeType::Subject)
            .build_graph_view(FillRule::NonZero)
            .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut Default::default()));

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_2() {
        let shape_0 = vec![vec![
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]];

        let shape_1 = vec![vec![FPoint::new(-500.0, -500.0)]];

        let shapes = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero)
            .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut Default::default()));

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_3() {
        let shape_0 = vec![vec![
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]];

        let shape_1 = vec![vec![
            FPoint::new(-500.0, -500.0),
            FPoint::new(-500.0, 500.0),
        ]];

        let shapes = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .build_graph_view(FillRule::NonZero)
            .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut Default::default()));

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_4() {
        let path = vec![FPoint::new(0.0, 0.0)];
        let shapes =
            FloatOverlay::with_adapter(FloatPointAdapter::with_iter(path.iter()), path.len())
                .unsafe_add_contour(&path, ShapeType::Subject)
                .build_graph_view(FillRule::NonZero)
                .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut Default::default()));

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_5() {
        let shapes = FloatOverlay::with_subj_and_clip(
            &vec![FPoint::new(0.0, 0.0)],
            &vec![FPoint::new(1.0, 0.0)],
        )
            .build_graph_view(FillRule::NonZero)
            .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut Default::default()));

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_6() {
        let path = vec![FPoint::new(0.0, 0.0), FPoint::new(1.0, 0.0)];
        let shapes =
            FloatOverlay::with_adapter(FloatPointAdapter::with_iter(path.iter()), path.len())
                .unsafe_add_contour(&path, ShapeType::Subject)
                .build_graph_view(FillRule::NonZero)
                .map_or(Default::default(), |graph|graph.extract_shapes(OverlayRule::Subject, &mut Default::default()));

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_slice_0() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .slice_by(
            &[FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_1() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .slice_by(
            &[FPoint::new(0.0, -5.0), FPoint::new(0.0, 5.0)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_slice_2() {
        let path_0 = vec![
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];

        let path_1 = vec![
            FPoint::new(-15.0, -15.0),
            FPoint::new(0.0, 0.0),
            FPoint::new(-15.0, 15.0),
        ];

        let shapes = path_0.slice_by(&path_1, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_3() {
        let path_0 = vec![
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];

        let path_1 = vec![
            FPoint::new(0.0, -5.0),
            FPoint::new(0.0, 5.0),
            FPoint::new(15.0, 5.0),
            FPoint::new(15.0, -5.0),
            FPoint::new(0.0, -5.0), // close
        ];

        let shapes = path_0.slice_by(&path_1, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_4() {
        let path_0 = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];

        let path_1 = [
            FPoint::new(-5.0, -5.0),
            FPoint::new(-5.0, 5.0),
            FPoint::new(5.0, 5.0),
            FPoint::new(5.0, -5.0),
            FPoint::new(-5.0, -5.0), // close
        ];

        let shapes = path_0.slice_by(&path_1, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_0() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .slice_by(
            &[FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_1() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .slice_by(
            &[FPoint::new(0.0, -5.0), FPoint::new(0.0, 5.0)],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_ext_slice_2() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .slice_by(
            &vec![
                FPoint::new(-15.0, -15.0),
                FPoint::new(0.0, 0.0),
                FPoint::new(-15.0, 15.0),
            ],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_3() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .slice_by(
            &vec![
                FPoint::new(0.0, -5.0),
                FPoint::new(0.0, 5.0),
                FPoint::new(15.0, 5.0),
                FPoint::new(15.0, -5.0),
                FPoint::new(0.0, -5.0), // close
            ],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_4() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ]
        .slice_by(
            &vec![
                FPoint::new(-5.0, -5.0),
                FPoint::new(-5.0, 5.0),
                FPoint::new(5.0, 5.0),
                FPoint::new(5.0, -5.0),
                FPoint::new(-5.0, -5.0), // close
            ],
            FillRule::NonZero,
        );

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_clip_empty_path() {
        let contour = [FPoint::new(0.0, 0.0); 0];
        let result_0 = [FPoint::new(0.0, 0.0), FPoint::new(1.0, 0.0)].clip_by(
            &contour,
            FillRule::NonZero,
            ClipRule {
                invert: false,
                boundary_included: false,
            },
        );

        let result_1 = [FPoint::new(0.0, 0.0), FPoint::new(1.0, 0.0)].clip_by(
            &contour,
            FillRule::NonZero,
            ClipRule {
                invert: true,
                boundary_included: false,
            },
        );

        assert!(result_0.is_empty());
        assert_eq!(result_1.len(), 1);
    }

    #[test]
    fn test_clip_simple() {
        let contour = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];
        let result_0 = [FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)].clip_by(
            &contour,
            FillRule::NonZero,
            ClipRule {
                invert: false,
                boundary_included: false,
            },
        );

        let result_1 = [FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)].clip_by(
            &contour,
            FillRule::NonZero,
            ClipRule {
                invert: true,
                boundary_included: false,
            },
        );

        assert_eq!(result_0.len(), 1);
        assert_eq!(result_1.len(), 2);
    }

    #[test]
    fn test_clip_boundary() {
        let contour = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];
        let result_0 = [FPoint::new(-10.0, -15.0), FPoint::new(-10.0, 15.0)].clip_by(
            &contour,
            FillRule::NonZero,
            ClipRule {
                invert: false,
                boundary_included: false,
            },
        );

        let result_1 = [FPoint::new(-10.0, -15.0), FPoint::new(-10.0, 15.0)].clip_by(
            &contour,
            FillRule::NonZero,
            ClipRule {
                invert: false,
                boundary_included: true,
            },
        );

        assert_eq!(result_0.len(), 0);
        assert_eq!(result_1.len(), 1);
    }

    #[test]
    fn test_simplify() {
        let shape_0 = [[
            [48.239437f32, -54.70892f32],
            [47.195786, -55.457626],
            [46.968903, -56.886974],
            [36.532383, -55.07193],
            [37.961735, -46.7454],
            [40.02635, -47.085724],
            [40.094414, -46.7454],
            [44.51859, -47.516796],
            [44.473213, -47.83443],
            [48.398254, -48.51507],
            [48.10331, -49.9898],
            [48.874702, -50.965385],
        ]
        .to_vec()];

        let shape_1 = [[
            [48.398247, -48.515068],
            [48.10331, -49.989796],
            [44.473213, -47.834427],
        ]
        .to_vec()];

        let result_no_filter = FloatOverlay::with_subj_and_clip(&shape_0, &shape_1)
            .overlay(OverlayRule::Intersect, FillRule::EvenOdd);

        let opt = OverlayOptions {
            preserve_input_collinear: false,
            output_direction: ContourDirection::CounterClockwise,
            preserve_output_collinear: false,
            min_output_area: 0.0,
            clean_result: false,
        };
        
        let result_with_filter = FloatOverlay::with_subj_and_clip_custom(&shape_0, &shape_1, opt, Default::default())
            .overlay(
                OverlayRule::Intersect,
                FillRule::EvenOdd,
            );

        assert_eq!(result_no_filter.len(), 1);
        assert_eq!(result_with_filter.len(), 2);
    }
}
