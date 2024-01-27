use i_overlay::bool::overlay_rule::OverlayRule;

#[cfg(test)]
mod tests {
    use i_float::fix_vec::FixVec;
    use i_shape::fix_path::FixPathExtension;
    use i_overlay::bool::fill_rule::FillRule;
    use i_overlay::layout::overlay::{Overlay, ShapeType};
    use super::*;

    #[test]
    fn test_clockwise_direct() {
        let mut overlay = Overlay::new(8);
        overlay.add_path(&vec![
            FixVec::new_f64(-10.0, -10.0),
            FixVec::new_f64(-10.0, 10.0),
            FixVec::new_f64(10.0, 10.0),
            FixVec::new_f64(10.0, -10.0),
        ], ShapeType::Subject);

        overlay.add_path(&vec![
            FixVec::new_f64(-5.0, -5.0),
            FixVec::new_f64(-5.0, 5.0),
            FixVec::new_f64(5.0, 5.0),
            FixVec::new_f64(5.0, -5.0),
        ], ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::EvenOdd);

        let shapes = graph.extract_shapes(OverlayRule::Difference);

        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.paths.len(), 2);

        assert_eq!(shape.contour().area() > 0, true);
        assert_eq!(shape.paths[1].area() > 0, false);
    }

    #[test]
    fn test_clockwise_reverse() {
        let mut overlay = Overlay::new(8);
        overlay.add_paths(&[
            [
                FixVec::new_f64(-10.0, -10.0),
                FixVec::new_f64(10.0, -10.0),
                FixVec::new_f64(10.0, 10.0),
                FixVec::new_f64(-10.0, 10.0)
            ].to_vec()
        ].to_vec(), ShapeType::Subject);
        overlay.add_paths(&[
            [
                FixVec::new_f64(-5.0, -5.0),
                FixVec::new_f64(5.0, -5.0),
                FixVec::new_f64(5.0, 5.0),
                FixVec::new_f64(-5.0, 5.0)
            ].to_vec()
        ].to_vec(), ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::EvenOdd);

        let shapes = graph.extract_shapes(OverlayRule::Difference);

        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.paths.len(), 2);

        assert_eq!(shape.contour().area() > 0, true);
        assert_eq!(shape.paths[1].area() > 0, false);
    }

    #[test]
    fn test_clockwise_all_opposite() {
        let mut overlay = Overlay::new(8);
        overlay.add_paths(&[
            [
                FixVec::new_f64(-10.0, -10.0),
                FixVec::new_f64(10.0, -10.0),
                FixVec::new_f64(10.0, 10.0),
                FixVec::new_f64(-10.0, 10.0)
            ].to_vec()
        ].to_vec(), ShapeType::Subject);
        overlay.add_paths(&[
            [
                FixVec::new_f64(-5.0, -5.0),
                FixVec::new_f64(-5.0, 5.0),
                FixVec::new_f64(5.0, 5.0),
                FixVec::new_f64(5.0, -5.0)
            ].to_vec()
        ].to_vec(), ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::EvenOdd);

        let shapes = graph.extract_shapes(OverlayRule::Difference);

        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.paths.len(), 2);

        assert_eq!(shape.contour().area() > 0, true);
        assert_eq!(shape.paths[1].area() > 0, false);
    }
}