#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_shape::int::path::IntPath;
    use rand::Rng;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::extension::line::IntLine;
    use i_overlay::extension::slice::Slice;

    #[test]
    fn test_miss_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_line(
            &[IntPoint::new(-15, -20), IntPoint::new(-15, 20)],
            FillRule::NonZero, 0,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_edge_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_line(
            &[IntPoint::new(-10, -20), IntPoint::new(-10, 20)],
            FillRule::NonZero, 0,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_inside_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_line(
            &[IntPoint::new(0, -5), IntPoint::new(0, 5)],
            FillRule::NonZero, 0,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_middle_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_line(
            &[IntPoint::new(0, -20), IntPoint::new(0, 20)],
            FillRule::NonZero, 0,
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_cross_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_lines(
            &[
                [IntPoint::new(0, -20), IntPoint::new(0, 20)],
                [IntPoint::new(-20, 0), IntPoint::new(20, 0)]
            ],
            FillRule::NonZero, 0,
        );

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[2].len(), 1);
        assert_eq!(result[3].len(), 1);
    }

    #[test]
    fn test_cross_inside_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_lines(
            &[
                [IntPoint::new(0, -5), IntPoint::new(0, 5)],
                [IntPoint::new(-5, 0), IntPoint::new(5, 0)]
            ],
            FillRule::NonZero, 0,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
    }

    #[test]
    fn test_window() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let window = [
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 5),
            IntPoint::new(5, 5),
            IntPoint::new(5, -5)
        ].to_vec();

        let result = path.slice_by_path(&window, FillRule::NonZero, 0);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_2_windows() {
        let path = [
            IntPoint::new(-15, -15),
            IntPoint::new(-15, 15),
            IntPoint::new(15, 15),
            IntPoint::new(15, -15)
        ].to_vec();

        let win_0 = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let win_1 = [
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 5),
            IntPoint::new(5, 5),
            IntPoint::new(5, -5)
        ].to_vec();

        let result = path.slice_by_paths(&[win_0, win_1], FillRule::NonZero, 0);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 2);
        assert_eq!(result[2].len(), 1);
    }

    #[test]
    fn test_ideal_triangle() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let triangle = [
            [IntPoint::new(-5, 0), IntPoint::new(5, 0)],
            [IntPoint::new(-5, 0), IntPoint::new(0, 5)],
            [IntPoint::new( 5, 0), IntPoint::new(0, 5)],
        ].to_vec();

        let result = path.slice_by_lines(&triangle, FillRule::NonZero, 0);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_not_ideal_triangle() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let triangle = [
            [IntPoint::new(-7, 0), IntPoint::new(7, 0)],
            [IntPoint::new(-5, 0), IntPoint::new(0, 5)],
            [IntPoint::new( 5, 0), IntPoint::new(0, 5)],
        ].to_vec();

        let result = path.slice_by_lines(&triangle, FillRule::NonZero, 0);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_0() {
        let path = [
            IntPoint::new(2, -2),
            IntPoint::new(0, -1),
            IntPoint::new(1, 2)
        ].to_vec();

        let line = [IntPoint::new(2, 1), IntPoint::new(-1, -2)];
        let result = path.slice_by_line(&line, FillRule::NonZero, 0);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_1() {
        let path = [
            IntPoint::new(0, 2),
            IntPoint::new(0, -1),
            IntPoint::new(-1, -2)
        ].to_vec();

        let lines = [
            [IntPoint::new(-1, -2), IntPoint::new(-1, -1)],
            [IntPoint::new(1, -1), IntPoint::new(-2, -1)]
        ].to_vec();
        let result = path.slice_by_lines(&lines, FillRule::NonZero, 0);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[2].len(), 1);
    }

    #[test]
    fn test_random_0() {
        for _ in 0..5000 {
            let path = random_polygon(5, 3);
            let lines = random_lines(5, 1);
            let shapes = path.slice_by_lines(lines.as_slice(), FillRule::NonZero, 0);

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
            let shapes = path.slice_by_lines(lines.as_slice(), FillRule::NonZero, 0);

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
        for _ in 0..5000 {
            let path = random_polygon(10, 8);
            let lines = random_lines(10, 8);
            let shapes = path.slice_by_lines(lines.as_slice(), FillRule::NonZero, 0);

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
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x = rng.gen_range(range.clone());
            let y = rng.gen_range(range.clone());
            points.push(IntPoint { x, y })
        }

        points
    }

    fn random_lines(radius: i32, n: usize) -> Vec<IntLine> {
        let a = radius / 2;
        let range = -a..=a;
        let mut lines = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x0 = rng.gen_range(range.clone());
            let y0 = rng.gen_range(range.clone());
            let x1 = rng.gen_range(range.clone());
            let y1 = rng.gen_range(range.clone());
            lines.push([IntPoint {x: x0, y: y0}, IntPoint {x: x1, y: y1}])
        }

        lines
    }
}