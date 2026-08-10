use crate::core::fill_rule::FillRule;
use crate::core::integer::OverlayInt;
use crate::core::overlay::Overlay;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::OverlayOptions;
use crate::float::scale::FixedScaleOverlayError;
use crate::mesh::variable_stroke::builder::VariableStrokeBuilder;
use crate::mesh::variable_stroke::resource::VariableStrokeSource;
use crate::mesh::variable_stroke::style::VariableStrokeStyle;
use alloc::vec;
use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_float::int::number::int::IntNumber;
use i_float::int::number::uint::UIntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_shape::base::data::Shapes;
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::flat::float::FloatFlatContoursBuffer;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::simple::SimplifyContour;

/// Builds round-cap, round-join strokes whose width is stored at each centerline vertex.
pub trait VariableStrokeOffset<P>: VariableStrokeSource<P>
where
    P: FloatPointCompatible + 'static,
{
    fn variable_stroke(&self, style: VariableStrokeStyle<P::Scalar>, is_closed: bool) -> Shapes<P> {
        self.variable_stroke_custom(style, is_closed, Default::default())
    }

    fn variable_stroke_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        self.variable_stroke_custom_into(style, is_closed, Default::default(), output)
    }

    fn variable_stroke_custom(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar>,
    ) -> Shapes<P> {
        self.variable_stroke_custom_as::<i32>(style, is_closed, options)
    }

    fn variable_stroke_custom_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        self.variable_stroke_custom_into_as::<i32>(style, is_closed, options, output)
    }

    fn variable_stroke_fixed_scale(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale(style, is_closed, Default::default(), scale)
    }

    fn variable_stroke_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_into(style, is_closed, Default::default(), scale, output)
    }

    fn variable_stroke_custom_fixed_scale(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_as::<i32>(style, is_closed, options, scale)
    }

    fn variable_stroke_custom_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_into_as::<i32>(style, is_closed, options, scale, output)
    }

    fn variable_stroke_as<I>(&self, style: VariableStrokeStyle<P::Scalar>, is_closed: bool) -> Shapes<P>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_as::<I>(style, is_closed, Default::default())
    }

    fn variable_stroke_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        output: &mut FloatFlatContoursBuffer<P>,
    ) where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_into_as::<I>(style, is_closed, Default::default(), output)
    }

    fn variable_stroke_custom_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar, I>,
    ) -> Shapes<P>
    where
        I: OverlayInt + 'static,
    {
        match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver.build(self, is_closed, options),
            None => vec![],
        }
    }

    fn variable_stroke_custom_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar, I>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) where
        I: OverlayInt + 'static,
    {
        match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver.build_into(self, is_closed, options, output),
            None => output.clear_and_reserve(0, 0),
        }
    }

    fn variable_stroke_fixed_scale_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_fixed_scale_as::<I>(style, is_closed, Default::default(), scale)
    }

    fn variable_stroke_fixed_scale_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_fixed_scale_into_as::<I>(
            style,
            is_closed,
            Default::default(),
            scale,
            output,
        )
    }

    fn variable_stroke_custom_fixed_scale_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar, I>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        let mut solver = match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver,
            None => return Ok(vec![]),
        };
        solver.apply_scale(scale)?;
        Ok(solver.build(self, is_closed, options))
    }

    fn variable_stroke_custom_fixed_scale_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        is_closed: bool,
        options: OverlayOptions<P::Scalar, I>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        let mut solver = match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver,
            None => {
                output.clear_and_reserve(0, 0);
                return Ok(());
            }
        };
        solver.apply_scale(scale)?;
        solver.build_into(self, is_closed, options, output);
        Ok(())
    }
}

impl<S, P> VariableStrokeOffset<P> for S
where
    S: VariableStrokeSource<P>,
    P: FloatPointCompatible + 'static,
{
}

struct VariableStrokeSolver<P: FloatPointCompatible, I: IntNumber> {
    max_radius: P::Scalar,
    builder: VariableStrokeBuilder<P::Scalar>,
    adapter: FloatPointAdapter<P, I>,
    paths_count: usize,
    points_count: usize,
}

impl<P, I> VariableStrokeSolver<P, I>
where
    P: FloatPointCompatible + 'static,
    I: OverlayInt + 'static,
{
    fn prepare<S: VariableStrokeSource<P> + ?Sized>(
        source: &S,
        style: VariableStrokeStyle<P::Scalar>,
    ) -> Option<Self> {
        let mut max_radius = P::Scalar::ZERO;
        let mut paths_count = 0;
        let mut points_count = 0;
        let mut rect: Option<FloatRect<P::Scalar>> = None;

        for path in source.iter_variable_paths() {
            if path.is_empty() {
                continue;
            }
            paths_count += 1;
            points_count += path.len();
            for vertex in path {
                if !vertex.point.x().is_finite() || !vertex.point.y().is_finite() || !vertex.width.is_finite()
                {
                    continue;
                }
                max_radius = max_radius.max(vertex.radius());
                if let Some(rect) = rect.as_mut() {
                    rect.add_point(&vertex.point);
                } else {
                    rect = Some(FloatRect::with_point(vertex.point));
                }
            }
        }

        if paths_count == 0 || points_count < 2 || max_radius <= P::Scalar::ZERO {
            return None;
        }

        let builder = VariableStrokeBuilder::new(style);
        let mut rect = rect?;
        rect.add_offset(builder.additional_offset(max_radius));
        let adapter = FloatPointAdapter::<P, I>::new(rect);

        Some(Self {
            max_radius,
            builder,
            adapter,
            paths_count,
            points_count,
        })
    }

    fn apply_scale(&mut self, scale: P::Scalar) -> Result<(), FixedScaleOverlayError> {
        self.adapter = FloatPointAdapter::try_with_scale(*self.adapter.rect(), scale)?;
        Ok(())
    }

    fn build<S: VariableStrokeSource<P> + ?Sized>(
        self,
        source: &S,
        is_closed: bool,
        options: OverlayOptions<P::Scalar, I>,
    ) -> Shapes<P> {
        if self.radius_is_too_small() {
            return vec![];
        }

        let mut segments = Vec::with_capacity(self.builder.capacity(
            self.paths_count,
            self.points_count,
            is_closed,
        ));
        for path in source.iter_variable_paths() {
            self.builder.build(path, is_closed, &self.adapter, &mut segments);
        }

        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options.int_with_adapter(&self.adapter);
        let shapes = overlay.overlay(OverlayRule::Subject, FillRule::Positive);
        let mut float = shapes.to_float(&self.adapter);

        if options.clean_result {
            if options.preserve_output_collinear {
                float.despike_contour(&self.adapter);
            } else {
                float.simplify_contour(&self.adapter);
            }
        }
        float
    }

    fn build_into<S: VariableStrokeSource<P> + ?Sized>(
        self,
        source: &S,
        is_closed: bool,
        options: OverlayOptions<P::Scalar, I>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        if self.radius_is_too_small() {
            output.clear_and_reserve(0, 0);
            return;
        }

        let mut segments = Vec::with_capacity(self.builder.capacity(
            self.paths_count,
            self.points_count,
            is_closed,
        ));
        for path in source.iter_variable_paths() {
            self.builder.build(path, is_closed, &self.adapter, &mut segments);
        }

        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options.int_with_adapter(&self.adapter);
        let mut int_output = FlatContoursBuffer::<I>::with_capacity(0);
        overlay.overlay_into(OverlayRule::Subject, FillRule::Positive, &mut int_output);

        let iter = int_output
            .points
            .iter()
            .map(|point| self.adapter.int_to_float(point));
        output.set_with_iter(iter, &int_output.ranges);
        if options.clean_result {
            if options.preserve_output_collinear {
                output.despike_contour(&self.adapter);
            } else {
                output.simplify_contour(&self.adapter);
            }
        }
    }

    #[inline]
    fn radius_is_too_small(&self) -> bool {
        let radius = self
            .adapter
            .round_len_to_int(self.max_radius)
            .to_wide()
            .unsigned_abs();
        radius <= I::WideUInt::ONE
    }
}

#[cfg(test)]
mod tests {
    use super::VariableStrokeOffset;
    use crate::mesh::stroke::offset::StrokeOffset;
    use crate::mesh::style::{LineCap, LineJoin, StrokeStyle};
    use crate::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};
    use alloc::vec;
    use i_shape::float::area::Area;

    #[test]
    fn equal_width_builds_round_stroke() {
        let path = vec![
            StrokeVertex::new([0.0, 0.0], 4.0),
            StrokeVertex::new([10.0, 0.0], 4.0),
        ];
        let shapes = path.variable_stroke(VariableStrokeStyle::new(), false);
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
    }

    #[test]
    fn supports_i64_engine() {
        let path = [
            StrokeVertex::new([0.0, 0.0], 2.0),
            StrokeVertex::new([10.0, 0.0], 6.0),
        ];
        let shapes = path.variable_stroke_as::<i64>(VariableStrokeStyle::new(), false);
        assert!(!shapes.is_empty());
    }

    #[test]
    fn covered_section_does_not_panic() {
        let path = [
            StrokeVertex::new([0.0, 0.0], 2.0),
            StrokeVertex::new([2.0, 0.0], 8.0),
        ];
        let shapes = path.variable_stroke(VariableStrokeStyle::new(), false);
        assert!(!shapes.is_empty());
    }

    #[test]
    fn zero_width_is_empty() {
        let path = [
            StrokeVertex::new([0.0, 0.0], 0.0),
            StrokeVertex::new([10.0, 0.0], 0.0),
        ];
        let shapes = path.variable_stroke(VariableStrokeStyle::new(), false);
        assert!(shapes.is_empty());
    }

    #[test]
    fn constant_width_matches_static_round_stroke_area() {
        let points = [[0.0f64, 0.0], [10.0, 0.0], [15.0, 8.0]];
        let path = points.map(|point| StrokeVertex::new(point, 4.0));
        let angle = 0.1;
        let actual = path
            .variable_stroke_fixed_scale(VariableStrokeStyle::new().round_angle(angle), false, 1_000.0)
            .unwrap();
        let expected = points
            .stroke_fixed_scale(
                StrokeStyle::new(4.0)
                    .start_cap(LineCap::Round(angle))
                    .end_cap(LineCap::Round(angle))
                    .line_join(LineJoin::Round(angle)),
                false,
                1_000.0,
            )
            .unwrap();

        let delta = (actual.area() - expected.area()).abs();
        assert!(delta < 0.1, "area delta: {delta}");
    }

    #[test]
    fn covered_edge_adds_cap_not_full_circle() {
        let path = [
            StrokeVertex::new([0.0f64, 0.0], 2.0),
            StrokeVertex::new([2.0, 0.0], 8.0),
        ];
        let shapes = path
            .variable_stroke_fixed_scale(VariableStrokeStyle::new(), false, 1_000.0)
            .unwrap();
        let area = shapes.area();
        let full_circle = core::f64::consts::PI * 16.0;

        assert!(area > 0.0);
        assert!(area < 0.6 * full_circle, "unexpected full-circle area: {area}");
    }

    #[test]
    fn reversing_regular_path_preserves_area() {
        let path = vec![
            StrokeVertex::new([0.0f64, 0.0], 2.0),
            StrokeVertex::new([10.0, 3.0], 7.0),
            StrokeVertex::new([18.0, -2.0], 4.0),
        ];
        let mut reversed = path.clone();
        reversed.reverse();
        let style = VariableStrokeStyle::new().round_angle(0.08);
        let forward = path.variable_stroke_fixed_scale(style, false, 10_000.0).unwrap();
        let backward = reversed
            .variable_stroke_fixed_scale(style, false, 10_000.0)
            .unwrap();

        assert!((forward.area() - backward.area()).abs() < 0.01);
    }

    #[test]
    fn reverse_covered_edge_breaks_from_remaining_chain() {
        let paths = vec![vec![
            StrokeVertex::new([1.41_f32, 5.16_f32], 80.0_f32),
            StrokeVertex::new([27.369_999_f32, 4.11_f32], 6.0_f32),
            StrokeVertex::new([65.0_f32, -20.0_f32], 18.0_f32),
            StrokeVertex::new([105.0_f32, 0.0_f32], 8.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style, false);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn variable_tangent_join_does_not_leave_center_notch() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 6.0_f32),
            StrokeVertex::new([46.829_998_f32, 14.88_f32], 18.0_f32),
            StrokeVertex::new([70.0_f32, 0.0_f32], 42.0_f32),
            StrokeVertex::new([105.0_f32, 25.0_f32], 15.0_f32),
            StrokeVertex::new([140.0_f32, 15.0_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style, false);
        let join = paths[0][1].point;
        let has_center_notch = result
            .iter()
            .flatten()
            .flatten()
            .any(|point| (point[0] - join[0]).abs() < 0.001 && (point[1] - join[1]).abs() < 0.001);

        assert!(!has_center_notch);
    }

    #[test]
    fn reversed_tangent_order_builds_local_join_fan() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 4.5_f32),
            StrokeVertex::new([45.32_f32, 9.559_999_f32], 13.5_f32),
            StrokeVertex::new([91.299_995_f32, 0.89_f32], 31.5_f32),
            StrokeVertex::new([102.88_f32, -2.44_f32], 11.25_f32),
            StrokeVertex::new([140.0_f32, 15.0_f32], 22.5_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style, false);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn near_u_turn_preserves_both_edge_widths() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 6.0_f32),
            StrokeVertex::new([96.93_f32, 0.669_999_96_f32], 18.0_f32),
            StrokeVertex::new([70.0_f32, 0.0_f32], 42.0_f32),
            StrokeVertex::new([105.0_f32, 25.0_f32], 15.0_f32),
            StrokeVertex::new([140.0_f32, 15.0_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style, false);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn wide_turn_preserves_outer_radius() {
        let paths = vec![vec![
            StrokeVertex::new([-11.599_999_f32, 35.16_f32], 6.0_f32),
            StrokeVertex::new([149.599_99_f32, 86.88_f32], 18.0_f32),
            StrokeVertex::new([70.0_f32, 0.0_f32], 42.0_f32),
            StrokeVertex::new([107.149_994_f32, 27.82_f32], 15.0_f32),
            StrokeVertex::new([156.72_f32, -34.079_998_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style, false);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn covered_edge_breaks_from_regular_chain() {
        let path = vec![
            StrokeVertex::new([-20.0_f32, 0.0_f32], 2.0_f32),
            StrokeVertex::new([0.0_f32, 0.0_f32], 10.0_f32),
            StrokeVertex::new([2.0_f32, 0.0_f32], 20.0_f32),
        ];
        let result = path.variable_stroke(VariableStrokeStyle::new(), false);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn covered_last_edge_builds_only_larger_cap() {
        let paths = vec![vec![
            StrokeVertex::new([-55.78_f32, 25.289_999_f32], 6.0_f32),
            StrokeVertex::new([-16.39_f32, 28.369_999_f32], 18.0_f32),
            StrokeVertex::new([36.82_f32, 30.15_f32], 42.0_f32),
            StrokeVertex::new([105.0_f32, 25.0_f32], 15.0_f32),
            StrokeVertex::new([109.49_f32, 24.859_999_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style, false);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn covered_two_point_edge_caps_outside_larger_end() {
        let path = vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 3.48_f32),
            StrokeVertex::new([14.849_999_f32, 0.06_f32], 38.28_f32),
        ];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = path.variable_stroke(style, false);
        let center = path[1].point;
        let vector = [center[0] - path[0].point[0], center[1] - path[0].point[1]];
        let length = (vector[0] * vector[0] + vector[1] * vector[1]).sqrt();
        let direction = [vector[0] / length, vector[1] / length];
        let min_projection = result
            .iter()
            .flatten()
            .flatten()
            .map(|point| (point[0] - center[0]) * direction[0] + (point[1] - center[1]) * direction[1])
            .fold(f32::MAX, f32::min);

        assert!(
            min_projection >= -0.01,
            "covered cap points into the broken edge: projection={min_projection}"
        );
    }
}
