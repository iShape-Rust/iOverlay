#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};

    #[test]
    fn test_00() {
        let mut overlay = Overlay::new(1);
        overlay.add_contour(&[IntPoint::new(0, 0)], ShapeType::Subject);

        let graph = overlay.build_graph_view(FillRule::NonZero);
        assert!(graph.is_none());
    }

    #[test]
    fn test_01() {
        let mut overlay = Overlay::new(1);
        overlay.add_contour(&[IntPoint::new(0, 0), IntPoint::new(1, 0)], ShapeType::Subject);

        let graph = overlay.build_graph_view(FillRule::NonZero);
        assert!(graph.is_none());
    }
}