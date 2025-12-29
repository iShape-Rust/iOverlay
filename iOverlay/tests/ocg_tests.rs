#[cfg(test)]
mod tests {
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{IntOverlayOptions, Overlay};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_shape::{int_path, int_shape};

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

        let subj_paths = int_shape![[[0, 0], [5, 0], [5, 5], [0, 5],]];

        let clip_paths = int_shape![
            [[1, 2], [1, 4], [3, 4], [3, 3], [2, 3], [2, 2]],
            [[2, 1], [2, 2], [3, 2], [3, 3], [4, 3], [4, 1]],
        ];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ocg(),
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

        let subj_paths = int_shape![[[0, 0], [5, 0], [5, 5], [0, 5],]];

        let clip_paths = int_shape![
            [[1, 2], [1, 3], [2, 3], [2, 2]],
            [[2, 1], [2, 2], [3, 2], [3, 1]],
            [[2, 3], [2, 4], [3, 4], [3, 3]],
            [[3, 2], [3, 3], [4, 3], [4, 2]],
        ];

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &clip_paths,
            IntOverlayOptions::ocg(),
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

        let subj_paths = int_shape![[[0, 0], [7, 0], [7, 7], [0, 7],]];

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
            IntOverlayOptions::ocg(),
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
    fn test_checkerboard() {
        for n in 3..50 {
            checkerboard(n);
        }
    }

    fn checkerboard(n: usize) {
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
        let x1 = 2 * m + 2;
        let y1 = 2 * m + 2;

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

        let mut overlay = Overlay::with_contours_custom(
            &subj_paths,
            &[],
            IntOverlayOptions::ocg(),
            Default::default(),
        );

        let result = overlay.overlay(OverlayRule::Subject, FillRule::EvenOdd);

        let polygons_count = n * n + (n - 1) * (n - 1) + 1;

        assert_eq!(result.len(), polygons_count);
        assert_eq!(result[0].len(), 2);
    }
}
