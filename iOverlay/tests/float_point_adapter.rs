#[cfg(test)]
mod tests {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::overlay::FloatOverlay;
    use i_shape::source::resource::ShapeResource;

    #[test]
    fn test_adapter_with_rect() {
        let s = 1.0 / 3.0;
        let shape = vec![vec![
            [s * 0.0, s * 0.0],
            [s * 0.0, s * 1.0],
            [s * 1.0, s * 1.0],
            [s * 1.0, s * 0.0],
        ]];

        let adapter_100 = FloatPointAdapter::<_, i32>::new(FloatRect::new(-100.0, 100.0, -100.0, 100.0));
        let adapter_1000 = FloatPointAdapter::<_, i32>::new(FloatRect::new(-1000.0, 1000.0, -1000.0, 1000.0));

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

        let adapter_100 = FloatPointAdapter::<_, i32>::with_scale(buffer_rect, 100.0);
        let adapter_1000 = FloatPointAdapter::<_, i32>::with_scale(buffer_rect, 1000.0);

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
    fn nan_coordinates_do_not_panic() {
        let nan_contour: Vec<[f64; 2]> = vec![[f64::NAN, f64::NAN]];

        let result = FloatOverlay::<[f64; 2]>::from_subj(&nan_contour)
            .overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn nan_in_mixed_contour_does_not_panic() {
        let contour: Vec<[f64; 2]> = vec![[0.0, 0.0], [f64::NAN, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let result =
            FloatOverlay::<[f64; 2]>::from_subj(&contour).overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn infinity_coordinates_do_not_panic() {
        let inf_contour: Vec<[f64; 2]> = vec![[f64::INFINITY, f64::NEG_INFINITY]];

        let result = FloatOverlay::<[f64; 2]>::from_subj(&inf_contour)
            .overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn single_point_degenerate_does_not_panic() {
        let contour: Vec<[f64; 2]> = vec![[1.0, 1.0]];

        let result =
            FloatOverlay::<[f64; 2]>::from_subj(&contour).overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn two_point_degenerate_does_not_panic() {
        let contour: Vec<[f64; 2]> = vec![[1.0, 1.0], [2.0, 2.0]];

        let result =
            FloatOverlay::<[f64; 2]>::from_subj(&contour).overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn all_same_points_degenerate_does_not_panic() {
        let contour: Vec<[f64; 2]> = vec![[1.0, 1.0], [1.0, 1.0], [1.0, 1.0], [1.0, 1.0]];

        let result =
            FloatOverlay::<[f64; 2]>::from_subj(&contour).overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn nan_overlay_with_valid_clip_does_not_panic() {
        let nan_contour: Vec<[f64; 2]> = vec![[f64::NAN, f64::NAN]];
        let valid_contour: Vec<[f64; 2]> = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let result = FloatOverlay::<[f64; 2]>::from_subj_and_clip(&nan_contour, &valid_contour)
            .overlay(OverlayRule::Intersect, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn nan_self_intersection_does_not_panic() {
        let nan_contour: Vec<[f64; 2]> = vec![[f64::NAN, f64::NAN]];

        let result = FloatOverlay::<[f64; 2]>::from_subj_and_clip(&nan_contour, &nan_contour)
            .overlay(OverlayRule::Union, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn f32_nan_coordinates_do_not_panic() {
        let nan_contour: Vec<[f32; 2]> = vec![[f32::NAN, f32::NAN]];

        let result = FloatOverlay::<[f32; 2]>::from_subj(&nan_contour)
            .overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }

    #[test]
    fn shapes_with_nan_contour_does_not_panic() {
        let shapes: Vec<Vec<[f64; 2]>> = vec![
            vec![[f64::NAN, f64::NAN]],
            vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
        ];

        let result =
            FloatOverlay::<[f64; 2]>::from_subj(&shapes).overlay(OverlayRule::Subject, FillRule::NonZero);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
    }

    #[test]
    fn empty_contour_does_not_panic() {
        let contour: Vec<[f64; 2]> = vec![];

        let result =
            FloatOverlay::<[f64; 2]>::from_subj(&contour).overlay(OverlayRule::Subject, FillRule::NonZero);

        assert!(result.is_empty());
    }
}
