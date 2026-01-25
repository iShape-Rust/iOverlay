use i_shape::source::resource::ShapeResource;
use alloc::vec;
use alloc::vec::Vec;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{ContourDirection, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::OverlayOptions;
use crate::mesh::outline::builder::OutlineBuilder;
use crate::mesh::style::OutlineStyle;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::int_area::IntArea;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::simple::SimplifyContour;
use crate::mesh::overlay::OffsetOverlay;

pub trait OutlineOffset<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Generates an outline shapes for contours, or shapes.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    fn outline(&self, style: &OutlineStyle<T>) -> Shapes<P>;

    /// Generates an outline shapes for contours, or shapes with optional filtering.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    /// - `options`: Adjust custom behavior.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn outline_custom(&self, style: &OutlineStyle<T>, options: OverlayOptions<T>) -> Shapes<P>;
}

impl<S, P, T> OutlineOffset<P, T> for S
where
    S: ShapeResource<P, T>,
    P: FloatPointCompatible<T> + 'static,
    T: FloatNumber + 'static,
{
    fn outline(&self, style: &OutlineStyle<T>) -> Shapes<P> {
        self.outline_custom(style, Default::default())
    }

    fn outline_custom(&self, style: &OutlineStyle<T>, options: OverlayOptions<T>) -> Shapes<P> {
        let (points_count, paths_count) = {
            let mut points_count = 0;
            let mut paths_count = 0;
            for path in self.iter_paths() {
                points_count += path.len();
                paths_count += 1;
            }
            (points_count, paths_count)
        };

        let join = style.join.clone().normalize();

        let outer_builder = OutlineBuilder::new(-style.outer_offset, &join);
        let inner_builder = OutlineBuilder::new(style.inner_offset, &join);

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
            // FloatPointAdapter::with_scale(rect, 1.0) // Debug !!!
        };

        let int_min_area = adapter.sqr_float_to_int(options.min_output_area).max(1);

        let shapes = if paths_count <= 1 {
            // fast solution for a single path

            let path = if let Some(first) = self.iter_paths().next() {
                first
            } else {
                return vec![];
            };

            let area = path.unsafe_int_area(&adapter);
            if area >= -1 {
                // single path must be clock-wised
                return vec![];
            }

            let capacity = outer_builder.capacity(path.len());
            let mut segments = Vec::with_capacity(capacity);
            outer_builder.build(path, &adapter, &mut segments);

            OffsetOverlay::with_segments(segments)
                .build_graph_view_with_solver(Default::default())
                .map(|graph| graph.extract_offset(options.output_direction, int_min_area))
                .unwrap_or_default()
        } else {
            let total_capacity = outer_builder.capacity(points_count);

            let mut overlay = Overlay::new_custom(total_capacity, options.int_with_adapter(&adapter), Default::default());
            let mut offset_overlay = OffsetOverlay::new(128);

            let mut segments = Vec::new();

            for path in self.iter_paths() {
                let area = path.unsafe_int_area(&adapter);
                if area.abs() <= 1 {
                    // ignore degenerate paths
                    continue;
                }

                if area < 0 {
                    let capacity = outer_builder.capacity(path.len());
                    let additional = capacity.saturating_sub(segments.capacity());
                    if additional > 0 {
                        segments.reserve(additional);
                    }
                    segments.clear();

                    outer_builder.build(path, &adapter, &mut segments);

                    offset_overlay.clear();
                    offset_overlay.add_segments(&segments);

                    let shapes = offset_overlay
                        .build_graph_view_with_solver(Default::default())
                        .map(|graph| graph.extract_offset(ContourDirection::CounterClockwise, 0))
                        .unwrap_or_default();

                    overlay.add_shapes(&shapes, ShapeType::Subject);
                } else {
                    // TODO switch to reverse
                    let mut inverted = Vec::with_capacity(path.len());
                    for p in path.iter().rev() {
                        inverted.push(*p);
                    }

                    let capacity = inner_builder.capacity(inverted.len());
                    let additional = capacity.saturating_sub(segments.capacity());
                    if additional > 0 {
                        segments.reserve(additional);
                    }
                    segments.clear();

                    inner_builder.build(&inverted, &adapter, &mut segments);

                    offset_overlay.clear();
                    offset_overlay.add_segments(&segments);

                    let mut shapes = offset_overlay
                        .build_graph_view_with_solver(Default::default())
                        .map(|graph| graph.extract_offset(ContourDirection::CounterClockwise, 0))
                        .unwrap_or_default();

                    for shape in shapes.iter_mut() {
                        for path in shape.iter_mut() {
                            path.reverse();
                        }
                    }

                    overlay.add_shapes(&shapes, ShapeType::Subject);
                }
            }

            overlay.overlay(
                OverlayRule::Subject,
                FillRule::Positive
            )
        };

        if options.clean_result {
            let mut float = shapes.to_float(&adapter);
            if options.preserve_output_collinear {
                float.despike_contour(&adapter);
            } else {
                float.simplify_contour(&adapter);
            }
            float
        } else {
            shapes.to_float(&adapter)
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
use crate::mesh::outline::offset::OutlineOffset;
    use crate::mesh::style::{LineCap, LineJoin, OutlineStyle, StrokeStyle};
    use core::f32::consts::PI;
    use crate::mesh::stroke::offset::StrokeOffset;

    #[test]
    fn test_doc() {
        let shape = vec![
            vec![
                [2.0, 1.0],
                [4.0, 1.0],
                [5.0, 2.0],
                [13.0, 2.0],
                [13.0, 3.0],
                [12.0, 3.0],
                [12.0, 4.0],
                [11.0, 4.0],
                [11.0, 3.0],
                [10.0, 3.0],
                [9.0, 4.0],
                [8.0, 4.0],
                [8.0, 3.0],
                [5.0, 3.0],
                [5.0, 4.0],
                [4.0, 5.0],
                [2.0, 5.0],
                [1.0, 4.0],
                [1.0, 2.0],
            ],
            vec![[2.0, 4.0], [4.0, 4.0], [4.0, 2.0], [2.0, 2.0]],
        ];

        let style = OutlineStyle::new(0.2).line_join(LineJoin::Round(0.1));
        let shapes = shape.outline(&style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 2);
    }

    #[test]
    fn test_triangle_round_corner() {
        let path = [[0.0, 0.0f32], [10.0, 0.0f32], [0.0, 10.0f32]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_reversed_triangle_round_corner() {
        let path = [[0.0, 0.0f32], [0.0, 10.0f32], [10.0, 0.0f32]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_square() {
        let path = [[-5.0, -5.0f32], [5.0, -5.0], [5.0, 5.0], [-5.0, 5.0]];

        let style = OutlineStyle::new(10.0);
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);

        let path = shape.first().unwrap();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_square_offset() {
        let path = [[-5.0, -5.0f32], [5.0, -5.0], [5.0, 5.0], [-5.0, 5.0]];

        let style = OutlineStyle::new(-20.0);
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_rhombus_miter() {
        let path = [[-10.0, 0.0], [0.0, -10.0], [10.0, 0.0], [0.0, 10.0]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Miter(0.01));
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes.first().unwrap().len(), 1);
    }

    #[test]
    fn test_window() {
        let window = vec![
            vec![[-10.0, -10.0], [10.0, -10.0], [10.0, 10.0], [-10.0, 10.0]],
            vec![[-5.0, -5.0], [-5.0, 5.0], [5.0, 5.0], [5.0, -5.0]],
        ];

        let style = OutlineStyle::new(1.0).line_join(LineJoin::Bevel);
        let shapes = window.outline(&style);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 2);
    }

    #[test]
    fn test_float_square_0() {
        let shape = vec![vec![
            [300.0, 300.0], [500.0, 300.0], [500.0, 500.0], [300.0, 500.0]
        ]];

        let style = OutlineStyle::default().outer_offset(50.0).inner_offset(50.0);

        let shapes = shape.outline(&style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);

        let path = shape.first().unwrap();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_infinity_loop_0() {
        let path = [
            [2681.39599938213, 5892784.488998892],
            [5419.06964821636, 5891947.742386343],
            [5419.1446127397, 5891949.316633703],
            [5422.8669123155, 5892027.484991552],
            [5034.8682417375, 5892817.151239874],
            [4804.8188261491, 5892876.799252035],
            [4804.81882805645, 5892876.799253942],
            [4551.3436274034, 5892942.5211854],
            [2681.39599938213, 5892784.488998892],
        ];

        let angle = 10.0f64 / (core::f64::consts::PI / 2.0f64);
        let style = OutlineStyle::new(150.0).line_join(LineJoin::Round(angle));

        if let Some(shape) = path.outline(&style).first() {
            assert!(shape[0].len() < 1_000);
        };
    }

    #[test]
    fn test_infinity_loop_1() {
        let path = [
            [2681.39599938213, 5892876.0],
            [5400.0, 5891947.742386343],
            [5400.0, 5892817.151239874],
            [4804.8188261491, 5892876.799252035],
            [4804.81882805645, 5892876.799253942]
        ];

        let angle = 10.0f64 / (core::f64::consts::PI / 2.0f64);
        let style = OutlineStyle::new(150.0).line_join(LineJoin::Round(angle));

        if let Some(shape) = path.outline(&style).first() {
            assert!(shape[0].len() < 1_000);
        };
    }
}
