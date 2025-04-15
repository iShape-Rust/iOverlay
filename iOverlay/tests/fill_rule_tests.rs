#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_shape::int::path::IntPath;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;

    #[test]
    fn test_both_clock_wise() {
        let mut overlay = Overlay::new(2);

        overlay.add_contour(&square(10, true), ShapeType::Subject);
        overlay.add_contour(&square(5, true), ShapeType::Subject);

        let even_odd = overlay.clone().into_graph(FillRule::EvenOdd).extract_shapes(OverlayRule::Subject);
        let non_zero = overlay.clone().into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject);
        let positive = overlay.clone().into_graph(FillRule::Positive).extract_shapes(OverlayRule::Subject);
        let negative = overlay.clone().into_graph(FillRule::Negative).extract_shapes(OverlayRule::Subject);

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
        let mut overlay = Overlay::new(2);

        overlay.add_contour(&square(10, false), ShapeType::Subject);
        overlay.add_contour(&square(5, false), ShapeType::Subject);

        let even_odd = overlay.clone().into_graph(FillRule::EvenOdd).extract_shapes(OverlayRule::Subject);
        let non_zero = overlay.clone().into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject);
        let positive = overlay.clone().into_graph(FillRule::Positive).extract_shapes(OverlayRule::Subject);
        let negative = overlay.clone().into_graph(FillRule::Negative).extract_shapes(OverlayRule::Subject);

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
        let mut overlay = Overlay::new(2);

        overlay.add_contour(&square(10, true), ShapeType::Subject);
        overlay.add_contour(&square(5, false), ShapeType::Subject);

        let even_odd = overlay.clone().into_graph(FillRule::EvenOdd).extract_shapes(OverlayRule::Subject);
        let non_zero = overlay.clone().into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject);
        let positive = overlay.clone().into_graph(FillRule::Positive).extract_shapes(OverlayRule::Subject);
        let negative = overlay.clone().into_graph(FillRule::Negative).extract_shapes(OverlayRule::Subject);

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
        let mut overlay = Overlay::new(2);

        overlay.add_contour(&square(10, false), ShapeType::Subject);
        overlay.add_contour(&square(5, true), ShapeType::Subject);

        let even_odd = overlay.clone().into_graph(FillRule::EvenOdd).extract_shapes(OverlayRule::Subject);
        let non_zero = overlay.clone().into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject);
        let positive = overlay.clone().into_graph(FillRule::Positive).extract_shapes(OverlayRule::Subject);
        let negative = overlay.clone().into_graph(FillRule::Negative).extract_shapes(OverlayRule::Subject);

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