#[cfg(test)]
mod tests {
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_shape::base::data::Shape;
    use i_shape::int_shape;
    use i_overlay::core::solver::{Precision, Solver, Strategy};

    #[test]
    fn test_00() {
        let subj: Shape<_> = int_shape![
            [[0i16, 0], [0, 4], [3, -5]],
            [[0, 0], [1, 7], [2, -8]],
            [[0, 0], [4, -4], [5, 7]],
        ];

        let solver = Solver {
            strategy: Strategy::List,
            precision: Precision { start: 0, progression: 1 },
            multithreading: None,
        };

        let mut overlay = Overlay::new_custom(4, Default::default(), solver);
        overlay.add_contours(&subj, ShapeType::Subject);
        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.validate();
            let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_01() {
        let subj: Shape<_> = int_shape![
            [[0i32, 0], [0, 4], [3, -5]],
            [[0, 0], [1, 7], [2, -8]],
            [[0, 0], [4, -4], [5, 7]],
        ];

        let solver = Solver {
            strategy: Strategy::List,
            precision: Precision { start: 0, progression: 1 },
            multithreading: None,
        };

        let mut overlay = Overlay::new_custom(4, Default::default(), solver);
        overlay.add_contours(&subj, ShapeType::Subject);
        if let Some(graph) = overlay.build_graph_view(FillRule::NonZero) {
            graph.validate();
            let result = graph.extract_shapes(OverlayRule::Subject, &mut Default::default());
            assert!(!result.is_empty());
        }
    }
}
