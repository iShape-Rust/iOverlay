#[cfg(test)]
mod tests {
    use i_float::int::number::int::IntNumber;
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::integer::OverlayInt;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;
    use i_overlay::float::overlay::FloatOverlay;
    use i_shape::base::data::Path;
    use i_shape::int::path::IntPath;
    use i_shape::int::shape::IntShape;
    use rand::RngExt;
    use std::f64::consts::PI;

    const SOLVERS: [Solver; 4] = [Solver::LIST, Solver::TREE, Solver::FRAG, Solver::AUTO];

    trait TestInt: OverlayInt {}

    impl<I: OverlayInt> TestInt for I {}

    #[test]
    fn test_0() {
        test_0_as::<i16>();
        test_0_as::<i32>();
        test_0_as::<i64>();
    }

    fn test_0_as<I: TestInt>() {
        let scale = scale_for::<I>(2.0);
        let clip = create_star::<I>(1.0, 2.0, 7, 0.0, scale);
        for &solver in SOLVERS.iter() {
            let mut r = 0.9;
            while r < 1.2 {
                let mut a = 0.0;
                while a < 2.0 * PI {
                    let subj = create_star::<I>(1.0, r, 7, a, scale);

                    if let Some(graph) =
                        Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                            .build_graph_view(FillRule::NonZero)
                    {
                        graph.validate();
                        let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
                        assert!(!result.is_empty());
                    }
                    a += 0.01
                }
                r += 0.02
            }
        }
    }

    #[test]
    fn test_1() {
        test_1_as::<i16>();
        test_1_as::<i32>();
        test_1_as::<i64>();
    }

    fn test_1_as<I: TestInt>() {
        let scale = scale_for::<I>(200.0);
        let clip = create_star::<I>(200.0, 30.0, 7, 0.0, scale);
        for &solver in SOLVERS.iter() {
            let mut a = 0.0;
            while a < 4.0 * PI {
                let subj = create_star::<I>(200.0, 30.0, 7, a, scale);
                if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                    .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.005
            }
        }
    }

    #[test]
    fn test_2() {
        test_2_as::<i16>();
        test_2_as::<i32>();
        test_2_as::<i64>();
    }

    fn test_2_as<I: TestInt>() {
        let scale = scale_for::<I>(202.5);
        let clip = create_star::<I>(202.5, 33.75, 24, 0.0, scale);
        for &solver in SOLVERS.iter() {
            let mut a = 0.0;

            while a < 2.0 * PI {
                let subj = create_star::<I>(202.5, 33.75, 24, a, scale);
                if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                    .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.005
            }
        }
    }

    #[test]
    fn test_3() {
        test_3_as::<i16>();
        test_3_as::<i32>();
        test_3_as::<i64>();
    }

    fn test_3_as<I: TestInt>() {
        let scale = scale_for::<I>(100.0);
        let clip = create_star::<I>(100.0, 10.0, 17, 0.0, scale);
        for &solver in SOLVERS.iter() {
            let mut a = 0.0;

            while a < 4.0 * PI {
                let subj = create_star::<I>(100.0, 10.0, 17, a, scale);
                if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                    .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.005
            }
        }
    }

    #[test]
    fn test_4() {
        test_4_as::<i16>();
        test_4_as::<i32>();
        test_4_as::<i64>();
    }

    fn test_4_as<I: TestInt>() {
        let scale = scale_for::<I>(202.5);
        let clip = create_star::<I>(202.5, 33.75, 24, 0.0, scale);
        for &solver in SOLVERS.iter() {
            let mut a = -0.000_001;

            while a < 0.000_001 {
                let subj = create_star::<I>(202.5, 33.75, 24, a, scale);
                if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                    .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.000_000_01
            }
        }
    }

    #[test]
    fn test_5() {
        test_5_as::<i16>();
        test_5_as::<i32>();
        test_5_as::<i64>();
    }

    fn test_5_as<I: TestInt>() {
        let scale = scale_for::<I>(202.5);
        let clip = create_star::<I>(202.5, 33.75, 24, 0.0, scale);
        let a = -1E-6;
        let subj = create_star::<I>(202.5, 33.75, 24, a, scale);

        // println!("subj {:?}", subj);
        for &solver in SOLVERS.iter() {
            if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                .build_graph_view(FillRule::NonZero)
            {
                graph.validate();
                let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
            }
        }
    }

    #[test]
    fn test_6() {
        test_6_as::<i16>();
        test_6_as::<i32>();
        test_6_as::<i64>();
    }

    fn test_6_as<I: TestInt>() {
        let scale = scale_for::<I>(100.0);
        let clip = create_star::<I>(100.0, 50.0, 24, 0.0, scale);
        for &solver in SOLVERS.iter() {
            let mut a = -0.000_001;

            while a < 0.000_001 {
                let subj = create_star::<I>(100.0, 50.0, 24, a, scale);
                if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                    .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.000_000_1
            }
        }
    }

    #[test]
    fn test_7() {
        test_7_as::<i16>();
        test_7_as::<i32>();
        test_7_as::<i64>();
    }

    fn test_7_as<I: TestInt>() {
        let n = 1010;
        let scale = scale_for::<I>(1_000_000.0);
        let subj_paths = random_polygon::<I>(1_000_000.0, 0.0, n, scale);

        for &solver in SOLVERS.iter() {
            let mut overlay = Overlay::new_custom(n, Default::default(), solver);
            overlay.add_contour(&subj_paths, ShapeType::Subject);

            if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                graph.validate();
                let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                assert!(!result.is_empty());
            }
        }
    }

    #[test]
    fn test_8() {
        test_8_as::<i16>();
        test_8_as::<i32>();
        test_8_as::<i64>();
    }

    fn test_8_as<I: TestInt>() {
        for &solver in SOLVERS.iter() {
            let mut r = 0.004;
            while r < 1.0 {
                for n in 5..10 {
                    let subj_paths = random_polygon::<I>(r, 0.0, n, scale_for::<I>(r));

                    let mut overlay = Overlay::new_custom(n, Default::default(), solver);
                    overlay.add_contour(&subj_paths, ShapeType::Subject);

                    if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                        graph.validate();
                        let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                        assert!(!result.is_empty());
                    }
                }
                r += 0.001;
            }
        }
    }

    #[test]
    fn test_9() {
        test_9_as::<i16>();
        test_9_as::<i32>();
        test_9_as::<i64>();
    }

    fn test_9_as<I: TestInt>() {
        let s = 0.02;
        let r0 = s * 1.0;
        let scale = scale_for::<I>(s * 2.0);
        let clip = create_star::<I>(r0, s * 2.0, 4, 0.0, scale);
        for &solver in SOLVERS.iter() {
            let mut r = s * 0.9;
            while r < 1.2 * s {
                let mut a = 0.0;
                while a < 2.0 * PI {
                    let subj = create_star::<I>(r0, r, 4, a, scale);
                    if let Some(graph) =
                        Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                            .build_graph_view(FillRule::NonZero)
                    {
                        graph.validate();
                        let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
                        assert!(!result.is_empty());
                    }
                    a += 0.005
                }
                r += 0.01 * s
            }
        }
    }

    #[test]
    fn test_10() {
        test_10_as::<i16>();
        test_10_as::<i32>();
        test_10_as::<i64>();
    }

    fn test_10_as<I: TestInt>() {
        let solver = Solver::AUTO;
        let scale = scale_for::<I>(2.0);
        let clip = create_star::<I>(1.0, 2.0, 7, 0.0, scale);
        let a = 0.440_000_000_000_000_3;
        let r = 1.01;
        let subj = create_star::<I>(1.0, r, 7, a, scale);

        if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
            .build_graph_view(FillRule::NonZero)
        {
            graph.validate();
            let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_11() {
        test_11_as::<i16>();
        test_11_as::<i32>();
        test_11_as::<i64>();
    }

    fn test_11_as<I: TestInt>() {
        let n = 6;
        let scale = scale_for::<I>(100.0);
        for &solver in SOLVERS.iter() {
            for _ in 0..2000 {
                let subj_path = random_polygon::<I>(100.0, 0.0, n, scale);
                let clip_path = random_polygon::<I>(100.0, 0.5 * PI, n, scale);
                let mut overlay = Overlay::new_custom(2 * n, Default::default(), solver);

                overlay.add_contour(&subj_path, ShapeType::Subject);
                overlay.add_contour(&clip_path, ShapeType::Clip);

                if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                    graph.validate();
                    let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
                    assert!(!result.is_empty());
                }
            }
        }
    }

    #[test]
    fn test_12() {
        test_12_as::<i16>();
        // test_12_as::<i32>();
        // test_12_as::<i64>();
    }

    fn test_12_as<I: TestInt>() {
        let n = 5;
        for &solver in SOLVERS.iter() {
            for _ in 0..10000 {
                let subj_path = random::<I>(10, n);
                let mut overlay = Overlay::new_custom(2 * n, Default::default(), solver);
                overlay.add_contour(&subj_path, ShapeType::Subject);
                if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                }
            }
        }
    }

    #[test]
    fn test_13() {
        test_13_as::<i16>();
        test_13_as::<i32>();
        test_13_as::<i64>();
    }

    fn test_13_as<I: TestInt>() {
        let n = 5;
        for i in 1..10000 {
            let r = i as f64;
            let subj_path = random_float(r, n);
            let mut overlay =
                FloatOverlay::<_, I>::from_subj_custom(&subj_path, Default::default(), Default::default());
            if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                graph.graph.validate();
                let _ = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
            }
        }
    }

    #[test]
    fn test_14() {
        test_14_as::<i16>();
        test_14_as::<i32>();
        test_14_as::<i64>();
    }

    fn test_14_as<I: TestInt>() {
        let p = point::<I>(0, 0);
        let mut rng = rand::rng();
        let paths_count = 3;
        let mut subj_paths = Vec::with_capacity(paths_count);

        for _ in 0..100_000 {
            subj_paths.clear();
            let x_range = 0..=8;
            let y_range = -8..=8;
            for _ in 0..paths_count {
                let ax = rng.random_range(x_range.clone());
                let ay = rng.random_range(y_range.clone());
                let bx = rng.random_range(x_range.clone());
                let by = rng.random_range(y_range.clone());
                subj_paths.push(vec![p, point::<I>(ax, ay), point::<I>(bx, by)]);
            }

            let mut overlay = Overlay::new_custom(4, Default::default(), Default::default());
            overlay.add_contours(&subj_paths, ShapeType::Subject);
            if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                graph.validate();
                let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                assert!(!result.is_empty());
            }
        }
    }

    fn create_star<I: IntNumber>(r0: f64, r1: f64, count: usize, angle: f64, scale: f64) -> IntShape<I> {
        let da = PI / count as f64;
        let mut a = angle;

        let mut points = Vec::new();

        let sr0 = r0 * scale;
        let sr1 = r1 * scale;

        for _ in 0..count {
            let xr0 = I::from_float(sr0 * a.cos());
            let yr0 = I::from_float(sr0 * a.sin());

            a += da;

            let xr1 = I::from_float(sr1 * a.cos());
            let yr1 = I::from_float(sr1 * a.sin());

            a += da;

            points.push(IntPoint::new(xr0, yr0));
            points.push(IntPoint::new(xr1, yr1));
        }

        [points].to_vec()
    }

    fn random_polygon<I: IntNumber>(radius: f64, angle: f64, n: usize, scale: f64) -> IntPath<I> {
        let mut result = Vec::with_capacity(n);
        let da: f64 = PI * 0.7;
        let mut a: f64 = angle;
        let r = scale * radius;
        for _ in 0..n {
            let (sin, cos) = a.sin_cos();

            let x = r * cos;
            let y = r * sin;

            result.push(IntPoint::new(I::from_float(x), I::from_float(y)));
            a += da;
        }

        result
    }

    fn random<I: IntNumber>(radius: i32, n: usize) -> IntPath<I> {
        let a = radius / 2;
        let range = -a..=a;
        let mut points = Vec::with_capacity(n);
        let mut rng = rand::rng();
        for _ in 0..n {
            let x = rng.random_range(range.clone());
            let y = rng.random_range(range.clone());
            points.push(point(x, y))
        }

        points
    }

    fn point<I: IntNumber>(x: i32, y: i32) -> IntPoint<I> {
        IntPoint {
            x: I::from_float(x as f64),
            y: I::from_float(y as f64),
        }
    }

    fn scale_for<I: IntNumber>(max_abs_coord: f64) -> f64 {
        if max_abs_coord <= 0.0 {
            return 1024.0;
        }

        1024.0_f64.min(I::MAX.to_f64() * 0.25 / max_abs_coord)
    }

    fn random_float(radius: f64, n: usize) -> Path<[f64; 2]> {
        let a = 0.5 * radius;
        let range = -a..=a;
        let mut points = Vec::with_capacity(n);
        let mut rng = rand::rng();
        for _ in 0..n {
            let x = rng.random_range(range.clone());
            let y = rng.random_range(range.clone());
            points.push([x, y])
        }

        points
    }
}
