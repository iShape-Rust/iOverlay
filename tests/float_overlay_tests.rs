#[cfg(test)]
mod tests {
    use i_float::f64_point::F64Point;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::float_overlay::FloatOverlay;
    use i_overlay::core::overlay_rule::OverlayRule;


    #[test]
    fn test_00() {
        let shape_0 = [
          [
              F64Point::new(0.0, 0.0),
              F64Point::new(0.0, 1.0),
              F64Point::new(1.0, 1.0),
              F64Point::new(1.0, 0.0)
          ].to_vec()
        ].to_vec();
        let shape_1 = [
            [
                F64Point::new(1.0, 0.0),
                F64Point::new(1.0, 1.0),
                F64Point::new(2.0, 1.0),
                F64Point::new(2.0, 0.0)
            ].to_vec()
        ].to_vec();

        let overlay = FloatOverlay::with_paths(shape_0, shape_1);
        let graph = overlay.build_graph(FillRule::NonZero);
        let union = graph.extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }
}