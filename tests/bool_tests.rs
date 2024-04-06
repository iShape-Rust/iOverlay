
#[cfg(test)]
mod tests {
    use i_float::fix_vec::FixVec;
    use i_shape::fix_shape::FixShape;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;

    #[test]
    fn test_fill_rule() {
        let mut overlay = Overlay::new(2);

        let left_bottom_square = FixShape::new_with_contour([
            FixVec::new_f64(-10.0, -10.0),
            FixVec::new_f64(-10.0, 10.0),
            FixVec::new_f64(10.0, 10.0),
            FixVec::new_f64(10.0, -10.0)
        ].to_vec());

        let right_top_square = FixShape::new_with_contour([
            FixVec::new_f64(-5.0, -5.0),
            FixVec::new_f64(-5.0, 15.0),
            FixVec::new_f64(15.0, 15.0),
            FixVec::new_f64(15.0, -5.0)
        ].to_vec());

        // add new geometry
        overlay.add_shape(&left_bottom_square, ShapeType::Subject);
        overlay.add_shape(&right_top_square, ShapeType::Clip);

        // resolve shapes geometry
        let graph = overlay.build_graph(FillRule::EvenOdd);

        // apply union operation and get result (in our case it will be only one element)
        let shapes = graph.extract_shapes(OverlayRule::Union);

        // do something with new shapes...

        print!("shapes: {:?}", shapes)
    }
}