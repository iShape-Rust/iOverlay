use crate::buffering::stroke::builder::StrokeBuilder;
use crate::buffering::stroke::style::{LineJoin, StrokeStyle};
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
    fn stroke_with_filter(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        filter: ContourFilter<T>,
    ) -> Shapes<P>;
}

impl<S, P, T> Outline<P, T> for S
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
        let a = match style.join {
            LineJoin::Miter(a) => a.max(r),
            _ => r,
        };
        let mut rect =
            FloatRect::with_iter(self.iter_paths().flatten()).unwrap_or(FloatRect::zero());
        rect.add_offset(a);
        let adapter = FloatPointAdapter::new(rect);

        let builder = StrokeBuilder::new(style);
        let mut segments = Vec::new();

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