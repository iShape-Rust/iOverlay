#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::float_overlay::FloatOverlay;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;

    #[test]
    fn test_00() {
        let overlay = FloatOverlay::new();
        let graph = overlay.build_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 0);
    }

    #[test]
    fn test_01() {
        let mut overlay = Overlay::new(1);
        overlay.add_path(&[IntPoint::new(0, 0)], ShapeType::Subject);

        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 0);
    }

    #[test]
    fn test_02() {
        let mut overlay = Overlay::new(1);
        overlay.add_path(&[IntPoint::new(0, 0), IntPoint::new(1, 0)], ShapeType::Subject);

        let graph = overlay.into_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 0);
    }
}