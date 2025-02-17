use crate::mesh::stroke::builder::StrokeBuilder;
use crate::mesh::style::StrokeStyle;
use crate::float::filter::ContourFilter;
use crate::float::source::resource::OverlayResource;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::simple::SimplifyContour;
use crate::core::graph::OverlayGraph;

pub trait StrokeOffset<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Generates a stroke shapes for paths, contours, or shapes.
    ///
    /// - `style`: Defines the stroke properties, including width, line caps, and joins.
    /// - `is_closed_path`: Specifies whether the path is closed (true) or open (false).
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the stroke geometry.
    fn stroke(&self, style: StrokeStyle<P, T>, is_closed_path: bool) -> Shapes<P>;

    /// Generates a stroke mesh for paths, contours, or shapes with optional filtering and scaling.
    ///
    /// - `style`: Defines the stroke properties, including width, line caps, and joins.
    /// - `is_closed_path`: Specifies whether the path is closed (true) or open (false).
    /// - `filter`: Defines optional contour filtering and simplification:
    ///     - `min_area`: Retains only contours with an area larger than this value.
    ///     - `simplify`: If `true`, simplifies contours and removes degenerate edges.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the stroke geometry.
    fn stroke_with_filter(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        filter: ContourFilter<T>,
    ) -> Shapes<P>;
}

impl<S, P, T> StrokeOffset<P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T> + 'static,
    T: FloatNumber + 'static,
{
    fn stroke(&self, style: StrokeStyle<P, T>, is_closed_path: bool) -> Shapes<P> {
        self.stroke_with_filter(style, is_closed_path, ContourFilter { min_area: T::from_float(0.0), simplify: false })
    }

    fn stroke_with_filter(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        filter: ContourFilter<T>,
    ) -> Shapes<P> {
        let mut paths_count = 0;
        let mut points_count = 0;
        for path in self.iter_paths() {
            paths_count += 1;
            points_count += path.len();
        }

        let r = T::from_float(0.5 * style.width.to_f64());
        let builder = StrokeBuilder::new(style);
        let a = builder.additional_offset(r);

        let mut rect = FloatRect::with_iter(self.iter_paths().flatten()).unwrap_or(FloatRect::zero());
        rect.add_offset(a);
        let adapter = FloatPointAdapter::new(rect);

        let ir= adapter.len_float_to_int(r).abs();
        if ir <= 1 {
            // offset is too small
            return vec![];
        }

        let capacity = builder.capacity(paths_count, points_count, is_closed_path);
        let mut segments = Vec::with_capacity(capacity);

        for path in self.iter_paths() {
            builder.build(path, is_closed_path, &adapter, &mut segments);
        }

        let shapes = OverlayGraph::offset_graph_with_solver(segments, Default::default())
            .extract_offset_min_area(0);

        let mut float = shapes.to_float(&adapter);

        if filter.simplify {
            float.simplify_contour(&adapter);
        };

        float
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::mesh::stroke::offset::StrokeOffset;
    use crate::mesh::style::{LineJoin, StrokeStyle};
    use crate::float::filter::ContourFilter;

    #[test]
    fn test_simple() {
        let path = [
            [0.0, 0.0],
            [10.0, 0.0],
        ];

        let style = StrokeStyle::new(2.0);
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_bevel_join() {
        let path = [
            [-10.0, 0.0],
            [0.0, 0.0],
            [0.0, 10.0],
        ];

        let style = StrokeStyle::new(2.0);
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);

        let path = shape.first().unwrap();
        assert_eq!(path.len(), 7);
    }

    #[test]
    fn test_round_join() {
        let path = [
            [-10.0, 0.0],
            [0.0, 0.0],
            [0.0, 10.0],
        ];

        let style = StrokeStyle::new(2.0)
            .line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_miter_join_turn_right() {
        let path = [
            [-6.0, -12.0],
            [ 0.0,  0.0],
            [ 6.0, -12.0],
        ];

        let style = StrokeStyle::new(2.0).line_join(LineJoin::Miter(5.0 * PI / 180.0));
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_simple_closed() {
        let path = [
            [-5.0, -5.0],
            [-5.0,  5.0],
            [ 5.0,  5.0],
            [ 5.0, -5.0],
        ];

        let style = StrokeStyle::new(2.0);

        let shapes = path.stroke_with_filter(style, true, ContourFilter { min_area: 0.0, simplify: false });
        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 2);

        assert_eq!(shape[0].len(), 8);
        assert_eq!(shape[1].len(), 4);
    }
}