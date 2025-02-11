use crate::buffering::stroke::builder::StrokeBuilder;
use crate::buffering::stroke::style::StrokeStyle;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::Overlay;
use crate::core::overlay_rule::OverlayRule;
use crate::float::filter::ContourFilter;
use crate::float::source::resource::OverlayResource;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::simple::SimplifyContour;

pub trait Outline<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn stroke(&self, style: StrokeStyle<P, T>, is_closed_path: bool) -> Shapes<P>;
    fn stroke_with_filter_and_scale(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        filter: ContourFilter<T>,
        scale: f64,
    ) -> Shapes<P>;
}

impl<S, P, T> Outline<P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T> + 'static,
    T: FloatNumber + 'static,
{
    fn stroke(&self, style: StrokeStyle<P, T>, is_closed_path: bool) -> Shapes<P> {
        self.stroke_with_filter_and_scale(style, is_closed_path, ContourFilter { min_area: T::from_float(0.0), simplify: false }, 0.0)
    }

    fn stroke_with_filter_and_scale(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        filter: ContourFilter<T>,
        scale: f64,
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
        let adapter = if scale > 0.0 {
            FloatPointAdapter::with_scale(rect, scale)
        } else {
            FloatPointAdapter::new(rect)
        };

        let capacity = builder.capacity(paths_count, points_count, is_closed_path);
        let mut segments = Vec::with_capacity(capacity);

        for path in self.iter_paths() {
            builder.build(path, is_closed_path, &adapter, &mut segments);
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
    use std::f32::consts::PI;
    use crate::buffering::stroke::outline::Outline;
    use crate::buffering::stroke::style::{LineJoin, StrokeStyle};
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
        // let shapes = path.stroke(style, true);
        let shapes = path.stroke_with_filter_and_scale(style, true, ContourFilter { min_area: 0.0, simplify: false }, 1.0);
        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 2);

        assert_eq!(shape[0].len(), 8);
        assert_eq!(shape[1].len(), 4);
    }
}