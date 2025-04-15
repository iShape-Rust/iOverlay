use crate::core::fill_rule::FillRule;
use crate::core::graph::OverlayGraph;
use crate::core::overlay::{ContourDirection, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::float::filter::ContourFilter;
use crate::float::source::resource::OverlayResource;
use crate::mesh::outline::builder::OutlineBuilder;
use crate::mesh::style::OutlineStyle;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::area::IntArea;
use i_shape::float::simple::SimplifyContour;

pub trait OutlineOffset<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Generates an outline shapes for contours, or shapes.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    fn outline(&self, style: OutlineStyle<T>) -> Shapes<P>;

    /// Generates an outline shapes for contours, or shapes with optional filtering.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    /// - `main_direction`: Winding direction for the **output** main (outer) contour. All hole contours will automatically use the opposite direction. Impact on **output** only!
    /// - `filter`: Defines optional contour filtering and simplification:
    ///     - `min_area`: Retains only contours with an area larger than this value.
    ///     - `simplify`: If `true`, simplifies contours and removes degenerate edges.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn outline_custom(&self, style: OutlineStyle<T>, main_direction: ContourDirection, filter: ContourFilter<T>) -> Shapes<P>;
}

impl<S, P, T> OutlineOffset<P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T> + 'static,
    T: FloatNumber + 'static,
{
    fn outline(&self, style: OutlineStyle<T>) -> Shapes<P> {
        self.outline_custom(
            style,
            ContourDirection::CounterClockwise,
            ContourFilter {
                min_area: T::from_float(0.0),
                simplify: false,
            },
        )
    }

    fn outline_custom(&self, style: OutlineStyle<T>, main_direction: ContourDirection, filter: ContourFilter<T>) -> Shapes<P> {
        let (points_count, paths_count) = {
            let mut points_count = 0;
            let mut paths_count = 0;
            for path in self.iter_paths() {
                points_count += path.len();
                paths_count += 1;
            }
            (points_count, paths_count)
        };

        let join = style.join.normalize();

        let outer_builder = OutlineBuilder::new(style.outer_offset, &join);
        let inner_builder = OutlineBuilder::new(-style.inner_offset, &join);

        let adapter = {
            let outer_radius = style.outer_offset;
            let inner_radius = style.inner_offset;

            let outer_additional_offset = outer_builder.additional_offset(outer_radius);
            let inner_additional_offset = inner_builder.additional_offset(inner_radius);

            let additional_offset = outer_additional_offset.abs() + inner_additional_offset.abs();

            let mut rect =
                FloatRect::with_iter(self.iter_paths().flatten()).unwrap_or(FloatRect::zero());
            rect.add_offset(additional_offset);

            FloatPointAdapter::new(rect)
        };

        let int_min_area = adapter.sqr_float_to_int(filter.min_area).max(1);

        let shapes = if paths_count <= 1 {
            // fast solution for a single path

            let path = if let Some(first) = self.iter_paths().next() {
                first
            } else {
                return vec![];
            };

            let area = path.unsafe_int_area(&adapter);
            if area <= 1 {
                // single path must be positive ordered
                return vec![];
            }

            let capacity = outer_builder.capacity(path.len());
            let mut segments = Vec::with_capacity(capacity);
            outer_builder.build(path, &adapter, &mut segments);

            OverlayGraph::offset_graph_with_solver(segments, Default::default())
                .extract_offset_min_area(int_min_area)
        } else {
            let total_capacity = outer_builder.capacity(points_count);

            let mut overlay = Overlay::new(total_capacity);

            for path in self.iter_paths() {
                let area = path.unsafe_int_area(&adapter);
                if area.abs() <= 1 {
                    // ignore degenerate paths
                    continue;
                }

                if area > 0 {
                    let capacity = outer_builder.capacity(path.len());
                    let mut segments = Vec::with_capacity(capacity);
                    outer_builder.build(path, &adapter, &mut segments);
                    let shapes = OverlayGraph::offset_graph_with_solver(segments, Default::default())
                        .extract_offset_min_area(0);
                    overlay.add_shapes(&shapes, ShapeType::Subject);
                } else {
                    let mut inverted = Vec::with_capacity(path.len());
                    for p in path.iter().rev() {
                        inverted.push(*p);
                    }

                    let capacity = inner_builder.capacity(inverted.len());
                    let mut segments = Vec::with_capacity(capacity);
                    inner_builder.build(&inverted, &adapter, &mut segments);
                    let mut shapes =
                        OverlayGraph::offset_graph_with_solver(segments, Default::default())
                            .extract_offset_min_area(0);

                    for shape in shapes.iter_mut() {
                        for path in shape.iter_mut() {
                            path.reverse();
                        }
                    }

                    overlay.add_shapes(&shapes, ShapeType::Subject);
                }
            }

            overlay.overlay_custom(
                OverlayRule::Subject,
                FillRule::Positive,
                main_direction,
                int_min_area,
                Default::default(),
            )
        };

        if filter.simplify {
            let mut float = shapes.to_float(&adapter);
            float.simplify_contour(&adapter);
            float
        } else {
            shapes.to_float(&adapter)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::outline::offset::OutlineOffset;
    use crate::mesh::style::{LineJoin, OutlineStyle};
    use std::f32::consts::PI;

    #[test]
    fn test_doc() {
        let shape = vec![
            vec![
                [1.0, 2.0],
                [1.0, 4.0],
                [2.0, 5.0],
                [4.0, 5.0],
                [5.0, 4.0],
                [5.0, 3.0],
                [8.0, 3.0],
                [8.0, 4.0],
                [9.0, 4.0],
                [10.0, 3.0],
                [11.0, 3.0],
                [11.0, 4.0],
                [12.0, 4.0],
                [12.0, 3.0],
                [13.0, 3.0],
                [13.0, 2.0],
                [5.0, 2.0],
                [4.0, 1.0],
                [2.0, 1.0],
            ],
            vec![[2.0, 2.0], [4.0, 2.0], [4.0, 4.0], [2.0, 4.0]],
        ];

        let style = OutlineStyle::new(0.2).line_join(LineJoin::Round(0.1));
        let shapes = shape.outline(style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 2);
    }

    #[test]
    fn test_triangle_round_corner() {
        let path = [[0.0, 0.0f32], [0.0, 10.0f32], [10.0, 0.0f32]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.outline(style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_reversed_triangle_round_corner() {
        let path = [[0.0, 0.0f32], [10.0, 0.0f32], [0.0, 10.0f32]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.outline(style);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_square() {
        let path = [
            [-5.0, -5.0f32],
            [-5.0, 5.0f32],
            [5.0, 5.0f32],
            [5.0, -5.0f32],
        ];

        let style = OutlineStyle::new(10.0);
        let shapes = path.outline(style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);

        let path = shape.first().unwrap();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_square_offset() {
        let path = [
            [-5.0, -5.0f32],
            [-5.0, 5.0f32],
            [5.0, 5.0f32],
            [5.0, -5.0f32],
        ];

        let style = OutlineStyle::new(-20.0);
        let shapes = path.outline(style);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_rhombus_miter() {
        let path = [[-10.0, 0.0], [0.0, 10.0], [10.0, 0.0], [0.0, -10.0]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Miter(0.01));
        let shapes = path.outline(style);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes.first().unwrap().len(), 1);
    }
}
