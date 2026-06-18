#[cfg(test)]
mod tests {
    #![allow(clippy::explicit_counter_loop)]

    use i_float::adapter::FloatPointAdapter;
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{ContourDirection, IntOverlayOptions, Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::overlay::{FloatOverlay, OverlayOptions};
    use i_shape::int::area::Area;
    use i_shape::{int_path, int_shape};
    use std::f64::consts::PI;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::time::{Duration, Instant};

    #[test]
    fn test_0() {
        //     0   1   2   3   4   5
        //   5 ┌───────────────────┐
        //     │                   │
        //   4 │   ┌───────┐       │
        //     │   │ ░   ░ │       │   Two L-shaped holes share vertices at (2,2) and (3,3)
        //   3 │   │   ┌───●───┐   │
        //     │   │ ░ │   │ ░ │   │   ░ = holes
        //   2 │   └───●───┘   │   │
        //     │       │ ░   ░ │   │   The shared edge disconnects the interior
        //   1 │       └───────┘   │
        //     │                   │
        //   0 └───────────────────┘
        //
        // OGC Simple Feature Specification (ISO 19125-1) states:
        // "The interior of every Surface is a connected point set."

        let subj_paths = int_shape![[[0, 0], [5, 0], [5, 5], [0, 5]]];

        let clip_paths = int_shape![
            [[1, 2], [1, 4], [3, 4], [3, 3], [2, 3], [2, 2]],
            [[2, 1], [2, 2], [3, 2], [3, 3], [4, 3], [4, 1]],
        ];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[0][1].len(), 8);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 4);
    }

    #[test]
    fn test_0_invert() {
        //     0   1   2   3   4   5
        //   5 ┌───────────────────┐
        //     │                   │
        //   4 │   ┌───────┐       │
        //     │   │ ░   ░ │       │   Two L-shaped holes share vertices at (2,2) and (3,3)
        //   3 │   │   ┌───●───┐   │
        //     │   │ ░ │   │ ░ │   │   ░ = holes
        //   2 │   └───●───┘   │   │
        //     │       │ ░   ░ │   │   The shared edge disconnects the interior
        //   1 │       └───────┘   │
        //     │                   │
        //   0 └───────────────────┘
        //
        // OGC Simple Feature Specification (ISO 19125-1) states:
        // "The interior of every Surface is a connected point set."

        let subj_paths = int_shape![[[0, 0], [5, 0], [5, 5], [0, 5]]];

        let clip_paths = int_shape![
            [[1, 2], [1, 4], [3, 4], [3, 3], [2, 3], [2, 2]],
            [[2, 1], [2, 2], [3, 2], [3, 3], [4, 3], [4, 1]],
        ];

        let mut opts = IntOverlayOptions::ogc();
        opts.output_direction = ContourDirection::Clockwise;

        let mut overlay = Overlay::with_contours_custom(&subj_paths, &clip_paths, opts, Default::default());

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[0][1].len(), 8);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 4);
    }

    #[test]
    fn test_1() {
        //     0   1   2   3   4   5
        //   5 ┌───────────────────┐
        //     │                   │
        //   4 │       ┌───┐       │
        //     │       │ ░ │       │
        //   3 │   ┌───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │
        //   2 │   └───●───●───┘   │
        //     │       │ ░ │       │
        //   1 │       └───┘       │
        //     │                   │
        //   0 └───────────────────┘

        let subj_paths = int_shape![[[0, 0], [5, 0], [5, 5], [0, 5]]];

        let clip_paths = int_shape![
            [[1, 2], [1, 3], [2, 3], [2, 2]],
            [[2, 1], [2, 2], [3, 2], [3, 1]],
            [[2, 3], [2, 4], [3, 4], [3, 3]],
            [[3, 2], [3, 3], [4, 3], [4, 2]],
        ];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[0][1].len(), 12);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 4);
    }

    #[test]
    fn test_2() {
        //     0   1   2   3   4   5   6   7
        //   7 ┌───────────────────────────┐
        //     │                           │
        //   6 │           ┌───┐           │
        //     │           │ ░ │           │
        //   5 │       ┌───●───●───┐       │
        //     │       │ ░ │   │ ░ │       │
        //   4 │   ┌───●───●───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │
        //   3 │   └───●───●───●───●───┘   │
        //     │       │ ░ │   │ ░ │       │
        //   2 │       └───●───●───┘       │
        //     │           │ ░ │           │
        //   1 │           └───┘           │
        //     │                           │
        //   0 └───────────────────────────┘

        let subj_paths = int_shape![[[0, 0], [7, 0], [7, 7], [0, 7]]];

        let clip_paths = int_shape![
            [[1, 3], [1, 4], [2, 4], [2, 3]],
            [[2, 2], [2, 3], [3, 3], [3, 2]],
            [[2, 4], [2, 5], [3, 5], [3, 4]],
            [[3, 1], [3, 2], [4, 2], [4, 1]],
            [[3, 3], [3, 4], [4, 4], [4, 3]],
            [[3, 5], [3, 6], [4, 6], [4, 5]],
            [[4, 2], [4, 3], [5, 3], [5, 2]],
            [[4, 4], [4, 5], [5, 5], [5, 4]],
            [[5, 3], [5, 4], [6, 4], [6, 3]],
        ];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[2].len(), 1);
        assert_eq!(result[3].len(), 1);
        assert_eq!(result[4].len(), 1);
    }

    #[test]
    fn test_3() {
        //     0   1   2   3
        //   3 ┌───────┐
        //     │       │
        //   2 │   ┌───●───┐
        //     │   │ ░ │   │
        //   1 │   └───┘   │
        //     │           │
        //   0 └───────────┘

        let subj_paths = int_shape![[[0, 3], [0, 0], [3, 0], [3, 2], [1, 2], [1, 1], [2, 1], [2, 3]]];

        let mut overlay =
            Overlay::with_contours_custom(&subj_paths, &[], IntOverlayOptions::ogc(), Default::default());

        let result = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][0].len(), 6);
        assert_eq!(result[0][1].len(), 4);
    }

    #[test]
    fn test_4() {
        //     0   1   2   3   4
        //   4 ┌───────────┐
        //     │           │
        //   3 │       ┌───●───┐
        //     │       │ ░ │   │
        //   2 │   ┌───●───┘   │
        //     │   │ ░ │       │
        //   1 │   └───┘       │
        //     │               │
        //   0 └───────────────┘

        let subj_paths = int_shape![[[0, 4], [0, 0], [4, 0], [4, 3], [3, 3], [3, 4]]];

        let clip_paths = int_shape![[[1, 2], [1, 1], [2, 1], [2, 2]], [[2, 3], [2, 2], [3, 2], [3, 3]],];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(result[0][0].len(), 6);
        assert_eq!(result[0][1].len(), 4);
        assert_eq!(result[0][2].len(), 4);
    }

    #[test]
    fn test_5() {
        //     0   1   2   3   4
        //   4 ┌───────────────┐
        //     │               │
        //   3 │       ┌───┐   │
        //     │       │ ░ │   │
        //   2 │   ┌───●───┘   │
        //     │   │ ░ │       │
        //   1 │   └───┘       │
        //     │               │
        //   0 └───────────────┘

        let subj_paths = int_shape![[[0, 4], [0, 0], [4, 0], [4, 4]]];

        let clip_paths = int_shape![[[1, 2], [1, 1], [2, 1], [2, 2]], [[2, 3], [2, 2], [3, 2], [3, 3]],];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[0][1].len(), 4);
        assert_eq!(result[0][2].len(), 4);
    }

    #[test]
    fn test_5_invert() {
        //     0   1   2   3   4
        //   4 ┌───────────────┐
        //     │               │
        //   3 │       ┌───┐   │
        //     │       │ ░ │   │
        //   2 │   ┌───●───┘   │
        //     │   │ ░ │       │
        //   1 │   └───┘       │
        //     │               │
        //   0 └───────────────┘

        let subj_paths = int_shape![[[0, 4], [0, 0], [4, 0], [4, 4]]];

        let clip_paths = int_shape![[[1, 2], [1, 1], [2, 1], [2, 2]], [[2, 3], [2, 2], [3, 2], [3, 3]],];

        let mut opts = IntOverlayOptions::ogc();
        opts.output_direction = ContourDirection::Clockwise;

        let mut overlay = Overlay::with_contours_custom(&subj_paths, &clip_paths, opts, Default::default());

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[0][1].len(), 4);
        assert_eq!(result[0][2].len(), 4);
    }

    #[test]
    fn test_6() {
        //     0   1   2   3   4   5
        //   3 ┌───────┐   ┌───────┐
        //     │       │   │       │
        //   2 │   ┌───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │
        //   1 │   └───┘   └───┘   │
        //     │                   │
        //   0 └───────────────────┘

        let subj_paths = int_shape![[[0, 3], [0, 0], [5, 0], [5, 3], [3, 3], [3, 2], [2, 2], [2, 3]],];
        let clip_paths = int_shape![[[1, 2], [1, 1], [2, 1], [2, 2]], [[3, 2], [3, 1], [4, 1], [4, 2]],];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(result[0][0].len(), 8);
        assert_eq!(result[0][1].len(), 4);
        assert_eq!(result[0][2].len(), 4);
    }

    #[test]
    fn test_7() {
        //     0   1   2   3
        //   3     ┌───┐
        //         │   │
        //   2 ┌───●───●───┐
        //     │   │ ░ │   │
        //   1 └───●───●───┘
        //         │   │
        //   0     └───┘

        let subj_paths = int_shape![
            [[0, 2], [0, 1], [1, 1], [1, 2]],
            [[2, 2], [2, 1], [3, 1], [3, 2]],
            [[1, 1], [1, 0], [2, 0], [2, 1]],
            [[1, 3], [1, 2], [2, 2], [2, 3]],
        ];

        let mut overlay =
            Overlay::with_contours_custom(&subj_paths, &[], IntOverlayOptions::ogc(), Default::default());

        let result = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 4);
        assert_eq!(result[2].len(), 1);
        assert_eq!(result[2][0].len(), 4);
        assert_eq!(result[3].len(), 1);
        assert_eq!(result[3][0].len(), 4);
    }

    #[test]
    fn test_8() {
        //     0   1   2   3   4   5
        //   4 ┌───────┐   ┌───────┐
        //     │       │   │       │
        //   3 │   ┌───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │
        //   2 │   └───●───●───┘   │
        //     │       │ ░ │       │
        //   1 │       └───┘       │
        //     │                   │
        //   0 └───────────────────┘

        let subj_paths = int_shape![[
            [0, 4],
            [0, 0],
            [5, 0],
            [5, 4],
            [3, 4],
            [3, 3],
            [4, 3],
            [4, 2],
            [3, 2],
            [3, 1],
            [2, 1],
            [2, 2],
            [1, 2],
            [1, 3],
            [2, 3],
            [2, 4]
        ]];
        let clip_paths = int_shape![[[2, 3], [2, 2], [3, 2], [3, 3]]];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 16);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 4);
    }

    #[test]
    fn test_9() {
        let subj_paths = int_shape![
            [[-3, 0], [-3, -3], [0, -3], [0, 0], [3, 0], [3, 3], [0, 3], [0, 0]],
            [[-1, -2], [-2, -1], [0, 0], [1, 2], [2, 1], [0, 0]],
        ];

        let mut overlay =
            Overlay::with_contours_custom(&subj_paths, &[], IntOverlayOptions::ogc(), Default::default());

        let result = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[0][1].len(), 3);

        assert_eq!(result[1].len(), 2);
        assert_eq!(result[1][0].len(), 4);
        assert_eq!(result[1][1].len(), 3);
    }

    #[test]
    fn test_checkerboard_a() {
        for n in 4..50 {
            checkerboard_a(n);
        }
    }

    #[test]
    fn test_checkerboard_b() {
        for n in 3..50 {
            checkerboard_b(n);
        }
    }

    fn checkerboard_a(n: usize) {
        //     0   1   2   3   4   5   6   7   8   9
        //   9 ┌───────────────────────────────────┐
        //     │                                   │
        //   8 │       ┌───┐   ┌───┐   ┌───┐       │
        //     │       │ ░ │   │ ░ │   │ ░ │       │
        //   7 │   ┌───●───●───●───●───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │ ░ │   │
        //   6 │   └───●───●───●───●───●───●───┘   │
        //     │       │ ░ │   │ ░ │   │ ░ │       │
        //   5 │   ┌───●───●───●───●───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │ ░ │   │
        //   4 │   └───●───●───●───●───●───●───┘   │
        //     │       │ ░ │   │ ░ │   │ ░ │       │
        //   3 │   ┌───●───●───●───●───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │ ░ │   │
        //   2 │   └───●───●───●───●───●───●───┘   │
        //     │       │ ░ │   │ ░ │   │ ░ │       │
        //   1 │       └───┘   └───┘   └───┘       │
        //     │                                   │
        //   0 └───────────────────────────────────┘

        let mut subj_paths = Vec::new();

        let m = n as i32;

        let x0 = 1;
        let y0 = 1;
        let x1 = 2 * (m + 1);
        let y1 = 2 * (m + 1);

        subj_paths.push(int_path!(
            [x0 - 1, y1 + 1],
            [x0 - 1, y0 - 1],
            [x1 + 1, y0 - 1],
            [x1 + 1, y1 + 1]
        ));

        for i in 0..m {
            let x = 2 * (i + 1);
            let vr_line = int_path!([x, y0], [x, y1], [x + 1, y1], [x + 1, y0]);

            let y = 2 * (i + 1);
            let hz_line = int_path!([x0, y], [x0, y + 1], [x1, y + 1], [x1, y]);

            subj_paths.push(vr_line);
            subj_paths.push(hz_line);
        }

        let mut overlay =
            Overlay::with_contours_custom(&subj_paths, &[], IntOverlayOptions::ogc(), Default::default());

        let result = overlay.overlay(OverlayRule::Subject, FillRule::EvenOdd);

        let polygons_count = n * n + (n - 1) * (n - 1) + 1;

        assert_eq!(result.len(), polygons_count);
        assert_eq!(result[0].len(), 2);
    }

    fn checkerboard_b(n: usize) {
        //     0   1   2   3   4   5   6   7   8   9
        //   9 ┌───────────────────────────────────┐
        //     │                                   │
        //   8 │   ┌───┐   ┌───┐   ┌───┐   ┌───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │ ░ │   │
        //   7 │   └───●───●───●───●───●───●───┘   │
        //     │       │ ░ │   │ ░ │   │ ░ │       │
        //   6 │   ┌───●───●───●───●───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │ ░ │   │
        //   5 │   └───●───●───●───●───●───●───┘   │
        //     │       │ ░ │   │ ░ │   │ ░ │       │
        //   4 │   ┌───●───●───●───●───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │ ░ │   │
        //   3 │   └───●───●───●───●───●───●───┘   │
        //     │       │ ░ │   │ ░ │   │ ░ │       │
        //   2 │   ┌───●───●───●───●───●───●───┐   │
        //     │   │ ░ │   │ ░ │   │ ░ │   │ ░ │   │
        //   1 │   └───┘   └───┘   └───┘   └───┘   │
        //     │                                   │
        //   0 └───────────────────────────────────┘

        let mut subj_paths = Vec::new();

        let m = n as i32;

        let x0 = 1;
        let y0 = 1;
        let x1 = 2 * m;
        let y1 = 2 * m;

        subj_paths.push(int_path!(
            [x0 - 1, y1 + 1],
            [x0 - 1, y0 - 1],
            [x1 + 1, y0 - 1],
            [x1 + 1, y1 + 1]
        ));

        let mut y = y0;
        for i in 0..2 * m - 1 {
            let offset = i & 1;
            let mut x = x0 + offset;
            while x < x1 {
                let square = int_path!([x, y + 1], [x, y], [x + 1, y], [x + 1, y + 1]);
                subj_paths.push(square);
                x += 2;
            }
            y += 1;
        }

        let mut overlay =
            Overlay::with_contours_custom(&subj_paths, &[], IntOverlayOptions::ogc(), Default::default());

        let result = overlay.overlay(OverlayRule::Subject, FillRule::EvenOdd);

        let a = 2 * n - 3;
        let polygons_count = a * a / 2 + 1;

        assert_eq!(result.len(), polygons_count);

        let main_polygon_index = result.iter().position(|shape| shape.len() > 1);
        assert!(main_polygon_index.is_some());

        let main = &result[main_polygon_index.unwrap()];

        assert_eq!(main.len(), 6);
    }

    #[test]
    fn test_random_grid_holes() {
        for seed in 0..256 {
            random_grid_holes(seed, 8, 35);
            random_grid_holes(seed ^ 0x9e37_79b9_7f4a_7c15, 10, 45);
            random_grid_holes(seed ^ 0xd1b5_4a32_d192_ed03, 12, 55);
        }
    }

    fn random_grid_holes(seed: u64, n: usize, fill_percent: u32) {
        let mut rng = GridRng::new(seed);
        let mut clipped = vec![false; n * n];
        let mut clipped_count = 0;

        for y in 0..n {
            for x in 0..n {
                let is_clipped = rng.percent(fill_percent);
                clipped[y * n + x] = is_clipped;
                clipped_count += is_clipped as usize;
            }
        }

        if clipped_count == 0 || clipped_count == n * n {
            return;
        }

        let expected_count = remaining_components(n, &clipped);
        let subj_paths = vec![rect_path(0, 0, n as i32, n as i32)];
        let mut clip_paths = Vec::with_capacity(clipped_count);

        for y in 0..n {
            for x in 0..n {
                if clipped[y * n + x] {
                    clip_paths.push(rect_path(x as i32, y as i32, x as i32 + 1, y as i32 + 1));
                }
            }
        }

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert_eq!(
            result.len(),
            expected_count,
            "seed={seed} n={n} fill={fill_percent}% grid:\n{}",
            grid_debug(n, &clipped)
        );

        for shape in result.iter() {
            assert!(!shape.is_empty(), "seed={seed} n={n}");
            for contour in shape.iter() {
                assert!(contour.len() >= 3, "seed={seed} n={n} contour={contour:?}");
            }
        }
    }

    #[test]
    fn test_random_self_intersections() {
        for seed in 0..128 {
            random_self_intersections(seed, 1, 12);
            random_self_intersections(seed ^ 0x9e37_79b9_7f4a_7c15, 2, 20);
            random_self_intersections(seed ^ 0xd1b5_4a32_d192_ed03, 3, 28);
        }
    }

    #[test]
    #[ignore = "long randomized OGC stress test"]
    fn test_random_self_intersections_stress() {
        let seconds = env_u64("OGC_STRESS_SECONDS", 600);
        let mut seed = env_u64("OGC_STRESS_SEED", 0xa076_1d64_78bd_642f);
        let deadline = Instant::now() + Duration::from_secs(seconds);
        let mut iteration = 0usize;

        while Instant::now() < deadline {
            run_random_self_intersections_case(seed, iteration, 1, 12);
            run_random_self_intersections_case(seed ^ 0x9e37_79b9_7f4a_7c15, iteration, 2, 20);
            run_random_self_intersections_case(seed ^ 0xd1b5_4a32_d192_ed03, iteration, 3, 28);

            seed = next_stress_seed(seed);
            iteration += 1;
        }

        eprintln!("OGC stress completed: iterations={iteration} seconds={seconds}");
    }

    fn run_random_self_intersections_case(
        seed: u64,
        iteration: usize,
        contour_count: usize,
        hole_count: usize,
    ) {
        let result = catch_unwind(AssertUnwindSafe(|| {
            random_self_intersections(seed, contour_count, hole_count);
        }));

        if let Err(payload) = result {
            panic!(
                "OGC stress failed: iteration={iteration} seed={seed} contour_count={contour_count} hole_count={hole_count} panic={}",
                panic_message(payload)
            );
        }
    }

    fn random_self_intersections(seed: u64, contour_count: usize, hole_count: usize) {
        let mut rng = GridRng::new(seed);
        let mut subj_paths = Vec::with_capacity(contour_count);
        let mut clip_paths = Vec::with_capacity(hole_count);

        for _ in 0..contour_count {
            subj_paths.push(random_star_contour(&mut rng, 760, 260, 640));
        }

        for _ in 0..hole_count {
            clip_paths.push(random_star_contour(&mut rng, 680, 60, 220));
        }

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ogc(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        let mut overlay =
            Overlay::with_shapes_options(&result, &[], IntOverlayOptions::ogc(), Default::default());
        let normalized = overlay.overlay(OverlayRule::Union, FillRule::EvenOdd);

        let result_area = result.area().abs();
        let normalized_area = normalized.area().abs();
        let area_delta = (result_area - normalized_area).abs();
        let max_area = result_area.max(normalized_area);
        let area_tolerance = 20_000.max(max_area / 5);
        assert!(
            area_delta <= area_tolerance,
            "seed={seed} contour_count={contour_count} hole_count={hole_count} area_delta={area_delta} area_tolerance={area_tolerance} result_area={result_area} normalized_area={normalized_area}"
        );

        for shape in result.iter() {
            assert!(
                !shape.is_empty(),
                "seed={seed} contour_count={contour_count} hole_count={hole_count}"
            );
            for contour in shape.iter() {
                assert!(
                    contour.len() >= 3,
                    "seed={seed} contour_count={contour_count} hole_count={hole_count} contour={contour:?}"
                );
            }
        }
    }

    fn random_star_contour(
        rng: &mut GridRng,
        center_abs: i32,
        min_radius: i32,
        max_radius: i32,
    ) -> Vec<IntPoint<i32>> {
        let n = 9 + 2 * rng.range_usize(0, 5);
        let mut step = rng.range_usize(2, n / 2);
        while gcd(n, step) != 1 {
            step += 1;
            if step >= n / 2 {
                step = 2;
            }
        }

        let center_x = rng.range_i32(-center_abs, center_abs);
        let center_y = rng.range_i32(-center_abs, center_abs);
        let radius = rng.range_i32(min_radius, max_radius) as f64;
        let angle_shift = rng.unit_f64() * 2.0 * PI;

        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            let angle_jitter = (rng.unit_f64() - 0.5) * 0.18;
            let radius_jitter = 0.72 + rng.unit_f64() * 0.56;
            let angle = angle_shift + 2.0 * PI * i as f64 / n as f64 + angle_jitter;
            let r = radius * radius_jitter;
            points.push(IntPoint::new(
                center_x + (r * angle.cos()).round() as i32,
                center_y + (r * angle.sin()).round() as i32,
            ));
        }

        let mut contour = Vec::with_capacity(n);
        let mut index = 0;
        for _ in 0..n {
            contour.push(points[index]);
            index = (index + step) % n;
        }

        contour
    }

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    fn remaining_components(n: usize, clipped: &[bool]) -> usize {
        let mut visited = vec![false; clipped.len()];
        let mut components = 0;
        let mut stack = Vec::new();

        for start in 0..clipped.len() {
            if clipped[start] || visited[start] {
                continue;
            }

            components += 1;
            visited[start] = true;
            stack.push(start);

            while let Some(index) = stack.pop() {
                let x = index % n;
                let y = index / n;

                if x > 0 {
                    visit_cell(index - 1, clipped, &mut visited, &mut stack);
                }
                if x + 1 < n {
                    visit_cell(index + 1, clipped, &mut visited, &mut stack);
                }
                if y > 0 {
                    visit_cell(index - n, clipped, &mut visited, &mut stack);
                }
                if y + 1 < n {
                    visit_cell(index + n, clipped, &mut visited, &mut stack);
                }
            }
        }

        components
    }

    fn visit_cell(index: usize, clipped: &[bool], visited: &mut [bool], stack: &mut Vec<usize>) {
        if clipped[index] || visited[index] {
            return;
        }
        visited[index] = true;
        stack.push(index);
    }

    fn rect_path(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<IntPoint<i32>> {
        vec![
            IntPoint::new(x0, y0),
            IntPoint::new(x1, y0),
            IntPoint::new(x1, y1),
            IntPoint::new(x0, y1),
        ]
    }

    fn grid_debug(n: usize, clipped: &[bool]) -> String {
        let mut s = String::new();
        for y in (0..n).rev() {
            for x in 0..n {
                s.push(if clipped[y * n + x] { '#' } else { '.' });
            }
            s.push('\n');
        }
        s
    }

    struct GridRng {
        state: u64,
    }

    impl GridRng {
        fn new(seed: u64) -> Self {
            Self {
                state: seed ^ 0xa076_1d64_78bd_642f,
            }
        }

        fn percent(&mut self, value: u32) -> bool {
            self.next_u32() % 100 < value
        }

        fn range_i32(&mut self, min: i32, max: i32) -> i32 {
            let width = (max - min + 1) as u32;
            min + (self.next_u32() % width) as i32
        }

        fn range_usize(&mut self, min: usize, max: usize) -> usize {
            let width = max - min + 1;
            min + self.next_u32() as usize % width
        }

        fn unit_f64(&mut self) -> f64 {
            self.next_u32() as f64 / u32::MAX as f64
        }

        fn next_u32(&mut self) -> u32 {
            self.state = self
                .state
                .wrapping_mul(0xe703_7ed1_a0b4_28db)
                .wrapping_add(0x8ebc_6af0_9c88_c6e3);
            (self.state >> 32) as u32
        }
    }

    fn next_stress_seed(seed: u64) -> u64 {
        seed.wrapping_mul(0xe703_7ed1_a0b4_28db)
            .wrapping_add(0x8ebc_6af0_9c88_c6e3)
    }

    fn env_u64(name: &str, default: u64) -> u64 {
        std::env::var(name)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(default)
    }

    fn panic_message(payload: Box<dyn core::any::Any + Send>) -> String {
        if let Some(message) = payload.downcast_ref::<&str>() {
            return (*message).to_string();
        }
        if let Some(message) = payload.downcast_ref::<String>() {
            return message.clone();
        }
        "non-string panic payload".to_string()
    }

    #[test]
    fn test_crash() {
        // Reduced from contours produced by non-OGC extraction of the original crash case.
        let subj_paths = int_shape![
            [[0, 0], [-6, 2], [-2, -6]],
            [[-3, 0], [0, 0], [-3, -1]],
            [[0, 0], [4, -6], [4, 6]],
        ];

        let mut overlay =
            Overlay::with_contours_custom(&subj_paths, &[], IntOverlayOptions::ogc(), Default::default());

        let result = overlay.overlay(OverlayRule::Union, FillRule::NonZero);
        let main_polygon_index = result.iter().position(|shape| shape.len() > 1);
        assert!(main_polygon_index.is_some());
    }
}
