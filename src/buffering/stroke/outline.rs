use crate::core::fill_rule::FillRule;
use crate::core::overlay::{Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::float::filter::ContourFilter;
use crate::float::source::resource::OverlayResource;
use crate::segm::segment::Segment;
use crate::segm::winding_count::{ShapeCountBoolean, WindingCount};
use crate::buffering::stroke::style::{LineCap, LineJoin, StrokeStyle};
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::simple::SimplifyContour;

pub trait Outline<P, T: FloatNumber> {
    fn stroke(&self, style: StrokeStyle<T>, is_closed_path: bool) -> Shapes<P>;
    fn stroke_with_filter(
        &self,
        style: StrokeStyle<T>,
        is_closed_path: bool,
        filter: ContourFilter<T>,
    ) -> Shapes<P>;
}

impl<S, P, T> Outline<P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    fn stroke(&self, style: StrokeStyle<T>, is_closed_path: bool) -> Shapes<P> {
        let adapter = style.adapter();
        let capacity = style.calculate_paths_capacity(self.iter_paths(), is_closed_path);
        let segments = style.build_segments(self.iter_paths(), is_closed_path, capacity, &adapter);
        let shapes = Overlay { segments }
            .into_graph(FillRule::Positive)
            .extract_shapes(OverlayRule::Subject);
        shapes.to_float(&adapter)
    }

    fn stroke_with_filter(
        &self,
        style: StrokeStyle<T>,
        is_closed_path: bool,
        filter: ContourFilter<T>,
    ) -> Shapes<P> {
        let adapter = style.adapter();
        let capacity = style.calculate_paths_capacity(self.iter_paths(), is_closed_path);
        let segments = style.build_segments(self.iter_paths(), is_closed_path, capacity, &adapter);
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

impl<T: FloatNumber, P: FloatPointCompatible<T>> StrokeStyle<T> {
    fn adapter(&self) -> FloatPointAdapter<P, T> {
        let r = 0.5 * self.width;
        let a = match self.join {
            LineJoin::Miter(limit) => limit.max(r),
            LineJoin::Round(_) => r,
            LineJoin::Bevel => r,
        };
        let mut rect =
            FloatRect::with_iter(self.iter_paths().flatten()).unwrap_or(FloatRect::zero());
        rect.add_offset(T::from_float(a as f64));
        FloatPointAdapter::new(rect)
    }

    fn calculate_paths_capacity(
        &self,
        iter: impl Iterator<Item = &'_ [P]>,
        is_closed_path: bool,
    ) -> usize {
        let avg_per_join = if let LineJoin::Round(count) = self.join {
            (3 * (count >> 2)) as usize
        } else {
            0
        };

        let additional_cap = if is_closed_path {
            0
        } else {
            let mut caps_capacity = 0;
            if let LineCap::Round(count) = self.begin_cap {
                caps_capacity += count as usize;
            };

            if let LineCap::Round(count) = self.end_cap {
                caps_capacity += count as usize;
            };

            caps_capacity
        };

        let mut capacity = 0;
        for path in iter {
            let n = path.len();
            if n < 2 {
                continue;
            }
            let mut capacity = n << 2; // each segment generate extra 4 points
            capacity += n * avg_per_join;
            capacity += additional_cap;
        }

        capacity
    }

    fn build_segments(
        &self,
        iter: impl Iterator<Item = &'_ [P]>,
        is_closed_path: bool,
        capacity: usize,
        adapter: &FloatPointAdapter<P, T>,
    ) -> Vec<Segment<ShapeCountBoolean>> {
        let (direct, invert) = WindingCount::with_shape_type(ShapeType::Subject);

        let mut segments = Vec::with_capacity(capacity);
        for path in iter {
            if path.len() < 2 {
                continue;
            }

            let mut n0 = Math::normal(path[0], path[1]);

            for [a, b] in path.windows(2) {
                let n = Math::normal(a, b);
            }
        }

        segments
    }
    // Miter(f32),
    // Round(u32),
    // Bevel,
    fn build_bevel_segments(
        &self,
        iter: impl Iterator<Item = &'_ [P]>,
        width: f32,
        is_closed_path: bool,
        capacity: usize,
        adapter: &FloatPointAdapter<P, T>,
    ) -> Vec<Segment<ShapeCountBoolean>> {
        let (direct, invert) = WindingCount::with_shape_type(ShapeType::Subject);

        let mut segments = Vec::with_capacity(capacity);
        let r = T::from_float(0.5 * width as f64);
        for path in iter {
            if path.len() < 2 {
                continue;
            }

            for [a, b] in path.windows(2) {
                let (s0, s1) = Self::create_simple_segments(a, b, r, direct, invert, adapter);
                segments.push(s0);
                segments.push(s1);
            }

            if is_closed_path {
                let a = path.last().unwrap();
                let b = &path[0];
                let (s0, s1) = Self::create_simple_segments(a, b, r, direct, invert, adapter);
                segments.push(s0);
                segments.push(s1);
            }
        }

        segments
    }

}