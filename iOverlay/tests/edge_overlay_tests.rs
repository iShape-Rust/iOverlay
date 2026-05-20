#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_float::int_pnt;
    use i_overlay::core::edge_data::{EdgeDataMerge, OverlayEdgeData};
    use i_overlay::core::edge_overlay::{EdgeOverlay, InputEdge};
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::segm::boolean::ShapeCountBoolean;
    use i_overlay::vector::edge::DataVectorEdge;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Color {
        Red,
        Green,
        None,
        Undefined,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct EdgeColor {
        subj: Color,
        clip: Color,
    }

    impl OverlayEdgeData for EdgeColor {
        fn merge(ctx: EdgeDataMerge<ShapeCountBoolean, Self>) -> Self {
            let subj = if ctx.lhs_data.subj == ctx.rhs_data.subj {
                ctx.lhs_data.subj
            } else if ctx.rhs_count.subj == 0 || ctx.lhs_count.subj == 0 {
                if ctx.rhs_count.subj == ctx.lhs_count.subj {
                    Color::None
                } else if ctx.rhs_count.subj == 0 {
                    ctx.lhs_data.subj
                } else {
                    ctx.rhs_data.subj
                }
            } else {
                Color::Undefined
            };

            let clip = if ctx.lhs_data.clip == ctx.rhs_data.clip {
                ctx.lhs_data.clip
            } else if ctx.rhs_count.clip == 0 || ctx.lhs_count.clip == 0 {
                if ctx.rhs_count.clip == ctx.lhs_count.clip {
                    Color::None
                } else if ctx.rhs_count.clip == 0 {
                    ctx.lhs_data.clip
                } else {
                    ctx.rhs_data.clip
                }
            } else {
                Color::Undefined
            };
            Self { subj, clip }
        }
    }

    #[test]
    fn union_squares() {
        let subj = square(
            0,
            0,
            4,
            4,
            EdgeColor {
                subj: Color::Red,
                clip: Color::None,
            },
        );
        let clip = square(
            4,
            0,
            8,
            4,
            EdgeColor {
                subj: Color::None,
                clip: Color::Green,
            },
        );

        let mut overlay = EdgeOverlay::<i32, EdgeColor>::new(subj.len() + clip.len());
        overlay.add_edges(subj, ShapeType::Subject);
        overlay.add_edges(clip, ShapeType::Clip);

        let shapes = overlay.build_vector_shapes(OverlayRule::Union, FillRule::NonZero);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        let rect = &shapes[0][0];
        assert_eq!(rect.len(), 6);

        let template = vec![
            DataVectorEdge {
                a: int_pnt!(0, 4),
                b: int_pnt!(0, 0),
                fill: 1,
                data: EdgeColor {
                    subj: Color::Red,
                    clip: Color::None,
                },
            },
            DataVectorEdge {
                a: int_pnt!(0, 0),
                b: int_pnt!(4, 0),
                fill: 1,
                data: EdgeColor {
                    subj: Color::Red,
                    clip: Color::None,
                },
            },
            DataVectorEdge {
                a: int_pnt!(4, 0),
                b: int_pnt!(8, 0),
                fill: 4,
                data: EdgeColor {
                    subj: Color::None,
                    clip: Color::Green,
                },
            },
            DataVectorEdge {
                a: int_pnt!(8, 0),
                b: int_pnt!(8, 4),
                fill: 4,
                data: EdgeColor {
                    subj: Color::None,
                    clip: Color::Green,
                },
            },
            DataVectorEdge {
                a: int_pnt!(8, 4),
                b: int_pnt!(4, 4),
                fill: 4,
                data: EdgeColor {
                    subj: Color::None,
                    clip: Color::Green,
                },
            },
            DataVectorEdge {
                a: int_pnt!(4, 4),
                b: int_pnt!(0, 4),
                fill: 1,
                data: EdgeColor {
                    subj: Color::Red,
                    clip: Color::None,
                },
            },
        ];
        assert_eq!(rect, &template);
    }

    #[test]
    fn intersect_squares() {
        let subj = square(
            0,
            0,
            4,
            4,
            EdgeColor {
                subj: Color::Red,
                clip: Color::None,
            },
        );
        let clip = square(
            2,
            0,
            6,
            4,
            EdgeColor {
                subj: Color::None,
                clip: Color::Green,
            },
        );

        let mut overlay = EdgeOverlay::<i32, EdgeColor>::new(subj.len() + clip.len());
        overlay.add_edges(subj, ShapeType::Subject);
        overlay.add_edges(clip, ShapeType::Clip);

        let shapes = overlay.build_vector_shapes(OverlayRule::Intersect, FillRule::NonZero);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        let rect = &shapes[0][0];
        assert_eq!(rect.len(), 4);

        let template = vec![
            DataVectorEdge {
                a: int_pnt!(2, 4),
                b: int_pnt!(2, 0),
                fill: 7,
                data: EdgeColor {
                    subj: Color::None,
                    clip: Color::Green,
                },
            },
            DataVectorEdge {
                a: int_pnt!(2, 0),
                b: int_pnt!(4, 0),
                fill: 5,
                data: EdgeColor {
                    subj: Color::Red,
                    clip: Color::Green,
                },
            },
            DataVectorEdge {
                a: int_pnt!(4, 0),
                b: int_pnt!(4, 4),
                fill: 13,
                data: EdgeColor {
                    subj: Color::Red,
                    clip: Color::None,
                },
            },
            DataVectorEdge {
                a: int_pnt!(4, 4),
                b: int_pnt!(2, 4),
                fill: 5,
                data: EdgeColor {
                    subj: Color::Red,
                    clip: Color::Green,
                },
            },
        ];
        assert_eq!(rect, &template);
    }

    #[test]
    fn union_squares_i64() {
        let subj = square_i64(
            0,
            0,
            4,
            4,
            EdgeColor {
                subj: Color::Red,
                clip: Color::None,
            },
        );
        let clip = square_i64(
            4,
            0,
            8,
            4,
            EdgeColor {
                subj: Color::None,
                clip: Color::Green,
            },
        );

        let mut overlay = EdgeOverlay::<i64, EdgeColor>::new(subj.len() + clip.len());
        overlay.add_edges(subj, ShapeType::Subject);
        overlay.add_edges(clip, ShapeType::Clip);

        let shapes = overlay.build_vector_shapes(OverlayRule::Union, FillRule::NonZero);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 6);
    }

    fn square(x0: i32, y0: i32, x1: i32, y1: i32, data: EdgeColor) -> Vec<InputEdge<i32, EdgeColor>> {
        let points = [
            IntPoint::new(x0, y0),
            IntPoint::new(x1, y0),
            IntPoint::new(x1, y1),
            IntPoint::new(x0, y1),
        ];

        points
            .iter()
            .copied()
            .zip(points.iter().copied().cycle().skip(1))
            .take(points.len())
            .map(|(a, b)| InputEdge { a, b, data })
            .collect()
    }

    fn square_i64(x0: i64, y0: i64, x1: i64, y1: i64, data: EdgeColor) -> Vec<InputEdge<i64, EdgeColor>> {
        let points = [
            IntPoint::new(x0, y0),
            IntPoint::new(x1, y0),
            IntPoint::new(x1, y1),
            IntPoint::new(x0, y1),
        ];

        points
            .iter()
            .copied()
            .zip(points.iter().copied().cycle().skip(1))
            .take(points.len())
            .map(|(a, b)| InputEdge { a, b, data })
            .collect()
    }
}
