#[cfg(test)]
mod tests {
    use i_shape::int_shape;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::Overlay;
    use i_overlay::core::overlay_rule::OverlayRule;

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

        let subj_paths = int_shape![[
            [0, 0],
            [5, 0],
            [5, 5],
            [0, 5],
        ]];

        let clip_paths = int_shape![
            [[1, 2], [1, 4], [3, 4], [3, 3], [2, 3], [2, 2]],
            [[2, 1], [2, 2], [3, 2], [3, 3], [4, 3], [4, 1]],
        ];

        let mut overlay = Overlay::with_contours(&subj_paths, &clip_paths);

        let result = overlay.overlay(OverlayRule::Difference, FillRule::EvenOdd);

        assert!(
            result.len() >= 2,
            "Expected multiple polygons to keep interiors connected, got {} shape(s)",
            result.len()
        );
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
    }
}
