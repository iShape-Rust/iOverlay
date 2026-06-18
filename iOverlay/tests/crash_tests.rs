#[cfg(test)]
mod tests {
    use i_float::adapter::FloatPointAdapter;
    use i_float::int::number::int::IntNumber;
    use i_float::int::point::IntPoint;
    use i_key_sort::sort::key::SortKey;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::{Precision, Solver, Strategy};
    use i_overlay::float::overlay::{FloatOverlay, OverlayOptions};
    use i_shape::base::data::{Path, Shape};
    use i_shape::{int_path, int_shape};
    use i_tree::{Expiration, LayoutNumber};

    trait TestInt: IntNumber + Expiration + LayoutNumber + SortKey {}
    impl<I> TestInt for I where I: IntNumber + Expiration + LayoutNumber + SortKey {}
    const SOLVERS: [Solver; 4] = [Solver::LIST, Solver::TREE, Solver::FRAG, Solver::AUTO];

    fn point<I: IntNumber>(x: i32, y: i32) -> IntPoint<I> {
        IntPoint {
            x: I::from_float(x as f64),
            y: I::from_float(y as f64),
        }
    }

    #[test]
    fn test_00() {
        let subj: Shape<_> = int_shape![
            [[0i16, 0], [0, 4], [3, -5]],
            [[0, 0], [1, 7], [2, -8]],
            [[0, 0], [4, -4], [5, 7]],
        ];

        let solver = Solver {
            strategy: Strategy::List,
            precision: Precision {
                start: 0,
                progression: 1,
            },
            multithreading: None,
        };

        let mut overlay = Overlay::new_custom(4, Default::default(), solver);
        overlay.add_contours(&subj, ShapeType::Subject);
        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.validate();
            let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_01() {
        let subj = [
            [-117.04171489206965, 1820.3621519926919],
            [4619.6817058891429, -2133.11539650432],
            [1902.5599837294722, -133.53167784432389],
            [-3572.1275050425684, 3909.4677532724309],
            [3047.0491344383845, -4087.6336157702817],
        ];

        let solver = Solver {
            strategy: Strategy::Frag,
            precision: Precision {
                start: 0,
                progression: 1,
            },
            multithreading: None,
        };

        let mut overlay = FloatOverlay::<_, i16>::from_subj_custom(&subj, Default::default(), solver);
        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.graph.validate();
            let _ = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
        }
    }

    #[test]
    fn test_02() {
        test_02_as::<i16>();
        test_02_as::<i32>();
        test_02_as::<i64>();
    }

    fn test_02_as<I: TestInt>() {
        let subj_paths = [
            vec![point::<I>(0, 0), point(1, 6), point(6, 4)],
            vec![point::<I>(0, 0), point(6, 5), point(2, -2)],
            vec![point::<I>(0, 0), point(3, -1), point(1, 3)],
        ];
        for &solver in SOLVERS.iter() {
            let mut overlay = Overlay::new_custom(4, Default::default(), solver);
            overlay.add_contours(&subj_paths, ShapeType::Subject);
            if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                graph.validate();
                let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                assert!(!result.is_empty());
            }
        }
    }

    #[test]
    fn test_03() {
        let subj: Path<_> = int_path![[3, 4], [5, 0], [3, 3], [4, 2], [5, -2]];

        let solver = Solver {
            strategy: Strategy::Tree,
            precision: Precision {
                start: 0,
                progression: 1,
            },
            multithreading: None,
        };

        let mut overlay = Overlay::new_custom(10, Default::default(), solver);
        overlay.add_contour(&subj, ShapeType::Subject);
        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.validate();
            let _ = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
        }
    }

    #[test]
    fn test_04() {
        let subj: Path<IntPoint<i16>> = int_path![[-4, -2], [1, -3], [-1, 3], [1, -4], [4, -3]];

        let solver = Solver {
            strategy: Strategy::Tree,
            precision: Precision {
                start: 0,
                progression: 1,
            },
            multithreading: None,
        };

        let mut overlay = Overlay::new_custom(10, Default::default(), solver);
        overlay.add_contour(&subj, ShapeType::Subject);
        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.validate();
            let _ = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
        }
    }

    #[test]
    fn test_05() {
        let subj = vec![
            vec![
                [24902.9222201258, 11129.9683052215],
                [24821.9592401258, 11107.1269052215],
                [24902.9218201258, 11129.9681852215],
                [24898.9601001258, 11128.8505052215],
            ],
            vec![
                [20094.9253001258, 12125.6660652215],
                [20094.9253001258, 12125.6647652215],
                [29795.5156201258, 10942.5275852215],
            ],
            vec![
                [24902.2200401258, 11129.7702052215],
                [24902.3098801258, 11129.7955452215],
                [24902.4788601258, 11129.8432252215],
            ],
            vec![
                [24902.4819801258, 11129.8441052215],
                [24902.4832001258, 11129.8444452215],
                [24902.4821401258, 11129.8441452215],
            ],
        ];
        let points = subj.iter().flatten().collect::<Vec<_>>();
        let adapter =
            FloatPointAdapter::<_, i64>::with_iter_and_scale_checked(points.into_iter(), 50_000.0).unwrap();
        let mut options = OverlayOptions::default();
        options.clean_result = true;
        options.ogc = true;
        options.preserve_output_collinear = true;
        let mut overlay = FloatOverlay::new_custom(adapter, options, Default::default(), 13)
            .unsafe_add_source(&subj, ShapeType::Subject);

        let _ = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);
    }
}
