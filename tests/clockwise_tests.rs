#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_shape::int::path::PointPathExtension;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;

    #[test]
    fn test_clockwise_direct() {
        let mut overlay = Overlay::new(8);
        overlay.add_path(&vec![
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10),
        ], ShapeType::Subject);

        overlay.add_path(&vec![
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 5),
            IntPoint::new(5, 5),
            IntPoint::new(5, -5),
        ], ShapeType::Clip);

        let graph = overlay.into_graph(FillRule::EvenOdd);

        let shapes = graph.extract_shapes(OverlayRule::Difference);

        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.len(), 2);

        assert_eq!(shape[0].unsafe_area() > 0, true);
        assert_eq!(shape[1].unsafe_area() > 0, false);
    }

    #[test]
    fn test_clockwise_reverse() {
        let mut overlay = Overlay::new(8);
        overlay.add_paths(&[
            [
                IntPoint::new(-10, -10),
                IntPoint::new(10, -10),
                IntPoint::new(10, 10),
                IntPoint::new(-10, 10)
            ].to_vec()
        ].to_vec(), ShapeType::Subject);
        overlay.add_paths(&[
            [
                IntPoint::new(-5, -5),
                IntPoint::new(5, -5),
                IntPoint::new(5, 5),
                IntPoint::new(-5, 5)
            ].to_vec()
        ].to_vec(), ShapeType::Clip);

        let graph = overlay.into_graph(FillRule::EvenOdd);

        let shapes = graph.extract_shapes(OverlayRule::Difference);

        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.len(), 2);

        assert_eq!(shape[0].unsafe_area() > 0, true);
        assert_eq!(shape[1].unsafe_area() > 0, false);
    }

    #[test]
    fn test_clockwise_all_opposite() {
        let mut overlay = Overlay::new(8);
        overlay.add_paths(&[
            [
                IntPoint::new(-10, -10),
                IntPoint::new(10, -10),
                IntPoint::new(10, 10),
                IntPoint::new(-10, 10)
            ].to_vec()
        ].to_vec(), ShapeType::Subject);
        overlay.add_paths(&[
            [
                IntPoint::new(-5, -5),
                IntPoint::new(-5, 5),
                IntPoint::new(5, 5),
                IntPoint::new(5, -5)
            ].to_vec()
        ].to_vec(), ShapeType::Clip);

        let graph = overlay.into_graph(FillRule::EvenOdd);

        let shapes = graph.extract_shapes(OverlayRule::Difference);

        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.len(), 2);

        assert_eq!(shape[0].unsafe_area() > 0, true);
        assert_eq!(shape[1].unsafe_area() > 0, false);
    }
}