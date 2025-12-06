#[cfg(test)]
mod tests {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;
    use i_shape::source::resource::ShapeResource;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::overlay::FloatOverlay;

    #[test]
    fn test_adapter_with_rect() {
        let s = 1.0 / 3.0;
        let shape = vec![vec![
            [s * 0.0, s * 0.0],
            [s * 0.0, s * 1.0],
            [s * 1.0, s * 1.0],
            [s * 1.0, s * 0.0],
        ]];

        let adapter_100 = FloatPointAdapter::new(FloatRect::new(-100.0, 100.0, -100.0, 100.0));
        let adapter_1000 = FloatPointAdapter::new(FloatRect::new(-1000.0, 1000.0, -1000.0, 1000.0));

        let subj_100 = FloatOverlay::with_adapter(adapter_100, shape.len())
            .unsafe_add_source(&shape, ShapeType::Subject)
            .overlay(OverlayRule::Subject, FillRule::NonZero);

        assert_eq!(subj_100.len(), 1);
        assert_eq!(subj_100[0].len(), 1);
        assert_eq!(subj_100[0][0].len(), 4);

        let subj_1000 = FloatOverlay::with_adapter(adapter_1000, shape.len())
            .unsafe_add_source(&shape, ShapeType::Subject)
            .overlay(OverlayRule::Subject, FillRule::NonZero);

        let c100 = &subj_100[0][0];
        let c1000 = &subj_1000[0][0];

        println!("100: {:?}", c100);
        println!("1000: {:?}", c1000);
    }

    #[test]
    fn test_adapter_with_scale() {
        let s = 1.0 / 3.0;
        let shape = vec![vec![
            [s * 0.0, s * 0.0],
            [s * 0.0, s * 1.0],
            [s * 1.0, s * 1.0],
            [s * 1.0, s * 0.0],
        ]];

        let rect = FloatRect::with_iter(shape.iter_paths().flatten()).unwrap();
        let buffer_rect = FloatRect::new(
            rect.min_x - 0.1,
            rect.max_x + 0.1,
            rect.min_y - 0.1,
            rect.max_y + 0.1,
        );

        let adapter_100 = FloatPointAdapter::with_scale(buffer_rect.clone(), 100.0);
        let adapter_1000 = FloatPointAdapter::with_scale(buffer_rect, 1000.0);

        let subj_100 = FloatOverlay::with_adapter(adapter_100, shape.len())
            .unsafe_add_source(&shape, ShapeType::Subject)
            .overlay(OverlayRule::Subject, FillRule::NonZero);

        assert_eq!(subj_100.len(), 1);
        assert_eq!(subj_100[0].len(), 1);
        assert_eq!(subj_100[0][0].len(), 4);

        let subj_1000 = FloatOverlay::with_adapter(adapter_1000, shape.len())
            .unsafe_add_source(&shape, ShapeType::Subject)
            .overlay(OverlayRule::Subject, FillRule::NonZero);

        let c100 = &subj_100[0][0];
        let c1000 = &subj_1000[0][0];

        println!("100: {:?}", c100);
        println!("1000: {:?}", c1000);
    }
}