use crate::core::fill_rule::FillRule;
use crate::core::overlay::Overlay;
use crate::core::overlay_rule::OverlayRule;
use crate::float::filter::ContourFilter;
use crate::float::simplify::SimplifyShape;
use crate::float::source::resource::OverlayResource;
use crate::mesh::outline::builder::OutlineBuilder;
use crate::mesh::style::OutlineStyle;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::simple::SimplifyContour;

pub trait OutlineOffset<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Generates an outline shapes for contours, or shapes.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    fn outline(&self, style: OutlineStyle<T>) -> Shapes<P>;

    /// Generates an outline shapes for contours, or shapes with optional filtering.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    /// - `filter`: Defines optional contour filtering and simplification:
    ///     - `min_area`: Retains only contours with an area larger than this value.
    ///     - `simplify`: If `true`, simplifies contours and removes degenerate edges.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    fn outline_with_filter(&self, style: OutlineStyle<T>, filter: ContourFilter<T>) -> Shapes<P>;
}

impl<S, P, T> OutlineOffset<P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T> + 'static,
    T: FloatNumber + 'static,
{
    fn outline(&self, style: OutlineStyle<T>) -> Shapes<P> {
        self.outline_with_filter(
            style,
            ContourFilter {
                min_area: T::from_float(0.0),
                simplify: false,
            },
        )
    }

    fn outline_with_filter(&self, style: OutlineStyle<T>, filter: ContourFilter<T>) -> Shapes<P> {
        let r = style.offset;

        if r.to_f64().abs() < 0.000_0001 {
            return self.simplify_shape(FillRule::Positive, filter.min_area)
        }

        let mut points_count = 0;
        for path in self.iter_paths() {
            points_count += path.len();
        }

        let builder = OutlineBuilder::new(style);
        let a = builder.additional_offset(r);

        let mut rect =
            FloatRect::with_iter(self.iter_paths().flatten()).unwrap_or(FloatRect::zero());
        rect.add_offset(a.abs());

        let adapter = FloatPointAdapter::new(rect);

        let capacity = builder.capacity(points_count);
        let mut segments = Vec::with_capacity(capacity);

        for path in self.iter_paths() {
            builder.build(path, &adapter, &mut segments);
        }

        let shapes = Overlay { segments }
            .into_graph(FillRule::Positive)
            .extract_shapes(OverlayRule::Subject);
        let mut float = shapes.to_float(&adapter);

        if filter.simplify {
            float.simplify_contour(&adapter);
        };

        float
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::outline::offset::OutlineOffset;
    use crate::mesh::style::{LineJoin, OutlineStyle};
    use std::f32::consts::PI;

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

        assert_eq!(shapes.len(), 3);
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
}
