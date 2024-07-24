#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;

    #[test]
    fn test_fill_rule() {
        let mut overlay = Overlay::new(2);

        let left_bottom_square = [[
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec()].to_vec();

        let right_top_square = [[
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 15),
            IntPoint::new(15, 15),
            IntPoint::new(15, -5)
        ].to_vec()].to_vec();

        // add new geometry
        overlay.add_shape(&left_bottom_square, ShapeType::Subject);
        overlay.add_shape(&right_top_square, ShapeType::Clip);

        // resolve shapes geometry
        let graph = overlay.into_graph(FillRule::EvenOdd);

        // apply union operation and get result (in our case it will be only one element)
        let shapes = graph.extract_shapes(OverlayRule::Union);

        // do something with new shapes...

        print!("shapes: {:?}", shapes)
    }
}