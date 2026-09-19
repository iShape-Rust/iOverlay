#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::vector::edge::{CLIP_LEFT, DataVectorEdge, SUBJ_LEFT};

    #[test]
    fn union_preserves_fill_changes_on_collinear_edges() {
        for redundant_vertex in [false, true] {
            let mut subj = vec![
                IntPoint::new(0, 0),
                IntPoint::new(10, 0),
                IntPoint::new(10, 10),
                IntPoint::new(0, 10),
            ];
            if redundant_vertex {
                // Force simplification of an equal-fill run in the same contour.
                subj.insert(1, IntPoint::new(2, 0));
            }
            let clip = [
                IntPoint::new(5, 0),
                IntPoint::new(15, 0),
                IntPoint::new(15, 10),
                IntPoint::new(5, 10),
            ];
            let mut overlay = Overlay::new(9);
            overlay.options.preserve_input_collinear = true;
            overlay.options.preserve_output_collinear = false;
            overlay.add_contour(&subj, ShapeType::Subject);
            overlay.add_contour(&clip, ShapeType::Clip);

            let shapes = overlay.build_shape_vectors(FillRule::NonZero, OverlayRule::Union);
            assert_eq!(shapes.len(), 1);
            assert_eq!(shapes[0].len(), 1);
            let bottom: Vec<_> = shapes[0][0]
                .iter()
                .filter(|edge| edge.a.y == 0 && edge.b.y == 0)
                .map(|edge| (edge.a.x, edge.b.x, edge.fill))
                .collect();
            assert_eq!(
                bottom,
                vec![
                    (0, 5, SUBJ_LEFT),
                    (5, 10, SUBJ_LEFT | CLIP_LEFT),
                    (10, 15, CLIP_LEFT)
                ],
                "redundant_vertex={redundant_vertex}"
            );
            assert_eq!(shapes[0][0].len(), 8);
        }
    }

    #[test]
    fn test_0() {
        let subj = [
            IntPoint::new(-10240, -10240),
            IntPoint::new(-10240, 10240),
            IntPoint::new(10240, 10240),
            IntPoint::new(10240, -10240),
        ];

        let clip = [
            IntPoint::new(-5120, -5120),
            IntPoint::new(-5120, 5120),
            IntPoint::new(5120, 5120),
            IntPoint::new(5120, -5120),
        ];

        let mut overlay = Overlay::new(2);
        overlay.add_contour(&subj, ShapeType::Subject);
        overlay.add_contour(&clip, ShapeType::Clip);

        let shapes = overlay.build_shape_vectors(FillRule::NonZero, OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);

        let vectors = &shapes[0][0];
        let template = [
            DataVectorEdge {
                a: IntPoint::new(-10240, 10240),
                b: IntPoint::new(-10240, -10240),
                fill: 1,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(-10240, -10240),
                b: IntPoint::new(10240, -10240),
                fill: 1,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(10240, -10240),
                b: IntPoint::new(10240, 10240),
                fill: 1,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(10240, 10240),
                b: IntPoint::new(-10240, 10240),
                fill: 1,
                data: (),
            },
        ];

        assert_eq!(vectors.as_slice(), template.as_slice());
    }

    #[test]
    fn test_1() {
        let subj = [
            IntPoint::new(-10240, -10240),
            IntPoint::new(-10240, 10240),
            IntPoint::new(10240, 10240),
            IntPoint::new(10240, -10240),
        ];

        let clip = [
            IntPoint::new(-5120, -5120),
            IntPoint::new(-5120, 15360),
            IntPoint::new(15360, 15360),
            IntPoint::new(15360, -5120),
        ];

        let mut overlay = Overlay::new(2);
        overlay.add_contour(&subj, ShapeType::Subject);
        overlay.add_contour(&clip, ShapeType::Clip);

        let shapes = overlay.build_shape_vectors(FillRule::NonZero, OverlayRule::Difference);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);

        let vectors = &shapes[0][0];
        let template = [
            DataVectorEdge {
                a: IntPoint::new(-10240, 10240),
                b: IntPoint::new(-10240, -10240),
                fill: 1,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(-10240, -10240),
                b: IntPoint::new(10240, -10240),
                fill: 1,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(10240, -10240),
                b: IntPoint::new(10240, -5120),
                fill: 1,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(10240, -5120),
                b: IntPoint::new(-5120, -5120),
                fill: 11,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(-5120, -5120),
                b: IntPoint::new(-5120, 10240),
                fill: 11,
                data: (),
            },
            DataVectorEdge {
                a: IntPoint::new(-5120, 10240),
                b: IntPoint::new(-10240, 10240),
                fill: 1,
                data: (),
            },
        ];

        assert_eq!(vectors.as_slice(), template.as_slice());
    }
}
