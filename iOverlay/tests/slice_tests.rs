#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::string::line::IntLine;
    use i_overlay::string::slice::IntSlice;
    use i_shape::int::path::IntPath;
    use rand::Rng;

    #[test]
    fn test_miss_slice() {
        let path = vec![
            IntPoint::new(10, -10),
            IntPoint::new(10, 10),
            IntPoint::new(-10, 10),
            IntPoint::new(-10, -10),
        ];

        let result = path.slice_by_line(
            [IntPoint::new(-15, -20), IntPoint::new(-15, 20)],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_edge_slice() {
        let path = vec![
            IntPoint::new(10, -10),
            IntPoint::new(10, 10),
            IntPoint::new(-10, 10),
            IntPoint::new(-10, -10),
        ];

        let result = path.slice_by_line(
            [IntPoint::new(-10, -20), IntPoint::new(-10, 20)],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_inside_slice() {
        let path = vec![
            IntPoint::new(10, -10),
            IntPoint::new(10, 10),
            IntPoint::new(-10, 10),
            IntPoint::new(-10, -10),
        ];

        let result = path.slice_by_line(
            [IntPoint::new(0, -5), IntPoint::new(0, 5)],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_middle_slice() {
        let path = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let result = path.slice_by_line(
            [IntPoint::new(0, -20), IntPoint::new(0, 20)],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_cross_slice() {
        let path = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let cross = [
            [IntPoint::new(0, -20), IntPoint::new(0, 20)],
            [IntPoint::new(-20, 0), IntPoint::new(20, 0)],
        ];

        let result = path.slice_by_lines(&cross, FillRule::NonZero);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[2].len(), 1);
        assert_eq!(result[3].len(), 1);
    }

    #[test]
    fn test_cross_inside_slice() {
        let path = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let cross = [
            [IntPoint::new(0, -5), IntPoint::new(0, 5)],
            [IntPoint::new(-5, 0), IntPoint::new(5, 0)],
        ];

        let result = path.slice_by_lines(&cross, FillRule::NonZero);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
    }

    #[test]
    fn test_window() {
        let path = vec![
            IntPoint::new(10, -10),
            IntPoint::new(10, 10),
            IntPoint::new(-10, 10),
            IntPoint::new(-10, -10),
        ];

        let window = vec![
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 5),
            IntPoint::new(5, 5),
            IntPoint::new(5, -5),
            IntPoint::new(-5, -5), // close
        ];

        let result = path.slice_by_path(&window, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_2_windows() {
        let path = vec![
            IntPoint::new(15, -15),
            IntPoint::new(15, 15),
            IntPoint::new(-15, 15),
            IntPoint::new(-15, -15),
        ];

        let win_0 = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
            IntPoint::new(-10, -10), // close
        ];

        let win_1 = vec![
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 5),
            IntPoint::new(5, 5),
            IntPoint::new(5, -5),
            IntPoint::new(-5, -5), // close
        ];

        let result = path.slice_by_paths(&[win_0, win_1], FillRule::NonZero);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 2);
        assert_eq!(result[2].len(), 1);
    }

    #[test]
    fn test_ideal_triangle() {
        let path = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let triangle = [
            [IntPoint::new(-5, 0), IntPoint::new(5, 0)],
            [IntPoint::new(-5, 0), IntPoint::new(0, 5)],
            [IntPoint::new(5, 0), IntPoint::new(0, 5)],
        ];

        let result = path.slice_by_lines(&triangle, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_not_ideal_triangle() {
        let path = vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ];

        let triangle = [
            [IntPoint::new(-7, 0), IntPoint::new(7, 0)],
            [IntPoint::new(-5, 0), IntPoint::new(0, 5)],
            [IntPoint::new(5, 0), IntPoint::new(0, 5)],
        ];

        let result = path.slice_by_lines(&triangle, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_bridge_to_triangle_0() {
        let path = vec![
            IntPoint::new(-4, -4),
            IntPoint::new(-4, 4),
            IntPoint::new(4, 4),
            IntPoint::new(4, -4),
        ];

        let triangle = [
            [IntPoint::new(0, 2), IntPoint::new(0, 1)],
            [IntPoint::new(-1, -1), IntPoint::new(0, 1)],
            [IntPoint::new(-1, -1), IntPoint::new(1, -1)],
            [IntPoint::new(0, 1), IntPoint::new(1, -1)],
        ];

        let result = path.slice_by_lines(&triangle, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_bridge_to_triangle_1() {
        let path = vec![
            IntPoint::new(-2, -2),
            IntPoint::new(-2, 2),
            IntPoint::new(2, 2),
            IntPoint::new(2, -2),
        ];

        let triangle = [
            [IntPoint::new(-2, -2), IntPoint::new(-1, -1)],
            [IntPoint::new(-1, -1), IntPoint::new(0, 1)],
            [IntPoint::new(-1, -1), IntPoint::new(1, -1)],
            [IntPoint::new(0, 1), IntPoint::new(1, -1)],
        ];

        let result = path.slice_by_lines(&triangle, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_join_to_hole() {
        let shape = vec![
            vec![
                IntPoint::new(-3, -2),
                IntPoint::new(-3, 2),
                IntPoint::new(3, 2),
                IntPoint::new(3, -2),
            ],
            vec![
                IntPoint::new(0, -1),
                IntPoint::new(1, -1),
                IntPoint::new(1, 1),
                IntPoint::new(0, 1),
            ],
        ];

        let triangle = [
            [IntPoint::new(-2, -1), IntPoint::new(0, 0)],
            [IntPoint::new(-2, 1), IntPoint::new(0, 0)],
            [IntPoint::new(-2, -1), IntPoint::new(-2, 1)],
        ];

        let result = shape.slice_by_lines(&triangle, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 3);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_0() {
        let path = [
            IntPoint::new(2, -2),
            IntPoint::new(0, -1),
            IntPoint::new(1, 2),
        ];

        let line = [IntPoint::new(2, 1), IntPoint::new(-1, -2)];
        let result = path.slice_by_line(line, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_1() {
        let path = [
            IntPoint::new(0, 2),
            IntPoint::new(0, -1),
            IntPoint::new(-1, -2),
        ]
        .to_vec();

        let lines = [
            [IntPoint::new(-1, -2), IntPoint::new(-1, -1)],
            [IntPoint::new(1, -1), IntPoint::new(-2, -1)],
        ];

        let result = path.slice_by_lines(&lines, FillRule::NonZero);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[2].len(), 1);
    }

    #[test]
    fn test_2() {
        let path = [
            IntPoint::new(1, 4),
            IntPoint::new(-4, 4),
            IntPoint::new(-2, -4),
        ];

        let lines = [
            [IntPoint::new(1, 4), IntPoint::new(-2, 2)],
            [IntPoint::new(-4, 4), IntPoint::new(3, 3)],
            [IntPoint::new(-2, 2), IntPoint::new(-2, 1)],
        ];

        let result = path.slice_by_lines(&lines, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 3);
    }

    #[test]
    fn test_3() {
        let path = [
            IntPoint::new(-4, -2),
            IntPoint::new(2, 2),
            IntPoint::new(3, -3),
        ];

        let lines = [
            [IntPoint::new(-1, -2), IntPoint::new(1, 0)],
            [IntPoint::new(-2, -2), IntPoint::new(3, -1)],
            [IntPoint::new(-2, -4), IntPoint::new(2, -1)],
        ];

        let result = path.slice_by_lines(&lines, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_4() {
        let path = [
            IntPoint::new(4, 0),
            IntPoint::new(0, -3),
            IntPoint::new(-1, 3),
            IntPoint::new(3, 4),
        ];

        let lines = vec![
            [IntPoint::new(0, 3), IntPoint::new(0, -1)],
            [IntPoint::new(1, -2), IntPoint::new(1, 2)],
            [IntPoint::new(-1, 3), IntPoint::new(3, 0)],
            [IntPoint::new(2, 2), IntPoint::new(0, -1)],
        ];

        let result = path.slice_by_lines(&lines, FillRule::NonZero);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[0][1].len(), 4);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 4);
    }

    #[test]
    fn test_5() {
        let path = [
            IntPoint::new(1, -1),
            IntPoint::new(-1, -1),
            IntPoint::new(-2, -1),
        ];

        let lines = vec![
            [IntPoint::new(1, 1), IntPoint::new(0, 0)],
            [IntPoint::new(-1, -1), IntPoint::new(2, 2)],
        ];

        let result = path.slice_by_lines(&lines, FillRule::NonZero);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_random_0() {
        for _ in 0..5000 {
            let path = random_polygon(5, 3);
            let lines = random_lines(5, 1);

            let shapes = path.slice_by_lines(&lines, FillRule::NonZero);

            for shape in shapes.iter() {
                assert!(shape.len() >= 1);
                for path in shape.iter() {
                    assert!(path.len() > 2);
                }
            }
        }
    }

    #[test]
    fn test_random_1() {
        for _ in 0..5000 {
            let path = random_polygon(5, 3);
            let lines = random_lines(5, 2);

            let shapes = path.slice_by_lines(&lines, FillRule::NonZero);

            for shape in shapes.iter() {
                assert!(shape.len() >= 1);
                for path in shape.iter() {
                    assert!(path.len() > 2);
                }
            }
        }
    }

    #[test]
    fn test_random_2() {
        for _ in 0..50000 {
            let path = random_polygon(8, 3);
            let lines = random_lines(8, 3);

            let shapes = path.slice_by_lines(&lines, FillRule::NonZero);

            for shape in shapes.iter() {
                assert!(shape.len() >= 1);
                for path in shape.iter() {
                    assert!(path.len() > 2);
                }
            }
        }
    }

    #[test]
    fn test_random_3() {
        for _ in 0..50000 {
            let path = random_polygon(8, 4);
            let lines = random_lines(8, 4);

            let shapes = path.slice_by_lines(&lines, FillRule::NonZero);

            for shape in shapes.iter() {
                assert!(shape.len() >= 1);
                for path in shape.iter() {
                    assert!(path.len() > 2);
                }
            }
        }
    }

    #[test]
    fn test_random_4() {
        for _ in 0..50000 {
            let path = random_polygon(8, 8);
            let lines = random_lines(8, 8);

            let shapes = path.slice_by_lines(&lines, FillRule::NonZero);

            for shape in shapes.iter() {
                assert!(shape.len() >= 1);
                for path in shape.iter() {
                    assert!(path.len() > 2);
                }
            }
        }
    }

    fn random_polygon(radius: i32, n: usize) -> IntPath {
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

    fn random_lines(radius: i32, n: usize) -> Vec<IntLine> {
        let a = radius / 2;
        let range = -a..=a;
        let mut lines = Vec::with_capacity(n);
        let mut rng = rand::rng();
        for _ in 0..n {
            let x0 = rng.random_range(range.clone());
            let y0 = rng.random_range(range.clone());
            let x1 = rng.random_range(range.clone());
            let y1 = rng.random_range(range.clone());
            lines.push([IntPoint { x: x0, y: y0 }, IntPoint { x: x1, y: y1 }])
        }

        lines
    }
}
