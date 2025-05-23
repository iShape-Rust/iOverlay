#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_shape::int::path::IntPath;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;

    #[test]
    fn test_both_clock_wise() {
        fn overlay() -> Overlay {
            let mut overlay = Overlay::new(2);

            overlay.add_contour(&square(10, true), ShapeType::Subject);
            overlay.add_contour(&square(5, true), ShapeType::Subject);

            overlay
        }

        let mut buffer = Default::default();

        let even_odd = overlay().build_graph_view(FillRule::EvenOdd).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let non_zero = overlay().build_graph_view(FillRule::NonZero).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let positive = overlay().build_graph_view(FillRule::Positive).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let negative = overlay().build_graph_view(FillRule::Negative).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);

        assert_eq!(even_odd.len(), 1);
        assert_eq!(even_odd[0].len(), 2);

        assert_eq!(non_zero.len(), 1);
        assert_eq!(non_zero[0].len(), 1);

        assert_eq!(negative.len(), 1);
        assert_eq!(negative[0].len(), 1);

        assert_eq!(positive.len(), 0);
    }

    #[test]
    fn test_both_counter_clock_wise() {
        fn overlay() -> Overlay {
            let mut overlay = Overlay::new(2);

            overlay.add_contour(&square(10, false), ShapeType::Subject);
            overlay.add_contour(&square(5, false), ShapeType::Subject);

            overlay
        }

        let mut buffer = Default::default();

        let even_odd = overlay().build_graph_view(FillRule::EvenOdd).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let non_zero = overlay().build_graph_view(FillRule::NonZero).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let positive = overlay().build_graph_view(FillRule::Positive).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let negative = overlay().build_graph_view(FillRule::Negative).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);

        assert_eq!(even_odd.len(), 1);
        assert_eq!(even_odd[0].len(), 2);

        assert_eq!(non_zero.len(), 1);
        assert_eq!(non_zero[0].len(), 1);

        assert_eq!(negative.len(), 0);

        assert_eq!(positive.len(), 1);
        assert_eq!(positive[0].len(), 1);
    }

    #[test]
    fn test_cw_and_ccw() {
        fn overlay() -> Overlay {
            let mut overlay = Overlay::new(2);

            overlay.add_contour(&square(10, true), ShapeType::Subject);
            overlay.add_contour(&square(5, false), ShapeType::Subject);

            overlay
        }

        let mut buffer = Default::default();

        let even_odd = overlay().build_graph_view(FillRule::EvenOdd).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let non_zero = overlay().build_graph_view(FillRule::NonZero).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let positive = overlay().build_graph_view(FillRule::Positive).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let negative = overlay().build_graph_view(FillRule::Negative).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);

        assert_eq!(even_odd.len(), 1);
        assert_eq!(even_odd[0].len(), 2);

        assert_eq!(non_zero.len(), 1);
        assert_eq!(non_zero[0].len(), 2);

        assert_eq!(negative.len(), 1);
        assert_eq!(negative[0].len(), 2);

        assert_eq!(positive.len(), 0);
    }



    #[test]
    fn test_ccw_and_cw() {
        fn overlay() -> Overlay {
            let mut overlay = Overlay::new(2);

            overlay.add_contour(&square(10, false), ShapeType::Subject);
            overlay.add_contour(&square(5, true), ShapeType::Subject);

            overlay
        }

        let mut buffer = Default::default();

        let even_odd = overlay().build_graph_view(FillRule::EvenOdd).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let non_zero = overlay().build_graph_view(FillRule::NonZero).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let positive = overlay().build_graph_view(FillRule::Positive).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);
        let negative = overlay().build_graph_view(FillRule::Negative).unwrap().extract_shapes(OverlayRule::Subject, &mut buffer);

        assert_eq!(even_odd.len(), 1);
        assert_eq!(even_odd[0].len(), 2);

        assert_eq!(non_zero.len(), 1);
        assert_eq!(non_zero[0].len(), 2);

        assert_eq!(negative.len(), 0);

        assert_eq!(positive.len(), 1);
        assert_eq!(positive[0].len(), 2);
    }

    fn square(radius: i32, is_clockwise: bool) -> IntPath {
        let mut square = [
            IntPoint::new(-radius, -radius),
            IntPoint::new(-radius, radius),
            IntPoint::new(radius, radius),
            IntPoint::new(radius, -radius)
        ].to_vec();

        if !is_clockwise {
            square.reverse()
        }

        square
    }

}