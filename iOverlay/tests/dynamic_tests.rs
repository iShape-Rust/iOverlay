#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;
    use i_overlay::float::overlay::FloatOverlay;
    use i_shape::base::data::Path;
    use i_shape::int::path::IntPath;
    use i_shape::int::shape::IntShape;
    use rand::Rng;
    use std::f64::consts::PI;

    const SOLVERS: [Solver; 3] = [Solver::LIST, Solver::TREE, Solver::AUTO];

    #[test]
    fn test_0() {
        let clip = create_star(1.0, 2.0, 7, 0.0);
        for &solver in SOLVERS.iter() {
            let mut r = 0.9;
            while r < 1.2 {
                let mut a = 0.0;
                while a < 2.0 * PI {
                    let subj = create_star(1.0, r, 7, a);

                    if let Some(graph) =
                        Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                            .build_graph_view(FillRule::NonZero)
                    {
                        graph.validate();
                        let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
                        assert!(result.len() > 0);
                    }
                    a += 0.005
                }
                r += 0.01
            }
        }
    }

    #[test]
    fn test_1() {
        let clip = create_star(200.0, 30.0, 7, 0.0);
        for &solver in SOLVERS.iter() {
            let mut a = 0.0;
            while a < 4.0 * PI {
                let subj = create_star(200.0, 30.0, 7, a);
                if let Some(graph) =
                    Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                        .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.001
            }
        }
    }

    #[test]
    fn test_2() {
        let clip = create_star(202.5, 33.75, 24, 0.0);
        for &solver in SOLVERS.iter() {
            let mut a = 0.0;

            while a < 2.0 * PI {
                let subj = create_star(202.5, 33.75, 24, a);
                if let Some(graph) =
                    Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                        .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.001
            }
        }
    }

    #[test]
    fn test_3() {
        let clip = create_star(100.0, 10.0, 17, 0.0);
        for &solver in SOLVERS.iter() {
            let mut a = 0.0;

            while a < 4.0 * PI {
                let subj = create_star(100.0, 10.0, 17, a);
                if let Some(graph) =
                    Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                        .build_graph_view(FillRule::NonZero)
                {
                    graph.validate();
                    let _ = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                }
                a += 0.001
            }
        }
    }

    #[test]
    fn test_4() {
        let clip = create_star(202.5, 33.75, 24, 0.0);
        for &solver in SOLVERS.iter() {
            let mut a = -0.000_001;

            while a < 0.000_001 {
                let subj = create_star(202.5, 33.75, 24, a);
                if let Some(graph) =
                    Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
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
        let clip = create_star(202.5, 33.75, 24, 0.0);
        let a = -9.9999999999999995E-7;
        let subj = create_star(202.5, 33.75, 24, a);

        // println!("subj {:?}", subj);
        for &solver in SOLVERS.iter() {
            if let Some(graph) =
                Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                    .build_graph_view(FillRule::NonZero)
            {
                graph.validate();
                let result = graph.extract_shapes(OverlayRule::Xor, &mut Default::default());
                assert!(result.len() > 0);
            }
        }
    }

    #[test]
    fn test_6() {
        let clip = create_star(100.0, 50.0, 24, 0.0);
        for &solver in SOLVERS.iter() {
            let mut a = -0.000_001;

            while a < 0.000_001 {
                let subj = create_star(100.0, 50.0, 24, a);
                if let Some(graph) =
                    Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
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
        let n = 1010;
        let subj_paths = random_polygon(1000_000.0, 0.0, n);

        let mut overlay = Overlay::new(n);
        overlay.add_contour(&subj_paths, ShapeType::Subject);

        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.validate();
            let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
            assert!(result.len() > 0);
        }
    }

    #[test]
    fn test_8() {
        for &solver in SOLVERS.iter() {
            let mut r = 0.004;
            while r < 1.0 {
                for n in 5..10 {
                    let subj_paths = random_polygon(r, 0.0, n);

                    let mut overlay = Overlay::new_custom(n, Default::default(), solver);
                    overlay.add_contour(&subj_paths, ShapeType::Subject);

                    if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                        graph.validate();
                        let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                        assert!(result.len() > 0);
                    }
                }
                r += 0.001;
            }
        }
    }

    #[test]
    fn test_9() {
        let s = 0.02;
        let r0 = s * 1.0;
        let clip = create_star(r0, s * 2.0, 4, 0.0);
        for &solver in SOLVERS.iter() {
            let mut r = s * 0.9;
            while r < 1.2 * s {
                let mut a = 0.0;
                while a < 2.0 * PI {
                    let subj = create_star(r0, r, 4, a);
                    if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
                        .build_graph_view(FillRule::NonZero)
                    {
                        graph.validate();
                        let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
                        assert!(result.len() > 0);
                    }
                    a += 0.005
                }
                r += 0.01 * s
            }
        }
    }

    #[test]
    fn test_10() {
        let solver = Solver::AUTO;
        let clip = create_star(1.0, 2.0, 7, 0.0);
        let a = 0.44000000000000028;
        let r = 1.01;
        let subj = create_star(1.0, r, 7, a);

        if let Some(graph) = Overlay::with_contours_custom(&subj, &clip, Default::default(), solver)
            .build_graph_view(FillRule::NonZero)
        {
            graph.validate();
            let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
            assert!(result.len() > 0);
        }
    }

    #[test]
    fn test_11() {
        let n = 6;
        for _ in 0..10000 {
            let subj_path = random_polygon(100.0, 0.0, n);
            let clip_path = random_polygon(100.0, 0.5 * PI, n);
            let mut overlay = Overlay::new(2 * n);

            overlay.add_contour(&subj_path, ShapeType::Subject);
            overlay.add_contour(&clip_path, ShapeType::Clip);

            if let Some(graph) =
                overlay.build_graph_view(FillRule::NonZero)
            {
                graph.validate();
                let result = graph.extract_shapes(OverlayRule::Union, &mut Default::default());
                assert!(result.len() > 0);
            }
        }
    }

    #[test]
    fn test_12() {
        let n = 5;
        for _ in 0..10000 {
            let subj_path = random(10, n);
            let mut overlay = Overlay::new(2 * n);
            overlay.add_contour(&subj_path, ShapeType::Subject);
            if let Some(graph) =
                overlay.build_graph_view(FillRule::NonZero)
            {
                graph.validate();
                let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                assert!(result.len() > 0);
            }
        }
    }

    #[test]
    fn test_13() {
        let n = 5;
        for i in 1..50000 {
            let r = i as f64;
            let subj_path = random_float(r, n);
            let mut overlay = FloatOverlay::with_subj(&subj_path);
            if let Some(graph) =
                overlay.build_graph_view(FillRule::NonZero)
            {
                graph.graph.validate();
                let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                assert!(result.len() > 0);
            }
        }
    }

    #[test]
    fn test_14() {
        let p = IntPoint::new(0, 0);
        let mut rng = rand::rng();
        let paths_count = 3;
        let mut subj_paths = Vec::with_capacity(paths_count);
        for _ in 0..1000_000 {
            subj_paths.clear();
            let x_range = 0..=8;
            let y_range = -8..=8;
            for _ in 0..paths_count {
                let ax = rng.random_range(x_range.clone());
                let ay = rng.random_range(y_range.clone());
                let bx = rng.random_range(x_range.clone());
                let by = rng.random_range(y_range.clone());
                subj_paths.push(vec![p, IntPoint::new(ax, ay), IntPoint::new(bx, by)]);
            }

            let mut overlay = Overlay::new(4);
            overlay.add_contours(&subj_paths, ShapeType::Subject);
            if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
                graph.validate();
                let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
                assert!(result.len() > 0);
            }
        }
    }

    #[test]
    fn test_15() {
        let subj_paths = [
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(1, 6),
                IntPoint::new(6, 4),
            ],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(6, 5),
                IntPoint::new(2, -2),
            ],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(3, -1),
                IntPoint::new(1, 3),
            ],
        ];
        let mut overlay = Overlay::new(4);
        overlay.add_contours(&subj_paths, ShapeType::Subject);
        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.validate();
            let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
            assert!(result.len() > 0);
        }
    }

    fn create_star(r0: f64, r1: f64, count: usize, angle: f64) -> IntShape {
        let da = PI / count as f64;
        let mut a = angle;

        let mut points = Vec::new();

        let sr0 = r0 * 1024.0;
        let sr1 = r1 * 1024.0;

        for _ in 0..count {
            let xr0 = (sr0 * a.cos()) as i32;
            let yr0 = (sr0 * a.sin()) as i32;

            a += da;

            let xr1 = (sr1 * a.cos()) as i32;
            let yr1 = (sr1 * a.sin()) as i32;

            a += da;

            points.push(IntPoint::new(xr0, yr0));
            points.push(IntPoint::new(xr1, yr1));
        }

        [points].to_vec()
    }

    fn random_polygon(radius: f64, angle: f64, n: usize) -> IntPath {
        let mut result = Vec::with_capacity(n);
        let da: f64 = PI * 0.7;
        let mut a: f64 = angle;
        let r = 1024.0 * radius;
        for _ in 0..n {
            let (sin, cos) = a.sin_cos();

            let x = r * cos;
            let y = r * sin;

            result.push(IntPoint::new(x as i32, y as i32));
            a += da;
        }

        result
    }

    fn random(radius: i32, n: usize) -> IntPath {
        let a = radius / 2;
        let range = -a..=a;
        let mut points = Vec::with_capacity(n);
        let mut rng = rand::rng();
        for _ in 0..n {
            let x = rng.random_range(range.clone());
            let y = rng.random_range(range.clone());
            points.push(IntPoint { x, y })
        }

        points
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
