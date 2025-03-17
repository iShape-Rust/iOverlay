#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;
    use i_overlay::iso::core::overlay::IsoOverlay;

    #[test]
    fn test_0() {
        let subj =
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, 0)
                ].to_vec()
            ].to_vec();
        let clip = vec![];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let result = overlay
            .into_graph_with_solver(FillRule::NonZero, Solver::default())
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 4);
    }

    #[test]
    fn test_1() {
        let subj =
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, 0)
                ].to_vec()
            ].to_vec();
        let clip =
            [
                [
                    IntPoint::new(5, 5),
                    IntPoint::new(5, 15),
                    IntPoint::new(15, 15),
                    IntPoint::new(15, 5)
                ].to_vec()
            ].to_vec();

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let result = overlay
            .into_graph_with_solver(FillRule::NonZero, Solver::default())
            .extract_shapes(OverlayRule::Union);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 8);
    }

    #[test]
    fn test_2() {
        let subj =
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 10),
                    IntPoint::new(1, 10),
                    IntPoint::new(1, 0)
                ].to_vec(),
                [
                    IntPoint::new(1, 0),
                    IntPoint::new(1, 10),
                    IntPoint::new(2, 10),
                    IntPoint::new(2, 0)
                ].to_vec(),
                [
                    IntPoint::new(2, 0),
                    IntPoint::new(2, 10),
                    IntPoint::new(3, 10),
                    IntPoint::new(3, 0)
                ].to_vec(),
                [
                    IntPoint::new(3, 0),
                    IntPoint::new(3, 10),
                    IntPoint::new(4, 10),
                    IntPoint::new(4, 0)
                ].to_vec(),
                [
                    IntPoint::new(4, 0),
                    IntPoint::new(4, 10),
                    IntPoint::new(5, 10),
                    IntPoint::new(5, 0)
                ].to_vec()
            ].to_vec();

        let clip = vec![];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let result = overlay
            .into_graph_with_solver(FillRule::NonZero, Solver::default())
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 4);
    }

    #[test]
    fn test_3() {
        let subj =
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 10),
                    IntPoint::new(1, 10),
                    IntPoint::new(1, 0)
                ].to_vec(),
                [
                    IntPoint::new(2, 0),
                    IntPoint::new(2, 10),
                    IntPoint::new(3, 10),
                    IntPoint::new(3, 0)
                ].to_vec(),
                [
                    IntPoint::new(4, 0),
                    IntPoint::new(4, 10),
                    IntPoint::new(5, 10),
                    IntPoint::new(5, 0)
                ].to_vec(),
                [
                    IntPoint::new(6, 0),
                    IntPoint::new(6, 10),
                    IntPoint::new(7, 10),
                    IntPoint::new(7, 0)
                ].to_vec(),
                [
                    IntPoint::new(8, 0),
                    IntPoint::new(8, 10),
                    IntPoint::new(9, 10),
                    IntPoint::new(9, 0)
                ].to_vec()
            ].to_vec();

        let clip = vec![];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let result = overlay
            .into_graph_with_solver(FillRule::NonZero, Solver::default())
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_4() {
        let subj =
            [
                [
                    IntPoint::new(-2, -2),
                    IntPoint::new(-2, 2),
                    IntPoint::new(2, 2),
                    IntPoint::new(2, -2)
                ].to_vec(),
                [
                    IntPoint::new(0, -3),
                    IntPoint::new(-1, -2),
                    IntPoint::new(1, -2)
                ].to_vec()
            ].to_vec();

        let clip = vec![];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let result = overlay
            .into_graph_with_solver(FillRule::NonZero, Solver::default())
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 7);
    }

    #[test]
    fn test_5() {
        let subj =
            [
                [
                    IntPoint::new(-2, -2),
                    IntPoint::new(-2, 2),
                    IntPoint::new(2, 2),
                    IntPoint::new(2, -2)
                ].to_vec(),
                [
                    IntPoint::new(-3, 0),
                    IntPoint::new(-2, 1),
                    IntPoint::new(-2, -1)
                ].to_vec(),
                // [
                //     IntPoint::new(3, 0),
                //     IntPoint::new(2, -1),
                //     IntPoint::new(2, 1)
                // ].to_vec(),
                // [
                //     IntPoint::new(0, 3),
                //     IntPoint::new(1, 2),
                //     IntPoint::new(-1, 2)
                // ].to_vec(),
                [
                    IntPoint::new(0, -3),
                    IntPoint::new(-1, -2),
                    IntPoint::new(1, -2)
                ].to_vec()
            ].to_vec();

        let clip = vec![];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let result = overlay
            .into_graph_with_solver(FillRule::NonZero, Solver::default())
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 10);
    }

    #[test]
    fn test_6() {
        let subj =
            [
                [
                    IntPoint::new(-2, -2),
                    IntPoint::new(-2, 2),
                    IntPoint::new(2, 2),
                    IntPoint::new(2, -2)
                ].to_vec(),
                [
                    IntPoint::new(-3, 0),
                    IntPoint::new(-2, 1),
                    IntPoint::new(-2, -1)
                ].to_vec(),
                [
                    IntPoint::new(3, 0),
                    IntPoint::new(2, -1),
                    IntPoint::new(2, 1)
                ].to_vec(),
                [
                    IntPoint::new(0, 3),
                    IntPoint::new(1, 2),
                    IntPoint::new(-1, 2)
                ].to_vec(),
                [
                    IntPoint::new(0, -3),
                    IntPoint::new(-1, -2),
                    IntPoint::new(1, -2)
                ].to_vec()
            ].to_vec();

        let clip = vec![];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let result = overlay
            .into_graph_with_solver(FillRule::NonZero, Solver::default())
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 16);
    }
}