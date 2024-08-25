#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use i_float::f64_point::F64Point;
    use i_float::point::IntPoint;
    use i_shape::f64::shape::F64Path;
    use i_shape::int::path::IntPath;
    use i_shape::int::shape::IntShape;
    use rand::Rng;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;
    use i_overlay::f64::overlay::F64Overlay;

    const SOLVERS: [Solver; 3] = [
        Solver::LIST,
        Solver::TREE,
        Solver::AUTO
    ];

    #[test]
    fn test_0() {
        let clip = create_star(1.0, 2.0, 7, 0.0);
        for &solver in SOLVERS.iter() {
            let mut r = 0.9;
            while r < 1.2 {
                let mut a = 0.0;
                while a < 2.0 * PI {
                    let subj = create_star(1.0, r, 7, a);

                    let overlay = Overlay::with_paths(&subj, &clip);
                    let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
                    let result = graph.extract_shapes(OverlayRule::Union);
                    assert!(result.len() > 0);
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
                let overlay = Overlay::with_paths(&subj, &clip);
                let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
                let result = graph.extract_shapes(OverlayRule::Xor);
                assert!(result.len() > 1 || result.len() == 0);
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
                let overlay = Overlay::with_paths(&subj, &clip);
                let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
                let result = graph.extract_shapes(OverlayRule::Xor);
                assert!(result.len() > 1 || result.len() == 0);
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
                let overlay = Overlay::with_paths(&subj, &clip);
                let graph = overlay.into_graph_with_solver(FillRule::EvenOdd, solver);
                let result = graph.extract_shapes(OverlayRule::Xor);
                assert!(result.len() > 1 || result.len() == 0);
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
                let overlay = Overlay::with_paths(&subj, &clip);
                let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
                let result = graph.extract_shapes(OverlayRule::Xor);
                assert!(result.len() > 1 || result.len() == 0);
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
            let overlay = Overlay::with_paths(&subj, &clip);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
            let result = graph.extract_shapes(OverlayRule::Xor);
            assert!(result.len() > 1 || result.len() == 0);
        }
    }

    #[test]
    fn test_6() {
        let clip = create_star(100.0, 50.0, 24, 0.0);
        for &solver in SOLVERS.iter() {
            let mut a = -0.000_001;

            while a < 0.000_001 {
                let subj = create_star(100.0, 50.0, 24, a);
                let overlay = Overlay::with_paths(&subj, &clip);
                let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
                let result = graph.extract_shapes(OverlayRule::Xor);
                assert!(result.len() > 1 || result.len() == 0);
                a += 0.000_000_1
            }
        }
    }

    #[test]
    fn test_7() {
        let n = 1010;
        let subj_paths = random_polygon(1000_000.0, 0.0, n);


        let mut overlay = Overlay::new(n);
        overlay.add_path(&subj_paths, ShapeType::Subject);

        let graph = overlay.into_graph_with_solver(FillRule::NonZero, Solver::AUTO);
        let result = graph.extract_shapes(OverlayRule::Subject);

        assert!(!result.is_empty());
    }

    #[test]
    fn test_8() {
        for &solver in SOLVERS.iter() {
            let mut r = 0.004;
            while r < 1.0 {
                for n in 5..10 {
                    let subj_paths = random_polygon(r, 0.0, n);

                    let mut overlay = Overlay::new(n);
                    overlay.add_path(&subj_paths, ShapeType::Subject);

                    let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
                    _ = graph.extract_shapes(OverlayRule::Subject);
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

                    let overlay = Overlay::with_paths(&subj, &clip);
                    let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
                    let result = graph.extract_shapes(OverlayRule::Union);
                    assert!(result.len() > 0);
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

        let overlay = Overlay::with_paths(&subj, &clip);
        let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
        let result = graph.extract_shapes(OverlayRule::Union);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_11() {
        let n = 6;
        for _ in 0..10000 {
            let subj_path = random_polygon(100.0, 0.0, n);
            let clip_path = random_polygon(100.0, 0.5 * PI, n);
            let mut overlay = Overlay::new(2 * n);

            overlay.add_path(&subj_path, ShapeType::Subject);
            overlay.add_path(&clip_path, ShapeType::Clip);

            let graph = overlay.into_graph_with_solver(FillRule::NonZero, Solver::AUTO);
            _ = graph.extract_shapes(OverlayRule::Union);
        }
    }

    #[test]
    fn test_12() {
        let n = 5;
        for _ in 0..10000 {
            let subj_path = random(10, n);
            let mut overlay = Overlay::new(2 * n);
            overlay.add_path(&subj_path, ShapeType::Subject);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, Solver::AUTO);
            _ = graph.extract_shapes(OverlayRule::Subject);
        }
    }

    #[test]
    fn test_13() {
        let n = 5;
        for i in 1..50000 {
            let r = i as f64;
            let subj_path = random_float(r, n);

            let mut overlay = F64Overlay::new();
            overlay.add_path(subj_path, ShapeType::Subject);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, Solver::AUTO);
            _ = graph.extract_shapes(OverlayRule::Subject);
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
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x = rng.gen_range(range.clone());
            let y = rng.gen_range(range.clone());
            points.push(IntPoint { x, y })
        }

        points
    }

    fn random_float(radius: f64, n: usize) -> F64Path {
        let a = 0.5 * radius;
        let range = -a..=a;
        let mut points = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x = rng.gen_range(range.clone());
            let y = rng.gen_range(range.clone());
            points.push(F64Point { x, y })
        }

        points
    }
}